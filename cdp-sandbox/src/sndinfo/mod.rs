//! CDP sndinfo operations - sound file information and analysis
//!
//! Provides information about sound files including format, duration, peak levels, etc.

use std::io;
use std::path::Path;

pub mod props;

/// Result type for sndinfo operations
pub type Result<T> = std::result::Result<T, SndInfoError>;

/// Errors that can occur during sndinfo operations
#[derive(Debug, thiserror::Error)]
pub enum SndInfoError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid audio file: {0}")]
    InvalidFile(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),
}

/// CLI compatibility layer for sndinfo
pub fn sndinfo(operation: &str, args: &[&str]) -> Result<()> {
    match operation {
        "props" => {
            if args.is_empty() {
                return Err(SndInfoError::InvalidFile(
                    "Usage: sndinfo props <infile>".into(),
                ));
            }
            let input = Path::new(args[0]);
            props::show_props(input)
        }
        _ => Err(SndInfoError::UnsupportedOperation(format!(
            "Unknown operation: {}",
            operation
        ))),
    }
}
