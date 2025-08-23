# CDP Source Code Map

This document maps our Rust modules to their corresponding CDP source files.
CDP source is located in `build/cdp-8.7.1/dev/` after running `make build-cdp`.

## Finding CDP Source

The CDP source is organized by program groups:

```
build/cdp-8.7.1/dev/
├── housekeep/    # File operations
├── modify/       # Audio modifications  
├── distort/      # Distortion effects
├── pvoc/         # Phase vocoder
├── blur/         # Spectral blur
├── stretch/      # Time stretch
├── morph/        # Spectral morphing
└── ... (many more)
```

## Module to Source Mapping

### cdp-housekeep
| Our Module | CDP Source | Key Files |
|------------|------------|-----------|
| copy | `housekeep/copy.c` | Main copy implementation |
| chans | `housekeep/chans.c` | Channel operations |
| | `housekeep/housekeep.c` | Main dispatcher |
| | `housekeep/housekeep.h` | Shared definitions |

### cdp-modify
| Our Module | CDP Source | Key Files |
|------------|------------|-----------|
| loudness | `modify/loudness.c` | Gain, normalize |
| | `modify/modify.c` | Main dispatcher |
| | `modify/sffuncs.c` | Soundfile functions |

### cdp-distort
| Our Module | CDP Source | Key Files |
|------------|------------|-----------|
| multiply | `distort/distort.c` | All distort modes |
| divide | `distort/distort.c` | (same file, different mode) |
| overload | `distort/distort.c` | (same file, different mode) |
| | `distort/distort1.c` | Helper functions |

### cdp-pvoc
| Our Module | CDP Source | Key Files |
|------------|------------|-----------|
| anal/synth | `pvoc/pvoc.c` | Main pvoc implementation |
| | `pvoc/pvocana.c` | Analysis functions |
| | `pvoc/pvocsyn.c` | Synthesis functions |
| | `pvoc/pvfileio.c` | .ana file I/O |

### cdp-spectral
| Our Module | CDP Source | Key Files |
|------------|------------|-----------|
| blur | `blur/blur.c` | Spectral blurring |
| stretch | `stretch/stretch.c` | Time stretching |
| pitch | `morph/morph.c` | Contains pitch shift |
| | `include/speccon.h` | Spectral constants |

## Important CDP Headers

Key header files that define structures and constants:

```
build/cdp-8.7.1/include/
├── cdp.h          # Main CDP definitions
├── sfsys.h        # Soundfile system
├── pvdefs.h       # Phase vocoder definitions  
├── speccon.h      # Spectral processing constants
└── multichan.h    # Multi-channel definitions
```

## CDP WAV Format Specifics

CDP uses standard WAV with extensions:
- **PEAK chunk**: Contains peak amplitude information
- **LIST chunk**: Contains CDP-specific metadata
- Format documented in `sfsys/sfsys.c`

## How to Study CDP Source

1. Start with the main dispatcher (e.g., `distort/distort.c`)
2. Find the specific mode/function you're implementing
3. Trace through the processing functions
4. Pay attention to:
   - Buffer sizes and FFT parameters
   - Scaling factors and normalization
   - Edge case handling
   - Error messages (helps understand expected behavior)

## Example: Finding multiply implementation

```bash
# After running make build-cdp
cd build/cdp-8.7.1/dev/distort
grep -n "DISTORT_MLT" distort.c  # Find multiply mode
# Look for the case statement handling this mode
# Follow the function calls to understand the algorithm
```

## Notes

- CDP uses 1-based channel numbering (we use 0-based)
- CDP often has different modes within a single program
- Many CDP programs share common utility functions in `sfsys/`
- The `cmdline/` directory has the command-line parsing but not the DSP