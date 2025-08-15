//! Demonstrates how oracle testing works with CDP binaries

use cdp_oracle::{OracleConfig, Validator, TestGenerator};
use cdp_sandbox::experiments::ExperimentalPvoc;

fn main() {
    println!("CDP-RS Oracle Testing Demo");
    println!("==========================\n");
    
    // Configure oracle
    let mut config = OracleConfig::default();
    config.keep_temp_files = true; // Keep files for inspection
    
    println!("1. Creating test signals:");
    let sine = TestGenerator::sine_wave(440.0, 0.1, 44100);
    println!("   - Sine wave: 440Hz, 0.1s, {} samples", sine.len());
    
    let noise = TestGenerator::white_noise(0.1, 44100);
    println!("   - White noise: 0.1s, {} samples", noise.len());
    
    let chirp = TestGenerator::chirp(100.0, 1000.0, 0.1, 44100);
    println!("   - Chirp: 100-1000Hz, 0.1s, {} samples", chirp.len());
    
    println!("\n2. Setting up validator (would connect to CDP binaries)");
    let mut validator = match Validator::new(config) {
        Ok(v) => v,
        Err(e) => {
            println!("   Error: {}", e);
            println!("   (This is expected if CDP binaries aren't installed)");
            return;
        }
    };
    
    println!("\n3. Testing ExperimentalPvoc implementation:");
    let pvoc = ExperimentalPvoc::new(2048, 4).unwrap();
    println!("   - FFT size: 2048");
    println!("   - Overlap: 4");
    println!("   - CDP equivalent: pvoc -N2048 -W4");
    
    println!("\n4. Running validation (would compare against CDP):");
    
    for (name, signal) in [("sine", sine), ("noise", noise), ("chirp", chirp)] {
        print!("   Testing with {} signal... ", name);
        
        match validator.validate(&pvoc, &signal, 44100) {
            Ok(result) => {
                if result.passed {
                    println!("PASSED");
                    println!("     Spectral correlation: {:.4}", result.spectral_correlation);
                    println!("     RMS difference: {:.6}", result.rms_difference);
                } else {
                    println!("FAILED");
                    println!("     {}", result.report());
                }
            }
            Err(e) => {
                println!("ERROR: {}", e);
                println!("     (CDP binaries needed for actual validation)");
            }
        }
    }
    
    println!("\n5. Once validated:");
    println!("   - Move implementation from cdp-sandbox to cdp-pvoc");
    println!("   - Mark cdp-pvoc as FROZEN");
    println!("   - Update FROZEN_MODULES.md");
    println!("\nThis ensures our Rust implementation matches CDP exactly!");
}