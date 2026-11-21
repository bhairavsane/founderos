use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("Database error: {0}")]
    Database(String),

    #[error("Vector error: {0}")]
    Vector(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Incompatible signature: {0}")]
    IncompatibleSignature(String),
}
