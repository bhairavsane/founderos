pub mod commands;

use commands::*;
use founderos_domain::VectorSignature;
use founderos_pipeline::HybridRetriever;
use founderos_runtime::SidecarEmbeddingAdapter;
use founderos_storage::{InMemoryVectorStore, SqliteStorage};
use std::sync::Arc;

pub fn run() {
    let sig = VectorSignature {
        model_tag: "embeddinggemma-2:740m".to_string(),
        quantization: "q8_0".to_string(),
        prefix_config: "v1_prefixed".to_string(),
        dimension: 768,
    };
    let storage = SqliteStorage::open_in_memory().expect("Failed to open SQLite");
    let vector_repo = Arc::new(InMemoryVectorStore::new(sig));
    let embedding_adapter = Arc::new(SidecarEmbeddingAdapter::new(
        "sidecar".to_string(),
        "embeddinggemma-2:740m".to_string(),
    ));
    let retriever = Arc::new(HybridRetriever::new(storage, vector_repo, embedding_adapter));

    tauri::Builder::default()
        .manage(retriever)
        .invoke_handler(tauri::generate_handler![
            search,
            ask,
            list_assets,
            get_model_status,
            open_source_file
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
