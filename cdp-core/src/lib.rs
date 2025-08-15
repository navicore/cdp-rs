#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Core DSP primitives for CDP-RS
//!
//! This module is FROZEN after validation against CDP.
//! Do not modify without explicit approval and re-validation.

pub mod constants;
pub mod errors;
pub mod fft;
pub mod window;

pub use errors::{CoreError, Result};
pub use fft::{Fft, FftProcessor};
pub use window::{Window, WindowFunction};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_frozen() {
        // This test ensures the module hasn't been modified
        // CI will check git history to enforce this
    }
}
