pub mod chat_adapter;
pub mod embedding_adapter;
pub mod error;
pub mod sidecar_adapter;
pub mod types;

pub use chat_adapter::{ChatAdapter, OllamaChatAdapter};
pub use embedding_adapter::{to_base64, EmbeddingAdapter, OllamaEmbeddingAdapter};
pub use error::RuntimeError;
pub use sidecar_adapter::SidecarEmbeddingAdapter;
pub use types::{ModelInfo, OllamaChatMessage, OllamaChatRequest, OllamaEmbedRequest};
