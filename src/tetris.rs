use crate::tetromino::{Tetromino, TetrominoTrait};
use crate::tetromino_type::TetrominoType;
use rand::seq::SliceRandom;
use ratatui::prelude::{Line, Span};
use ratatui::style::{Color, Stylize};

pub const TRUE_MAP_HEIGHT: usize = 22;
pub const MAP_WIDTH: usize = 10;
pub const MAP_HEIGHT: usize = 20;
pub const HIDDEN_ROWS: usize = TRUE_MAP_HEIGHT - MAP_HEIGHT;

pub struct Tetris {
    score: usize,
    level: usize,
    hold: TetrominoType,
    bag: Vec<TetrominoType>,
    next_bag: Vec<TetrominoType>,
    map: [[TetrominoType; MAP_WIDTH]; TRUE_MAP_HEIGHT],
    current: Tetromino,
    tick: u8,
    is_blocked: bool,
    is_lost: bool,
    has_hold_this_round: bool,
}

impl Tetris {
    fn refill_bag(&mut self) {
        self.bag = self.next_bag.clone();
        let mut new_bag: Vec<TetrominoType> = (1..8).map(TetrominoType::from_u8).collect();
        new_bag.shuffle(&mut rand::rng());
        self.next_bag = new_bag;
    }

    pub fn new() -> Self {
        let mut bag: Vec<TetrominoType> = (1..8).map(TetrominoType::from_u8).collect();
        let mut next_bag: Vec<TetrominoType> = (1..8).map(TetrominoType::from_u8).collect();
        bag.shuffle(&mut rand::rng());
        next_bag.shuffle(&mut rand::rng());
        let current = Tetromino::new(bag.pop().unwrap_or(TetrominoType::E));
        Self {
            score: 0,
            level: 0,
            hold: TetrominoType::E,
            bag,
            next_bag,
            map: [[TetrominoType::E; MAP_WIDTH]; TRUE_MAP_HEIGHT],
            current,
            tick: 0,
            is_blocked: false,
            is_lost: false,
            has_hold_this_round: false,
        }
    }

    pub fn on_tick(&mut self) {
        self.tick = (self.tick + 1) % 60; //TODO update the value "60" to increase speed at higher levels
        if self.tick != 0 {
            return;
        }
        if self.can_move([1, 0]) {
            self.r#move([1, 0]);
            return;
        }
        if self.is_blocked {
            self.lock_current();
            self.is_blocked = false;
            return;
        }
        self.is_blocked = true;
    }

    fn lock_current(&mut self) {
        for y in 0..self.current.pieces().len() {
            for x in 0..self.current.pieces()[y].len() {
                if self.current.pieces()[y][x] == TetrominoType::E {
                    continue;
                }
                self.map[(self.current.pos().0 + y as i8) as usize]
                    [(self.current.pos().1 + x as i8) as usize] = self.current.shape();
            }
        }
        self.current = Tetromino::new(self.bag.pop().unwrap_or(TetrominoType::E));

        for y in 0..self.current.pieces().len() {
            for x in 0..self.current.pieces()[y].len() {
                if self.current.pieces()[y][x] == TetrominoType::E {
                    continue;
                }
                if self.map[(self.current.pos().0 + y as i8) as usize]
                    [(self.current.pos().1 + x as i8) as usize]
                    != TetrominoType::E
                {
                    self.is_lost = true;
                }
            }
        }

        if self.bag.is_empty() {
            self.refill_bag();
        }
        self.score += (self.score + 1)
            * match self.check_lines() {
                0 => 0,
                1 => 40,
                2 => 100,
                3 => 300,
                4 => 1200,
                _ => {
                    panic!("You shouldn't clear that much line at once")
                }
            };
        self.has_hold_this_round = false;
    }

