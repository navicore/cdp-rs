# CDP-RS Next Modules Implementation Checklist

## Completed Modules ✅
- [x] **cdp-core**: Core DSP primitives (FFT, windows)
- [x] **cdp-housekeep**: File operations (copy, channel extraction, mixing)
- [x] **cdp-modify**: Audio modifications (gain, normalize)
- [x] **cdp-sndinfo**: File analysis and properties

## Priority 1: Phase Vocoder (Foundation for spectral processing) ✅
### cdp-pvoc module
- [x] **pvoc anal** - Convert soundfile to frequency domain
  - [x] Basic FFT analysis
  - [x] Window overlap handling
  - [x] Phase tracking
  - [x] Write .ana file format
  - [x] Oracle test against CDP pvoc anal
  
- [x] **pvoc synth** - Convert frequency domain back to audio
  - [x] IFFT synthesis
  - [x] Phase reconstruction
  - [x] Overlap-add
  - [x] Oracle test against CDP pvoc synth

- [x] **pvoc extract** - Extract frequency band
  - [x] Frequency bin selection
  - [x] Amplitude/phase preservation
  - [x] Oracle test

## Priority 2: Essential Spectral Processors
### cdp-spectral expansion
- [x] **blur blur** - Spectral blurring ✅
  - [x] Frequency smearing algorithm
  - [x] Time window control
  - [x] Oracle test against CDP blur

- [x] **stretch time** - Time stretching ✅
  - [x] Phase vocoder time manipulation
  - [x] Pitch preservation
  - [x] Oracle test

- [ ] **modify pitch** - Pitch shifting
  - [ ] Frequency bin shifting
  - [ ] Formant correction option
  - [ ] Oracle test

## Priority 3: Distortion & Effects
### cdp-distort module (new)
- [ ] **distort multiply** - Harmonic distortion
  - [ ] Frequency multiplication
  - [ ] Amplitude control
  - [ ] Oracle test

- [ ] **distort divide** - Subharmonic generation
  - [ ] Frequency division
  - [ ] Oracle test

- [ ] **distort overload** - Clipping distortion
  - [ ] Various clipping curves
  - [ ] Oracle test

## Priority 4: Filters
### cdp-filter module (new)
- [ ] **filter fltbankn** - Filter bank
  - [ ] Multiple bandpass filters
  - [ ] Q control
  - [ ] Oracle test

- [ ] **filter lopass** - Low pass filter
  - [ ] Butterworth implementation
  - [ ] Cutoff frequency control
  - [ ] Oracle test

- [ ] **filter hipass** - High pass filter
  - [ ] Oracle test

## Priority 5: Granular Synthesis
### cdp-texture module (new)
- [ ] **texture simple** - Basic granulation
  - [ ] Grain extraction
  - [ ] Grain overlap
  - [ ] Oracle test

- [ ] **texture ornate** - Complex granulation
  - [ ] Multiple grain streams
  - [ ] Pitch/time independence
  - [ ] Oracle test

## Implementation Guidelines

### For Each New Module:
1. **Create crate structure**
   ```
   crates/cdp-{module}/
   ├── Cargo.toml
   ├── src/
   │   ├── lib.rs
   │   ├── bin/{program}.rs  # CLI compatibility
   │   └── {operation}.rs     # Core implementation
   └── examples/
   ```

2. **Implementation steps**
   - [ ] Read CDP source code for algorithm
   - [ ] Implement core algorithm in Rust
   - [ ] Add CLI binary for oracle testing
   - [ ] Write unit tests
   - [ ] Add oracle test to scripts/ci-oracle-test.sh
   - [ ] Create example demonstrating usage
   - [ ] Document algorithm and usage

3. **Oracle validation**
   - [ ] Generate test input
   - [ ] Run CDP original
   - [ ] Run Rust implementation
   - [ ] Compare outputs (allowing for timestamp differences)
   - [ ] Add to CI pipeline

4. **After validation**
   - [ ] Mark module as FROZEN in FROZEN_MODULES.md
   - [ ] Update README.md status section
   - [ ] Create PR with oracle test results

## Testing Priority Order
1. Start with pvoc (enables all spectral processing)
2. Then blur (simplest spectral effect)
3. Then stretch/pitch (most useful transformations)
4. Then filters (widely used)
5. Finally granular (complex but powerful)

## Notes
- Each module should be developed in cdp-sandbox first
- After oracle validation, move to its own crate
- Maintain CLI compatibility for oracle testing
- Focus on correctness over optimization initially
- Document any deviations from CDP behavior

## Resources
- CDP source: build/cdp/dev/
- CDP documentation: http://www.composersdesktop.com/cdpdocs/
- Test files: Use cdp-housekeep generate_samples for consistency