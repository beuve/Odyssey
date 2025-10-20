use clap::Parser;

pub mod cli;

use crate::cli::Cli;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    cli.exec();
    Ok(())
}
