# CDP-RS Implementation Roadmap

## 🎯 Project Vision
Port CDP's core audio processing algorithms to pure Rust, creating a modern, safe, and performant implementation while maintaining perfect compatibility with the original.

## 📊 Current Status
- ✅ **Phase 0: Infrastructure** - COMPLETE
  - Project structure with frozen/sandbox separation
  - CDP oracle built from source (218 programs)
  - CI/CD pipeline with cross-platform testing
  - Test suite with audio generation

## 🗺️ Implementation Phases

### Phase 1: Foundation (Weeks 1-2) 🏗️
**Goal**: Implement core audio I/O and basic transformations

1. **WAV File I/O** (`cdp-core`)
   - [ ] Read/write WAV files matching CDP format exactly
   - [ ] Support for mono/stereo, 16/24/32 bit
   - [ ] Oracle test: byte-perfect WAV copying

2. **Housekeep Operations** (`cdp-housekeep`)
   - [ ] `copy` - File duplication
   - [ ] `chans` - Channel extraction/interleaving
   - [ ] `extract` - Time segment extraction
   - [ ] Oracle tests against CDP housekeep

3. **Basic Modify Operations** (`cdp-modify`)
   - [ ] `speed` - Time stretching (simple)
   - [ ] `loudness` - Amplitude scaling
   - [ ] `reverse` - Time reversal
   - [ ] Oracle validation for each

### Phase 2: Spectral Core (Weeks 3-6) 🌊
**Goal**: Implement PVOC - the heart of CDP

1. **Phase Vocoder Analysis** (`cdp-pvoc`)
   - [ ] FFT implementation or integration
   - [ ] Window functions (Hamming, Kaiser, etc.)
   - [ ] Analysis frame extraction
   - [ ] .ana file format specification
   - [ ] Oracle: Compare .ana files with CDP

2. **Phase Vocoder Synthesis**
   - [ ] Inverse FFT
   - [ ] Overlap-add reconstruction
   - [ ] Phase unwrapping
   - [ ] Oracle: Resynthesize and compare audio

3. **Basic Spectral Operations** (`cdp-spectral`)
   - [ ] `blur` - Frequency blurring
   - [ ] `clean` - Noise reduction
   - [ ] `filter` - Frequency filtering
   - [ ] Each validated against CDP output

### Phase 3: Advanced Processing (Weeks 7-10) 🎨
**Goal**: Implement complex transformations

1. **Granular Synthesis** (`cdp-grain`)
   - [ ] Grain detection algorithms
   - [ ] Grain manipulation (duplicate, reverse, etc.)
   - [ ] Grain reassembly
   - [ ] Oracle validation

2. **Distortion Effects** (`cdp-distort`)
   - [ ] Waveshaping algorithms
   - [ ] Harmonic distortion
   - [ ] Dynamic processing
   - [ ] Compare with CDP outputs

3. **Texture Operations** (`cdp-texture`)
   - [ ] `extend` - Time manipulation
   - [ ] `texture` - Textural transformations
   - [ ] Complex granular operations

### Phase 4: Integration (Weeks 11-12) 🔧
**Goal**: Polish and integrate all components

1. **Unified CLI** (`cdp-cli`)
   - [ ] Single binary with all operations
   - [ ] Argument parsing matching CDP
   - [ ] Help system
   - [ ] Progress indicators

2. **Performance Optimization**
   - [ ] Profile against CDP
   - [ ] SIMD optimizations
   - [ ] Parallel processing where applicable
   - [ ] Memory efficiency

3. **Comprehensive Testing**
   - [ ] Full oracle test suite
   - [ ] Fuzzing tests
   - [ ] Edge case validation
   - [ ] Performance benchmarks

## 🎯 First Implementation Target

Let's start with **`housekeep copy`** - the simplest CDP operation:

### Why Start Here?
1. **Simple algorithm** - Just file I/O
2. **Easy to validate** - Binary comparison
3. **Builds foundation** - WAV file handling
4. **Quick win** - Builds confidence

### Implementation Plan:
```rust
// cdp-housekeep/src/copy.rs
pub fn copy(input: &Path, output: &Path) -> Result<()> {
    // 1. Read WAV file
    // 2. Validate format
    // 3. Write identical copy
    // 4. Preserve all metadata
}
```

### Oracle Test:
```bash
# CDP version
cdp-bin/housekeep copy 1 input.wav cdp_output.wav

# Rust version  
cargo run --bin housekeep copy input.wav rust_output.wav

# Compare
diff cdp_output.wav rust_output.wav
```

## 📈 Success Metrics

1. **Accuracy**: <0.01% difference from CDP outputs
2. **Performance**: Within 2x of CDP speed
3. **Compatibility**: Process all CDP-compatible files
4. **Reliability**: Zero crashes on CDP test suite
5. **Coverage**: 80%+ of core CDP operations

## 🔄 Development Workflow

1. **Choose CDP operation** to implement
2. **Study CDP source** code (build/cdp/dev/)
3. **Write Rust implementation** in sandbox
4. **Create oracle tests** comparing outputs
5. **Iterate until matching** CDP exactly
6. **Move to frozen** once validated
7. **Document** algorithm and decisions

## 📚 Resources Needed

1. **DSP Knowledge**
   - FFT algorithms
   - Window functions
   - Phase vocoding
   - Granular synthesis

2. **CDP Documentation**
   - File format specifications
   - Algorithm descriptions
   - Parameter ranges

3. **Rust Libraries**
   - `hound` or `wav` for audio I/O
   - `rustfft` for spectral operations
   - `rayon` for parallelization
   - `clap` for CLI parsing

## 🚀 Next Steps

1. **Create `cdp-housekeep` module**
2. **Implement WAV file reader/writer**
3. **Port `housekeep copy` operation**
4. **Create first oracle comparison test**
5. **Validate byte-perfect copying**

## 🎉 Quick Wins Path

Start with these easy operations to build momentum:

1. `housekeep copy` - File copying ⭐
2. `housekeep extract` - Time extraction
3. `modify reverse` - Reverse audio
4. `modify loudness` - Volume scaling
5. `housekeep chans` - Channel operations

Each can be done in a day and validates our approach!

## 💡 Key Insights

- **Start simple**: File I/O before DSP
- **Test everything**: Oracle validates correctness
- **Incremental progress**: One operation at a time
- **Document thoroughly**: Future maintainers need context
- **Preserve behavior**: Bug-for-bug compatibility if needed

Ready to implement the first CDP operation in Rust? Let's start with `housekeep copy`!