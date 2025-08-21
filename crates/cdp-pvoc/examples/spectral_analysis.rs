//! Example: Phase vocoder spectral analysis and synthesis
//!
//! This example demonstrates:
//! - Converting audio to spectral domain (.ana files)
//! - Synthesizing audio back from spectral data
//! - Extracting frequency bands
//!
//! First generate the sample files:
//!   cargo run -p cdp-housekeep --example generate_samples
//!
//! Then run this example:
//!   cargo run -p cdp-pvoc --example spectral_analysis

use cdp_pvoc::{pvoc_anal, pvoc_extract, pvoc_synth};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CDP-RS Phase Vocoder Example");
    println!("============================\n");

    // Input file from housekeep examples
    let input_file = Path::new("crates/cdp-housekeep/examples/mono_sine.wav");

    // Check if sample file exists
    if !input_file.exists() {
        println!("Sample files not found!");
        println!("Please run: cargo run -p cdp-housekeep --example generate_samples");
        return Ok(());
    }

    // Output files in pvoc examples directory
    let examples_dir = Path::new("crates/cdp-pvoc/examples");
    std::fs::create_dir_all(examples_dir)?;

    println!("Input: mono_sine.wav (440Hz sine wave)\n");

    // 1. Basic analysis with default settings
    println!("1. Basic spectral analysis (1024-point FFT, overlap 3):");
    let basic_ana = examples_dir.join("sine_basic.ana");
    pvoc_anal(input_file, &basic_ana, 1, None, None)?;
    println!("   ✓ Created: {}", basic_ana.display());

    // Get file size to show compression
    let input_size = std::fs::metadata(input_file)?.len();
    let ana_size = std::fs::metadata(&basic_ana)?.len();
    println!("   Input size:  {} bytes", input_size);
    println!(
        "   .ana size:   {} bytes ({}x larger)",
        ana_size,
        ana_size / input_size
    );

    // 2. High-resolution analysis
    println!("\n2. High-resolution analysis (4096-point FFT):");
    let hires_ana = examples_dir.join("sine_hires.ana");
    pvoc_anal(input_file, &hires_ana, 1, Some(4096), Some(4))?;
    println!("   ✓ Created: {}", hires_ana.display());
    println!("   Better frequency resolution, slower time resolution");

    // 3. Synthesis back to audio
    println!("\n3. Resynthesizing audio from spectral data:");
    let resynth_wav = examples_dir.join("sine_resynth.wav");
    pvoc_synth(&basic_ana, &resynth_wav)?;
    println!("   ✓ Created: {}", resynth_wav.display());
    println!("   Should sound identical to original");

    // 4. Extract frequency band (around 440Hz)
    println!("\n4. Extracting frequency band (300-600 Hz):");
    let filtered_ana = examples_dir.join("sine_filtered.ana");
    pvoc_extract(&basic_ana, &filtered_ana, 300.0, 600.0)?;
    println!("   ✓ Created: {}", filtered_ana.display());

    // Synthesize the filtered version
    let filtered_wav = examples_dir.join("sine_filtered.wav");
    pvoc_synth(&filtered_ana, &filtered_wav)?;
    println!("   ✓ Synthesized: {}", filtered_wav.display());
    println!("   440Hz sine should pass through this filter");

    // 5. Extract band that excludes 440Hz
    println!("\n5. Extracting frequency band that excludes 440Hz (100-400 Hz):");
    let notch_ana = examples_dir.join("sine_notch.ana");
    pvoc_extract(&basic_ana, &notch_ana, 100.0, 400.0)?;

    let notch_wav = examples_dir.join("sine_notch.wav");
    pvoc_synth(&notch_ana, &notch_wav)?;
    println!("   ✓ Created: {}", notch_wav.display());
    println!("   Should be mostly silent (440Hz is filtered out)");

    // Process a more complex signal
    let chirp_file = Path::new("crates/cdp-housekeep/examples/chirp.wav");
    if chirp_file.exists() {
        println!("\n6. Analyzing chirp signal (frequency sweep):");
        let chirp_ana = examples_dir.join("chirp.ana");
        pvoc_anal(chirp_file, &chirp_ana, 1, Some(2048), Some(4))?;
        println!("   ✓ Created: {}", chirp_ana.display());

        // Extract low frequencies only
        println!("\n7. Extracting low frequencies from chirp (100-500 Hz):");
        let chirp_low_ana = examples_dir.join("chirp_low.ana");
        pvoc_extract(&chirp_ana, &chirp_low_ana, 100.0, 500.0)?;

        let chirp_low_wav = examples_dir.join("chirp_low.wav");
        pvoc_synth(&chirp_low_ana, &chirp_low_wav)?;
        println!("   ✓ Created: {}", chirp_low_wav.display());
        println!("   Only the beginning of the sweep should be audible");
    }

    println!("\n✓ Spectral analysis examples complete!");
    println!("\nGenerated files in crates/cdp-pvoc/examples/:");
    println!("  .ana files - Spectral analysis data (IEEE float WAV)");
    println!("  .wav files - Resynthesized audio");
    println!("\nTry comparing the original and resynthesized files!");

    Ok(())
}
