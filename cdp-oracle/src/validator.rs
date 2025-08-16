use crate::audio::{AudioFile, SpectralAnalyzer};
use crate::{CdpOracle, OracleConfig, Result};

/// Trait that all CDP processors must implement for oracle testing
pub trait CdpProcessor: Send + Sync {
    /// The name of the equivalent CDP binary
    fn cdp_program_name(&self) -> &str;

    /// Arguments to pass to the CDP binary
    fn cdp_args(&self) -> Vec<String>;

    /// Process audio data
    fn process(&self, input: &[f32], sample_rate: u32) -> Result<Vec<f32>>;
}

#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub passed: bool,
    pub program: String,
    pub sample_correlation: f32,
    pub spectral_correlation: f32,
    pub max_difference: f32,
    pub rms_difference: f32,
}

impl ValidationResult {
    pub fn report(&self) -> String {
        format!(
            "Program: {}\nPassed: {}\nSample Correlation: {:.6}\nSpectral Correlation: {:.6}\nMax Difference: {:.6}\nRMS Difference: {:.6}",
            self.program,
            self.passed,
            self.sample_correlation,
            self.spectral_correlation,
            self.max_difference,
            self.rms_difference
        )
    }
}

pub struct Validator {
    oracle: CdpOracle,
    analyzer: SpectralAnalyzer,
}

impl Validator {
    pub fn new(config: OracleConfig) -> Result<Self> {
        Ok(Self {
            oracle: CdpOracle::new(config)?,
            analyzer: SpectralAnalyzer::new(2048),
        })
    }

    /// Validate a Rust processor against its CDP equivalent
    pub fn validate<P: CdpProcessor>(
        &mut self,
        processor: &P,
        test_audio: &[f32],
        sample_rate: u32,
    ) -> Result<ValidationResult> {
        // Save input to temp file
        let temp_dir = self.oracle.temp_dir()?;
        let input_path = temp_dir.join("input.wav");
        let output_path = temp_dir.join("output_cdp.wav");

        AudioFile::write(&input_path, test_audio, sample_rate)?;

        // Run CDP binary
        let cdp_args = processor.cdp_args();
        let mut args = vec![input_path.to_str().unwrap(), output_path.to_str().unwrap()];
        for arg in cdp_args.iter() {
            args.push(arg);
        }

        self.oracle.run_cdp(processor.cdp_program_name(), &args)?;

        // Load CDP output
        let cdp_output = AudioFile::read(&output_path)?;

        // Run Rust implementation
        let rust_output = processor.process(test_audio, sample_rate)?;

        // Compare outputs
        self.compare_outputs(
            processor.cdp_program_name(),
            &cdp_output.samples,
            &rust_output,
        )
    }

    fn compare_outputs(
        &mut self,
        program: &str,
        cdp: &[f32],
        rust: &[f32],
    ) -> Result<ValidationResult> {
        // Ensure same length (CDP might add/remove samples)
        let min_len = cdp.len().min(rust.len());
        let cdp = &cdp[..min_len];
        let rust = &rust[..min_len];

        // Sample-level comparison
        let sample_correlation = self.calculate_correlation(cdp, rust);

        // Spectral comparison (more forgiving of small differences)
        let cdp_spectrum = self.analyzer.analyze(cdp);
        let rust_spectrum = self.analyzer.analyze(rust);
        let spectral_correlation = self.analyzer.compare_spectra(&cdp_spectrum, &rust_spectrum);

        // Calculate differences
        let max_diff = cdp
            .iter()
            .zip(rust.iter())
            .map(|(a, b)| (a - b).abs())
            .fold(0.0f32, f32::max);

        let rms_diff = {
            let sum: f32 = cdp
                .iter()
                .zip(rust.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum();
            (sum / min_len as f32).sqrt()
        };

        let passed = spectral_correlation >= self.oracle.config.spectral_threshold;

        Ok(ValidationResult {
            passed,
            program: program.to_string(),
            sample_correlation,
            spectral_correlation,
            max_difference: max_diff,
            rms_difference: rms_diff,
        })
    }

    fn calculate_correlation(&self, a: &[f32], b: &[f32]) -> f32 {
        let n = a.len() as f32;
        let sum_a: f32 = a.iter().sum();
        let sum_b: f32 = b.iter().sum();
        let sum_aa: f32 = a.iter().map(|x| x * x).sum();
        let sum_bb: f32 = b.iter().map(|x| x * x).sum();
        let sum_ab: f32 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();

        let numerator = n * sum_ab - sum_a * sum_b;
        let denominator = ((n * sum_aa - sum_a * sum_a) * (n * sum_bb - sum_b * sum_b)).sqrt();

        if denominator == 0.0 {
            0.0
        } else {
            numerator / denominator
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[allow(dead_code)]
    struct TestProcessor;

    impl CdpProcessor for TestProcessor {
        fn cdp_program_name(&self) -> &str {
            "test"
        }

        fn cdp_args(&self) -> Vec<String> {
            vec![]
        }

        fn process(&self, input: &[f32], _sample_rate: u32) -> Result<Vec<f32>> {
            Ok(input.to_vec())
        }
    }

    #[test]
    fn test_validator_creation() {
        let config = OracleConfig::default();
        let validator = Validator::new(config);
        assert!(validator.is_ok());
    }
}
