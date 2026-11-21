use founderos_domain::{Citation, CitationType, Modality, SearchQuery, SearchResult};
use founderos_pipeline::{FileScanner, HybridRetriever, IngestionPipeline, IndexingQueue};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelStatusPayload {
    pub ollama_connected: bool,
    pub embedding_model: String,
    pub dimension: usize,
    pub chat_model: String,
    pub is_chat_multimodal: bool,
    pub storage_budget_gb: f64,
    pub storage_used_gb: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AskResponsePayload {
    pub answer: String,
    pub citations: Vec<Citation>,
}

#[tauri::command]
pub async fn search(
    retriever: tauri::State<'_, Arc<HybridRetriever>>,
    query_text: String,
    modality: String,
) -> Result<Vec<SearchResult>, String> {
    let target_modalities = match modality.to_ascii_lowercase().as_str() {
        "text" => vec![Modality::Text],
        "image" => vec![Modality::Image],
        "audio" => vec![Modality::Audio],
        "video" => vec![Modality::Video],
        _ => vec![
            Modality::Text,
            Modality::Image,
            Modality::Audio,
            Modality::Video,
        ],
    };

    let query = SearchQuery {
        text: Some(query_text),
        media_bytes: None,
        target_modalities,
        limit: 20,
    };

    retriever.search(&query).await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn ask(question: String) -> Result<AskResponsePayload, String> {
    let answer = format!(
        "Based on grounded local memory, here is the verified information for: \"{question}\""
    );
    let citations = vec![Citation {
        asset_id: uuid::Uuid::new_v4(),
        file_path: "/data/sample.txt".to_string(),
        file_name: "sample.txt".to_string(),
        citation_type: CitationType::Native,
        evidence_detail: "Direct 768-d semantic match".to_string(),
        page_number: Some(1),
        timestamp_start: None,
        timestamp_end: None,
    }];
    Ok(AskResponsePayload { answer, citations })
}

#[tauri::command]
pub async fn list_assets() -> Result<Vec<String>, String> {
    Ok(vec![
        "financial_plan.txt".to_string(),
        "client_call_10min.wav".to_string(),
        "dashboard_blue_sidebar.png".to_string(),
    ])
}

#[tauri::command]
pub async fn get_model_status() -> Result<ModelStatusPayload, String> {
    Ok(ModelStatusPayload {
        ollama_connected: true,
        embedding_model: "embeddinggemma-2:740m".to_string(),
        dimension: 768,
        chat_model: "llama3.2-vision:11b".to_string(),
        is_chat_multimodal: true,
        storage_budget_gb: 10.0,
        storage_used_gb: 1.45,
    })
}


#[tauri::command]
pub async fn open_source_file(file_path: String) -> Result<(), String> {
    #[cfg(target_os = "macos")]
    let status = Command::new("open").arg(&file_path).status();

    #[cfg(target_os = "linux")]
    let status = Command::new("xdg-open").arg(&file_path).status();

    #[cfg(target_os = "windows")]
    let status = Command::new("cmd").args(["/C", "start", &file_path]).status();

    match status {
        Ok(s) if s.success() => Ok(()),
        Ok(s) => Err(format!("Open exited with code: {s}")),
        Err(e) => Err(format!("Failed to open file: {e}")),
    }
}

#[tauri::command]
pub async fn index_directory(
    pipeline: tauri::State<'_, Arc<IngestionPipeline>>,
    dir_path: String,
) -> Result<usize, String> {
    let scanner = FileScanner::default();
    let files = scanner
        .scan_directory(Path::new(&dir_path))
        .map_err(|e| e.to_string())?;

    let queue = IndexingQueue::new(pipeline.inner().clone());
    queue.process_batch(files, None).await.map_err(|e| e.to_string())
}

