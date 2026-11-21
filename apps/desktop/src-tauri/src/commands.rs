use crate::dto::*;
use founderos_domain::{Modality, SearchQuery};
use founderos_pipeline::{FileScanner, HybridRetriever, IngestionPipeline, IndexingQueue};
use std::path::Path;
use std::process::Command;
use std::sync::Arc;

#[tauri::command]
pub async fn search(
    retriever: tauri::State<'_, Arc<HybridRetriever>>,
    query_text: String,
    modality: String,
) -> Result<Vec<SearchResultDto>, String> {
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

    let list = retriever.search(&query).await.map_err(|e| e.to_string())?;
    Ok(list
        .into_iter()
        .map(|r| SearchResultDto {
            id: r.chunk_id.to_string(),
            asset_id: r.asset_id.to_string(),
            file_name: r.file_name,
            file_path: r.file_path,
            modality: r.modality,
            snippet: r.snippet,
            timestamp_or_page: r.timestamp_or_page,
            score: r.score,
            match_type: r.match_type,
        })
        .collect())
}

#[tauri::command]
pub async fn ask(
    retriever: tauri::State<'_, Arc<HybridRetriever>>,
    question: String,
) -> Result<AskResponsePayload, String> {
    let rag = founderos_pipeline::RAGService::new(retriever.inner().clone());
    let grounded = rag.ask(&question).await.map_err(|e| e.to_string())?;
    let citations = grounded
        .citations
        .into_iter()
        .map(|c| {
            let loc = match (c.page_number, c.timestamp_start, c.timestamp_end) {
                (Some(p), _, _) => Some(format!("Page {p}")),
                (_, Some(s), Some(e)) => {
                    let s_min = (s / 60.0).floor() as u32;
                    let s_sec = (s % 60.0).floor() as u32;
                    let e_min = (e / 60.0).floor() as u32;
                    let e_sec = (e % 60.0).floor() as u32;
                    Some(format!("{s_min:02}:{s_sec:02} - {e_min:02}:{e_sec:02}"))
                }
                _ => None,
            };
            CitationDto {
                file_name: c.file_name,
                file_path: c.file_path,
                citation_type: c.citation_type,
                evidence: c.evidence_detail,
                timestamp_or_page: loc,
            }
        })
        .collect();
    Ok(AskResponsePayload {
        answer: grounded.answer,
        citations,
    })
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
pub async fn list_installed_models() -> Result<Vec<InstalledModelInfo>, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let models_dir = std::path::PathBuf::from(home).join(".founderos").join("models");
    let downloader = founderos_runtime::ModelDownloader::new(models_dir);
    let manifest = downloader.manifest();
    let artifacts = [
        &manifest.embedding_text,
        &manifest.embedding_projector,
        &manifest.chat_standard,
        &manifest.chat_lite,
    ];
    let list = artifacts
        .iter()
        .map(|a| InstalledModelInfo {
            name: a.name.clone(),
            filename: a.filename.clone(),
            installed: downloader.is_model_installed(&a.filename),
            size_bytes: a.size_bytes,
            engine_version: a.engine_version.clone(),
        })
        .collect();
    Ok(list)
}

#[tauri::command]
pub async fn download_model(model_name: String) -> Result<String, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let models_dir = std::path::PathBuf::from(home).join(".founderos").join("models");
    let downloader = founderos_runtime::ModelDownloader::new(models_dir);
    let artifact = downloader
        .manifest()
        .find_artifact(&model_name)
        .ok_or_else(|| format!("Unknown model: {model_name}"))?;
    let path = downloader
        .simulate_download(artifact, None)
        .await
        .map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
}

#[tauri::command]
pub async fn import_model(source_path: String, model_name: String) -> Result<String, String> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let models_dir = std::path::PathBuf::from(home).join(".founderos").join("models");
    let downloader = founderos_runtime::ModelDownloader::new(models_dir);
    let artifact = downloader
        .manifest()
        .find_artifact(&model_name)
        .ok_or_else(|| format!("Unknown model: {model_name}"))?;
    let path = downloader
        .import_offline_model(Path::new(&source_path), artifact)
        .map_err(|e| e.to_string())?;
    Ok(path.to_string_lossy().to_string())
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

