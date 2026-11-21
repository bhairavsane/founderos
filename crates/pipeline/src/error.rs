use thiserror::Error;

#[derive(Error, Debug)]
pub enum PipelineError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Extraction error: {0}")]
    Extraction(String),

    #[error("Media error: {0}")]
    Media(String),

    #[error("Search error: {0}")]
    Search(String),
}
