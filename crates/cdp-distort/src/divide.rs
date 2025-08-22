//! Subharmonic generation through frequency division
//!
//! Creates subharmonics by dividing signal frequency content.

use crate::error::{DistortError, Result};
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use std::path::Path;

/// Apply subharmonic division distortion
///
/// # Arguments
/// * `input_path` - Path to input audio file
/// * `output_path` - Path to output audio file
/// * `divide_factor` - Division factor (2-16)
/// * `mix` - Dry/wet mix (0.0 = dry, 1.0 = wet)
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(DistortError)` on failure
pub fn divide(input_path: &Path, output_path: &Path, divide_factor: u32, mix: f32) -> Result<()> {
    // Validate parameters
    if !(2..=16).contains(&divide_factor) {
        return Err(DistortError::InvalidInput(
            "Divide factor must be between 2 and 16".to_string(),
        ));
    }

    if !(0.0..=1.0).contains(&mix) {
        return Err(DistortError::InvalidInput(
            "Mix must be between 0.0 and 1.0".to_string(),
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

    // Process samples with subharmonic generation
    let mut output = Vec::with_capacity(samples.len());
    let mut last_sample = 0.0f32;
    let mut sub_counter = 0;

    for sample in samples.iter() {
        // Detect zero crossings for phase reset - use a more sensitive threshold
        if last_sample.signum() != sample.signum() && sample.abs() > 0.001 {
            sub_counter = (sub_counter + 1) % divide_factor;
        }

        // Generate subharmonic based on counter
        let sub_phase = (sub_counter as f32 / divide_factor as f32) * 2.0 * std::f32::consts::PI;
        let subharmonic = sub_phase.sin() * sample.abs();

        // Mix with dry signal
        let result = sample * (1.0 - mix) + subharmonic * mix;
        output.push(result);

        last_sample = *sample;
    }

    // Normalize
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divide_validation() {
        let input = Path::new("test.wav");
        let output = Path::new("out.wav");

        // Test invalid divide factor
        let result = divide(input, output, 1, 0.5);
        assert!(result.is_err());

        let result = divide(input, output, 20, 0.5);
        assert!(result.is_err());

        // Test invalid mix
        let result = divide(input, output, 2, -0.1);
        assert!(result.is_err());

        let result = divide(input, output, 2, 1.5);
        assert!(result.is_err());
    }
}
