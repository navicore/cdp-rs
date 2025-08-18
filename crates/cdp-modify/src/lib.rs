//! CDP Modify module - Audio modification operations
//!
//! This module implements CDP's modification operations including:
//! - Gain adjustment (linear and dB)
//! - Normalization
//! - Phase inversion
//!
//! All operations are validated against CDP binaries for byte-perfect compatibility.

use thiserror::Error;

pub mod loudness;

/// Result type for modify operations
pub type Result<T> = std::result::Result<T, ModifyError>;

/// Errors that can occur during modify operations
#[derive(Error, Debug)]
pub enum ModifyError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}

// Re-export main functions for convenience
pub use loudness::{apply_db_gain, apply_gain, normalize};

/// CLI compatibility layer - matches CDP's command-line interface
/// This is just for oracle testing. Real users should use the library functions directly.
pub fn modify(operation: &str, mode: i32, args: &[&str]) -> Result<()> {
    match operation {
        "loudness" => loudness::loudness(mode, args),
        _ => Err(ModifyError::UnsupportedOperation(format!(
            "Unknown operation: {}",
            operation
        ))),
    }
}
