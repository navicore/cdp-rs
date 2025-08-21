//! Example: Creative spectral effects with phase vocoder
//!
//! This example demonstrates more creative uses of spectral analysis:
//! - Multi-band filtering
//! - Spectral envelope extraction
//! - Magnitude-only processing
//!
//! First generate the sample files:
//!   cargo run -p cdp-housekeep --example generate_samples
//!
//! Then run this example:
//!   cargo run -p cdp-pvoc --example spectral_effects

use cdp_pvoc::{pvoc_anal, pvoc_extract, pvoc_synth};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CDP-RS Spectral Effects Example");
    println!("===============================\n");

    // Use white noise for more interesting spectral content
    let noise_file = Path::new("crates/cdp-housekeep/examples/white_noise.wav");

    if !noise_file.exists() {
        println!("Sample files not found!");
        println!("Please run: cargo run -p cdp-housekeep --example generate_samples");
        return Ok(());
    }

    // Output directory
    let examples_dir = Path::new("crates/cdp-pvoc/examples");
    std::fs::create_dir_all(examples_dir)?;

    println!("Processing white noise for spectral effects\n");

    // 1. Analyze white noise
    println!("1. Analyzing white noise:");
    let noise_ana = examples_dir.join("noise.ana");
    pvoc_anal(noise_file, &noise_ana, 1, Some(2048), Some(3))?;
    println!("   ✓ Created: {}", noise_ana.display());

    // 2. Create multiple band-pass filtered versions
    println!("\n2. Creating band-pass filtered versions:");

    // Low band (rumble)
    let low_ana = examples_dir.join("noise_low.ana");
    pvoc_extract(&noise_ana, &low_ana, 20.0, 200.0)?;
    let low_wav = examples_dir.join("noise_low.wav");
    pvoc_synth(&low_ana, &low_wav)?;
    println!("   ✓ Low band (20-200 Hz): {}", low_wav.display());

    // Mid band (body)
    let mid_ana = examples_dir.join("noise_mid.ana");
    pvoc_extract(&noise_ana, &mid_ana, 200.0, 2000.0)?;
    let mid_wav = examples_dir.join("noise_mid.wav");
    pvoc_synth(&mid_ana, &mid_wav)?;
    println!("   ✓ Mid band (200-2000 Hz): {}", mid_wav.display());

    // High band (air)
    let high_ana = examples_dir.join("noise_high.ana");
    pvoc_extract(&noise_ana, &high_ana, 2000.0, 10000.0)?;
    let high_wav = examples_dir.join("noise_high.wav");
    pvoc_synth(&high_ana, &high_wav)?;
    println!("   ✓ High band (2000-10000 Hz): {}", high_wav.display());

    // 3. Spectral envelope mode (mode 2)
    println!("\n3. Extracting spectral envelope (mode 2):");
    let envelope_ana = examples_dir.join("noise_envelope.ana");
    pvoc_anal(noise_file, &envelope_ana, 2, Some(1024), Some(3))?;
    let envelope_wav = examples_dir.join("noise_envelope.wav");
    pvoc_synth(&envelope_ana, &envelope_wav)?;
    println!("   ✓ Envelope mode: {}", envelope_wav.display());
    println!("   Phase information removed, only envelope preserved");

    // 4. Magnitude-only mode (mode 3)
    println!("\n4. Magnitude-only analysis (mode 3):");
    let magnitude_ana = examples_dir.join("noise_magnitude.ana");
    pvoc_anal(noise_file, &magnitude_ana, 3, Some(1024), Some(3))?;
    let magnitude_wav = examples_dir.join("noise_magnitude.wav");
    pvoc_synth(&magnitude_ana, &magnitude_wav)?;
    println!("   ✓ Magnitude mode: {}", magnitude_wav.display());
    println!("   Phase set to zero, creates different texture");

    // 5. Process stereo file
    let stereo_file = Path::new("crates/cdp-housekeep/examples/stereo_tone.wav");
    if stereo_file.exists() {
        println!("\n5. Processing stereo file:");
        println!("   Note: pvoc processes each channel independently");

        let stereo_ana = examples_dir.join("stereo.ana");
        pvoc_anal(stereo_file, &stereo_ana, 1, Some(2048), Some(4))?;
        println!("   ✓ Analyzed: {}", stereo_ana.display());

        // Create a "telephone" effect by extreme band-pass
        let phone_ana = examples_dir.join("stereo_phone.ana");
        pvoc_extract(&stereo_ana, &phone_ana, 300.0, 3400.0)?;
        let phone_wav = examples_dir.join("stereo_phone.wav");
        pvoc_synth(&phone_ana, &phone_wav)?;
        println!(
            "   ✓ Telephone effect (300-3400 Hz): {}",
            phone_wav.display()
        );
    }

    // 6. Extreme filtering demonstration
    println!("\n6. Extreme filtering - narrow bands:");

    // Very narrow band around 1kHz
    let narrow_ana = examples_dir.join("noise_narrow_1k.ana");
    pvoc_extract(&noise_ana, &narrow_ana, 950.0, 1050.0)?;
    let narrow_wav = examples_dir.join("noise_narrow_1k.wav");
    pvoc_synth(&narrow_ana, &narrow_wav)?;
    println!("   ✓ Narrow band (950-1050 Hz): {}", narrow_wav.display());
    println!("   Creates resonant, ringing quality");

    // Multiple narrow bands (comb filter effect)
    println!("\n7. Creating comb filter effect:");
    println!("   Extract 200-250 Hz band:");
    let comb1_ana = examples_dir.join("comb1.ana");
    pvoc_extract(&noise_ana, &comb1_ana, 200.0, 250.0)?;
    let comb1_wav = examples_dir.join("comb1.wav");
    pvoc_synth(&comb1_ana, &comb1_wav)?;
    println!("   ✓ {}", comb1_wav.display());

    println!("   Extract 400-450 Hz band:");
    let comb2_ana = examples_dir.join("comb2.ana");
    pvoc_extract(&noise_ana, &comb2_ana, 400.0, 450.0)?;
    let comb2_wav = examples_dir.join("comb2.wav");
    pvoc_synth(&comb2_ana, &comb2_wav)?;
    println!("   ✓ {}", comb2_wav.display());

    println!("   Extract 800-850 Hz band:");
    let comb3_ana = examples_dir.join("comb3.ana");
    pvoc_extract(&noise_ana, &comb3_ana, 800.0, 850.0)?;
    let comb3_wav = examples_dir.join("comb3.wav");
    pvoc_synth(&comb3_ana, &comb3_wav)?;
    println!("   ✓ {}", comb3_wav.display());

    println!("\n✓ Spectral effects examples complete!");
    println!("\nGenerated files demonstrate:");
    println!("  - Multi-band filtering (low/mid/high)");
    println!("  - Envelope and magnitude modes");
    println!("  - Narrow-band resonant filtering");
    println!("  - Telephone/lo-fi effects");
    println!("\nExperiment with different frequency ranges and FFT sizes!");

    Ok(())
}
