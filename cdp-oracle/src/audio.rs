use crate::Result;
use cdp_core::fft::FftProcessor;
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use num_complex::Complex32;
use std::path::Path;

pub struct AudioFile {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
}

impl AudioFile {
    pub fn read<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut reader = WavReader::open(path)?;
        let spec = reader.spec();

        let samples: Vec<f32> = match spec.sample_format {
            SampleFormat::Float => reader.samples::<f32>().map(|s| s.unwrap()).collect(),
            SampleFormat::Int => {
                let bits = spec.bits_per_sample;
                let max = (1 << (bits - 1)) as f32;
                reader.samples::<i32>().map(|s| s.unwrap() as f32 / max).collect()
            }
        };

        Ok(AudioFile {
            samples,
            sample_rate: spec.sample_rate,
        })
    }

    pub fn write<P: AsRef<Path>>(path: P, samples: &[f32], sample_rate: u32) -> Result<()> {
        let spec = WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 32,
            sample_format: SampleFormat::Float,
        };

        let mut writer = WavWriter::create(path, spec)?;
        for &sample in samples {
            writer.write_sample(sample)?;
        }
        writer.finalize()?;

        Ok(())
    }
}

pub struct SpectralAnalyzer {
    fft_size: usize,
    processor: FftProcessor,
}

impl SpectralAnalyzer {
    pub fn new(fft_size: usize) -> Self {
        Self {
            fft_size,
            processor: FftProcessor::new(fft_size).unwrap(),
        }
    }

    pub fn analyze(&mut self, audio: &[f32]) -> Vec<f32> {
        let mut magnitudes = Vec::new();
        let mut buffer = vec![0.0; self.fft_size];
        let mut spectrum = vec![Complex32::new(0.0, 0.0); self.fft_size];

        // Process in chunks
        for chunk in audio.chunks(self.fft_size) {
            buffer.clear();
            buffer.extend_from_slice(chunk);

            // Pad if necessary
            while buffer.len() < self.fft_size {
                buffer.push(0.0);
            }

            // Compute FFT
            if self.processor.forward(&buffer, &mut spectrum).is_ok() {
                // Store magnitudes
                for c in spectrum.iter() {
                    magnitudes.push(c.norm());
                }
            }
        }

        magnitudes
    }

    pub fn compare_spectra(&self, a: &[f32], b: &[f32]) -> f32 {
        let min_len = a.len().min(b.len());
        if min_len == 0 {
            return 0.0;
        }

        let a = &a[..min_len];
        let b = &b[..min_len];

        // Normalized correlation
        let sum_a: f32 = a.iter().sum();
        let sum_b: f32 = b.iter().sum();

        if sum_a == 0.0 || sum_b == 0.0 {
            return 0.0;
        }

        let norm_a: Vec<f32> = a.iter().map(|x| x / sum_a).collect();
        let norm_b: Vec<f32> = b.iter().map(|x| x / sum_b).collect();

        // Cosine similarity
        let dot: f32 = norm_a.iter().zip(norm_b.iter()).map(|(x, y)| x * y).sum();
        let mag_a: f32 = norm_a.iter().map(|x| x * x).sum::<f32>().sqrt();
        let mag_b: f32 = norm_b.iter().map(|x| x * x).sum::<f32>().sqrt();

        if mag_a == 0.0 || mag_b == 0.0 {
            0.0
        } else {
            dot / (mag_a * mag_b)
        }
    }
}
