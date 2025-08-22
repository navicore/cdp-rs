//! Example demonstrating pitch shifting effects

use cdp_pvoc::{pvoc_anal, pvoc_synth};
use cdp_spectral::{pitch_shift, pitch_shift_formant, semitones_to_factor};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Pitch Shifting Examples");
    println!("=======================");

    // Create examples directory
    let examples_dir = Path::new("crates/cdp-spectral/examples");
    fs::create_dir_all(examples_dir)?;

    // Generate sample audio
    println!("\nGenerating sample audio...");
    Command::new("cargo")
        .args([
            "run",
            "-p",
            "cdp-housekeep",
            "--example",
            "generate_samples",
        ])
        .output()?;

    // Use a harmonic-rich sample
    let input_wav = Path::new("crates/cdp-housekeep/examples/sawtooth_tone.wav");
    if !input_wav.exists() {
        let alt_input = Path::new("crates/cdp-housekeep/examples/sine_tone.wav");
        if !alt_input.exists() {
            eprintln!("Sample file not found. Please run generate_samples first.");
            return Ok(());
        }
        fs::copy(alt_input, examples_dir.join("input.wav"))?;
    } else {
        fs::copy(input_wav, examples_dir.join("input.wav"))?;
    }

    let input_wav = examples_dir.join("input.wav");

    // Convert to spectral domain
    println!("\nConverting to spectral domain...");
    let ana_file = examples_dir.join("input.ana");
    pvoc_anal(&input_wav, &ana_file, 1, Some(2048), Some(4))?;

    // Example 1: Octave up
    println!("\n1. Octave up (+12 semitones):");
    let octave_up = examples_dir.join("octave_up.ana");
    pitch_shift(&ana_file, &octave_up, 2.0)?;
    println!("   Created: {}", octave_up.display());
    println!("   Effect: Doubles frequency, chipmunk effect");

    // Example 2: Octave down
    println!("\n2. Octave down (-12 semitones):");
    let octave_down = examples_dir.join("octave_down.ana");
    pitch_shift(&ana_file, &octave_down, 0.5)?;
    println!("   Created: {}", octave_down.display());
    println!("   Effect: Halves frequency, deep voice effect");

    // Example 3: Perfect fifth up
    println!("\n3. Perfect fifth up (+7 semitones):");
    let fifth_up = examples_dir.join("fifth_up.ana");
    let fifth_factor = semitones_to_factor(7.0);
    pitch_shift(&ana_file, &fifth_up, fifth_factor)?;
    println!("   Created: {}", fifth_up.display());
    println!("   Effect: Musical interval, harmonious");

    // Example 4: Minor third down
    println!("\n4. Minor third down (-3 semitones):");
    let third_down = examples_dir.join("third_down.ana");
    let third_factor = semitones_to_factor(-3.0);
    pitch_shift(&ana_file, &third_down, third_factor)?;
    println!("   Created: {}", third_down.display());
    println!("   Effect: Subtle pitch drop, darker tone");

    // Example 5: Microtonal shift
    println!("\n5. Quarter-tone up (+0.5 semitones):");
    let quarter_tone = examples_dir.join("quarter_tone.ana");
    let quarter_factor = semitones_to_factor(0.5);
    pitch_shift(&ana_file, &quarter_tone, quarter_factor)?;
    println!("   Created: {}", quarter_tone.display());
    println!("   Effect: Slightly sharp, detuned effect");

    // Example 6: Formant-preserved pitch shift
    println!("\n6. Octave up with formant preservation:");
    let formant_shift = examples_dir.join("formant_preserved.ana");
    pitch_shift_formant(&ana_file, &formant_shift, 2.0, true)?;
    println!("   Created: {}", formant_shift.display());
    println!("   Effect: Pitch changes but timbre preserved");

    // Convert back to audio
    println!("\nConverting pitched spectra back to audio...");

    for (ana, wav_name) in &[
        (&octave_up, "octave_up.wav"),
        (&octave_down, "octave_down.wav"),
        (&fifth_up, "fifth_up.wav"),
        (&third_down, "third_down.wav"),
        (&quarter_tone, "quarter_tone.wav"),
        (&formant_shift, "formant_preserved.wav"),
    ] {
        let output_wav = examples_dir.join(wav_name);
        pvoc_synth(ana, &output_wav)?;
        println!("   Created audio: {}", output_wav.display());
    }

    println!("\nTips:");
    println!("- Pitch shifting changes frequency without changing duration");
    println!("- Musical intervals: octave=12, fifth=7, fourth=5, major third=4 semitones");
    println!("- Formant preservation maintains vocal/instrument character");
    println!("- Extreme shifts (>2 octaves) may introduce artifacts");
    println!("- Combine with stretch for independent time/pitch control");

    Ok(())
}
