use crate::chunking::{AudioChunker, TextChunker};
use crate::document_parser::{ParsedDocument, ParsedPage};
use crate::error::PipelineError;
use founderos_domain::{Asset, Chunk, Modality, VectorRecord};
use founderos_runtime::EmbeddingAdapter;
use founderos_storage::{SqliteStorage, VectorRepository};
use std::sync::Arc;
use uuid::Uuid;

pub type VideoAudioTrack<'a> = (&'a [u8], &'a [(f64, f64, String)], f64);

pub struct IngestionPipeline {
    storage: SqliteStorage,
    vector_repo: Arc<dyn VectorRepository>,
    embedding_adapter: Arc<dyn EmbeddingAdapter>,
    text_chunker: TextChunker,
    audio_chunker: AudioChunker,
}

impl IngestionPipeline {
    pub fn new(
        storage: SqliteStorage,
        vector_repo: Arc<dyn VectorRepository>,
        embedding_adapter: Arc<dyn EmbeddingAdapter>,
    ) -> Self {
        Self {
            storage,
            vector_repo,
            embedding_adapter,
            text_chunker: TextChunker::default(),
            audio_chunker: AudioChunker::default(),
        }
    }

    async fn store_chunk_and_vec(&self, chunk: &Chunk, vec: Vec<f32>, m: Modality) -> Result<(), PipelineError> {
        self.storage
            .insert_chunk(chunk)
            .map_err(|e| PipelineError::Extraction(e.to_string()))?;
        let sig = self.vector_repo.active_signature();
        let rec = VectorRecord::new(chunk.id, chunk.asset_id, m, sig, vec)
            .map_err(|e| PipelineError::Extraction(e.to_string()))?;
        self.vector_repo
            .insert_vectors(&[rec])
            .await
            .map_err(|e| PipelineError::Extraction(e.to_string()))
    }

    fn persist_asset(&self, asset: &mut Asset, done: bool) -> Result<(), PipelineError> {
        if done {
            asset.mark_indexed();
        }
        self.storage
            .insert_asset(asset)
            .map_err(|e| PipelineError::Extraction(e.to_string()))
    }

    pub async fn ingest_text_document(
        &self,
        path: String,
        name: String,
        text: &str,
    ) -> Result<Asset, PipelineError> {
        let doc = ParsedDocument {
            file_name: name,
            pages: vec![ParsedPage {
                page_number: 1,
                text: text.to_string(),
            }],
        };
        self.ingest_document_asset(path, "text/plain".into(), &doc).await
    }

    pub async fn ingest_document_asset(
        &self,
        path: String,
        mime: String,
        doc: &ParsedDocument,
    ) -> Result<Asset, PipelineError> {
        let total: usize = doc.pages.iter().map(|p| p.text.len()).sum();
        let mut asset = Asset::new(path, doc.file_name.clone(), mime, total as u64, format!("{total:x}"));
        self.persist_asset(&mut asset, false)?;

        for page in &doc.pages {
            let chunks = self.text_chunker.chunk_text(asset.id, &page.text, Some(page.page_number as u32));
            for chunk in &chunks {
                if let Some(ref text) = chunk.text_content {
                    let vec = self.embedding_adapter.embed_text(text, false).await
                        .map_err(|e| PipelineError::Extraction(e.to_string()))?;
                    self.store_chunk_and_vec(chunk, vec, Modality::Text).await?;
                }
            }
        }
        self.persist_asset(&mut asset, true)?;
        Ok(asset)
    }

    pub async fn ingest_image_asset(
        &self,
        path: String,
        name: String,
        bytes: &[u8],
        desc: Option<String>,
    ) -> Result<Asset, PipelineError> {
        let mut asset = Asset::new(path, name, "image/png".into(), bytes.len() as u64, format!("{:x}", bytes.len()));
        self.persist_asset(&mut asset, false)?;

        let chunk = Chunk {
            id: Uuid::new_v4(),
            asset_id: asset.id,
            chunk_index: 0,
            modality: Modality::Image,
            text_content: desc,
            page_number: None,
            timestamp_start: None,
            timestamp_end: None,
            metadata: serde_json::json!({}),
        };
        let vec = self.embedding_adapter.embed_image(bytes, false).await
            .map_err(|e| PipelineError::Extraction(e.to_string()))?;
        self.store_chunk_and_vec(&chunk, vec, Modality::Image).await?;

        self.persist_asset(&mut asset, true)?;
        Ok(asset)
    }

    pub async fn ingest_audio_recording(
        &self,
        path: String,
        name: String,
        dur: f64,
        bytes: &[u8],
        transcripts: &[(f64, f64, String)],
    ) -> Result<Asset, PipelineError> {
        let mut asset = Asset::new(path, name, "audio/wav".into(), bytes.len() as u64, format!("{:x}", bytes.len()));
        self.persist_asset(&mut asset, false)?;

        let chunks = self.audio_chunker.chunk_audio(asset.id, dur, transcripts);
        for chunk in &chunks {
            let vec = self.embedding_adapter.embed_audio(bytes, false).await
                .map_err(|e| PipelineError::Media(e.to_string()))?;
            self.store_chunk_and_vec(chunk, vec, Modality::Audio).await?;
        }
        self.persist_asset(&mut asset, true)?;
        Ok(asset)
    }

    pub async fn ingest_video_asset(
        &self,
        path: String,
        name: String,
        keyframes: &[(f64, Vec<u8>)],
        audio_track: Option<VideoAudioTrack<'_>>,
    ) -> Result<Asset, PipelineError> {
        let mut asset = Asset::new(path, name, "video/mp4".into(), keyframes.len() as u64 * 1024, format!("{:x}", keyframes.len()));
        self.persist_asset(&mut asset, false)?;

        for (idx, (ts, img)) in keyframes.iter().enumerate() {
            let chunk = Chunk {
                id: Uuid::new_v4(),
                asset_id: asset.id,
                chunk_index: idx as u32,
                modality: Modality::Video,
                text_content: Some(format!("Keyframe at {:.1}s", ts)),
                page_number: None,
                timestamp_start: Some(*ts),
                timestamp_end: Some(*ts + 2.0),
                metadata: serde_json::json!({ "timestamp": ts }),
            };
            let vec = self.embedding_adapter.embed_image(img, false).await
                .map_err(|e| PipelineError::Media(e.to_string()))?;
            self.store_chunk_and_vec(&chunk, vec, Modality::Video).await?;
        }

        if let Some((audio_bytes, transcripts, dur)) = audio_track {
            let chunks = self.audio_chunker.chunk_audio(asset.id, dur, transcripts);
            for chunk in &chunks {
                let vec = self.embedding_adapter.embed_audio(audio_bytes, false).await
                    .map_err(|e| PipelineError::Media(e.to_string()))?;
                self.store_chunk_and_vec(chunk, vec, Modality::Audio).await?;
            }
        }
        self.persist_asset(&mut asset, true)?;
        Ok(asset)
    }
}
