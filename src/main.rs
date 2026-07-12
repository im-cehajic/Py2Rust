use anyhow::Result;
use clap::Parser;
use py2rust::cli::Cli;
use py2rust::convert;
use std::fs;

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    // Read Python source file
    let python_source = fs::read_to_string(&cli.input)?;

    // Convert to Rust
    let rust_code = convert(&python_source)?;

    // Write or print output
    if let Some(output_path) = cli.output {
        fs::write(&output_path, &rust_code)?;
        println!("✓ Converted to: {}", output_path);
    } else {
        println!("{}", rust_code);
    }

    Ok(())
}
