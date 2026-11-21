use crate::error::RuntimeError;
use crate::types::{OllamaEmbedRequest, OllamaEmbedResponse};
use async_trait::async_trait;
use founderos_domain::VectorRecord;

pub fn to_base64(data: &[u8]) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(data.len().div_ceil(3) * 4);
    for chunk in data.chunks(3) {
        let b = match chunk.len() {
            3 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8) | (chunk[2] as u32),
            2 => ((chunk[0] as u32) << 16) | ((chunk[1] as u32) << 8),
            1 => (chunk[0] as u32) << 16,
            _ => unreachable!(),
        };
        out.push(CHARSET[((b >> 18) & 63) as usize] as char);
        out.push(CHARSET[((b >> 12) & 63) as usize] as char);
        if chunk.len() > 1 {
            out.push(CHARSET[((b >> 6) & 63) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(CHARSET[(b & 63) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

#[async_trait]
pub trait EmbeddingAdapter: Send + Sync {
    async fn embed_text(&self, text: &str, is_query: bool) -> Result<Vec<f32>, RuntimeError>;
    async fn embed_image(&self, image_bytes: &[u8], is_query: bool) -> Result<Vec<f32>, RuntimeError>;
    async fn embed_audio(&self, audio_bytes: &[u8], is_query: bool) -> Result<Vec<f32>, RuntimeError>;
    fn dimension(&self) -> usize;
    fn model_tag(&self) -> &str;
}

pub struct OllamaEmbeddingAdapter {
    base_url: String,
    model_tag: String,
    dimension: usize,
    client: reqwest::Client,
}

impl OllamaEmbeddingAdapter {
    pub fn new(base_url: String, model_tag: String) -> Self {
        Self {
            base_url,
            model_tag,
            dimension: VectorRecord::CANONICAL_DIM,
            client: reqwest::Client::new(),
        }
    }

    fn format_prefix(content: &str, is_query: bool) -> String {
        if is_query {
            format!("task: search query | {content}")
        } else {
            format!("task: search result | {content}")
        }
    }

    async fn call_embed(&self, input_val: serde_json::Value) -> Result<Vec<f32>, RuntimeError> {
        let url = format!("{}/api/embed", self.base_url);
        let req = OllamaEmbedRequest {
            model: self.model_tag.clone(),
            input: input_val,
        };
        let res = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| RuntimeError::OllamaConnection(e.to_string()))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(RuntimeError::Inference(format!("Ollama error {status}: {text}")));
        }

        let parsed: OllamaEmbedResponse = res
            .json()
            .await
            .map_err(|e| RuntimeError::Serialization(e.to_string()))?;

        parsed
            .embeddings
            .into_iter()
            .next()
            .ok_or_else(|| RuntimeError::Inference("Empty embedding returned".to_string()))
    }
}

#[async_trait]
impl EmbeddingAdapter for OllamaEmbeddingAdapter {
    async fn embed_text(&self, text: &str, is_query: bool) -> Result<Vec<f32>, RuntimeError> {
        let prefixed = Self::format_prefix(text, is_query);
        self.call_embed(serde_json::Value::String(prefixed)).await
    }

    async fn embed_image(&self, image_bytes: &[u8], _is_query: bool) -> Result<Vec<f32>, RuntimeError> {
        let b64 = to_base64(image_bytes);
        let payload = serde_json::json!([{ "image": b64 }]);
        self.call_embed(payload).await
    }

    async fn embed_audio(&self, audio_bytes: &[u8], _is_query: bool) -> Result<Vec<f32>, RuntimeError> {
        let b64 = to_base64(audio_bytes);
        let payload = serde_json::json!([{ "audio": b64 }]);
        self.call_embed(payload).await
    }

    fn dimension(&self) -> usize {
        self.dimension
    }

    fn model_tag(&self) -> &str {
        &self.model_tag
    }
}
