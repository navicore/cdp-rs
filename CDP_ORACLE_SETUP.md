# CDP Oracle Setup - Verification Guide

## ✅ Build Confirmation

Your CDP oracle is **successfully built and operational**! We have:

- **218 CDP programs** compiled and ready
- **Spectral processing** (PVOC) working
- **Time domain processing** working
- **No external dependencies** required

## Quick Verification

Run these commands to confirm everything works:

```bash
# 1. Quick test - should show CDP version
build/cdp-install/bin/housekeep 2>&1 | head -1
# Expected: "CDP Release 7.1 2016"

# 2. Run full test suite
make test-cdp
# Expected: Multiple tests passing, generating WAV files

# 3. Run interactive demo
make demo-cdp
# Expected: Generates processed audio files
```

## What's Working

### Core CDP Operations Verified ✅
- **pvoc** - Phase vocoder analysis/synthesis
- **blur** - Spectral blurring  
- **modify** - Time/pitch modifications
- **housekeep** - File management
- **filter** - Frequency filtering
- **distort** - Distortion effects
- **grain** - Granular synthesis
- **extend** - Time stretching
- **envel** - Envelope shaping

### Test Audio Generation
- Python script generates test WAV files
- No external dependencies (numpy not required)
- Creates proper 16-bit PCM WAV format

## Oracle Testing Architecture

```
cdp-rs/
├── build/
│   ├── cdp/                    # CDP source (auto-downloaded)
│   └── cdp-install/
│       ├── bin/                 # 218 CDP binaries
│       └── env.sh               # Environment setup
├── scripts/
│   ├── build-cdp.sh            # Builds CDP from source
│   ├── test-cdp.sh             # Comprehensive test suite
│   ├── generate-test-audio.py  # WAV file generator
│   └── cdp-demo.sh             # Interactive demo
└── test-output/                 # Test results
```

## Using CDP for Oracle Tests

### In Your Rust Tests

```rust
// Example oracle test structure
#[test]
fn test_against_cdp_oracle() {
    // Set CDP path
    std::env::set_var("CDP_PATH", "build/cdp-install/bin");
    
    // Generate test input
    let input = generate_test_signal();
    
    // Run through CDP
    let cdp_output = run_cdp_command("blur", &input);
    
    // Run through Rust implementation
    let rust_output = our_blur_implementation(&input);
    
    // Compare outputs
    assert_close_enough(cdp_output, rust_output);
}
```

### Manual Testing

```bash
# Set environment
source build/cdp-install/env.sh

# Test a specific CDP program
echo "input.wav" | pvoc anal 1 - output.ana

# Compare with Rust implementation
cargo run --bin pvoc-anal input.wav output_rust.ana
diff output.ana output_rust.ana
```

## Key Programs for Oracle Testing

1. **pvoc** - The crown jewel, phase vocoder
2. **blur** - Spectral manipulation baseline
3. **modify speed** - Time stretching reference
4. **filter** - Frequency domain reference
5. **grain** - Granular synthesis reference

## Troubleshooting

If tests fail:

1. **Rebuild CDP**: `make clean-cdp && make build-cdp`
2. **Check paths**: `ls -la build/cdp-install/bin | head`
3. **Test manually**: `build/cdp-install/bin/housekeep`
4. **Check Python**: `python3 scripts/generate-test-audio.py test.wav`

## Next Steps

1. Start implementing Rust versions of CDP algorithms
2. Use CDP outputs as ground truth for testing
3. Create automated comparison tests
4. Build confidence in Rust implementations

The oracle is alive and ready to validate your Rust implementations! 🎉