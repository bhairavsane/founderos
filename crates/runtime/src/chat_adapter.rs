use crate::embedding_adapter::to_base64;
use crate::error::RuntimeError;
use crate::types::{OllamaChatMessage, OllamaChatRequest, OllamaChatResponse};
use async_trait::async_trait;
use founderos_domain::Citation;

#[async_trait]
pub trait ChatAdapter: Send + Sync {
    async fn generate_answer(
        &self,
        model: &str,
        query: &str,
        context_chunks: &[String],
        image_attachments: &[Vec<u8>],
        citations: &[Citation],
    ) -> Result<String, RuntimeError>;

    fn is_model_multimodal(&self, model: &str) -> bool;
}

pub struct OllamaChatAdapter {
    base_url: String,
    client: reqwest::Client,
}

impl OllamaChatAdapter {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: reqwest::Client::new(),
        }
    }

    pub fn check_multimodal_heuristic(model: &str) -> bool {
        let lower = model.to_ascii_lowercase();
        lower.contains("vision")
            || lower.contains("llava")
            || lower.contains("minicpm")
            || lower.contains("moondream")
            || lower.contains("qwen-vl")
            || lower.contains("vl")
    }
}

#[async_trait]
impl ChatAdapter for OllamaChatAdapter {
    fn is_model_multimodal(&self, model: &str) -> bool {
        Self::check_multimodal_heuristic(model)
    }

    async fn generate_answer(
        &self,
        model: &str,
        query: &str,
        context_chunks: &[String],
        image_attachments: &[Vec<u8>],
        citations: &[Citation],
    ) -> Result<String, RuntimeError> {
        let is_mm = self.is_model_multimodal(model);
        let mut prompt_content = "You are FounderOS, a private company intelligence assistant.\nAnswer the founder's question using ONLY the provided context below.\n\nCONTEXT:\n".to_string();
        for (i, chunk) in context_chunks.iter().enumerate() {
            prompt_content.push_str(&format!("[Evidence {}]: {}\n", i + 1, chunk));
        }

        let images = if is_mm && !image_attachments.is_empty() {
            Some(image_attachments.iter().map(|b| to_base64(b)).collect())
        } else {
            None
        };

        prompt_content.push_str(&format!("\nQUESTION: {query}\nProvide a concise and grounded answer with precise citations.\n"));

        let req = OllamaChatRequest {
            model: model.to_string(),
            messages: vec![OllamaChatMessage {
                role: "user".to_string(),
                content: prompt_content,
                images,
            }],
            stream: false,
        };

        let url = format!("{}/api/chat", self.base_url);
        let res = self
            .client
            .post(&url)
            .json(&req)
            .send()
            .await
            .map_err(|e| RuntimeError::OllamaConnection(e.to_string()))?;

        if !res.status().is_success() {
            let status = res.status();
            let err_txt = res.text().await.unwrap_or_default();
            return Err(RuntimeError::Inference(format!("Chat error {status}: {err_txt}")));
        }

        let chat_res: OllamaChatResponse = res
            .json()
            .await
            .map_err(|e| RuntimeError::Serialization(e.to_string()))?;

        let mut answer = chat_res.message.content;
        if !citations.is_empty() {
            answer.push_str("\n\n### Sources & Evidence:\n");
            for c in citations {
                answer.push_str(&format!("- {}\n", c.format_citation()));
            }
        }
        Ok(answer)
    }
}
