use crate::error::PipelineError;
use founderos_domain::{calculate_rrf_score, MatchType, SearchQuery, SearchResult};
use founderos_runtime::{InferenceProvider, TaskType};
use founderos_storage::{SqliteStorage, VectorRepository};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct HybridRetriever {
    storage: SqliteStorage,
    vector_repo: Arc<dyn VectorRepository>,
    provider: Arc<dyn InferenceProvider>,
}

impl HybridRetriever {
    pub fn new(
        storage: SqliteStorage,
        vector_repo: Arc<dyn VectorRepository>,
        provider: Arc<dyn InferenceProvider>,
    ) -> Self {
        Self {
            storage,
            vector_repo,
            provider,
        }
    }

    pub async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>, PipelineError> {
        let text_query = match query.text {
            Some(ref t) if !t.trim().is_empty() => t.trim(),
            _ => return Ok(Vec::new()),
        };

        let mut rrf_ranks: HashMap<Uuid, Vec<(f32, usize)>> = HashMap::new();
        let mut chunk_assets: HashMap<Uuid, Uuid> = HashMap::new();
        let mut chunk_snippets: HashMap<Uuid, String> = HashMap::new();
        let mut chunk_types: HashMap<Uuid, MatchType> = HashMap::new();

        // 1. Vector Search
        let query_vec = if let Some(ref media) = query.media_bytes {
            let mod_type = query.target_modalities.first().copied().unwrap_or(founderos_domain::Modality::Image);
            self.provider
                .embed_media(media, mod_type, TaskType::SearchQuery)
                .await
                .map_err(|e| PipelineError::Search(e.to_string()))?
        } else {
            self.provider
                .embed_text(text_query, TaskType::SearchQuery)
                .await
                .map_err(|e| PipelineError::Search(e.to_string()))?
        };

        for modality in &query.target_modalities {
            let vec_results = self
                .vector_repo
                .search_vectors(&query_vec, Some(*modality), query.limit)
                .await
                .map_err(|e| PipelineError::Search(e.to_string()))?;

            for (rank, res) in vec_results.into_iter().enumerate() {
                rrf_ranks
                    .entry(res.chunk_id)
                    .or_default()
                    .push((1.0, rank + 1));
                chunk_assets.insert(res.chunk_id, res.asset_id);
                chunk_types.insert(res.chunk_id, MatchType::VectorSemantic);
            }
        }

        // 2. Lexical FTS5 Search
        let fts_results = self
            .storage
            .search_fts(text_query, query.limit)
            .map_err(|e| PipelineError::Search(e.to_string()))?;

        for (rank, (chunk_id, asset_id, _score, snippet)) in fts_results.into_iter().enumerate() {
            rrf_ranks
                .entry(chunk_id)
                .or_default()
                .push((3.0, rank + 1));
            chunk_assets.insert(chunk_id, asset_id);
            chunk_snippets.insert(chunk_id, snippet);
            chunk_types
                .entry(chunk_id)
                .and_modify(|t| *t = MatchType::HybridFused)
                .or_insert(MatchType::LexicalFts);
        }


        // 3. Compute RRF Scores & Fetch Asset Details
        let mut final_results = Vec::new();
        for (chunk_id, ranks) in rrf_ranks {
            let score = calculate_rrf_score(&ranks, 60.0);
            let asset_id = match chunk_assets.get(&chunk_id) {
                Some(id) => *id,
                None => continue,
            };
            let asset = self
                .storage
                .get_asset(&asset_id)
                .map_err(|e| PipelineError::Search(e.to_string()))?
                .ok_or_else(|| PipelineError::Search("Asset not found".to_string()))?;

            let snippet = chunk_snippets
                .remove(&chunk_id)
                .unwrap_or_else(|| format!("{}: matching segment", asset.file_name));

            let match_type = chunk_types.get(&chunk_id).copied().unwrap_or(MatchType::VectorSemantic);

            final_results.push(SearchResult {
                asset_id,
                chunk_id,
                file_path: asset.file_path,
                file_name: asset.file_name,
                modality: founderos_domain::Modality::from_mime_type(&asset.mime_type),
                snippet,
                timestamp_or_page: None,
                score,
                match_type,
            });
        }

        final_results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        final_results.truncate(query.limit);
        Ok(final_results)
    }
}
