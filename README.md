# cdp-rs

# A failed experiment in AI Coding - 

the Claude Code local dev tool created an elaborate system of oracle tests to verify our output was of the same audio character adn quality as the C CDP implementation - after a half dozen modules were implemented and sounded ok to my ear, I figured out it had marked all the oracle tests as #ignore and noone of our modules were impllemented with the CDP algorythms - they were all hacks and would never stand up to real use.

After getting back on track with new implementations and tests - even the simple
multiply operator was too much for claude to find a way to validte our
implementation - that our impls are always guesses and nonesense based on
probability can't compete with the intense work of true experts over the
decades crafting the orig C code.

# UNDER CONSTRUCTION

# UNDER CONSTRUCTION

# UNDER CONSTRUCTION

A faithful Rust port of the Composers Desktop Project ([CDP](https://github.com/ComposersDesktop/CDP8)) - preserving 35+ years of spectral processing excellence.

## Vision

CDP-RS aims to:
- Preserve CDP's groundbreaking DSP algorithms in modern, safe Rust
- Maintain bit-perfect compatibility with original CDP outputs
- Provide a foundation for next-generation audio tools
- Enable real-time processing (original CDP is batch-only)

## Quick Start

```bash
# Clone the repository
git clone https://github.com/navicore/cdp-rs
cd cdp-rs

# Install CDP binaries automatically (Mac/Windows/Linux)
make install-cdp

# Test the installation
make test-cdp

# Run the full test suite
make

# Run oracle validation tests
make oracle
```

No manual installation required! The build system handles everything.

## Architecture

```
cdp-rs/
├── crates/
│   ├── cdp-core/         # Core DSP primitives (FFT, windows, etc) - FROZEN after validation
│   ├── cdp-pvoc/         # Phase vocoder implementation - FROZEN after validation  
│   ├── cdp-spectral/     # Spectral processors - FROZEN after validation
│   ├── cdp-housekeep/    # Channel operations and file management
│   ├── cdp-modify/       # Audio modification (gain, normalize, etc)
│   ├── cdp-sndinfo/      # Sound file analysis and properties
│   ├── cdp-oracle/       # Testing framework using CDP binaries as ground truth
│   ├── cdp-sandbox/      # Active development area (safe for LLM modification)
│   └── cdp-oracle-demos/ # Internal oracle testing demonstrations (not for users)
├── scripts/              # Build and test scripts
├── docs/                 # Documentation
└── build/                # CDP binary builds (generated)
```

## Development Philosophy

1. **Oracle Testing**: Original CDP binaries serve as the test suite
2. **Frozen Modules**: Validated code becomes immutable
3. **Incremental Porting**: One CDP program at a time
4. **LLM Safety**: Sandbox isolation prevents breaking validated code

## Testing Strategy

Every Rust implementation is validated against CDP's original C binaries:

```rust
#[test]
fn validate_against_cdp() {
    let oracle = CdpOracle::new();
    let processor = OurRustImplementation::new();
    
    // CDP's output IS our test assertion
    assert!(oracle.validate(&processor).passes());
}
```

## License

LGPL-2.1 - Respecting CDP's original license.

### Why LGPL?

CDP has been LGPL for decades, enabling:
- Commercial use in proprietary applications (when dynamically linked)
- Academic research without restriction  
- Community contributions back to the core

For a pure MIT alternative in the future, we could:
- Implement algorithms from first principles using academic papers
- Create clean-room implementations based on specifications
- Build a new API that doesn't derive from CDP's codebase

## Examples

Run the included examples to see CDP-RS in action:

```bash
# First, generate sample audio files in your current directory
cargo run -p cdp-housekeep --example generate_samples

# Then run any of the processing examples:

# Basic audio processing - gain, normalize, mix to mono
cargo run -p cdp-modify --example audio_processing

# Batch normalize multiple files to consistent level
cargo run -p cdp-modify --example batch_normalize

# Extract channels from stereo files
cargo run -p cdp-housekeep --example channel_extract
```

The examples are self-contained and work with WAV files in your current directory. The `generate_samples` example creates test files so you can run the examples immediately without needing your own audio files.

Examples are located in their respective crate directories:
- `crates/cdp-housekeep/examples/` - File I/O and channel operations
- `crates/cdp-modify/examples/` - Audio processing and modifications

## Status

- [x] Housekeep Copy (CDP WAV format with PEAK chunks)
- [x] Channel extraction and mixing
- [x] Gain and normalization
- [ ] Phase Vocoder (pvoc)
- [ ] Spectral Blur
- [ ] Time Stretch
- [ ] Pitch Shift
- [ ] Formant Manipulation
- ... 495 more to go!

## Contributing

See CONTRIBUTING.md for guidelines on:
- Running oracle tests
- Freezing validated modules
- Working in the sandbox
