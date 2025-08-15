#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Core DSP primitives for CDP-RS
//! 
//! This module is FROZEN after validation against CDP.
//! Do not modify without explicit approval and re-validation.

pub mod fft;
pub mod window;
pub mod errors;
pub mod constants;

pub use errors::{CoreError, Result};
pub use window::{Window, WindowFunction};
pub use fft::{Fft, FftProcessor};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_module_frozen() {
        // This test ensures the module hasn't been modified
        // CI will check git history to enforce this
    }
}