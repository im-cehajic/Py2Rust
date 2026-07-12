use clap::Parser;
use std::path::PathBuf;

/// Py2Rust: Convert Python code to Rust
#[derive(Parser, Debug)]
#[command(name = "Py2Rust")]
#[command(about = "Convert Python code to idiomatic Rust", long_about = None)]
pub struct Cli {
    /// Input Python file path
    #[arg(short, long)]
    pub input: PathBuf,

    /// Output Rust file path (optional, prints to stdout if not specified)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Enable verbose output
    #[arg(short, long)]
    pub verbose: bool,
}
