mod display;
mod tetris;

use crate::display::window::Window;
use tetris::Tetris;

fn main() {
    let mut game = Tetris::new();
    // game.debug();

    let mut terminal = ratatui::init();
    let mut display = Window::default();
    display
        .start(&mut terminal, &mut game)
        .expect("Error starting display");
}
