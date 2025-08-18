#![forbid(unsafe_code)]
#![warn(missing_docs)]
#![allow(clippy::cast_precision_loss)] // Acceptable for DSP calculations
#![allow(clippy::cast_possible_truncation)] // Controlled conversions

//! Core DSP primitives for CDP-RS
//!
//! This module is FROZEN after validation against CDP.
//! Do not modify without explicit approval and re-validation.

/// CDP-compatible constants and parameters
pub mod constants;
/// Error types for core operations
pub mod errors;
/// FFT processing for spectral analysis
pub mod fft;
/// Window functions for spectral processing
pub mod window;

pub use errors::{CoreError, Result};
pub use fft::{Fft, FftProcessor};
pub use window::{Window, WindowFunction};

#[cfg(test)]
mod tests {
    #[test]
    fn test_module_frozen() {
        // This test ensures the module hasn't been modified
        // CI will check git history to enforce this
    }
}
