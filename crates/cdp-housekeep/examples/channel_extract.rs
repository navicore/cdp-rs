//! Example: Extract individual channels from multi-channel files
//!
//! First generate the sample files:
//!   cargo run -p cdp-housekeep --example generate_samples
//!
//! Then run this example:
//!   cargo run -p cdp-housekeep --example channel_extract

use cdp_housekeep::{chans, wav_cdp};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CDP-RS Channel Extraction Example\n");
    println!("==================================\n");

    // Input stereo file (in current directory)
    let stereo_file = Path::new("stereo_tone.wav");

    // Check if sample file exists
    if !stereo_file.exists() {
        println!("Sample files not found!");
        println!("Please run: cargo run -p cdp-housekeep --example generate_samples");
        return Ok(());
    }

    // Get file info
    let (format, _) = wav_cdp::read_wav_basic(stereo_file)?;
    println!("Input file: {}", stereo_file.display());
    println!(
        "Format: {} channels, {} Hz, {} bit",
        format.channels, format.sample_rate, format.bits_per_sample
    );
    println!("Content: 440Hz on left channel, 880Hz on right channel\n");

    // 1. Extract left channel (440Hz)
    println!("1. Extracting left channel (440Hz tone):");
    let left_output = Path::new("stereo_tone_left.wav");
    chans::extract_channel_to(stereo_file, 1, left_output)?;
    println!("   ✓ Created: {}", left_output.display());

    // 2. Extract right channel (880Hz)
    println!("\n2. Extracting right channel (880Hz tone):");
    let right_output = Path::new("stereo_tone_right.wav");
    chans::extract_channel_to(stereo_file, 2, right_output)?;
    println!("   ✓ Created: {}", right_output.display());

    // 3. Mix to mono (sum both channels)
    println!("\n3. Creating mono mix (sum of both channels):");
    let mono_output = Path::new("stereo_tone_mono_mix.wav");
    chans::mix_to_mono(stereo_file, mono_output, false)?;
    println!("   ✓ Created: {}", mono_output.display());
    println!("   (Contains both 440Hz and 880Hz)");

    // 4. Mix with phase inversion (difference)
    println!("\n4. Creating difference signal (L - R):");
    let diff_output = Path::new("stereo_tone_difference.wav");
    chans::mix_to_mono(stereo_file, diff_output, true)?;
    println!("   ✓ Created: {}", diff_output.display());
    println!("   (Subtracts right from left channel)");

    // Verify the extracted channels
    println!("\n5. Verifying extracted channels:");
    let (left_format, _) = wav_cdp::read_wav_basic(left_output)?;
    let (right_format, _) = wav_cdp::read_wav_basic(right_output)?;
    println!(
        "   Left channel:  {} channels, {} samples",
        left_format.channels,
        left_format.data_size / 2
    );
    println!(
        "   Right channel: {} channels, {} samples",
        right_format.channels,
        right_format.data_size / 2
    );

    println!("\n✓ Channel extraction complete!");
    println!("\nGenerated files:");
    println!("  - stereo_tone_left.wav    (440Hz mono)");
    println!("  - stereo_tone_right.wav   (880Hz mono)");
    println!("  - stereo_tone_mono_mix.wav (both frequencies)");
    println!("  - stereo_tone_difference.wav (L-R difference)");

    Ok(())
}
