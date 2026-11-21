use thiserror::Error;

#[derive(Error, Debug)]
pub enum RuntimeError {
    #[error("Ollama connection error: {0}")]
    OllamaConnection(String),

    #[error("Embedding error: {0}")]
    Embedding(String),

    #[error("Model not found: {0}")]
    ModelNotFound(String),

    #[error("Inference error: {0}")]
    Inference(String),

    #[error("Serialization error: {0}")]
    Serialization(String),
}
