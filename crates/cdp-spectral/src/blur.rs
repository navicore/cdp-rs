//! Spectral blurring operations
//!
//! Time-averages the spectrum across multiple windows to create a blurred effect.

use crate::ana_io::{read_ana_file, write_ana_file};
use crate::error::{Result, SpectralError};
use std::path::Path;

/// Time-average the spectrum across multiple windows
///
/// # Arguments
/// * `input_path` - Path to input .ana file
/// * `output_path` - Path to output .ana file  
/// * `blur_windows` - Number of windows to average across (must be odd)
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(SpectralError)` on failure
pub fn blur(input_path: &Path, output_path: &Path, blur_windows: u32) -> Result<()> {
    // Validate blur_windows
    if blur_windows == 0 {
        return Err(SpectralError::InvalidInput(
            "Blur windows must be greater than 0".to_string(),
        ));
    }

    // Make blur_windows odd if it isn't already
    let blur_windows = if blur_windows % 2 == 0 {
        blur_windows + 1
    } else {
        blur_windows
    };

    let blur_span = blur_windows / 2; // Number of windows on each side

    // Read input .ana file
    let (header, samples) = read_ana_file(input_path)?;

    // Calculate window size (samples per window)
    let window_size = header.channels as usize;
    let num_windows = samples.len() / window_size;

    if num_windows == 0 {
        return Err(SpectralError::InvalidInput(
            "Input file has no spectral data".to_string(),
        ));
    }

    // Allocate output buffer
    let mut output = Vec::with_capacity(samples.len());

    // Process each window
    for window_idx in 0..num_windows {
        // Calculate averaging range
        let start_window = window_idx.saturating_sub(blur_span as usize);

        let end_window = if window_idx + (blur_span as usize) < num_windows {
            window_idx + blur_span as usize + 1
        } else {
            num_windows
        };

        let actual_blur_windows = end_window - start_window;

        // Average each channel across the blur windows
        for chan in 0..window_size {
            let mut sum = 0.0f32;

            for w in start_window..end_window {
                let sample_idx = w * window_size + chan;
                sum += samples[sample_idx];
            }

            output.push(sum / actual_blur_windows as f32);
        }
    }

    // Write output .ana file
    write_ana_file(output_path, &header, &output)?;

    Ok(())
}

/// Apply time-varying blur to spectrum
///
/// # Arguments
/// * `input_path` - Path to input .ana file
/// * `output_path` - Path to output .ana file
/// * `blur_values` - Vec of (time, blur_windows) pairs for time-varying blur
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(SpectralError)` on failure
pub fn blur_varying(
    input_path: &Path,
    output_path: &Path,
    blur_values: &[(f64, u32)],
) -> Result<()> {
    if blur_values.is_empty() {
        return Err(SpectralError::InvalidInput(
            "Blur values must not be empty".to_string(),
        ));
    }

    // Read input .ana file
    let (header, samples) = read_ana_file(input_path)?;

    let window_size = header.channels as usize;
    let num_windows = samples.len() / window_size;

    // Calculate time per window from header metadata
    let hop_size = header.window_len / header.dec_factor;
    let time_per_window = hop_size as f64 / header.sample_rate as f64;

    // Allocate output buffer
    let mut output = Vec::with_capacity(samples.len());

    // Process each window with interpolated blur value
    for window_idx in 0..num_windows {
        let current_time = window_idx as f64 * time_per_window;

        // Interpolate blur value at current time
        let blur_windows = interpolate_blur_value(current_time, blur_values);
        let blur_windows = if blur_windows % 2 == 0 {
            blur_windows + 1
        } else {
            blur_windows
        };
        let blur_span = blur_windows / 2;

        // Calculate averaging range
        let start_window = window_idx.saturating_sub(blur_span as usize);

        let end_window = if window_idx + (blur_span as usize) < num_windows {
            window_idx + blur_span as usize + 1
        } else {
            num_windows
        };

        let actual_blur_windows = end_window - start_window;

        // Average each channel
        for chan in 0..window_size {
            let mut sum = 0.0f32;

            for w in start_window..end_window {
                let sample_idx = w * window_size + chan;
                sum += samples[sample_idx];
            }

            output.push(sum / actual_blur_windows as f32);
        }
    }

    // Write output
    write_ana_file(output_path, &header, &output)?;

    Ok(())
}

/// Helper function to interpolate blur value at a given time
fn interpolate_blur_value(time: f64, blur_values: &[(f64, u32)]) -> u32 {
    // Find surrounding points
    let mut prev = blur_values[0];
    let mut next = blur_values[blur_values.len() - 1];

    for i in 0..blur_values.len() - 1 {
        if time >= blur_values[i].0 && time <= blur_values[i + 1].0 {
            prev = blur_values[i];
            next = blur_values[i + 1];
            break;
        }
    }

    // Before first point
    if time < blur_values[0].0 {
        return blur_values[0].1;
    }

    // After last point
    if time > blur_values[blur_values.len() - 1].0 {
        return blur_values[blur_values.len() - 1].1;
    }

    // Linear interpolation
    if (next.0 - prev.0).abs() < 1e-10 {
        return prev.1;
    }

    let ratio = (time - prev.0) / (next.0 - prev.0);
    let interpolated = prev.1 as f64 + ratio * (next.1 as f64 - prev.1 as f64);
    interpolated.round() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blur_validation() {
        let input = Path::new("test.ana");
        let output = Path::new("out.ana");

        // Test zero blur windows
        let result = blur(input, output, 0);
        assert!(result.is_err());
        assert!(matches!(result, Err(SpectralError::InvalidInput(_))));
    }

    #[test]
    fn test_interpolate_blur_value() {
        let blur_values = vec![(0.0, 1), (1.0, 5), (2.0, 3)];

        // Test exact points
        assert_eq!(interpolate_blur_value(0.0, &blur_values), 1);
        assert_eq!(interpolate_blur_value(1.0, &blur_values), 5);
        assert_eq!(interpolate_blur_value(2.0, &blur_values), 3);

        // Test interpolation
        assert_eq!(interpolate_blur_value(0.5, &blur_values), 3); // Halfway between 1 and 5
        assert_eq!(interpolate_blur_value(1.5, &blur_values), 4); // Halfway between 5 and 3

        // Test before first point
        assert_eq!(interpolate_blur_value(-1.0, &blur_values), 1);

        // Test after last point
        assert_eq!(interpolate_blur_value(3.0, &blur_values), 3);
    }
}
