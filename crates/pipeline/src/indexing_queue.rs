use crate::error::PipelineError;
use crate::file_discovery::DiscoveredFile;
use crate::ingestion::IngestionPipeline;
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
}

impl IndexingQueue {
    pub fn new(pipeline: Arc<IngestionPipeline>) -> Self {
        Self {
            pipeline,
            cancelled: Arc::new(AtomicBool::new(false)),
            paused: Arc::new(AtomicBool::new(false)),
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

            // Ingest based on file modality
            let path_str = file.path.to_string_lossy().to_string();
            match file.modality {
                founderos_domain::Modality::Text => {
                    let content = std::fs::read_to_string(&file.path).unwrap_or_default();
                    let _ = self.pipeline.ingest_text_document(path_str, file.name, &content).await;
                }
                founderos_domain::Modality::Audio => {
                    let dummy_bytes = std::fs::read(&file.path).unwrap_or_default();
                    let _ = self.pipeline.ingest_audio_recording(path_str, file.name, 30.0, &dummy_bytes, &[]).await;
                }
                _ => {
                    let dummy_bytes = std::fs::read(&file.path).unwrap_or_default();
                    let _ = self.pipeline.ingest_audio_recording(path_str, file.name, 10.0, &dummy_bytes, &[]).await;
                }
            }

            completed += 1;
        }

        Ok(completed)
    }
}
