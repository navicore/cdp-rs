//! CDP-compatible constants

/// Default sample rate used by CDP
pub const CDP_DEFAULT_SAMPLE_RATE: f32 = 44100.0;

/// Maximum number of channels supported
pub const CDP_MAX_CHANNELS: usize = 8;

/// Default FFT size for phase vocoder
pub const DEFAULT_FFT_SIZE: usize = 1024;

/// Default overlap factor
pub const DEFAULT_OVERLAP: usize = 4;

/// Minimum amplitude threshold (to match CDP's behavior)
pub const MIN_AMPLITUDE: f32 = 1e-10;
