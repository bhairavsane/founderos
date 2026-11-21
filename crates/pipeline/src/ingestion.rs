use crate::chunking::{AudioChunker, TextChunker};
use crate::error::PipelineError;
use founderos_domain::{Asset, Chunk, Modality, VectorRecord};
use founderos_runtime::EmbeddingAdapter;
use founderos_storage::{SqliteStorage, VectorRepository};
use std::sync::Arc;
use uuid::Uuid;

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

    pub async fn ingest_text_document(
        &self,
        file_path: String,
        file_name: String,
        text_content: &str,
    ) -> Result<Asset, PipelineError> {
        let mut asset = Asset::new(
            file_path,
            file_name,
            "text/plain".to_string(),
            text_content.len() as u64,
            format!("{:x}", text_content.len()),
        );
        self.storage
            .insert_asset(&asset)
            .map_err(|e| PipelineError::Extraction(e.to_string()))?;

        let chunks = self.text_chunker.chunk_text(asset.id, text_content, None);
        let sig = self.vector_repo.active_signature();

        for chunk in &chunks {
            self.storage
                .insert_chunk(chunk)
                .map_err(|e| PipelineError::Extraction(e.to_string()))?;

            if let Some(ref text) = chunk.text_content {
                let vec = self
                    .embedding_adapter
                    .embed_text(text, false)
                    .await
                    .map_err(|e| PipelineError::Extraction(e.to_string()))?;

                let rec = VectorRecord::new(chunk.id, asset.id, Modality::Text, sig, vec)
                    .map_err(|e| PipelineError::Extraction(e.to_string()))?;

                self.vector_repo
                    .insert_vectors(&[rec])
                    .await
                    .map_err(|e| PipelineError::Extraction(e.to_string()))?;
            }
        }

        asset.mark_indexed();
        self.storage
            .insert_asset(&asset)
            .map_err(|e| PipelineError::Extraction(e.to_string()))?;
        Ok(asset)
    }

    pub async fn ingest_image_asset(
        &self,
        file_path: String,
        file_name: String,
        image_bytes: &[u8],
        description: Option<String>,
    ) -> Result<Asset, PipelineError> {
        let mut asset = Asset::new(
            file_path,
            file_name,
            "image/png".to_string(),
            image_bytes.len() as u64,
            format!("{:x}", image_bytes.len()),
        );
        self.storage
            .insert_asset(&asset)
            .map_err(|e| PipelineError::Extraction(e.to_string()))?;

        let chunk = Chunk {
            id: Uuid::new_v4(),
            asset_id: asset.id,
            chunk_index: 0,
            modality: Modality::Image,
            text_content: description,
            page_number: None,
            timestamp_start: None,
            timestamp_end: None,
            metadata: serde_json::json!({}),
        };
        self.storage
            .insert_chunk(&chunk)
            .map_err(|e| PipelineError::Extraction(e.to_string()))?;

        let sig = self.vector_repo.active_signature();
        let vec = self
            .embedding_adapter
            .embed_image(image_bytes, false)
            .await
            .map_err(|e| PipelineError::Extraction(e.to_string()))?;

        let rec = VectorRecord::new(chunk.id, asset.id, Modality::Image, sig, vec)
            .map_err(|e| PipelineError::Extraction(e.to_string()))?;

        self.vector_repo
            .insert_vectors(&[rec])
            .await
            .map_err(|e| PipelineError::Extraction(e.to_string()))?;

        asset.mark_indexed();
        self.storage
            .insert_asset(&asset)
            .map_err(|e| PipelineError::Extraction(e.to_string()))?;
        Ok(asset)
    }

    pub async fn ingest_audio_recording(
        &self,
        file_path: String,
        file_name: String,
        duration_secs: f64,
        raw_audio_bytes: &[u8],
        transcript_segments: &[(f64, f64, String)],
    ) -> Result<Asset, PipelineError> {
        let mut asset = Asset::new(
            file_path,
            file_name,
            "audio/wav".to_string(),
            raw_audio_bytes.len() as u64,
            format!("{:x}", raw_audio_bytes.len()),
        );
        self.storage
            .insert_asset(&asset)
            .map_err(|e| PipelineError::Media(e.to_string()))?;

        let chunks = self.audio_chunker.chunk_audio(asset.id, duration_secs, transcript_segments);
        let sig = self.vector_repo.active_signature();

        for chunk in &chunks {
            self.storage
                .insert_chunk(chunk)
                .map_err(|e| PipelineError::Media(e.to_string()))?;

            let vec = self
                .embedding_adapter
                .embed_audio(raw_audio_bytes, false)
                .await
                .map_err(|e| PipelineError::Media(e.to_string()))?;

            let rec = VectorRecord::new(chunk.id, asset.id, Modality::Audio, sig, vec)
                .map_err(|e| PipelineError::Media(e.to_string()))?;

            self.vector_repo
                .insert_vectors(&[rec])
                .await
                .map_err(|e| PipelineError::Media(e.to_string()))?;
        }

        asset.mark_indexed();
        self.storage
            .insert_asset(&asset)
            .map_err(|e| PipelineError::Media(e.to_string()))?;
        Ok(asset)
    }
}
