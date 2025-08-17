# CDP Oracle CI Strategy

## Overview

Our CI/CD pipeline builds CDP from source and runs oracle tests on both Linux and macOS to ensure cross-platform compatibility.

## Strategy Decision

We chose **Option 3: Hybrid Approach** for the following reasons:

### ✅ What We Test in CI:
1. **CDP builds successfully** on Linux and macOS
2. **Binary operations work** (no audio playback required)
3. **File I/O operations** (WAV generation, analysis files)
4. **Core transformations** (time stretch, filtering, spectral ops)
5. **Cross-platform consistency** (same outputs on Linux/macOS)

### 🏠 What We Test Locally:
1. **Audio playback** (requires audio hardware)
2. **Perceptual quality** (human evaluation)
3. **Interactive demos** (real-time processing)
4. **Performance profiling** (hardware-specific)

## CI Workflows

### 1. Main Validation (`validation.yml`)
- Runs on every push/PR
- Basic Rust tests + CDP oracle tests
- Ubuntu-only for speed
- Non-blocking oracle tests (continue-on-error)

### 2. CDP Oracle Tests (`cdp-oracle.yml`)
- Triggered by CDP-related changes
- Matrix testing: Ubuntu + macOS
- Builds CDP from source (cached)
- Runs comprehensive test suite
- Uploads artifacts for inspection

## Test Levels

### Level 1: Build Verification ✅
```bash
# Can we build CDP?
make build-cdp
```

### Level 2: Basic Operations ✅
```bash
# Do core programs work?
./scripts/test-cdp-ci.sh
```

### Level 3: Oracle Comparison 🔄
```rust
// Compare CDP vs Rust outputs
assert!(compare_outputs(cdp_output, rust_output) < 0.01);
```

### Level 4: Performance 📊
```bash
# Benchmark CDP vs Rust
time cdp_program input.wav output.wav
time rust_program input.wav output.wav
```

## Platform Differences

### Linux (Ubuntu)
- **Pros**: Fast, free CI minutes, consistent environment
- **Cons**: No audio output, different from dev machines
- **Use for**: Build verification, binary testing

### macOS
- **Pros**: Matches development environment, audio support
- **Cons**: Slower, limited CI minutes
- **Use for**: Final validation, audio quality checks

## Caching Strategy

We cache:
1. **CDP source + build** (~5 min build time saved)
2. **Rust dependencies** (~2 min saved)
3. **Test artifacts** (for debugging failures)

Cache keys:
- CDP: `${{ runner.os }}-cdp-${{ hashFiles('scripts/build-cdp.sh') }}`
- Rust: `${{ runner.os }}-cargo-${{ hashFiles('**/Cargo.lock') }}`

## Audio Testing Without Playback

### Binary Comparison
```python
# Compare WAV files byte-by-byte (with tolerance)
def compare_audio(file1, file2, tolerance=0.001):
    # Read samples, compare values
    return difference < tolerance
```

### Spectral Analysis
```bash
# Use CDP's own analysis tools
pvoc anal input.wav input.ana
# Compare .ana files for spectral consistency
```

### Statistical Validation
- Sample count matching
- RMS energy comparison  
- Peak detection
- Zero-crossing rate

## Future Enhancements

1. **Golden Files**: Store reference outputs for regression testing
2. **Docker Containers**: Consistent build environment
3. **Self-Hosted Runners**: For audio hardware testing
4. **Fuzzing**: Generate random inputs for stability testing
5. **A/B Testing**: Compare different CDP versions

## Manual Testing Protocol

For releases, manually run:
```bash
# Full test suite with audio
make test-cdp

# Interactive demo
make demo-cdp

# Listen to outputs
afplay test-output/*.wav
```

## Debugging CI Failures

1. **Check artifacts**: Download test outputs from failed runs
2. **Run locally**: `act` tool simulates GitHub Actions locally
3. **Matrix isolation**: Test specific OS with `workflow_dispatch`
4. **Verbose mode**: Add `-v` flags to CDP commands for debugging

## Success Metrics

- ✅ **Build Success Rate**: >95% across platforms
- ✅ **Test Coverage**: Core CDP operations tested
- ✅ **Oracle Accuracy**: <1% difference in outputs
- ✅ **CI Time**: <10 minutes per run
- ✅ **Cache Hit Rate**: >80% for CDP builds

## Conclusion

This hybrid approach gives us:
- **Fast feedback** on every commit
- **Cross-platform validation** without manual testing
- **Oracle confidence** without audio hardware
- **Local flexibility** for perceptual testing

The CI ensures our Rust implementations match CDP's behavior, while local testing ensures they sound good!