#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Spectral processing algorithms matching CDP
//!
//! This module will be FROZEN after validation against CDP.
//! Do not modify without explicit approval and re-validation.

pub mod blur;
pub mod error;

pub use blur::{blur, blur_varying};
pub use error::{Result, SpectralError};
