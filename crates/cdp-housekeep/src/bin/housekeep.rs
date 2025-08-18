//! Thin binary wrapper for housekeep operations
//!
//! This exists purely for oracle validation against CDP.
//! The real implementation lives in the library.

use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("CDP-RS Housekeep (Oracle Validation Binary)");
        eprintln!("Usage: housekeep <operation> [args...]");
        eprintln!("Operations: copy, chans, extract");
        process::exit(1);
    }

    let operation = &args[1];
    let op_args: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();

    if let Err(e) = cdp_housekeep::housekeep(operation, &op_args) {
        eprintln!("ERROR: {}", e);
        process::exit(1);
    }
}
