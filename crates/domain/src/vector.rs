use crate::error::DomainError;
use crate::modality::Modality;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VectorSignature {
    pub model_tag: String,
    pub text_quantization: String,
    pub projector_quantization: String,
    pub engine_version: String,
    pub backend: String,
    pub task_prefix: String,
    pub dimension: usize,
}

impl VectorSignature {
    pub fn canonical(backend: &str, task_prefix: &str) -> Self {
        Self {
            model_tag: "embeddinggemma-2:740m".to_string(),
            text_quantization: "Q8_0".to_string(),
            projector_quantization: "F16".to_string(),
            engine_version: "llama.cpp-b3800".to_string(),
            backend: backend.to_string(),
            task_prefix: task_prefix.to_string(),
            dimension: 768,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VectorRecord {
    pub id: Uuid,
    pub chunk_id: Uuid,
    pub asset_id: Uuid,
    pub modality: Modality,
    pub signature: VectorSignature,
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
            signature: signature.clone(),
            embedding: normalized,
        })
    }

    pub fn signature(&self) -> &VectorSignature {
        &self.signature
    }

    pub fn is_compatible_with(&self, sig: &VectorSignature) -> bool {
        self.signature == *sig
    }

    pub fn l2_normalize(vec: &[f32]) -> Result<Vec<f32>, DomainError> {
        for &val in vec {
            if val.is_nan() || val.is_infinite() {
                return Err(DomainError::InvalidNumericValue("Found NaN/Inf in vector".into()));
            }
        }
        let norm_sq: f32 = vec.iter().map(|x| x * x).sum();
        let norm = norm_sq.sqrt();
        if norm < 1e-12 {
            return Err(DomainError::ZeroVector);
        }
        let normalized: Vec<f32> = vec.iter().map(|x| x / norm).collect();
        let check_norm: f32 = normalized.iter().map(|x| x * x).sum::<f32>().sqrt();
        if (check_norm - 1.0).abs() > 1e-3 {
            return Err(DomainError::NormOutOfRange(check_norm));
        }
        Ok(normalized)
    }

    pub fn truncate_mrl(&self, target_dim: usize) -> Result<Self, DomainError> {
        if target_dim > self.embedding.len() {
            return Err(DomainError::DimensionMismatch {
                expected: target_dim,
                actual: self.embedding.len(),
            });
        }
        let truncated = &self.embedding[..target_dim];
        let normalized = Self::l2_normalize(truncated)?;
        let mut new_sig = self.signature.clone();
        new_sig.dimension = target_dim;

        Ok(Self {
            id: Uuid::new_v4(),
            chunk_id: self.chunk_id,
            asset_id: self.asset_id,
            modality: self.modality,
            signature: new_sig,
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
        assert!((sum_sq - 1.0).abs() < 1e-3);

        let sig = VectorSignature::canonical("metal", "search_document");
        let mut raw_768 = vec![0.1f32; 768];
        raw_768[0] = 1.0;
        let rec = VectorRecord::new(Uuid::new_v4(), Uuid::new_v4(), Modality::Text, &sig, raw_768).unwrap();
        let mrl = rec.truncate_mrl(256).unwrap();
        assert_eq!(mrl.signature.dimension, 256);
        let mrl_sum_sq: f32 = mrl.embedding.iter().map(|x| x * x).sum();
        assert!((mrl_sum_sq - 1.0).abs() < 1e-3);
    }

    #[test]
    fn test_nan_rejection() {
        let invalid = vec![1.0, f32::NAN, 3.0];
        assert!(VectorRecord::l2_normalize(&invalid).is_err());
    }
}
