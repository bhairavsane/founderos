use crate::modality::Modality;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chunk {
    pub id: Uuid,
    pub asset_id: Uuid,
    pub chunk_index: u32,
    pub modality: Modality,
    pub text_content: Option<String>,
    pub page_number: Option<u32>,
    pub timestamp_start: Option<f64>,
    pub timestamp_end: Option<f64>,
    pub metadata: serde_json::Value,
}

impl Chunk {
    pub fn new_text(
        asset_id: Uuid,
        chunk_index: u32,
        text: String,
        page_number: Option<u32>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            asset_id,
            chunk_index,
            modality: Modality::Text,
            text_content: Some(text),
            page_number,
            timestamp_start: None,
            timestamp_end: None,
            metadata: serde_json::json!({}),
        }
    }

    pub fn new_audio_segment(
        asset_id: Uuid,
        chunk_index: u32,
        start_secs: f64,
        end_secs: f64,
        transcript: Option<String>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            asset_id,
            chunk_index,
            modality: Modality::Audio,
            text_content: transcript,
            page_number: None,
            timestamp_start: Some(start_secs),
            timestamp_end: Some(end_secs),
            metadata: serde_json::json!({ "duration": end_secs - start_secs }),
        }
    }

    pub fn new_video_frame(
        asset_id: Uuid,
        chunk_index: u32,
        timestamp_sec: f64,
        scene_score: f64,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            asset_id,
            chunk_index,
            modality: Modality::Video,
            text_content: None,
            page_number: None,
            timestamp_start: Some(timestamp_sec),
            timestamp_end: Some(timestamp_sec),
            metadata: serde_json::json!({ "scene_score": scene_score }),
        }
    }
}
