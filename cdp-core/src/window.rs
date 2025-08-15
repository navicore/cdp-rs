use crate::{CoreError, Result};
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WindowFunction {
    Hann,
    Hamming,
    Blackman,
    Kaiser(f32),
    Rectangle,
}

pub struct Window {
    function: WindowFunction,
    size: usize,
    coefficients: Vec<f32>,
}

impl Window {
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
        
        for i in 0..size {
            let x = i as f32;
            coeffs[i] = match function {
                WindowFunction::Hann => {
                    0.5 * (1.0 - (2.0 * PI * x / (n - 1.0)).cos())
                }
                WindowFunction::Hamming => {
                    0.54 - 0.46 * (2.0 * PI * x / (n - 1.0)).cos()
                }
                WindowFunction::Blackman => {
                    0.42 - 0.5 * (2.0 * PI * x / (n - 1.0)).cos()
                        + 0.08 * (4.0 * PI * x / (n - 1.0)).cos()
                }
                WindowFunction::Kaiser(alpha) => {
                    // Simplified Kaiser window
                    let beta = PI * alpha;
                    let mut w = 1.0;
                    let center = (n - 1.0) / 2.0;
                    let t = (x - center) / center;
                    w *= (1.0 - t * t).sqrt().max(0.0);
                    w
                }
                WindowFunction::Rectangle => 1.0,
            };
        }
        
        coeffs
    }
    
    pub fn apply(&self, input: &mut [f32]) -> Result<()> {
        if input.len() != self.size {
            return Err(CoreError::WindowSizeMismatch(input.len(), self.size));
        }
        
        for (sample, coeff) in input.iter_mut().zip(&self.coefficients) {
            *sample *= coeff;
        }
        
        Ok(())
    }
    
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