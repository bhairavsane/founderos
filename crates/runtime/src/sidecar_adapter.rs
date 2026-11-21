use crate::error::RuntimeError;
use crate::embedding_adapter::EmbeddingAdapter;
use async_trait::async_trait;
use founderos_domain::VectorRecord;

pub struct SidecarEmbeddingAdapter {
    sidecar_path: String,
    model_tag: String,
    dimension: usize,
}

impl SidecarEmbeddingAdapter {
    pub fn new(sidecar_path: String, model_tag: String) -> Self {
        Self {
            sidecar_path,
            model_tag,
            dimension: VectorRecord::CANONICAL_DIM,
        }
    }

    pub fn sidecar_path(&self) -> &str {
        &self.sidecar_path
    }

}

#[async_trait]
impl EmbeddingAdapter for SidecarEmbeddingAdapter {
    async fn embed_text(&self, text: &str, _is_query: bool) -> Result<Vec<f32>, RuntimeError> {
        generate_deterministic_vector(text.as_bytes(), self.dimension)
    }

    async fn embed_image(&self, image_bytes: &[u8], _is_query: bool) -> Result<Vec<f32>, RuntimeError> {
        generate_deterministic_vector(image_bytes, self.dimension)
    }

    async fn embed_audio(&self, audio_bytes: &[u8], _is_query: bool) -> Result<Vec<f32>, RuntimeError> {
        generate_deterministic_vector(audio_bytes, self.dimension)
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn model_tag(&self) -> &str {
        &self.model_tag
    }
}

fn generate_deterministic_vector(data: &[u8], dim: usize) -> Result<Vec<f32>, RuntimeError> {
    let mut vec = Vec::with_capacity(dim);
    let mut seed: u64 = 0x517cc1b727220a95;
    for b in data {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(*b as u64 + 1);
    }
    for i in 0..dim {
        seed = seed.wrapping_mul(6364136223846793005).wrapping_add(i as u64 + 1);
        let val = ((seed >> 33) as f32) / ((1u32 << 31) as f32) - 1.0;
        vec.push(val);
    }
    VectorRecord::l2_normalize(&vec).map_err(|e| RuntimeError::Inference(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sidecar_adapter_vector_generation() {
        let adapter = SidecarEmbeddingAdapter::new(
            "/usr/local/bin/llama-sidecar".to_string(),
            "embeddinggemma-2:740m".to_string(),
        );
        let vec = adapter.embed_text("sample founder pitch deck", true).await.unwrap();
        assert_eq!(vec.len(), 768);
        let sum_sq: f32 = vec.iter().map(|x| x * x).sum();
        assert!((sum_sq - 1.0).abs() < 1e-4);
    }
}
