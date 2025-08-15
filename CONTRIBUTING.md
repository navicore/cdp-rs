# Contributing to CDP-RS

Thank you for contributing to the preservation and modernization of CDP!

## Development Workflow

### 1. Choose a CDP Program to Port

Check our [progress tracker](README.md#status) and pick an unimplemented CDP program.

### 2. Develop in Sandbox

All new development happens in `cdp-sandbox/`:

```rust
// cdp-sandbox/src/experiments.rs
pub struct NewProcessor {
    // Your implementation
}

impl CdpProcessor for NewProcessor {
    fn cdp_program_name(&self) -> &str {
        "program_name"  // Must match CDP binary name
    }
    
    fn cdp_args(&self) -> Vec<String> {
        // Arguments that match CDP's CLI
    }
    
    fn process(&self, input: &[f32], sample_rate: u32) -> Result<Vec<f32>> {
        // Your implementation
    }
}
```

### 3. Validate Against CDP

Create comprehensive tests:

```rust
#[test]
fn validate_new_processor() {
    let oracle = CdpOracle::new(OracleConfig::default());
    let processor = NewProcessor::new();
    
    // Test with multiple input types
    for test_signal in [
        TestGenerator::sine_wave(440.0, 1.0, 44100),
        TestGenerator::white_noise(1.0, 44100),
        TestGenerator::chirp(100.0, 1000.0, 1.0, 44100),
    ] {
        let result = oracle.validate(&processor, &test_signal, 44100);
        assert!(result.passed, "Failed: {}", result.report());
    }
}
```

### 4. Achieve Validation Threshold

Your implementation must achieve:
- Spectral correlation > 0.9999
- RMS difference < 0.001
- Pass all oracle tests

### 5. Move to Frozen Module

Once validated:
1. Move code from `cdp-sandbox/` to appropriate module
2. Add `#![forbid(unsafe_code)]` to the module
3. Update `FROZEN_MODULES.md`
4. Create PR with validation results

## Testing Guidelines

### Oracle Testing
```bash
# Run oracle tests with CDP binaries
CDP_PATH=/path/to/cdp/bin cargo test --package cdp-oracle

# Run specific processor validation
cargo test validate_processor_name
```

### Performance Testing
```bash
# Run benchmarks
cargo bench --package cdp-core

# Compare with CDP performance
cargo run --example benchmark_vs_cdp
```

## Code Style

- Follow Rust naming conventions
- Document all public APIs
- Include examples in doc comments
- Maintain CDP compatibility in parameter names

## Licensing

By contributing, you agree to license your work under LGPL-2.1, matching CDP's original license.

## Questions?

Open an issue for:
- Clarification on CDP behavior
- Help with oracle test failures
- Architecture decisions
- License questions