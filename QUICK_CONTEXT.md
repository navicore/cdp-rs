# CDP-RS Quick Context Recovery

## What is this project?
Rust port of Composers Desktop Project (CDP) - 35+ year old C audio processing suite.
We're creating a faithful, safe Rust implementation validated against original CDP binaries.

## Current State
- **Working**: housekeep (copy, channel ops), modify (gain, normalize), sndinfo (file info)
- **Next**: pvoc (phase vocoder) - foundation for all spectral processing
- **Validation**: Using CDP binaries as oracle tests (must match output exactly)

## Project Structure
```
cdp-rs/
├── crates/
│   ├── cdp-core/         # ✅ FROZEN - Core DSP (FFT, windows)
│   ├── cdp-housekeep/    # ✅ FROZEN - File ops, channel processing  
│   ├── cdp-modify/       # ✅ FROZEN - Gain, normalize
│   ├── cdp-sndinfo/      # ✅ FROZEN - File analysis
│   ├── cdp-pvoc/         # 🚧 TODO - Phase vocoder
│   ├── cdp-spectral/     # 🚧 TODO - Spectral effects
│   ├── cdp-oracle/       # Test framework
│   └── cdp-sandbox/      # Active development area
├── scripts/              # Build and test scripts
└── build/                # CDP binaries (generated)
```

## Key Commands
```bash
make              # Run all checks (fmt, lint, build, test, oracle)
make oracle       # Run oracle validation tests
make build-cdp    # Build CDP from source (first time)

# Run examples
cargo run -p cdp-housekeep --example generate_samples
cargo run -p cdp-housekeep --example channel_extract
cargo run -p cdp-modify --example audio_processing
```

## Development Workflow
1. Implement in `cdp-sandbox` first
2. Test against CDP oracle: `./scripts/ci-oracle-test.sh`
3. When validated, move to proper crate
4. Mark as FROZEN after validation

## CDP WAV Format Quirks
- CDP adds PEAK chunk (16 bytes) with peak value and position
- CDP adds cue chunk (28 bytes) with "sfif" identifier
- CDP adds LIST/adtl/note chunk (2016 bytes) with timestamp
- Must match exactly for oracle tests to pass

## Critical Files
- `crates/cdp-housekeep/src/wav_cdp.rs` - CDP WAV format I/O
- `scripts/ci-oracle-test.sh` - Oracle validation script
- `scripts/oracle-compare.py` - Compares CDP vs Rust output
- `NEXT_MODULES.md` - Implementation roadmap

## Current Focus
Implementing pvoc (phase vocoder):
1. pvoc anal - Time domain → frequency domain
2. pvoc synth - Frequency domain → time domain
3. Foundation for all spectral processing

## Testing Philosophy
- CDP binaries are the test suite
- If CDP outputs X, we must output X (byte-for-byte, excluding timestamps)
- No "improvements" - faithful reproduction only
- Oracle tests run in CI on every commit

## Common Issues & Solutions
- **Size mismatch in oracle tests**: Check CDP chunk formats (PEAK, LIST, cue)
- **Format errors**: Run `make fmt` before committing
- **Can't find CDP binaries**: Run `make build-cdp` first
- **Tests fail locally**: Check if you have CDP built: `ls build/cdp/NewRelease/`

## Next Steps (see NEXT_MODULES.md for details)
1. Implement pvoc anal (FFT analysis)
2. Implement pvoc synth (IFFT synthesis)  
3. Validate against CDP pvoc
4. Move to crates/cdp-pvoc/
5. Mark as FROZEN