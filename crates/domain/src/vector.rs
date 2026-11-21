use crate::error::DomainError;
use crate::modality::Modality;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VectorSignature {
    pub model_tag: String,
    pub quantization: String,
    pub prefix_config: String,
    pub dimension: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorRecord {
    pub id: Uuid,
    pub chunk_id: Uuid,
    pub asset_id: Uuid,
    pub modality: Modality,
    pub model_tag: String,
    pub quantization: String,
    pub prefix_config: String,
    pub dimension: usize,
    pub embedding: Vec<f32>,
}

impl VectorRecord {
    pub const CANONICAL_DIM: usize = 768;
    pub const MRL_TRUNCATED_DIM: usize = 256;

    pub fn new(
        chunk_id: Uuid,
        asset_id: Uuid,
        modality: Modality,
        signature: &VectorSignature,
        embedding: Vec<f32>,
    ) -> Result<Self, DomainError> {
        if embedding.len() != signature.dimension {
            return Err(DomainError::DimensionMismatch {
                expected: signature.dimension,
                actual: embedding.len(),
            });
        }
        let normalized = Self::l2_normalize(&embedding)?;
        Ok(Self {
            id: Uuid::new_v4(),
            chunk_id,
            asset_id,
            modality,
            model_tag: signature.model_tag.clone(),
            quantization: signature.quantization.clone(),
            prefix_config: signature.prefix_config.clone(),
            dimension: signature.dimension,
            embedding: normalized,
        })
    }

    pub fn signature(&self) -> VectorSignature {
        VectorSignature {
            model_tag: self.model_tag.clone(),
            quantization: self.quantization.clone(),
            prefix_config: self.prefix_config.clone(),
            dimension: self.dimension,
        }
    }

    pub fn is_compatible_with(&self, signature: &VectorSignature) -> bool {
        self.signature() == *signature
    }

    pub fn l2_normalize(vec: &[f32]) -> Result<Vec<f32>, DomainError> {
        let norm_sq: f32 = vec.iter().map(|x| x * x).sum();
        let norm = norm_sq.sqrt();
        if norm < 1e-12 {
            return Err(DomainError::ZeroVector);
        }
        Ok(vec.iter().map(|x| x / norm).collect())
    }

    pub fn truncate_mrl(
        &self,
        target_dim: usize,
    ) -> Result<Self, DomainError> {
        if target_dim > self.embedding.len() {
            return Err(DomainError::DimensionMismatch {
                expected: target_dim,
                actual: self.embedding.len(),
            });
        }
        let truncated = &self.embedding[..target_dim];
        let normalized = Self::l2_normalize(truncated)?;
        Ok(Self {
            id: Uuid::new_v4(),
            chunk_id: self.chunk_id,
            asset_id: self.asset_id,
            modality: self.modality,
            model_tag: self.model_tag.clone(),
            quantization: self.quantization.clone(),
            prefix_config: self.prefix_config.clone(),
            dimension: target_dim,
            embedding: normalized,
        })
    }

    pub fn cosine_similarity(&self, other: &[f32]) -> Result<f32, DomainError> {
        if self.embedding.len() != other.len() {
            return Err(DomainError::DimensionMismatch {
                expected: self.embedding.len(),
                actual: other.len(),
            });
        }
        let dot: f32 = self.embedding.iter().zip(other.iter()).map(|(a, b)| a * b).sum();
        Ok(dot)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_l2_normalization_and_mrl() {
        let raw = vec![1.0, 2.0, 3.0, 4.0];
        let norm = VectorRecord::l2_normalize(&raw).unwrap();
        let sum_sq: f32 = norm.iter().map(|x| x * x).sum();
        assert!((sum_sq - 1.0).abs() < 1e-5);

        let sig = VectorSignature {
            model_tag: "embeddinggemma-2:740m".to_string(),
            quantization: "q8_0".to_string(),
            prefix_config: "v1_prefixed".to_string(),
            dimension: 4,
        };
        let rec = VectorRecord::new(Uuid::new_v4(), Uuid::new_v4(), Modality::Text, &sig, raw).unwrap();
        let mrl = rec.truncate_mrl(2).unwrap();
        assert_eq!(mrl.dimension, 2);
        let mrl_sum_sq: f32 = mrl.embedding.iter().map(|x| x * x).sum();
        assert!((mrl_sum_sq - 1.0).abs() < 1e-5);
    }
}
