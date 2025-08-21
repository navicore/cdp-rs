//! Error types for spectral processing

use std::io;
use thiserror::Error;

/// Spectral processing errors
#[derive(Error, Debug)]
pub enum SpectralError {
    /// Invalid input parameter
    #[error("Invalid input: {0}")]
    InvalidInput(String),

    /// IO error
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    /// Hound WAV file error
    #[error("WAV file error: {0}")]
    Hound(#[from] hound::Error),

    /// Core DSP error
    #[error("Core DSP error: {0}")]
    Core(#[from] cdp_core::CoreError),
}

/// Result type for spectral operations
pub type Result<T> = std::result::Result<T, SpectralError>;
