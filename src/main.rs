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

    /// whether Unicode symbols are used to improve the overall look of the app
    /// will probably be removed
    #[arg(short, long, default_value_t = true)]
    unicode: bool,
}

fn main() -> Result<(), Box<dyn Error>> {
    // fn main() {
    let cli = Cli::parse();
    let tick_rate = Duration::from_millis(cli.tick_rate);
    // let mut game = Tetris::new();
    // game.debug();

    /*let mut terminal = ratatui::init();
    let mut display = Window::default();
    display
        .start(&mut terminal, &mut game)
        .expect("Error starting display");
    // }*/

    display::crossterm::run(tick_rate, cli.unicode)?;
    Ok(())
}
