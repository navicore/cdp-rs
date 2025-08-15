use std::f32::consts::PI;

/// Generate test signals for validation
pub struct TestGenerator;

impl TestGenerator {
    /// Generate a sine wave
    pub fn sine_wave(frequency: f32, duration: f32, sample_rate: u32) -> Vec<f32> {
        let num_samples = (duration * sample_rate as f32) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            samples.push((2.0 * PI * frequency * t).sin());
        }

        samples
    }

    /// Generate white noise
    pub fn white_noise(duration: f32, sample_rate: u32) -> Vec<f32> {
        let num_samples = (duration * sample_rate as f32) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        // Simple LCG for reproducible "random" noise
        let mut seed = 12345u32;
        for _ in 0..num_samples {
            seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
            let value = ((seed / 65536) % 32768) as f32 / 16384.0 - 1.0;
            samples.push(value);
        }

        samples
    }

    /// Generate a chirp signal (frequency sweep)
    pub fn chirp(start_freq: f32, end_freq: f32, duration: f32, sample_rate: u32) -> Vec<f32> {
        let num_samples = (duration * sample_rate as f32) as usize;
        let mut samples = Vec::with_capacity(num_samples);

        for i in 0..num_samples {
            let t = i as f32 / sample_rate as f32;
            let progress = t / duration;
            let freq = start_freq + (end_freq - start_freq) * progress;
            samples.push((2.0 * PI * freq * t).sin());
        }

        samples
    }

    /// Generate an impulse
    pub fn impulse(sample_rate: u32) -> Vec<f32> {
        let mut samples = vec![0.0; sample_rate as usize];
        samples[0] = 1.0;
        samples
    }

    /// Generate a complex test signal with multiple harmonics
    pub fn harmonic_series(
        fundamental: f32,
        harmonics: usize,
        duration: f32,
        sample_rate: u32,
    ) -> Vec<f32> {
        let num_samples = (duration * sample_rate as f32) as usize;
        let mut samples = vec![0.0; num_samples];

        for h in 1..=harmonics {
            let freq = fundamental * h as f32;
            let amplitude = 1.0 / h as f32; // Natural harmonic decay

            for i in 0..num_samples {
                let t = i as f32 / sample_rate as f32;
                samples[i] += amplitude * (2.0 * PI * freq * t).sin();
            }
        }

        // Normalize
        let max = samples.iter().fold(0.0f32, |acc, &x| acc.max(x.abs()));
        if max > 0.0 {
            for sample in &mut samples {
                *sample /= max;
            }
        }

        samples
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sine_generation() {
        let signal = TestGenerator::sine_wave(440.0, 1.0, 44100);
        assert_eq!(signal.len(), 44100);
        assert!(signal.iter().all(|&x| x >= -1.0 && x <= 1.0));
    }

    #[test]
    fn test_noise_generation() {
        let signal = TestGenerator::white_noise(1.0, 44100);
        assert_eq!(signal.len(), 44100);
        assert!(signal.iter().all(|&x| x >= -1.0 && x <= 1.0));
    }
}
