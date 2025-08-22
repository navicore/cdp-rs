//! CDP-compatible distort command

use anyhow::Result;
use cdp_distort::{divide, multiply, overload, ClipType};
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "distort")]
#[command(about = "CDP-compatible distortion effects")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Harmonic multiplication distortion
    Multiply {
        /// Input audio file
        input: PathBuf,
        /// Output audio file
        output: PathBuf,
        /// Multiplication factor (1.0-16.0)
        #[arg(short, long, default_value = "2.0")]
        factor: f32,
        /// Dry/wet mix (0.0-1.0)
        #[arg(short, long, default_value = "1.0")]
        mix: f32,
    },
    /// Subharmonic division distortion
    Divide {
        /// Input audio file
        input: PathBuf,
        /// Output audio file
        output: PathBuf,
        /// Division factor (2-16)
        #[arg(short, long, default_value = "2")]
        factor: u32,
        /// Dry/wet mix (0.0-1.0)
        #[arg(short, long, default_value = "1.0")]
        mix: f32,
    },
    /// Clipping/overload distortion
    Overload {
        /// Input audio file
        input: PathBuf,
        /// Output audio file
        output: PathBuf,
        /// Clipping threshold (0.1-1.0)
        #[arg(short, long, default_value = "0.7")]
        threshold: f32,
        /// Drive amount (1.0-100.0)
        #[arg(short, long, default_value = "2.0")]
        drive: f32,
        /// Clipping type (hard, soft, tube, asymmetric)
        #[arg(short = 'c', long, default_value = "soft")]
        clip_type: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Multiply {
            input,
            output,
            factor,
            mix,
        } => {
            multiply(&input, &output, factor, mix)?;
            println!("Applied harmonic multiplication distortion");
        }
        Commands::Divide {
            input,
            output,
            factor,
            mix,
        } => {
            divide(&input, &output, factor, mix)?;
            println!("Applied subharmonic division distortion");
        }
        Commands::Overload {
            input,
            output,
            threshold,
            drive,
            clip_type,
        } => {
            let clip = match clip_type.to_lowercase().as_str() {
                "hard" => ClipType::Hard,
                "soft" => ClipType::Soft,
                "tube" => ClipType::Tube,
                "asymmetric" => ClipType::Asymmetric,
                _ => {
                    eprintln!("Invalid clip type. Using soft clipping.");
                    ClipType::Soft
                }
            };
            overload(&input, &output, threshold, drive, clip)?;
            println!("Applied {} clipping distortion", clip_type);
        }
    }

    Ok(())
}
