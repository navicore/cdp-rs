# CDP-Distort

Audio distortion and saturation effects library, part of the CDP-RS project.

## Features

### Distortion Types

1. **Harmonic Multiplication** (`multiply`)
   - Creates upper harmonics by frequency multiplication
   - Brightens and adds presence to sounds
   - Factor range: 1.0-16.0
   - Mix control for parallel processing

2. **Subharmonic Division** (`divide`)
   - Generates subharmonics for bass enhancement
   - Creates octave-down effects
   - Division factors: 2-16
   - Perfect for sub-bass generation

3. **Clipping Distortion** (`overload`)
   - Four clipping curve types:
     - **Hard**: Digital distortion, harsh clipping
     - **Soft**: Smooth saturation (tanh-based)
     - **Tube**: Analog-style tube saturation
     - **Asymmetric**: Even harmonics, amp-like distortion
   - Threshold: 0.1-1.0 (lower = more distortion)
   - Drive: 1.0-100.0 (input gain)

## Usage

```rust
use cdp_distort::{multiply, divide, overload, ClipType};

// Add harmonics
multiply(&input_path, &output_path, 2.0, 1.0)?;

// Generate sub-bass
divide(&input_path, &output_path, 2, 0.5)?;

// Apply tube saturation
overload(&input_path, &output_path, 0.7, 2.0, ClipType::Tube)?;
```

## Examples

The crate includes comprehensive examples:

- `distortion_showcase` - Overview of all distortion types
- `guitar_effects` - Guitar amp simulations and effects
- `vocal_processing` - Vocal distortion techniques
- `bass_enhancement` - Sub-bass and bass processing

Run examples with:
```bash
cargo run --example guitar_effects
```

## Command-Line Interface

Compatible with CDP distort commands:

```bash
# Harmonic multiplication
distort multiply input.wav output.wav -f 2.0 -m 1.0

# Subharmonic generation
distort divide input.wav output.wav -f 2 -m 0.5

# Clipping distortion
distort overload input.wav output.wav -t 0.5 -d 3.0 --clip-type tube
```

## Production Tips

### Guitar Processing
- **Clean Boost**: Threshold 0.9-1.0, Drive 1.0-2.0, Tube
- **Overdrive**: Threshold 0.6-0.8, Drive 2.0-4.0, Tube/Soft
- **Distortion**: Threshold 0.3-0.6, Drive 4.0-10.0, Asymmetric
- **Fuzz**: Threshold 0.1-0.3, Drive 10.0-20.0, Hard

### Vocal Processing
- **Warmth**: Subtle tube saturation (Threshold 0.85, Drive 1.3)
- **Telephone**: Hard clipping (Threshold 0.4, Drive 2.0)
- **Robot**: Harmonic multiplication (Factor 4.0, Mix 0.7)
- **Megaphone**: Chain multiply + hard clip

### Bass Enhancement
- **Sub-bass**: Divide by 2, Mix 30-50%
- **Warmth**: Low tube saturation
- **Rock/Metal**: Medium asymmetric clipping
- **808-style**: Divide + soft saturation chain

## Parallel Processing

Use the mix parameter to blend processed and dry signals:

```rust
// 50% mix for parallel compression effect
multiply(&input, &output, 3.0, 0.5)?;
```

## Oracle Testing

The module includes oracle tests comparing output with CDP original:

```bash
cargo test --package cdp-distort oracle -- --ignored
```

## Algorithm Notes

- **Multiply**: Uses tanh() for smooth harmonic folding
- **Divide**: Zero-crossing detection for subharmonic generation
- **Overload**: Multiple clipping curves with output normalization
- All processors include automatic gain compensation

## Performance

- Optimized for real-time processing
- Zero-copy where possible
- Automatic normalization prevents clipping
- Support for 16/24/32-bit audio files