//! CDP-compatible stretch command-line interface

use cdp_spectral::{calculate_output_duration, stretch_time};
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("CDP Release 7.1 2016");
        eprintln!();
        eprintln!("STRETCHING A SPECTRAL FILE");
        eprintln!();
        eprintln!("USAGE: stretch NAME (mode) infile outfile parameters:");
        eprintln!();
        eprintln!("where NAME can be any one of");
        eprintln!("spectrum      time");
        eprintln!();
        eprintln!("Type 'stretch spectrum' for more info on stretch spectrum..ETC.");
        std::process::exit(1);
    }

    let mode = &args[1];

    match mode.as_str() {
        "time" => {
            if args.len() < 3 {
                eprintln!("CDP Release 7.1 2016");
                eprintln!("stretch time 1 infile outfile timestretch");
                eprintln!("stretch time 2 infile timestretch");
                eprintln!();
                eprintln!("TIME-STRETCHING OF INFILE.");
                eprintln!("In mode 2, program calculates length of output, only.");
                eprintln!("Timestretch may itself vary over time.");
                std::process::exit(1);
            }

            let submode = args[2].parse::<i32>().unwrap_or(0);

            match submode {
                1 => {
                    // Mode 1: Actual time stretching
                    if args.len() < 6 {
                        eprintln!("CDP Release 7.1 2016");
                        eprintln!("stretch time 1 infile outfile timestretch");
                        eprintln!();
                        eprintln!("TIME-STRETCHING OF INFILE.");
                        eprintln!("Timestretch may itself vary over time.");
                        std::process::exit(1);
                    }

                    let infile = Path::new(&args[3]);
                    let outfile = Path::new(&args[4]);
                    let timestretch = args[5].parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("ERROR: Invalid timestretch value: {}", args[5]);
                        std::process::exit(1);
                    });

                    if timestretch <= 0.0 {
                        eprintln!("ERROR: Timestretch must be greater than 0");
                        std::process::exit(1);
                    }

                    eprintln!("CDP Release 7.1 2016");
                    eprintln!("stretch time 1 infile outfile timestretch");
                    eprintln!();
                    eprintln!("TIME-STRETCHING OF INFILE.");
                    eprintln!("Timestretch may itself vary over time.");
                    eprintln!();
                    eprintln!("time-stretching beginning");

                    match stretch_time(infile, outfile, timestretch) {
                        Ok(()) => {
                            eprintln!("COMPLETED");
                            std::process::exit(0);
                        }
                        Err(e) => {
                            eprintln!("ERROR: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                2 => {
                    // Mode 2: Calculate output duration only
                    if args.len() < 5 {
                        eprintln!("CDP Release 7.1 2016");
                        eprintln!("stretch time 2 infile timestretch");
                        eprintln!();
                        eprintln!("TIME-STRETCHING OF INFILE.");
                        eprintln!("In mode 2, program calculates length of output, only.");
                        eprintln!("Timestretch may itself vary over time.");
                        std::process::exit(1);
                    }

                    let infile = Path::new(&args[3]);
                    let timestretch = args[4].parse::<f64>().unwrap_or_else(|_| {
                        eprintln!("ERROR: Invalid timestretch value: {}", args[4]);
                        std::process::exit(1);
                    });

                    if timestretch <= 0.0 {
                        eprintln!("ERROR: Timestretch must be greater than 0");
                        std::process::exit(1);
                    }

                    eprintln!("CDP Release 7.1 2016");
                    eprintln!("stretch time 2 infile timestretch");
                    eprintln!();
                    eprintln!("TIME-STRETCHING OF INFILE.");
                    eprintln!("In mode 2, program calculates length of output, only.");
                    eprintln!("Timestretch may itself vary over time.");
                    eprintln!();

                    match calculate_output_duration(infile, timestretch) {
                        Ok(duration) => {
                            println!("INFO: Length of output file will be {:.3} secs.", duration);
                            std::process::exit(0);
                        }
                        Err(e) => {
                            eprintln!("ERROR: {}", e);
                            std::process::exit(1);
                        }
                    }
                }
                _ => {
                    eprintln!("ERROR: Invalid mode: {}. Use 1 or 2", submode);
                    std::process::exit(1);
                }
            }
        }
        "spectrum" => {
            eprintln!("CDP Release 7.1 2016");
            eprintln!("stretch spectrum    NOT YET IMPLEMENTED");
            std::process::exit(1);
        }
        _ => {
            eprintln!("CDP Release 7.1 2016");
            eprintln!("ERROR: Unknown mode: {}", mode);
            eprintln!();
            eprintln!("STRETCHING A SPECTRAL FILE");
            eprintln!();
            eprintln!("USAGE: stretch NAME (mode) infile outfile parameters:");
            eprintln!();
            eprintln!("where NAME can be any one of");
            eprintln!("spectrum      time");
            eprintln!();
            eprintln!("Type 'stretch spectrum' for more info on stretch spectrum..ETC.");
            std::process::exit(1);
        }
    }
}
