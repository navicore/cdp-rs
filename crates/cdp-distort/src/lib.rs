#![forbid(unsafe_code)]
#![warn(missing_docs)]

//! Distortion and saturation effects
//!
//! This module provides various distortion algorithms including
//! harmonic multiplication, subharmonic generation, and clipping.

pub mod divide;
pub mod error;
pub mod multiply;
pub mod overload;

pub use divide::divide;
pub use error::{DistortError, Result};
pub use multiply::multiply;
pub use overload::{overload, ClipType};
