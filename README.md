# cdp-rs

# UNDER CONSTRUCTION

# UNDER CONSTRUCTION

# UNDER CONSTRUCTION

A faithful Rust port of the Composers Desktop Project (CDP) - preserving 35+ years of spectral processing excellence.

## Vision

CDP-RS aims to:
- Preserve CDP's groundbreaking DSP algorithms in modern, safe Rust
- Maintain bit-perfect compatibility with original CDP outputs
- Provide a foundation for next-generation audio tools
- Enable real-time processing (original CDP is batch-only)

## Architecture

```
cdp-rs/
├── cdp-core/      # Core DSP primitives (FFT, windows, etc) - FROZEN after validation
├── cdp-pvoc/      # Phase vocoder implementation - FROZEN after validation  
├── cdp-spectral/  # Spectral processors - FROZEN after validation
├── cdp-oracle/    # Testing framework using CDP binaries as ground truth
└── cdp-sandbox/   # Active development area (safe for LLM modification)
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

## Status

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
