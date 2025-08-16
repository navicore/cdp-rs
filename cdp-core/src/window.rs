use crate::{CoreError, Result};
use std::f32::consts::PI;

/// Window function types for spectral processing
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowFunction {
    /// Hann (Hanning) window - good frequency resolution
    Hann,
    /// Hamming window - reduced spectral leakage
    Hamming,
    /// Blackman window - excellent sidelobe suppression
    Blackman,
    /// Kaiser window with configurable alpha parameter
    Kaiser(f32),
    /// Rectangular window (no windowing)
    Rectangle,
}

/// Window function generator and applicator
pub struct Window {
    #[allow(dead_code)] // Will be used for window type queries
    function: WindowFunction,
    size: usize,
    coefficients: Vec<f32>,
}

impl Window {
    /// Create a new window with the specified function and size
    pub fn new(function: WindowFunction, size: usize) -> Result<Self> {
        if size == 0 {
            return Err(CoreError::InvalidFftSize(size));
        }

        let coefficients = Self::calculate_coefficients(function, size);

        Ok(Window {
            function,
            size,
            coefficients,
        })
    }

    fn calculate_coefficients(function: WindowFunction, size: usize) -> Vec<f32> {
        let mut coeffs = vec![0.0; size];
        let n = size as f32;

        for (i, coeff) in coeffs.iter_mut().enumerate() {
            let x = i as f32;
            *coeff = match function {
                WindowFunction::Hann => 0.5 * (1.0 - (2.0 * PI * x / (n - 1.0)).cos()),
                WindowFunction::Hamming => 0.54 - 0.46 * (2.0 * PI * x / (n - 1.0)).cos(),
                WindowFunction::Blackman => {
                    0.42 - 0.5 * (2.0 * PI * x / (n - 1.0)).cos()
                        + 0.08 * (4.0 * PI * x / (n - 1.0)).cos()
                }
                WindowFunction::Kaiser(_alpha) => {
                    // Simplified Kaiser window - TODO: implement full Kaiser-Bessel
                    let center = (n - 1.0) / 2.0;
                    let t = (x - center) / center;
                    (1.0 - t * t).sqrt().max(0.0)
                }
                WindowFunction::Rectangle => 1.0,
            };
        }

        coeffs
    }

    /// Apply the window function to input samples
    pub fn apply(&self, input: &mut [f32]) -> Result<()> {
        if input.len() != self.size {
            return Err(CoreError::WindowSizeMismatch(input.len(), self.size));
        }

        for (sample, coeff) in input.iter_mut().zip(&self.coefficients) {
            *sample *= coeff;
        }

        Ok(())
    }

    /// Get the window coefficients
    pub fn coefficients(&self) -> &[f32] {
        &self.coefficients
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    #[test]
    fn test_hann_window() {
        let window = Window::new(WindowFunction::Hann, 4).unwrap();
        let coeffs = window.coefficients();

        assert_relative_eq!(coeffs[0], 0.0, epsilon = 1e-6);
        assert_relative_eq!(coeffs[1], 0.75, epsilon = 1e-6);
        assert_relative_eq!(coeffs[2], 0.75, epsilon = 1e-6);
        assert_relative_eq!(coeffs[3], 0.0, epsilon = 1e-6);
    }
}
