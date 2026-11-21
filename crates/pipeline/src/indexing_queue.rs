use crate::document_parser::DocumentParser;
use crate::error::PipelineError;
use crate::file_discovery::DiscoveredFile;
use crate::ingestion::IngestionPipeline;
use founderos_domain::Modality;
use serde::{Deserialize, Serialize};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexingProgress {
    pub file_name: String,
    pub file_path: String,
    pub current_file: usize,
    pub total_files: usize,
    pub stage: String,
}

pub struct IndexingQueue {
    pipeline: Arc<IngestionPipeline>,
    cancelled: Arc<AtomicBool>,
    paused: Arc<AtomicBool>,
    parser: DocumentParser,
}

impl IndexingQueue {
    pub fn new(pipeline: Arc<IngestionPipeline>) -> Self {
        Self {
            pipeline,
            cancelled: Arc::new(AtomicBool::new(false)),
            paused: Arc::new(AtomicBool::new(false)),
            parser: DocumentParser,
        }
    }

    pub fn cancel(&self) {
        self.cancelled.store(true, Ordering::SeqCst);
    }

    pub fn set_paused(&self, paused: bool) {
        self.paused.store(paused, Ordering::SeqCst);
    }

    pub async fn process_batch(
        &self,
        files: Vec<DiscoveredFile>,
        progress_tx: Option<mpsc::UnboundedSender<IndexingProgress>>,
    ) -> Result<usize, PipelineError> {
        let total = files.len();
        let mut completed = 0;

        for (idx, file) in files.into_iter().enumerate() {
            if self.cancelled.load(Ordering::SeqCst) {
                break;
            }

            while self.paused.load(Ordering::SeqCst) {
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                if self.cancelled.load(Ordering::SeqCst) {
                    return Ok(completed);
                }
            }

            if let Some(ref tx) = progress_tx {
                let _ = tx.send(IndexingProgress {
                    file_name: file.name.clone(),
                    file_path: file.path.to_string_lossy().to_string(),
                    current_file: idx + 1,
                    total_files: total,
                    stage: "embedding".to_string(),
                });
            }

            self.ingest_discovered_file(&file).await;
            completed += 1;
        }

        Ok(completed)
    }

    async fn ingest_discovered_file(&self, file: &DiscoveredFile) {
        let path_str = file.path.to_string_lossy().to_string();
        match file.modality {
            Modality::Text => {
                if let Ok(doc) = self.parser.parse_file(&file.path) {
                    let _ = self.pipeline.ingest_document_asset(path_str, file.mime_type.clone(), &doc).await;
                } else {
                    let text = std::fs::read_to_string(&file.path).unwrap_or_default();
                    let _ = self.pipeline.ingest_text_document(path_str, file.name.clone(), &text).await;
                }
            }
            Modality::Image => {
                let bytes = std::fs::read(&file.path).unwrap_or_default();
                let _ = self.pipeline.ingest_image_asset(path_str, file.name.clone(), &bytes, Some(file.name.clone())).await;
            }
            Modality::Audio => {
                let bytes = std::fs::read(&file.path).unwrap_or_default();
                let _ = self.pipeline.ingest_audio_recording(path_str, file.name.clone(), 30.0, &bytes, &[]).await;
            }
            Modality::Video => {
                let keyframes = vec![(0.0, vec![0u8; 128])];
                let _ = self.pipeline.ingest_video_asset(path_str, file.name.clone(), &keyframes, None).await;
            }
        }
    }
}
