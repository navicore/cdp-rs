//! Integration tests for CDP-RS

#[cfg(test)]
mod tests {
    use cdp_oracle::{TestGenerator, OracleConfig, Validator};
    use cdp_sandbox::experiments::ExperimentalPvoc;
    
    #[test]
    #[ignore] // Remove when CDP binaries are available
    fn test_pvoc_validation() {
        let config = OracleConfig::default();
        let validator = Validator::new(config).unwrap();
        
        let pvoc = ExperimentalPvoc::new(2048, 4).unwrap();
        let test_signal = TestGenerator::sine_wave(440.0, 0.1, 44100);
        
        let result = validator.validate(&pvoc, &test_signal, 44100);
        
        // This will fail until we have real CDP binaries and implementation
        // assert!(result.is_ok());
    }
    
    #[test]
    fn test_signal_generation() {
        let sine = TestGenerator::sine_wave(440.0, 1.0, 44100);
        assert_eq!(sine.len(), 44100);
        
        let noise = TestGenerator::white_noise(1.0, 44100);
        assert_eq!(noise.len(), 44100);
        
        let chirp = TestGenerator::chirp(100.0, 1000.0, 1.0, 44100);
        assert_eq!(chirp.len(), 44100);
    }
}