//! Harmonic distortion through frequency multiplication
//!
//! Creates harmonic distortion by multiplying signal frequency content.

use crate::error::{DistortError, Result};
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use std::path::Path;

/// Apply harmonic multiplication distortion
///
/// # Arguments
/// * `input_path` - Path to input audio file
/// * `output_path` - Path to output audio file
/// * `multiply_factor` - Multiplication factor (1.0-16.0)
/// * `mix` - Dry/wet mix (0.0 = dry, 1.0 = wet)
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(DistortError)` on failure
pub fn multiply(
    input_path: &Path,
    output_path: &Path,
    multiply_factor: f32,
    mix: f32,
) -> Result<()> {
    // Validate parameters
    if !(1.0..=16.0).contains(&multiply_factor) {
        return Err(DistortError::InvalidInput(
            "Multiply factor must be between 1.0 and 16.0".to_string(),
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
            let max_val = (1 << (spec.bits_per_sample - 1)) as f32;
            reader
                .into_samples::<i32>()
                .map(|s| s.map(|sample| sample as f32 / max_val))
                .collect::<std::result::Result<Vec<_>, _>>()?
        }
    };

    // Process samples
    let mut output = Vec::with_capacity(samples.len());

    for sample in samples.iter() {
        // Apply harmonic multiplication
        // This creates harmonics by folding the waveform
        let multiplied = (sample * multiply_factor).tanh();

        // Mix with dry signal
        let result = sample * (1.0 - mix) + multiplied * mix;
        output.push(result);
    }

    // Normalize to prevent clipping
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
    fn test_multiply_validation() {
        let input = Path::new("test.wav");
        let output = Path::new("out.wav");

        // Test invalid multiply factor
        let result = multiply(input, output, 0.5, 0.5);
        assert!(result.is_err());

        let result = multiply(input, output, 20.0, 0.5);
        assert!(result.is_err());

        // Test invalid mix
        let result = multiply(input, output, 2.0, -0.1);
        assert!(result.is_err());

        let result = multiply(input, output, 2.0, 1.5);
        assert!(result.is_err());
    }
}
