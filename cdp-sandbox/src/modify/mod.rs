//! CDP modify operations - sound transformations and effects
//!
//! Provides various sound modification operations including loudness control,
//! spatial effects, time-domain effects, and more.

use std::io;

pub mod loudness;

/// Result type for modify operations
pub type Result<T> = std::result::Result<T, ModifyError>;

/// Errors that can occur during modify operations
#[derive(Debug, thiserror::Error)]
pub enum ModifyError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Processing error: {0}")]
    ProcessingError(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}

/// CLI compatibility layer for modify operations
pub fn modify(operation: &str, mode: i32, args: &[&str]) -> Result<()> {
    match operation {
        "loudness" => loudness::loudness(mode, args),
        _ => Err(ModifyError::UnsupportedOperation(format!(
            "Unknown operation: {}",
            operation
        ))),
    }
}
