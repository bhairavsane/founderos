pub mod commands;
pub mod dto;

use commands::*;
use founderos_domain::VectorSignature;
use founderos_pipeline::HybridRetriever;
use founderos_runtime::DeterministicSidecarProvider;
use founderos_storage::{InMemoryVectorStore, SqliteStorage};
use std::sync::Arc;

pub fn run() {
    let sig = VectorSignature::canonical("metal", "canonical_v1");
    let storage = SqliteStorage::open_in_memory().expect("Failed to open SQLite");
    let vector_repo = Arc::new(InMemoryVectorStore::new(sig));
    let provider = Arc::new(DeterministicSidecarProvider::new("metal"));
    let pipeline = Arc::new(founderos_pipeline::IngestionPipeline::new(
        storage.clone(),
        vector_repo.clone(),
        provider.clone(),
    ));
    let retriever = Arc::new(HybridRetriever::new(storage, vector_repo, provider));

    tauri::Builder::default()
        .manage(retriever)
        .manage(pipeline)
        .invoke_handler(tauri::generate_handler![
            search,
            ask,
            list_assets,
            get_model_status,
            list_installed_models,
            download_model,
            open_source_file,
            index_directory
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
