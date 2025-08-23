# CDP-RS Implementation Plan

## Current Status

### Module Test Results
- ✅ **cdp-core**: All tests passing (FFT, windows - foundation working)
- ❌ **cdp-distort**: 5 oracle tests failing (multiply, divide, overload)
- ❌ **cdp-housekeep**: 1 test failing (basic_copy)
- ✅ **cdp-modify**: No tests yet (empty module)
- ❌ **cdp-pvoc**: 4 tests failing (format tests, oracle tests)
- ❌ **cdp-spectral**: 4 oracle tests failing (blur, stretch)
- ✅ **cdp-sndinfo**: No tests yet (empty module)

## Implementation Order

We'll implement modules in order of increasing complexity, ensuring each foundation is solid before building on it.

### Phase 1: File I/O Foundation (Week 1)
**Goal**: Get CDP WAV format working correctly

1. **cdp-housekeep/copy** 
   - Fix WAV format to match CDP (with PEAK chunks)
   - This is the simplest CDP program and tests our file I/O
   - Success metric: `test_basic_copy` passes

2. **cdp-housekeep/chans**
   - Channel extraction and manipulation
   - Builds on correct WAV I/O
   - Success metric: Oracle tests for channel operations pass

### Phase 2: Simple DSP (Week 2)
**Goal**: Implement basic time-domain processing

3. **cdp-modify/loudness**
   - Gain, normalize, balance
   - Simple amplitude modifications
   - Success metric: Oracle tests for modify operations pass

4. **cdp-distort**
   - multiply, divide, overload
   - Time-domain distortion effects
   - Success metric: All 5 oracle tests pass

### Phase 3: Phase Vocoder Core (Week 3-4)
**Goal**: Get spectral analysis/synthesis working

5. **cdp-pvoc**
   - Fix .ana file format (IEEE float WAV with CDP metadata)
   - Implement proper FFT analysis/synthesis
   - Success metric: Round-trip test passes, format tests pass

### Phase 4: Spectral Processing (Week 5-6)
**Goal**: Implement spectral domain effects

6. **cdp-spectral/blur**
   - Spectral blurring
   - First real spectral effect
   - Success metric: blur oracle tests pass

7. **cdp-spectral/stretch**
   - Time stretching
   - More complex spectral manipulation
   - Success metric: stretch oracle tests pass

8. **cdp-spectral/pitch**
   - Pitch shifting
   - Frequency domain manipulation
   - Success metric: pitch oracle tests pass

### Phase 5: Utilities (Week 7)
**Goal**: Complete the toolkit

9. **cdp-sndinfo**
   - File analysis and information
   - Success metric: Matches CDP sndinfo output

## Implementation Strategy

### For Each Module:

1. **Study CDP Source**
   - Find corresponding CDP source in `build/cdp-8.7.1/dev/`
   - Understand the exact algorithm
   - Note any special cases or quirks

2. **Analyze Oracle Test Failures**
   - Run oracle test with debug output
   - Binary compare outputs
   - Identify exact differences

3. **Fix Implementation**
   - Match CDP's algorithm exactly
   - Pay attention to:
     - Audio format details (bit depth, sample rate)
     - PEAK chunk generation
     - Metadata in LIST chunks
     - Edge cases and error handling

4. **Validate**
   - Oracle tests must pass
   - Add unit tests for edge cases
   - Document any CDP quirks discovered

## Disabling Failing Tests

To work incrementally, we'll temporarily disable failing tests:

### Option 1: Mark tests as ignored
```rust
#[test]
#[ignore] // TODO: Enable when implementing Phase X
fn test_multiply_matches_cdp() {
```

### Option 2: Feature flags for oracle tests
```toml
[features]
oracle-tests = []

[dev-dependencies]
cdp-oracle = { path = "../cdp-oracle", optional = true }
```

### Option 3: Separate oracle test binary
Move oracle tests to `tests/oracle/` and only run when ready.

## Success Criteria

- All oracle tests pass for implemented modules
- No use of `#[ignore]` in final code
- Each module has comprehensive unit tests
- Documentation explains any CDP compatibility notes

## Notes

- CDP has 35+ years of accumulated quirks - document them all
- Some CDP behaviors may seem wrong but we must match them exactly
- Use `hexdump` or binary editors to compare output files byte-by-byte
- CDP source is in `build/cdp-8.7.1/dev/` after running `make build-cdp`