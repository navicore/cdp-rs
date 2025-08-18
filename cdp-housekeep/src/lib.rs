//! CDP Housekeep module - File manipulation and format conversion
//!
//! This module implements CDP's housekeeping operations including:
//! - File copying with CDP metadata preservation
//! - Channel extraction and manipulation
//! - Format conversion
//!
//! All operations are validated against CDP binaries for byte-perfect compatibility.

use thiserror::Error;

pub mod chans;
pub mod copy;
pub mod wav_cdp;

/// Result type for housekeep operations
pub type Result<T> = std::result::Result<T, HousekeepError>;

/// Errors that can occur during housekeep operations
#[derive(Error, Debug)]
pub enum HousekeepError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Invalid file: {0}")]
    InvalidFile(String),

    #[error("Unsupported format: {0}")]
    UnsupportedFormat(String),
}

// Re-export main functions for convenience
pub use chans::{extract_channel, extract_channel_to, mix_to_mono};
pub use copy::{copy, copy_file};
pub use wav_cdp::{read_wav_basic, write_wav_cdp};

/// CLI compatibility layer - matches CDP's command-line interface
/// This is just for oracle testing. Real users should use the library functions directly.
pub fn housekeep(operation: &str, args: &[&str]) -> Result<()> {
    use std::path::Path;

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
        "chans" => {
            if args.is_empty() {
                return Err(HousekeepError::InvalidFile(
                    "Usage: chans <mode> <infile> [args...]".into(),
                ));
            }
            let mode = args[0].parse::<i32>().unwrap_or(1);
            chans::chans(mode, &args[1..])
        }
        _ => Err(HousekeepError::UnsupportedFormat(format!(
            "Unknown operation: {}",
            operation
        ))),
    }
}
