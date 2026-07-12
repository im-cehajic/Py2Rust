use thiserror::Error;

/// Result type for Py2Rust operations
pub type Result<T> = std::result::Result<T, Error>;

/// Error types for the converter
#[derive(Debug, Error)]
pub enum Error {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Type inference error: {0}")]
    TypeInferenceError(String),

    #[error("Code generation error: {0}")]
    GenerationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unsupported feature: {0}")]
    UnsupportedFeature(String),

    #[error("Internal error: {0}")]
    InternalError(String),
}
