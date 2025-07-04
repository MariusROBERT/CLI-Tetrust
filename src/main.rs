mod display;
mod tetris;

use std::error::Error;
use std::time::Duration;

use clap::Parser;

/// Demo
#[derive(Debug, Parser)]
struct Cli {
    /// time in ms between two ticks.
    /// will probably be removed
    #[arg(short, long, default_value_t = 250)]
    tick_rate: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let tick_rate = Duration::from_millis(cli.tick_rate);

    display::crossterm::run(tick_rate)?;
    Ok(())
}
