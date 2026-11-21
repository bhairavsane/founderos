use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum DomainError {
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("Zero vector cannot be L2-normalized")]
    ZeroVector,

    #[error("Invalid numeric activation value: {0}")]
    InvalidNumericValue(String),

    #[error("L2 norm out of range: {0} (expected 1.0 +- 1e-3)")]
    NormOutOfRange(f32),

    #[error("Incompatible vector signature: expected {expected}, got {actual}")]
    IncompatibleSignature { expected: String, actual: String },

    #[error("Parse error: {0}")]
    ParseError(String),
}
