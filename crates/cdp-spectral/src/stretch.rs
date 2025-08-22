//! Time-stretching operations using phase vocoder
//!
//! Stretches or compresses time without changing pitch.

use crate::ana_io::{read_ana_file, write_ana_file};
use crate::error::{Result, SpectralError};
use std::path::Path;

/// Time-stretch a spectral file
///
/// # Arguments
/// * `input_path` - Path to input .ana file
/// * `output_path` - Path to output .ana file
/// * `stretch_factor` - Time stretch factor (>1 = slower, <1 = faster)
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(SpectralError)` on failure
pub fn stretch_time(input_path: &Path, output_path: &Path, stretch_factor: f64) -> Result<()> {
    // Validate stretch factor
    if stretch_factor <= 0.0 {
        return Err(SpectralError::InvalidInput(
            "Stretch factor must be greater than 0".to_string(),
        ));
    }

    if !(0.01..=100.0).contains(&stretch_factor) {
        return Err(SpectralError::InvalidInput(
            "Stretch factor must be between 0.01 and 100".to_string(),
        ));
    }

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

    // Calculate output size
    let output_windows = (num_windows as f64 * stretch_factor).round() as usize;
    let mut output = Vec::with_capacity(output_windows * window_size);

    // Perform time stretching using linear interpolation of spectral frames
    for out_idx in 0..output_windows {
        // Calculate corresponding position in input
        let input_pos = out_idx as f64 / stretch_factor;
        let input_idx = input_pos.floor() as usize;
        let frac = input_pos - input_idx as f64;

        if input_idx >= num_windows - 1 {
            // Use last window
            let window_start = (num_windows - 1) * window_size;
            for chan in 0..window_size {
                output.push(samples[window_start + chan]);
            }
        } else {
            // Interpolate between two adjacent windows
            let window1_start = input_idx * window_size;
            let window2_start = (input_idx + 1) * window_size;

            // Process each channel (real/imaginary pairs)
            for chan in 0..window_size / 2 {
                let real_idx = chan * 2;
                let imag_idx = chan * 2 + 1;

                // Get complex values from both windows
                let real1 = samples[window1_start + real_idx];
                let imag1 = samples[window1_start + imag_idx];
                let real2 = samples[window2_start + real_idx];
                let imag2 = samples[window2_start + imag_idx];

                // Convert to polar
                let (mag1, phase1) = rect_to_polar(real1, imag1);
                let (mag2, phase2) = rect_to_polar(real2, imag2);

                // Interpolate magnitude
                let mag = mag1 + (mag2 - mag1) * frac as f32;

                // Interpolate phase (with unwrapping)
                let phase = interpolate_phase(phase1, phase2, frac as f32);

                // Convert back to rectangular
                let (real, imag) = polar_to_rect(mag, phase);

                output.push(real);
                output.push(imag);
            }
        }
    }

    // Write output .ana file
    write_ana_file(output_path, &header, &output)?;

    Ok(())
}

/// Apply time-varying stretch to spectrum
///
/// # Arguments
/// * `input_path` - Path to input .ana file
/// * `output_path` - Path to output .ana file
/// * `stretch_values` - Vec of (time, stretch_factor) pairs for time-varying stretch
///
/// # Returns
/// * `Ok(())` on success
/// * `Err(SpectralError)` on failure
pub fn stretch_time_varying(
    input_path: &Path,
    output_path: &Path,
    stretch_values: &[(f64, f64)],
) -> Result<()> {
    if stretch_values.is_empty() {
        return Err(SpectralError::InvalidInput(
            "Stretch values must not be empty".to_string(),
        ));
    }

    // Validate all stretch factors
    for (_, stretch) in stretch_values {
        if *stretch <= 0.0 || *stretch < 0.01 || *stretch > 100.0 {
            return Err(SpectralError::InvalidInput(
                "All stretch factors must be between 0.01 and 100".to_string(),
            ));
        }
    }

    // Read input .ana file
    let (header, samples) = read_ana_file(input_path)?;

    let window_size = header.channels as usize;
    let num_windows = samples.len() / window_size;

    // Calculate time per window from header metadata
    let hop_size = header.window_len / header.dec_factor;
    let time_per_window = hop_size as f64 / header.sample_rate as f64;

    // Calculate total output windows needed
    let mut output_windows = 0;
    let mut current_time = 0.0;
    let mut input_window = 0.0;

    while input_window < num_windows as f64 - 1.0 {
        let stretch = interpolate_stretch_value(current_time, stretch_values);
        let step = 1.0 / stretch;
        input_window += step;
        current_time = input_window * time_per_window;
        output_windows += 1;
    }

    // Allocate output buffer
    let mut output = Vec::with_capacity(output_windows * window_size);

    // Perform time-varying stretch
    current_time = 0.0;
    input_window = 0.0;

    for _ in 0..output_windows {
        let stretch = interpolate_stretch_value(current_time, stretch_values);

        // Get integer and fractional parts
        let input_idx = input_window.floor() as usize;
        let frac = input_window - input_idx as f64;

        if input_idx >= num_windows - 1 {
            // Use last window
            let window_start = (num_windows - 1) * window_size;
            for chan in 0..window_size {
                output.push(samples[window_start + chan]);
            }
        } else {
            // Interpolate between windows
            let window1_start = input_idx * window_size;
            let window2_start = (input_idx + 1) * window_size;

            for chan in 0..window_size / 2 {
                let real_idx = chan * 2;
                let imag_idx = chan * 2 + 1;

                let real1 = samples[window1_start + real_idx];
                let imag1 = samples[window1_start + imag_idx];
                let real2 = samples[window2_start + real_idx];
                let imag2 = samples[window2_start + imag_idx];

                let (mag1, phase1) = rect_to_polar(real1, imag1);
                let (mag2, phase2) = rect_to_polar(real2, imag2);

                let mag = mag1 + (mag2 - mag1) * frac as f32;
                let phase = interpolate_phase(phase1, phase2, frac as f32);

                let (real, imag) = polar_to_rect(mag, phase);

                output.push(real);
                output.push(imag);
            }
        }

        // Advance input position
        let step = 1.0 / stretch;
        input_window += step;
        current_time = input_window * time_per_window;
    }

    // Write output
    write_ana_file(output_path, &header, &output)?;

    Ok(())
}

/// Calculate output duration for a given stretch
pub fn calculate_output_duration(input_path: &Path, stretch_factor: f64) -> Result<f64> {
    // Open input to get duration
    let (header, samples) = read_ana_file(input_path)?;

    let num_samples = samples.len() as f64;
    let duration = num_samples / header.sample_rate as f64;

    Ok(duration * stretch_factor)
}

/// Convert rectangular to polar coordinates
fn rect_to_polar(real: f32, imag: f32) -> (f32, f32) {
    let mag = (real * real + imag * imag).sqrt();
    let phase = imag.atan2(real);
    (mag, phase)
}

/// Convert polar to rectangular coordinates
fn polar_to_rect(mag: f32, phase: f32) -> (f32, f32) {
    let real = mag * phase.cos();
    let imag = mag * phase.sin();
    (real, imag)
}

/// Interpolate phase with unwrapping
fn interpolate_phase(phase1: f32, phase2: f32, frac: f32) -> f32 {
    use std::f32::consts::PI;

    // Unwrap phase difference
    let mut diff = phase2 - phase1;
    while diff > PI {
        diff -= 2.0 * PI;
    }
    while diff < -PI {
        diff += 2.0 * PI;
    }

    // Linear interpolation
    let mut phase = phase1 + diff * frac;

    // Wrap result to [-PI, PI]
    while phase > PI {
        phase -= 2.0 * PI;
    }
    while phase < -PI {
        phase += 2.0 * PI;
    }

    phase
}

/// Helper function to interpolate stretch value at a given time
fn interpolate_stretch_value(time: f64, stretch_values: &[(f64, f64)]) -> f64 {
    // Find surrounding points
    let mut prev = stretch_values[0];
    let mut next = stretch_values[stretch_values.len() - 1];

    for i in 0..stretch_values.len() - 1 {
        if time >= stretch_values[i].0 && time <= stretch_values[i + 1].0 {
            prev = stretch_values[i];
            next = stretch_values[i + 1];
            break;
        }
    }

    // Before first point
    if time < stretch_values[0].0 {
        return stretch_values[0].1;
    }

    // After last point
    if time > stretch_values[stretch_values.len() - 1].0 {
        return stretch_values[stretch_values.len() - 1].1;
    }

    // Linear interpolation
    if (next.0 - prev.0).abs() < 1e-10 {
        return prev.1;
    }

    let ratio = (time - prev.0) / (next.0 - prev.0);
    prev.1 + ratio * (next.1 - prev.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stretch_validation() {
        let input = Path::new("test.ana");
        let output = Path::new("out.ana");

        // Test invalid stretch factors
        let result = stretch_time(input, output, 0.0);
        assert!(result.is_err());

        let result = stretch_time(input, output, -1.0);
        assert!(result.is_err());

        let result = stretch_time(input, output, 0.001);
        assert!(result.is_err());

        let result = stretch_time(input, output, 1000.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_phase_interpolation() {
        use std::f32::consts::PI;

        // Test basic interpolation
        let phase = interpolate_phase(0.0, PI / 2.0, 0.5);
        assert!((phase - PI / 4.0).abs() < 1e-6);

        // Test phase unwrapping
        let phase = interpolate_phase(3.0 * PI / 4.0, -3.0 * PI / 4.0, 0.5);
        assert!((phase - PI).abs() < 1e-6 || (phase + PI).abs() < 1e-6);
    }

    #[test]
    fn test_polar_conversion() {
        // Test conversion roundtrip
        let real = 3.0;
        let imag = 4.0;

        let (mag, phase) = rect_to_polar(real, imag);
        assert!((mag - 5.0).abs() < 1e-6);

        let (real2, imag2) = polar_to_rect(mag, phase);
        assert!((real2 - real).abs() < 1e-6);
        assert!((imag2 - imag).abs() < 1e-6);
    }
}
