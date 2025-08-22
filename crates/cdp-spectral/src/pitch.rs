//! Pitch shifting operations using spectral bin shifting
//!
//! Shifts pitch by moving frequency bins up or down.

use crate::ana_io::{read_ana_file, write_ana_file};
use crate::error::{Result, SpectralError};
use std::path::Path;

/// Pitch shift a spectral file
///
/// # Arguments
/// * `input_path` - Path to input .ana file
/// * `output_path` - Path to output .ana file  
/// * `shift_factor` - Pitch shift factor (2.0 = octave up, 0.5 = octave down)
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(SpectralError)` on failure
pub fn pitch_shift(input_path: &Path, output_path: &Path, shift_factor: f64) -> Result<()> {
    // Validate shift factor
    if shift_factor <= 0.0 || !(0.1..=10.0).contains(&shift_factor) {
        return Err(SpectralError::InvalidInput(
            "Shift factor must be between 0.1 and 10".to_string(),
        ));
    }

    // Read input .ana file
    let (header, samples) = read_ana_file(input_path)?;

    // Calculate window size (samples per window)
    let window_size = header.channels as usize;
    let num_windows = samples.len() / window_size;
    let num_bins = window_size / 2; // Real/imaginary pairs

    if num_windows == 0 {
        return Err(SpectralError::InvalidInput(
            "Input file has no spectral data".to_string(),
        ));
    }

    // Allocate output buffer
    let mut output = vec![0.0f32; samples.len()];

    // Process each window
    for window_idx in 0..num_windows {
        let window_start = window_idx * window_size;

        // Shift frequency bins
        for bin in 0..num_bins {
            let src_bin = bin;
            let dst_bin = (bin as f64 * shift_factor).round() as usize;

            if dst_bin < num_bins {
                let src_real = samples[window_start + src_bin * 2];
                let src_imag = samples[window_start + src_bin * 2 + 1];

                let dst_idx = window_start + dst_bin * 2;

                // Add to destination (allows overlapping bins)
                output[dst_idx] += src_real;
                output[dst_idx + 1] += src_imag;
            }
        }

        // Normalize to prevent clipping from overlapping bins
        let mut max_magnitude = 0.0f32;
        for bin in 0..num_bins {
            let real = output[window_start + bin * 2];
            let imag = output[window_start + bin * 2 + 1];
            let magnitude = (real * real + imag * imag).sqrt();
            max_magnitude = max_magnitude.max(magnitude);
        }

        // Apply normalization if needed
        if max_magnitude > 1.0 {
            let scale = 0.95 / max_magnitude; // Scale to 95% to prevent clipping
            for bin in 0..num_bins {
                output[window_start + bin * 2] *= scale;
                output[window_start + bin * 2 + 1] *= scale;
            }
        }
    }

    // Write output .ana file
    write_ana_file(output_path, &header, &output)?;

    Ok(())
}

/// Pitch shift with formant preservation (spectral envelope)
///
/// # Arguments
/// * `input_path` - Path to input .ana file
/// * `output_path` - Path to output .ana file  
/// * `shift_factor` - Pitch shift factor
/// * `preserve_formants` - If true, preserves spectral envelope
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(SpectralError)` on failure
pub fn pitch_shift_formant(
    input_path: &Path,
    output_path: &Path,
    shift_factor: f64,
    preserve_formants: bool,
) -> Result<()> {
    if !preserve_formants {
        return pitch_shift(input_path, output_path, shift_factor);
    }

    // Validate shift factor
    if shift_factor <= 0.0 || !(0.1..=10.0).contains(&shift_factor) {
        return Err(SpectralError::InvalidInput(
            "Shift factor must be between 0.1 and 10".to_string(),
        ));
    }

    // Read input .ana file
    let (header, samples) = read_ana_file(input_path)?;

    let window_size = header.channels as usize;
    let num_windows = samples.len() / window_size;
    let num_bins = window_size / 2;

    let mut output = vec![0.0f32; samples.len()];

    // Process each window with formant preservation
    for window_idx in 0..num_windows {
        let window_start = window_idx * window_size;

        // Extract spectral envelope (magnitude spectrum)
        let mut envelope = vec![0.0f32; num_bins];
        for bin in 0..num_bins {
            let real = samples[window_start + bin * 2];
            let imag = samples[window_start + bin * 2 + 1];
            envelope[bin] = (real * real + imag * imag).sqrt();
        }

        // Shift harmonics while preserving envelope
        for bin in 0..num_bins {
            let src_bin = (bin as f64 / shift_factor).round() as usize;

            if src_bin < num_bins {
                let src_real = samples[window_start + src_bin * 2];
                let src_imag = samples[window_start + src_bin * 2 + 1];

                // Get source magnitude and phase
                let src_mag = (src_real * src_real + src_imag * src_imag).sqrt();
                let src_phase = src_imag.atan2(src_real);

                // For formant preservation: keep original envelope magnitude ratios
                // Apply the envelope characteristic from the original position
                let envelope_factor = if envelope[src_bin] > 0.0 {
                    envelope[bin] / envelope[src_bin]
                } else {
                    1.0
                };
                let new_mag = src_mag * envelope_factor;

                // Convert back to rectangular using source phase
                output[window_start + bin * 2] = new_mag * src_phase.cos();
                output[window_start + bin * 2 + 1] = new_mag * src_phase.sin();
            }
        }
    }

    // Write output .ana file
    write_ana_file(output_path, &header, &output)?;

    Ok(())
}

/// Convert pitch shift factor to semitones
pub fn factor_to_semitones(factor: f64) -> f64 {
    12.0 * factor.log2()
}

/// Convert semitones to pitch shift factor
pub fn semitones_to_factor(semitones: f64) -> f64 {
    2.0_f64.powf(semitones / 12.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pitch_validation() {
        let input = Path::new("test.ana");
        let output = Path::new("out.ana");

        // Test invalid shift factors
        let result = pitch_shift(input, output, 0.0);
        assert!(result.is_err());

        let result = pitch_shift(input, output, -1.0);
        assert!(result.is_err());

        let result = pitch_shift(input, output, 0.05);
        assert!(result.is_err());

        let result = pitch_shift(input, output, 20.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_semitone_conversion() {
        // Test octave up (12 semitones = factor of 2)
        let factor = semitones_to_factor(12.0);
        assert!((factor - 2.0).abs() < 1e-6);

        // Test octave down (-12 semitones = factor of 0.5)
        let factor = semitones_to_factor(-12.0);
        assert!((factor - 0.5).abs() < 1e-6);

        // Test perfect fifth (7 semitones)
        let factor = semitones_to_factor(7.0);
        assert!((factor - 1.498307).abs() < 1e-6);

        // Test reverse conversion
        let semitones = factor_to_semitones(2.0);
        assert!((semitones - 12.0).abs() < 1e-6);

        let semitones = factor_to_semitones(0.5);
        assert!((semitones - (-12.0)).abs() < 1e-6);
    }
}
