use thiserror::Error;

#[derive(Error, Debug)]
pub enum CoreError {
    #[error("FFT size must be power of 2, got {0}")]
    InvalidFftSize(usize),

    #[error("Window size {0} doesn't match FFT size {1}")]
    WindowSizeMismatch(usize, usize),

    #[error("Invalid hop size {hop} for window size {window}")]
    InvalidHopSize { hop: usize, window: usize },

    #[error("Numerical error: {0}")]
    Numerical(String),
}

pub type Result<T> = std::result::Result<T, CoreError>;
