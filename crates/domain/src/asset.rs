use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AssetStatus {
    Pending,
    Processing,
    Indexed,
    Failed,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Asset {
    pub id: Uuid,
    pub file_path: String,
    pub file_name: String,
    pub mime_type: String,
    pub file_size: u64,
    pub sha256: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub indexed_at: Option<DateTime<Utc>>,
    pub status: AssetStatus,
    pub metadata: serde_json::Value,
}

impl Asset {
    pub fn new(
        file_path: String,
        file_name: String,
        mime_type: String,
        file_size: u64,
        sha256: String,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            file_path,
            file_name,
            mime_type,
            file_size,
            sha256,
            created_at: now,
            modified_at: now,
            indexed_at: None,
            status: AssetStatus::Pending,
            metadata: serde_json::json!({}),
        }
    }

    pub fn mark_indexed(&mut self) {
        self.status = AssetStatus::Indexed;
        self.indexed_at = Some(Utc::now());
    }

    pub fn mark_failed(&mut self, reason: &str) {
        self.status = AssetStatus::Failed;
        self.metadata["error"] = serde_json::json!(reason);
    }
}
