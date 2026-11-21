use founderos_domain::{CitationType, MatchType, Modality};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CitationDto {
    pub file_name: String,
    pub file_path: String,
    pub citation_type: CitationType,
    pub evidence: String,
    pub timestamp_or_page: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AskResponsePayload {
    pub answer: String,
    pub citations: Vec<CitationDto>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResultDto {
    pub id: String,
    pub asset_id: String,
    pub file_name: String,
    pub file_path: String,
    pub modality: Modality,
    pub snippet: String,
    pub timestamp_or_page: Option<String>,
    pub score: f32,
    pub match_type: MatchType,
}

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
pub struct InstalledModelInfo {
    pub name: String,
    pub filename: String,
    pub installed: bool,
    pub size_bytes: u64,
    pub engine_version: String,
}
