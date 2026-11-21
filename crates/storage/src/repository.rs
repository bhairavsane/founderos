use crate::error::StorageError;
use async_trait::async_trait;
use founderos_domain::{Modality, VectorRecord, VectorSignature};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct VectorSearchResult {
    pub chunk_id: Uuid,
    pub asset_id: Uuid,
    pub modality: Modality,
    pub score: f32,
    pub dimension: usize,
}

#[async_trait]
pub trait VectorRepository: Send + Sync {
    async fn insert_vectors(&self, records: &[VectorRecord]) -> Result<(), StorageError>;
    async fn search_vectors(
        &self,
        query: &[f32],
        modality: Option<Modality>,
        top_k: usize,
    ) -> Result<Vec<VectorSearchResult>, StorageError>;
    async fn delete_by_asset(&self, asset_id: &Uuid) -> Result<(), StorageError>;
    async fn count(&self) -> Result<usize, StorageError>;
    fn active_signature(&self) -> &VectorSignature;
}
