//! Error types for distortion operations

use thiserror::Error;

/// Errors that can occur during distortion operations
#[derive(Error, Debug)]
pub enum DistortError {
    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Audio format error
    #[error("Audio format error: {0}")]
    AudioFormat(#[from] hound::Error),

    /// Invalid input parameter
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// Processing error
    #[error("Processing error: {0}")]
    ProcessingError(String),
}

/// Result type for distortion operations
pub type Result<T> = std::result::Result<T, DistortError>;
