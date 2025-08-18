//! Example: Basic audio processing with CDP-RS
//!
//! First generate the sample files:
//!   cargo run -p cdp-housekeep --example generate_samples
//!
//! Then run this example:
//!   cargo run -p cdp-modify --example audio_processing

use cdp_housekeep::chans;
use cdp_modify::loudness;
use cdp_sndinfo::props;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CDP-RS Audio Processing Example\n");
    println!("================================\n");

    // Input and output paths (files in current directory)
    let stereo_input = Path::new("stereo_tone.wav");
    let mono_output = Path::new("stereo_tone_mono.wav");
    let normalized_output = Path::new("stereo_tone_normalized.wav");
    let quiet_input = Path::new("quiet_sine.wav");
    let amplified_output = Path::new("quiet_sine_amplified.wav");

    // Check if sample files exist
    if !stereo_input.exists() {
        println!("Sample files not found!");
        println!("Please run: cargo run -p cdp-housekeep --example generate_samples");
        return Ok(());
    }

    // 1. Display file information
    println!("1. Analyzing stereo file properties:");
    println!("   File: {}", stereo_input.display());
    props::show_props(stereo_input)?;
    println!();

    // 2. Convert stereo to mono
    println!("2. Converting stereo to mono:");
    println!("   Mixing left (440Hz) and right (880Hz) channels...");
    chans::mix_to_mono(stereo_input, mono_output, false)?;
    println!("   ✓ Created: {}", mono_output.display());
    println!("   Result properties:");
    props::show_props(mono_output)?;
    println!();

    // 3. Normalize the stereo file
    println!("3. Normalizing stereo file:");
    println!("   Normalizing to maximum level...");
    loudness::normalize(stereo_input, normalized_output, None)?;
    println!("   ✓ Created: {}", normalized_output.display());
    println!();

    // 4. Amplify a quiet signal
    println!("4. Amplifying quiet signal:");
    println!("   Input: {}", quiet_input.display());
    props::show_props(quiet_input)?;
    println!("   Applying +12 dB gain...");
    loudness::apply_db_gain(quiet_input, amplified_output, 12.0)?;
    println!("   ✓ Created: {}", amplified_output.display());
    println!("   Result properties:");
    props::show_props(amplified_output)?;
    println!();

    // 5. Phase inversion demo
    let inverted_output = Path::new("stereo_tone_phase_cancelled.wav");
    println!("5. Phase cancellation demo:");
    println!("   Mixing stereo with right channel phase inverted...");
    chans::mix_to_mono(stereo_input, inverted_output, true)?;
    println!("   ✓ Created: {}", inverted_output.display());
    println!("   (Should have reduced 880Hz component)");
    println!();

    println!("✓ Audio processing examples complete!");
    println!("\nGenerated files in current directory:");
    println!("  - stereo_tone_mono.wav");
    println!("  - stereo_tone_normalized.wav");
    println!("  - quiet_sine_amplified.wav");
    println!("  - stereo_tone_phase_cancelled.wav");

    Ok(())
}
