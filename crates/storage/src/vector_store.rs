use crate::error::StorageError;
use crate::repository::{VectorRepository, VectorSearchResult};
use async_trait::async_trait;
use founderos_domain::{Modality, VectorRecord, VectorSignature};
use std::sync::RwLock;
use uuid::Uuid;

pub struct InMemoryVectorStore {
    signature: VectorSignature,
    records: RwLock<Vec<VectorRecord>>,
}

impl InMemoryVectorStore {
    pub fn new(signature: VectorSignature) -> Self {
        Self {
            signature,
            records: RwLock::new(Vec::new()),
        }
    }
}

#[async_trait]
impl VectorRepository for InMemoryVectorStore {
    fn active_signature(&self) -> &VectorSignature {
        &self.signature
    }

    async fn insert_vectors(&self, records: &[VectorRecord]) -> Result<(), StorageError> {
        let mut store = self.records.write().map_err(|e| StorageError::Vector(e.to_string()))?;
        for rec in records {
            if !rec.is_compatible_with(&self.signature) {
                return Err(StorageError::IncompatibleSignature(format!(
                    "Cannot insert vector with signature {:?} into store with signature {:?}",
                    rec.signature(),
                    self.signature
                )));
            }
            store.push(rec.clone());
        }
        Ok(())
    }

    async fn search_vectors(
        &self,
        query: &[f32],
        modality: Option<Modality>,
        top_k: usize,
    ) -> Result<Vec<VectorSearchResult>, StorageError> {
        if query.len() != self.signature.dimension {
            return Err(StorageError::Vector(format!(
                "Query dimension {} does not match store dimension {}",
                query.len(),
                self.signature.dimension
            )));
        }
        let store = self.records.read().map_err(|e| StorageError::Vector(e.to_string()))?;
        let mut scored: Vec<VectorSearchResult> = store
            .iter()
            .filter(|r| modality.is_none_or(|m| r.modality == m))
            .map(|r| {
                let sim = r.cosine_similarity(query).unwrap_or(0.0);
                VectorSearchResult {
                    chunk_id: r.chunk_id,
                    asset_id: r.asset_id,
                    modality: r.modality,
                    score: sim,
                    dimension: r.signature.dimension,
                }
            })
            .collect();

        scored.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        scored.truncate(top_k);
        Ok(scored)
    }

    async fn delete_by_asset(&self, asset_id: &Uuid) -> Result<(), StorageError> {
        let mut store = self.records.write().map_err(|e| StorageError::Vector(e.to_string()))?;
        store.retain(|r| r.asset_id != *asset_id);
        Ok(())
    }

    async fn count(&self) -> Result<usize, StorageError> {
        let store = self.records.read().map_err(|e| StorageError::Vector(e.to_string()))?;
        Ok(store.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_vector_store_pinning_and_search() {
        let mut sig = VectorSignature::canonical("metal", "canonical_v1");
        sig.dimension = 4;
        let store = InMemoryVectorStore::new(sig.clone());

        let aid = Uuid::new_v4();
        let cid = Uuid::new_v4();
        let rec = VectorRecord::new(
            cid,
            aid,
            Modality::Image,
            &sig,
            vec![1.0, 0.0, 0.0, 0.0],
        ).unwrap();

        store.insert_vectors(&[rec]).await.unwrap();

        // Query matching exactly
        let results = store.search_vectors(&[1.0, 0.0, 0.0, 0.0], None, 5).await.unwrap();
        assert_eq!(results.len(), 1);
        assert!((results[0].score - 1.0).abs() < 1e-3);

        // Mismatched signature rejection
        let mut diff_sig = sig.clone();
        diff_sig.model_tag = "embeddinggemma-2:270m".to_string();
        let bad_rec = VectorRecord::new(
            Uuid::new_v4(),
            Uuid::new_v4(),
            Modality::Text,
            &diff_sig,
            vec![0.0, 1.0, 0.0, 0.0],
        ).unwrap();

        let err = store.insert_vectors(&[bad_rec]).await;
        assert!(err.is_err());
    }
}
