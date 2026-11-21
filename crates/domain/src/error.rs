use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum DomainError {
    #[error("Dimension mismatch: expected {expected}, got {actual}")]
    DimensionMismatch { expected: usize, actual: usize },

    #[error("Zero vector cannot be L2-normalized")]
    ZeroVector,

    #[error("Incompatible vector signature: expected {expected}, got {actual}")]
    IncompatibleSignature { expected: String, actual: String },

    #[error("Parse error: {0}")]
    ParseError(String),
}
