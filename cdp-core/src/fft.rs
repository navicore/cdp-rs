use crate::{CoreError, Result};
use num_complex::Complex32;
use rustfft::{FftPlanner, Fft as RustFft};
use std::sync::Arc;

pub struct FftProcessor {
    size: usize,
    forward: Arc<dyn RustFft<f32>>,
    inverse: Arc<dyn RustFft<f32>>,
    scratch: Vec<Complex32>,
}

impl FftProcessor {
    pub fn new(size: usize) -> Result<Self> {
        if !size.is_power_of_two() {
            return Err(CoreError::InvalidFftSize(size));
        }
        
        let mut planner = FftPlanner::new();
        let forward = planner.plan_fft_forward(size);
        let inverse = planner.plan_fft_inverse(size);
        
        Ok(FftProcessor {
            size,
            forward,
            inverse,
            scratch: vec![Complex32::new(0.0, 0.0); size],
        })
    }
    
    pub fn forward(&mut self, input: &[f32], output: &mut [Complex32]) -> Result<()> {
        if input.len() != self.size || output.len() != self.size {
            return Err(CoreError::InvalidFftSize(input.len()));
        }
        
        // Convert real to complex
        for (i, &sample) in input.iter().enumerate() {
            output[i] = Complex32::new(sample, 0.0);
        }
        
        self.forward.process_with_scratch(output, &mut self.scratch);
        Ok(())
    }
    
    pub fn inverse(&mut self, input: &mut [Complex32], output: &mut [f32]) -> Result<()> {
        if input.len() != self.size || output.len() != self.size {
            return Err(CoreError::InvalidFftSize(input.len()));
        }
        
        self.inverse.process_with_scratch(input, &mut self.scratch);
        
        // Normalize and convert to real
        let norm = 1.0 / self.size as f32;
        for (i, sample) in input.iter().enumerate() {
            output[i] = sample.re * norm;
        }
        
        Ok(())
    }
    
    pub fn size(&self) -> usize {
        self.size
    }
}

pub struct Fft;

impl Fft {
    /// Helper function to check if size is power of 2
    pub fn is_valid_size(size: usize) -> bool {
        size.is_power_of_two()
    }
    
    /// Get the next power of 2 >= n
    pub fn next_power_of_two(n: usize) -> usize {
        n.next_power_of_two()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    
    #[test]
    fn test_fft_roundtrip() {
        let mut processor = FftProcessor::new(64).unwrap();
        let input: Vec<f32> = (0..64).map(|i| (i as f32).sin()).collect();
        let mut spectrum = vec![Complex32::new(0.0, 0.0); 64];
        let mut output = vec![0.0; 64];
        
        processor.forward(&input, &mut spectrum).unwrap();
        processor.inverse(&mut spectrum, &mut output).unwrap();
        
        for (inp, out) in input.iter().zip(output.iter()) {
            assert_relative_eq!(inp, out, epsilon = 1e-5);
        }
    }
}