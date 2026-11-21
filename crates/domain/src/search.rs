use crate::modality::Modality;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchType {
    VectorSemantic,
    LexicalFts,
    HybridFused,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchResult {
    pub asset_id: Uuid,
    pub chunk_id: Uuid,
    pub file_path: String,
    pub file_name: String,
    pub modality: Modality,
    pub snippet: String,
    pub timestamp_or_page: Option<String>,
    pub score: f32,
    pub match_type: MatchType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchQuery {
    pub text: Option<String>,
    pub media_bytes: Option<Vec<u8>>,
    pub target_modalities: Vec<Modality>,
    pub limit: usize,
}

impl SearchQuery {
    pub fn new_text(text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            media_bytes: None,
            target_modalities: vec![
                Modality::Text,
                Modality::Image,
                Modality::Audio,
                Modality::Video,
            ],
            limit: 20,
        }
    }
}

pub fn calculate_rrf_score(ranks: &[(f32, usize)], k: f32) -> f32 {
    ranks.iter().map(|(weight, rank)| weight / (k + *rank as f32)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rrf_calculation() {
        let ranks = vec![(1.0, 1), (1.0, 5)];
        let score = calculate_rrf_score(&ranks, 60.0);
        assert!(score > 0.03);
    }
}
