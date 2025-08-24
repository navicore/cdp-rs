# CDP-RS Project: Lessons Learned

## Executive Summary

This project attempted to create Rust implementations of CDP (Composers Desktop Project) audio processing tools with exact compatibility verified through oracle testing. While we successfully implemented one module (`cdp-housekeep/copy`), we discovered that achieving exact compatibility with CDP's 35-year-old C codebase is effectively impossible without extensive reverse engineering that may not be worth the effort.

## What Worked

### 1. cdp-housekeep/copy ✅
- **Status**: Fully working with oracle tests passing
- **Key insight**: CDP uses a specific WAV format with additional chunks:
  - PEAK chunk: Contains peak amplitude and position with timestamp
  - cue chunk: Cue points for navigation
  - LIST chunk: Metadata
- **Implementation**: Created `wav_cdp.rs` module that writes CDP-compatible WAV files
- **Verification**: Oracle tests pass - our output matches CDP's byte-for-byte (except timestamps)

### 2. Test Infrastructure ✅
- **cdp-oracle module**: Successfully created test utilities and WAV comparison tools
- **Smart comparison**: `wav_compare.rs` can compare WAV files while ignoring expected differences (timestamps)
- **CDP binary detection**: `test_utils.rs` finds CDP binaries for oracle testing

### 3. Build System ✅
- **Makefile**: Unified build system where `make` locally matches CI exactly
- **CDP installation**: Automated CDP source compilation via `scripts/build-cdp.sh`
- **CI/CD**: GitHub Actions runs exact same commands as local development

## What Failed

### 1. cdp-distort/multiply ❌
- **Problem**: Even after studying CDP's source code extensively, our implementation doesn't match CDP's output
- **What we learned**:
  - "Multiply" means wavecycle frequency multiplication, not amplitude
  - Algorithm: Detect zero-crossings, compress cycles, repeat with alternating phase
  - Implementation has many subtle details (interpolation, smoothing, edge cases)
- **Why it failed**: Small differences in zero-crossing detection, interpolation, or rounding accumulate into different output

### 2. Other Modules ❌
- **cdp-pvoc**: Spectral analysis format (.ana files) too complex to reverse-engineer
- **cdp-spectral**: Depends on pvoc, never got working
- **cdp-modify**: Not attempted due to earlier failures

## Key Discoveries

### 1. CDP's Hidden Complexity
CDP operations that seem simple are actually complex:
- "multiply" isn't multiplication, it's wavecycle manipulation
- "blur" isn't simple blurring, it's spectral bin averaging with phase handling
- Every operation has undocumented edge cases and special handling

### 2. The Oracle Testing Paradox
- We need oracle tests to verify correctness
- But CDP's implementations have undocumented behaviors
- Small implementation differences cause test failures
- We can't distinguish between "different but correct" and "wrong"

### 3. Why Exact Compatibility Is Nearly Impossible

#### Undocumented Algorithms
- CDP's documentation describes what operations do musically, not how they work technically
- Source code is the only reference, but it's 35 years of accumulated complexity

#### Implementation Details Matter
```c
// CDP code example from distortm.c
index = round((double)m * (double)dz->iparam[DISTORTM_FACTOR]);
ob1[n] = b1[index];
```
Small details like:
- Rounding vs truncation
- Interpolation methods
- Buffer boundary handling
- Phase accumulation
- Smoothing filters

All affect the output, and getting them ALL exactly right is extremely difficult.

#### Floating Point Differences
- Different compilers/platforms may produce slightly different floating-point results
- CDP was written for 1990s hardware/compilers
- Modern optimizations change calculation order

### 4. The Module Dependency Problem
- Many CDP modules depend on each other
- Can't implement `spectral` without working `pvoc`
- Can't verify `pvoc` without understanding the undocumented .ana format
- Creates a chicken-and-egg problem

## Technical Debt in CDP

### 1. Magic Numbers
```c
ob1[mid_cycle] = ob1[mid_cycle-1]/2;  // Why /2?
val = sample * 0.5f;                   // Why 0.5?
```

### 2. Inconsistent Interfaces
- Some programs take integer parameters, others float
- Some require mono, others accept stereo
- Error handling is inconsistent

### 3. Format Assumptions
- Assumes specific WAV chunk ordering
- Hard-coded buffer sizes
- Platform-specific code

## What We Should Have Done Differently

### 1. Started with Better Documentation
Should have insisted on algorithmic documentation before starting implementation.

### 2. Relaxed Compatibility Requirements
Instead of exact output matching, could have aimed for "musically equivalent" results.

### 3. Contact Original Authors
The CDP authors might have provided insights into the algorithms that would save months of reverse engineering.

### 4. Different Testing Strategy
Instead of binary oracle testing (pass/fail), could use:
- Spectral analysis to verify frequency content matches
- Perceptual tests for musical equivalence
- Statistical similarity rather than exact matching

## Recommendations for Anyone Attempting This

### If You Want Exact CDP Compatibility
**Don't.** It's not feasible without:
- Complete algorithmic documentation
- Access to original developers
- Enormous reverse-engineering effort
- Acceptance that some differences are inevitable

### If You Want CDP-Like Functionality
1. Use CDP binaries via subprocess calls (what we originally rejected)
2. Create "inspired by" implementations that are musically similar
3. Focus on modern DSP techniques rather than copying 35-year-old code
4. Build comprehensive test suites based on musical outcomes, not binary comparison

### If You Must Continue This Project
1. Start with the working `cdp-housekeep/copy` module
2. Accept that oracle tests will need tolerance for small differences
3. Document every algorithm discovery meticulously
4. Consider it a learning/research project, not production software

## Code That Should Be Preserved

### 1. wav_cdp.rs
The CDP WAV format implementation works and could be useful for other projects that need to interact with CDP.

### 2. wav_compare.rs
Smart WAV comparison that ignores timestamps is genuinely useful.

### 3. test_utils.rs
CDP binary detection across different installation methods.

### 4. Build System
The Makefile and scripts/build-cdp.sh successfully build CDP from source on multiple platforms.

## Final Verdict

The project demonstrated that while it's possible to create Rust implementations of CDP operations, achieving exact compatibility is not feasible without unreasonable effort. The one working module (`cdp-housekeep/copy`) proves the concept but also shows how much work is required for even the simplest operation.

The fundamental issue is that **we cannot verify correctness without exact compatibility, and we cannot achieve exact compatibility without understanding every implementation detail of 35-year-old undocumented C code.**

This is a catch-22 that makes the project's goal - reliable, verifiably correct Rust implementations of CDP - effectively impossible to achieve.

## Useful Commands for Future Reference

```bash
# Build CDP from source
make install-cdp

# Run oracle tests (for the working module)
cargo test -p cdp-housekeep test_copy_matches_cdp

# Compare CDP vs Rust output
build/cdp-install/bin/housekeep copy 1 input.wav cdp_out.wav
cargo run --bin housekeep -- copy input.wav rust_out.wav

# Debug WAV file differences
hexdump -C cdp_out.wav > cdp.hex
hexdump -C rust_out.wav > rust.hex
diff cdp.hex rust.hex
```

---

*Project suspended: August 2024*
*Reason: Cannot achieve verifiable correctness through oracle testing*
*Success rate: 1/30+ modules working correctly*