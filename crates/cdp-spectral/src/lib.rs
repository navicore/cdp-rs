#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Spectral processing algorithms matching CDP
//!
//! This module will be FROZEN after validation against CDP.
//! Do not modify without explicit approval and re-validation.

mod ana_io;
pub mod blur;
pub mod error;
pub mod pitch;
pub mod stretch;

pub use blur::{blur, blur_varying};
pub use error::{Result, SpectralError};
pub use pitch::{factor_to_semitones, pitch_shift, pitch_shift_formant, semitones_to_factor};
pub use stretch::{calculate_output_duration, stretch_time, stretch_time_varying};
