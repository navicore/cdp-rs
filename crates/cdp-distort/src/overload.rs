//! Clipping distortion effects
//!
//! Various types of clipping and saturation distortion.

use crate::error::{DistortError, Result};
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use std::path::Path;

/// Clipping curve types
#[derive(Debug, Clone, Copy)]
pub enum ClipType {
    /// Hard clipping (digital)
    Hard,
    /// Soft clipping (analog-like)
    Soft,
    /// Tube-like saturation
    Tube,
    /// Asymmetric clipping
    Asymmetric,
}

/// Apply clipping/overload distortion
///
/// # Arguments
/// * `input_path` - Path to input audio file
/// * `output_path` - Path to output audio file
/// * `threshold` - Clipping threshold (0.1-1.0)
/// * `drive` - Input gain before clipping (1.0-100.0)
/// * `clip_type` - Type of clipping curve
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(DistortError)` on failure
pub fn overload(
    input_path: &Path,
    output_path: &Path,
    threshold: f32,
    drive: f32,
    clip_type: ClipType,
) -> Result<()> {
    // Validate parameters
    if !(0.1..=1.0).contains(&threshold) {
        return Err(DistortError::InvalidInput(
            "Threshold must be between 0.1 and 1.0".to_string(),
        ));
    }

    if !(1.0..=100.0).contains(&drive) {
        return Err(DistortError::InvalidInput(
            "Drive must be between 1.0 and 100.0".to_string(),
        ));
    }

    // Open input file
    let reader = WavReader::open(input_path)?;
    let spec = reader.spec();

    // Collect samples
    let samples: Vec<f32> = match spec.sample_format {
        SampleFormat::Float => reader
            .into_samples::<f32>()
            .collect::<std::result::Result<Vec<_>, _>>()?,
        SampleFormat::Int => {
            // Prevent integer overflow for large bit depths
            if spec.bits_per_sample >= 32 {
                return Err(DistortError::InvalidInput(
                    "Bit depth too large for safe processing".to_string(),
                ));
            }
            let max_val = (1 << (spec.bits_per_sample - 1)) as f32;
            reader
                .into_samples::<i32>()
                .map(|s| s.map(|sample| sample as f32 / max_val))
                .collect::<std::result::Result<Vec<_>, _>>()?
        }
        _ => {
            return Err(DistortError::InvalidInput(
                "Unsupported sample format".to_string(),
            ));
        }
    };

    // Process samples
    let mut output = Vec::with_capacity(samples.len());

    for sample in samples.iter() {
        // Apply drive (pre-gain)
        let driven = sample * drive;

        // Apply clipping based on type
        let clipped = match clip_type {
            ClipType::Hard => hard_clip(driven, threshold),
            ClipType::Soft => soft_clip(driven, threshold),
            ClipType::Tube => tube_saturate(driven, threshold),
            ClipType::Asymmetric => asymmetric_clip(driven, threshold),
        };

        // Output gain compensation - prevent division by zero
        let drive_sqrt = drive.sqrt();
        let result = if drive_sqrt > f32::EPSILON {
            clipped / drive_sqrt
        } else {
            clipped
        };
        output.push(result);
    }

    // Final normalization
    let max_val = output.iter().map(|s| s.abs()).fold(0.0f32, |a, b| a.max(b));

    if max_val > 1.0 {
        let scale = 0.99 / max_val;
        for sample in output.iter_mut() {
            *sample *= scale;
        }
    }

    // Write output
    let output_spec = WavSpec {
        channels: spec.channels,
        sample_rate: spec.sample_rate,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let mut writer = WavWriter::create(output_path, output_spec)?;
    for sample in output {
        writer.write_sample(sample)?;
    }
    writer.finalize()?;

    Ok(())
}

/// Hard clipping function
fn hard_clip(sample: f32, threshold: f32) -> f32 {
    if sample > threshold {
        threshold
    } else if sample < -threshold {
        -threshold
    } else {
        sample
    }
}

/// Soft clipping function (tanh-based)
fn soft_clip(sample: f32, threshold: f32) -> f32 {
    let scaled = sample / threshold;
    threshold * scaled.tanh()
}

/// Tube-like saturation
fn tube_saturate(sample: f32, threshold: f32) -> f32 {
    let scaled = sample / threshold;
    if scaled.abs() < 1.0 {
        // Below threshold - minimal distortion
        sample * (1.0 + 0.1 * scaled.abs())
    } else {
        // Above threshold - smooth compression
        let sign = scaled.signum();
        let denominator = 1.0 + scaled.abs() - 1.0;
        let compressed = if denominator > f32::EPSILON {
            1.0 - (1.0 / denominator)
        } else {
            0.99 // Safe fallback value
        };
        sign * threshold * compressed
    }
}

/// Asymmetric clipping (different thresholds for positive/negative)
fn asymmetric_clip(sample: f32, threshold: f32) -> f32 {
    if sample > threshold {
        // Harder clipping on positive side
        threshold
    } else if sample < -threshold * 0.7 {
        // Softer clipping on negative side
        let excess = (sample + threshold * 0.7) / (threshold * 0.3);
        -threshold * 0.7 - threshold * 0.3 * excess.tanh()
    } else {
        sample
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overload_validation() {
        let input = Path::new("test.wav");
        let output = Path::new("out.wav");

        // Test invalid threshold
        let result = overload(input, output, 0.05, 2.0, ClipType::Hard);
        assert!(result.is_err());

        let result = overload(input, output, 1.5, 2.0, ClipType::Hard);
        assert!(result.is_err());

        // Test invalid drive
        let result = overload(input, output, 0.5, 0.5, ClipType::Hard);
        assert!(result.is_err());

        let result = overload(input, output, 0.5, 150.0, ClipType::Hard);
        assert!(result.is_err());
    }

    #[test]
    fn test_clipping_functions() {
        // Test hard clip
        assert_eq!(hard_clip(0.5, 0.7), 0.5);
        assert_eq!(hard_clip(0.8, 0.7), 0.7);
        assert_eq!(hard_clip(-0.8, 0.7), -0.7);

        // Test soft clip stays within bounds
        let soft = soft_clip(2.0, 0.7);
        assert!(soft.abs() <= 0.7 * 1.01); // Allow small numerical error

        // Test tube saturation
        let tube = tube_saturate(0.5, 0.7);
        assert!(tube.abs() >= 0.5); // Should add harmonics

        // Test asymmetric clip
        assert_eq!(asymmetric_clip(0.8, 0.7), 0.7);
        assert!(asymmetric_clip(-0.6, 0.7).abs() <= 0.6);
    }
}
