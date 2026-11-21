use crate::error::RuntimeError;
use async_trait::async_trait;
use founderos_domain::{Modality, VectorSignature};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskType {
    SearchQuery,
    DocumentChunk,
    Classification,
}

impl TaskType {
    pub fn as_prefix(&self) -> &'static str {
        match self {
            Self::SearchQuery => "task: search query",
            Self::DocumentChunk => "task: search document",
            Self::Classification => "task: classification",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum HealthStatus {
    Healthy { memory_bytes: u64 },
    Degraded { reason: String },
    Down { error: String },
}

#[async_trait]
pub trait InferenceProvider: Send + Sync {
    async fn embed(&self, input: &[u8], modality: Modality, task: TaskType) -> Result<Vec<f32>, RuntimeError>;
    async fn embed_text(&self, text: &str, task: TaskType) -> Result<Vec<f32>, RuntimeError> {
        self.embed(text.as_bytes(), Modality::Text, task).await
    }
    async fn embed_media(&self, data: &[u8], modality: Modality, task: TaskType) -> Result<Vec<f32>, RuntimeError> {
        self.embed(data, modality, task).await
    }
    fn capabilities(&self) -> Vec<Modality>;
    async fn health(&self) -> Result<HealthStatus, RuntimeError>;
    fn identity(&self) -> VectorSignature;
}

pub struct DeterministicSidecarProvider {
    signature: VectorSignature,
}

impl DeterministicSidecarProvider {
    pub fn new(backend: &str) -> Self {
        Self {
            signature: VectorSignature::canonical(backend, "canonical_v1"),
        }
    }
}

#[async_trait]
impl InferenceProvider for DeterministicSidecarProvider {
    async fn embed(&self, input: &[u8], _modality: Modality, task: TaskType) -> Result<Vec<f32>, RuntimeError> {
        let prefix = task.as_prefix().as_bytes();
        let mut combined = Vec::with_capacity(prefix.len() + input.len());
        combined.extend_from_slice(prefix);
        combined.extend_from_slice(input);

        let mut vec = Vec::with_capacity(self.signature.dimension);
        let mut seed: u64 = 0x517cc1b727220a95;
        for b in &combined {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(*b as u64 + 1);
        }
        for i in 0..self.signature.dimension {
            seed = seed.wrapping_mul(6364136223846793005).wrapping_add(i as u64 + 1);
            let val = ((seed >> 33) as f32) / ((1u32 << 31) as f32) - 1.0;
            vec.push(val);
        }

        founderos_domain::VectorRecord::l2_normalize(&vec)
            .map_err(|e| RuntimeError::Embedding(e.to_string()))
    }

    fn capabilities(&self) -> Vec<Modality> {
        vec![Modality::Text, Modality::Image, Modality::Audio, Modality::Video]
    }

    async fn health(&self) -> Result<HealthStatus, RuntimeError> {
        Ok(HealthStatus::Healthy {
            memory_bytes: 567 * 1024 * 1024, // 567 MB reported by Google
        })
    }

    fn identity(&self) -> VectorSignature {
        self.signature.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_inference_provider_embed_and_identity() {
        let provider = DeterministicSidecarProvider::new("metal");
        let sig = provider.identity();
        assert_eq!(sig.dimension, 768);
        assert_eq!(sig.text_quantization, "Q8_0");

        let vec1 = provider.embed(b"Cap Table", Modality::Text, TaskType::DocumentChunk).await.unwrap();
        assert_eq!(vec1.len(), 768);

        // Verify task prefix creates distinct embedding
        let vec2 = provider.embed(b"Cap Table", Modality::Text, TaskType::SearchQuery).await.unwrap();
        assert_ne!(vec1, vec2);
    }
}
