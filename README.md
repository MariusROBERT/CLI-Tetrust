# TetRust

---

### Objectives

In this project, my goal is to learn Rust by coding a Tetris.

I also wanted to do it in TUI (Terminal UI), for this I use [RatatUi](https://ratatui.rs/)
with [CrossTerm](https://github.com/crossterm-rs/crossterm).

### References

To code Tetris, I mainly help myself with these 2 wikis : [Harddrop](https://harddrop.com/wiki/Tetris_Wiki)
and [Tetris fandom](https://tetris.fandom.com/wiki/Tetris_Wiki).

I will probably mix some features of different games to use the ones that seems the most useful and logical to me.
Don't expect this Tetris to be 100% compliant with the official rules.

---

### TODO

- [ ] Playable game
    - [x] Display the grid
    - [x] Display the current tetromino
    - [x] Move the current tetromino
    - [x] Basic rotation
    - [x] Tetromino automatically move down
    - [x] Tetromino lock when reaching bottom and spawning a new one
    - [x] Clear line when full
    - [x] Detect lose


- [ ] Advanced features
    - [ ] Super Rotation System
    - [x] Show the next tetrominoes
    - [x] Being able to hold a piece
    - [x] Basic scoring system (lines cleared)
    - [ ] Advanced scoring system (BtB, combos, T-Spin etc.)
    - [ ] Increase level and speed with score
    - [ ] Hard drop


- [ ] QOL features
    - [x] Show has a shadow where the piece will fall if not moved or rotate
    - [ ] Adapt display depending on the size of the window
    - [x] Add a start menu
    - [ ] Add a pause option and pause menu
    - [ ] Save scores somewhere

- [ ] Bugs
    - [ ] Shadow seems buggy when playfield is full or almost