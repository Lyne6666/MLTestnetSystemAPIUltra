// src/main.rs
/*
 * Main executable for MLTestnetSystemAPIUltra
 */

use clap::Parser;
use mltestnetsystemapiultra::{Result, run};

#[derive(Parser)]
#[command(version, about = "MLTestnetSystemAPIUltra - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