    fn check_lines(&mut self) -> u8 {
        let mut lines: u8 = 0;
        'row: for y in (0..self.map.len()).rev() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == TetrominoType::E {
                    // If line has Empty space, it's not empty
                    continue 'row;
                }
            }

            lines = self.delete_line(y);
            self.map[0] = [TetrominoType::E; MAP_WIDTH]; // Clear the top line as it won't be moved from the line -1
        }
        lines
    }

    fn delete_line(&mut self, line: usize) -> u8 {
        for i in (0..line).rev() {
            // Move each line below
            self.map[i + 1] = self.map[i];
        }
        if !self.map[line].into_iter().any(|x| x == TetrominoType::E)
        // If we just copied another full line, delete it again
        {
            return self.delete_line(line) + 1;
        }
        1
    }

    pub fn get_map(&self) -> Vec<Line> {
        let mut display_map_data: [[TetrominoType; MAP_WIDTH]; MAP_HEIGHT] =
            [[TetrominoType::E; MAP_WIDTH]; MAP_HEIGHT];
        display_map_data.copy_from_slice(&self.map[HIDDEN_ROWS..]);

        //TODO: probably move this to the part where the program create the Span etc.
        for y in 0..self.current.pieces().len() {
            for x in 0..self.current.pieces()[y].len() {
                if self.current.pieces()[y][x] != TetrominoType::E
                    && y as i8 + self.current.pos().0 >= HIDDEN_ROWS as i8
                {
                    display_map_data
                        [(y as i8 + self.current.pos().0 - HIDDEN_ROWS as i8) as usize]
                        [(x as i8 + self.current.pos().1) as usize] = self.current.pieces()[y][x];
                }
            }
        }

        let shadow_map = self.get_current_shadow();

        display_map_data
            .iter()
            .enumerate()
            .map(|(y, row)| {
                Line::from(
                    row.iter()
                        .enumerate()
                        .map(|(x, col)| {
                            if col != TetrominoType::E || shadow_map[y][x] == TetrominoType::E {
                                Span::raw("  ").bg(col.get_color())
                            } else {
                                Span::raw("::")
                                    .fg(shadow_map[y][x].get_color())
                                    .bg(Color::Reset)
                            }
                        })
                        .collect::<Vec<Span>>(),
                )
            })
            .collect()
    }

    pub fn get_score(&self) -> usize {
        self.score
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.current.rotate_counter_clockwise(self.map);
    }

    pub fn rotate_clockwise(&mut self) {
        self.current.rotate_clockwise(self.map);
    }

    fn can_move(&self, vector: [i8; 2]) -> bool {
        for (y, row) in self.current.pieces().as_slice().iter().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                if *piece == TetrominoType::E {
                    continue;
                }
                let next_y: i8 = y as i8 + self.current.pos().0 + vector[0];
                let next_x: i8 = x as i8 + self.current.pos().1 + vector[1];

                if next_y >= TRUE_MAP_HEIGHT as i8 {
                    return false;
                }
                if next_x < 0 || next_x >= MAP_WIDTH as i8 {
                    return false;
                }
                if self.map[next_y as usize][next_x as usize] != TetrominoType::E {
                    return false;
                }
            }
        }
        true
    }

    pub fn r#move(&mut self, vector: [i8; 2]) {
        if self.can_move(vector) {
            self.current.r#move(vector);
        }
    }

    pub fn is_lost(&self) -> bool {
        self.is_lost
    }

    pub fn hold(&mut self) {
        if self.has_hold_this_round {
            return;
        }
        let swap: TetrominoType = self.current.shape();
        if self.hold == TetrominoType::E {
            self.current = Tetromino::new(self.bag.pop().unwrap_or(TetrominoType::E));
        } else {
            self.current = Tetromino::new(self.hold);
        }
        self.hold = swap;
        self.has_hold_this_round = true;
    }

    pub fn get_hold(&self) -> TetrominoType {
        self.hold
    }

    pub fn get_nexts(&self) -> Vec<TetrominoType> {
        let mut nexts: Vec<TetrominoType> = Vec::new();
        for tetromino in self.bag.iter().rev() {
            if nexts.len() >= 5 {
                break;
            }
            nexts.push(*tetromino);
        }
        for tetromino in self.next_bag.iter().rev() {
            if nexts.len() >= 5 {
                break;
            }
            nexts.push(*tetromino);
        }
        nexts.to_vec()
    }

    fn get_current_shadow(&self) -> [[TetrominoType; MAP_WIDTH]; MAP_HEIGHT] {
        let mut shadow_map = [[TetrominoType::E; MAP_WIDTH]; MAP_HEIGHT];

        let mut worst_down: usize = MAP_HEIGHT - 2; // 2 is the minimum tetromino height IN THE WAY I HANDLED THEM
        for piece_y in 0..self.current.pieces().len() {
            for piece_x in 0..self.current.pieces()[piece_y].len() {
                let mut y: usize =
                    self.current.pos().0 as usize - (if self.current.pos().0 > 0 { 1 } else { 0 }); //minus 1 but don't do it if it's 0 as it's  usize
                if self.current.pieces()[piece_y][piece_x] == TetrominoType::E {
                    continue;
                }
                while piece_y + y < MAP_HEIGHT
                    && self.map[piece_y + y + HIDDEN_ROWS]
                        [(piece_x as i8 + self.current.pos().1) as usize]
                        == TetrominoType::E
                {
                    y += 1;
                }
                if y > 0 && y - 1 < worst_down {
                    // Avoid y - 1 if y is 0 since it's a usize
                    worst_down = y - 1;
                }
            }
        }

        for piece_y in 0..self.current.pieces().len() {
            for piece_x in 0..self.current.pieces()[piece_y].len() {
                if self.current.pieces()[piece_y][piece_x] != TetrominoType::E {
                    shadow_map[piece_y + worst_down]
                        [(piece_x as i8 + self.current.pos().1) as usize] = self.current.shape();
                }
            }
        }

        shadow_map
    }
}
