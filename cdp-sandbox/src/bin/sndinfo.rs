//! Thin binary wrapper for sndinfo operations
//!
//! This exists purely for oracle validation against CDP.

use cdp_sandbox::sndinfo;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("CDP-RS SndInfo (Oracle Validation Binary)");
        eprintln!("Usage: sndinfo <operation> <infile> [args...]");
        eprintln!("Operations: props, maxsamp, len, etc.");
        process::exit(1);
    }

    let operation = &args[1];
    let op_args: Vec<&str> = args[2..].iter().map(|s| s.as_str()).collect();

    if let Err(e) = sndinfo::sndinfo(operation, &op_args) {
        eprintln!("ERROR: {}", e);
        process::exit(1);
    }
}
