use thiserror::Error;

/// Core DSP errors
#[derive(Error, Debug)]
pub enum CoreError {
    /// FFT size is not a power of 2
    #[error("FFT size must be power of 2, got {0}")]
    InvalidFftSize(usize),

    /// Window size doesn't match expected size
    #[error("Window size {0} doesn't match FFT size {1}")]
    WindowSizeMismatch(usize, usize),

    /// Invalid hop size for overlap-add processing
    #[error("Invalid hop size {hop} for window size {window}")]
    InvalidHopSize { 
        /// Hop size in samples
        hop: usize, 
        /// Window size in samples
        window: usize 
    },

    /// General numerical computation error
    #[error("Numerical error: {0}")]
    Numerical(String),
}

/// Result type for core operations
pub type Result<T> = std::result::Result<T, CoreError>;
