mod display;
mod menu;
mod tetris;
mod tetromino;
mod tetromino_type;

use std::error::Error;
use std::time::Duration;

use clap::Parser;

/// Demo
#[derive(Debug, Parser)]
struct Cli {
    /// time in ms between two ticks.
    /// will maybe be removed as user shouldn't be able to edit it
    #[arg(short, long, default_value_t = 16)] //62.5 fps
    tick_rate: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    let tick_rate = Duration::from_millis(cli.tick_rate);

    display::crossterm::run(tick_rate)?;
    Ok(())
}
