//! CDP Housekeep operations - file management and basic transformations
//!
//! This module implements CDP's housekeep operations as a library,
//! with thin binary wrappers for oracle validation.

use std::io;
use std::path::Path;

pub mod copy;
pub mod wav_cdp;

/// Result type for housekeep operations
pub type Result<T> = std::result::Result<T, HousekeepError>;

/// Errors that can occur during housekeep operations
#[derive(Debug, thiserror::Error)]
pub enum HousekeepError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid audio file: {0}")]
    InvalidFile(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
}

/// CLI compatibility layer - matches CDP's command-line interface
/// This is just for oracle testing. Real users should use the library functions directly.
pub fn housekeep(operation: &str, args: &[&str]) -> Result<()> {
    match operation {
        "copy" => {
            if args.len() < 3 {
                return Err(HousekeepError::InvalidFile(
                    "Usage: copy <mode> <infile> <outfile>".into(),
                ));
            }
            let mode = args[0].parse::<i32>().unwrap_or(1);
            let input = Path::new(args[1]);
            let output = Path::new(args[2]);
            copy::copy_file(input, output, mode)
        }
        _ => Err(HousekeepError::UnsupportedFormat(format!(
            "Unknown operation: {}",
            operation
        ))),
    }
}
