mod tetris;
use tetris::{Tetris};

fn main() {
    println!("Hello, world!");
    let game = Tetris::new();
    game.debug();
}
