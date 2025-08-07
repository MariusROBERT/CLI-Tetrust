use rand::seq::SliceRandom;
use ratatui::prelude::{Line, Span};
use ratatui::style::{Color, Stylize};
use std::cmp::PartialEq;

const TRUE_MAP_HEIGHT: usize = 22;
const MAP_WIDTH: usize = 10;
const MAP_HEIGHT: usize = 20;
const HIDDEN_ROWS: usize = TRUE_MAP_HEIGHT - MAP_HEIGHT;

#[derive(Clone, Debug, PartialEq, Copy)]
pub enum TetrominoType {
    E = 0, //Empty
    I = 1,
    L = 2,
    J = 3,
    O = 4,
    Z = 5,
    S = 6,
    T = 7,
}

impl PartialEq<TetrominoType> for &TetrominoType {
    fn eq(&self, other: &TetrominoType) -> bool {
        match (self, other) {
            (TetrominoType::E, TetrominoType::E) => true,
            (TetrominoType::I, TetrominoType::I) => true,
            (TetrominoType::L, TetrominoType::L) => true,
            (TetrominoType::J, TetrominoType::J) => true,
            (TetrominoType::O, TetrominoType::O) => true,
            (TetrominoType::Z, TetrominoType::Z) => true,
            (TetrominoType::S, TetrominoType::S) => true,
            (TetrominoType::T, TetrominoType::T) => true,
            (_, _) => false,
        }
    }
}

impl TetrominoType {
    fn from_u8(value: u8) -> TetrominoType {
        match value {
            1 => TetrominoType::I,
            2 => TetrominoType::L,
            3 => TetrominoType::J,
            4 => TetrominoType::O,
            5 => TetrominoType::Z,
            6 => TetrominoType::S,
            7 => TetrominoType::T,
            _ => TetrominoType::E,
        }
    }

    pub fn get_color(&self) -> Color {
        match self {
            TetrominoType::I => Color::Cyan,
            TetrominoType::L => Color::Yellow,
            TetrominoType::J => Color::Blue,
            TetrominoType::O => Color::LightYellow,
            TetrominoType::Z => Color::Red,
            TetrominoType::S => Color::Green,
            TetrominoType::T => Color::Magenta,
            TetrominoType::E => Color::White,
        }
    }

    pub fn as_ratatui_text(&self) -> Vec<Line> {
        (match self {
            TetrominoType::E => {
                vec![vec![]]
            }
            TetrominoType::I => {
                vec![vec![], vec![], vec![TetrominoType::I; 4]]
            }
            TetrominoType::L => {
                vec![
                    vec![],
                    vec![
                        TetrominoType::E,
                        TetrominoType::E,
                        TetrominoType::E,
                        TetrominoType::L,
                    ],
                    vec![
                        TetrominoType::E,
                        TetrominoType::L,
                        TetrominoType::L,
                        TetrominoType::L,
                    ],
                ]
            }
            TetrominoType::J => {
                vec![
                    vec![],
                    vec![
                        TetrominoType::E,
                        TetrominoType::J,
                        TetrominoType::E,
                        TetrominoType::E,
                    ],
                    vec![
                        TetrominoType::E,
                        TetrominoType::J,
                        TetrominoType::J,
                        TetrominoType::J,
                    ],
                ]
            }
            TetrominoType::O => {
                vec![
                    vec![],
                    vec![TetrominoType::E, TetrominoType::O, TetrominoType::O],
                    vec![TetrominoType::E, TetrominoType::O, TetrominoType::O],
                ]
            }
            TetrominoType::Z => {
                vec![
                    vec![],
                    vec![TetrominoType::E, TetrominoType::Z, TetrominoType::Z],
                    vec![
                        TetrominoType::E,
                        TetrominoType::E,
                        TetrominoType::Z,
                        TetrominoType::Z,
                    ],
                ]
            }
            TetrominoType::S => {
                vec![
                    vec![],
                    vec![
                        TetrominoType::E,
                        TetrominoType::E,
                        TetrominoType::S,
                        TetrominoType::S,
                    ],
                    vec![TetrominoType::E, TetrominoType::S, TetrominoType::S],
                ]
            }
            TetrominoType::T => {
                vec![
                    vec![],
                    vec![TetrominoType::E, TetrominoType::E, TetrominoType::T],
                    vec![
                        TetrominoType::E,
                        TetrominoType::T,
                        TetrominoType::T,
                        TetrominoType::T,
                    ],
                ]
            }
        })
        .iter()
        .map(|row| {
            Line::from({
                if self == TetrominoType::I || self == TetrominoType::O {
                    let mut before = vec![Span::raw(" ")]; // Single space instead of double to center I and O Tetrominos
                    before.append(
                        &mut row
                            .iter()
                            .map(|tetromino_type| Span::raw("  ").bg(tetromino_type.get_color()))
                            .collect::<Vec<Span>>(),
                    );
                    before
                } else {
                    row.iter()
                        .map(|tetromino_type| Span::raw("  ").bg(tetromino_type.get_color()))
                        .collect::<Vec<Span>>()
                }
            })
        })
        .collect()
    }
}

#[derive(Clone, Debug)]
pub struct Tetromino {
    shape: TetrominoType,
    rotation: u8,
    pos: (i8, i8),
    pieces: Vec<Vec<TetrominoType>>,
}

pub trait TetrominoTrait {
    fn rotate_clockwise(&mut self, map: [[TetrominoType; MAP_WIDTH]; TRUE_MAP_HEIGHT]);
    fn rotate_counter_clockwise(&mut self, map: [[TetrominoType; MAP_WIDTH]; TRUE_MAP_HEIGHT]);
}

impl TetrominoTrait for Tetromino {
    fn rotate_clockwise(&mut self, map: [[TetrominoType; MAP_WIDTH]; TRUE_MAP_HEIGHT]) {
        match self.shape {
            TetrominoType::E => panic!("Empty tetromino shouldn't be here"),
            TetrominoType::O => { /*No rotation needed*/ }
            TetrominoType::I => self.rotate_i(
                map,
                [4, 8, 12, 13, 14, 15, 11, 7, 3, 2, 1, 0],
                [9, 10, 6, 5],
            ),
            _ => self.rotate(map, [3, 6, 7, 8, 5, 2, 1, 0]),
        };
        self.rotation = (self.rotation + 1) % 4;
    }

    fn rotate_counter_clockwise(&mut self, map: [[TetrominoType; MAP_WIDTH]; TRUE_MAP_HEIGHT]) {
        match self.shape {
            TetrominoType::E => panic!("Empty tetromino shouldn't be here"),
            TetrominoType::O => { /*No rotation needed*/ }
            TetrominoType::I => self.rotate_i(
                map,
                [0, 1, 2, 3, 7, 11, 15, 14, 13, 12, 8, 4],
                [5, 6, 10, 9],
            ),
            _ => self.rotate(map, [0, 1, 2, 5, 8, 7, 6, 3]),
        }
        self.rotation = (self.rotation + 3) % 4;
    }
}

impl Tetromino {
    pub fn new(shape: TetrominoType) -> Self {
        if shape == TetrominoType::E {
            panic!("Cannot create Tetris");
        }
        let pieces: Vec<Vec<TetrominoType>> = match shape {
            TetrominoType::E => {
                vec![vec![TetrominoType::E; 4]; 4]
            }
            TetrominoType::I => {
                vec![
                    vec![TetrominoType::E; 4],
                    vec![TetrominoType::I; 4],
                    vec![TetrominoType::E; 4],
                    vec![TetrominoType::E; 4],
                ]
            }
            TetrominoType::L => {
                vec![
                    vec![TetrominoType::E, TetrominoType::E, TetrominoType::L],
                    vec![TetrominoType::L, TetrominoType::L, TetrominoType::L],
                    vec![TetrominoType::E, TetrominoType::E, TetrominoType::E],
                ]
            }
            TetrominoType::J => {
                vec![
                    vec![TetrominoType::J, TetrominoType::E, TetrominoType::E],
                    vec![TetrominoType::J, TetrominoType::J, TetrominoType::J],
                    vec![TetrominoType::E, TetrominoType::E, TetrominoType::E],
                ]
            }
            TetrominoType::O => {
                vec![
                    vec![TetrominoType::E, TetrominoType::O, TetrominoType::O],
                    vec![TetrominoType::E, TetrominoType::O, TetrominoType::O],
                ]
            }
            TetrominoType::Z => {
                vec![
                    vec![TetrominoType::Z, TetrominoType::Z, TetrominoType::E],
                    vec![TetrominoType::E, TetrominoType::Z, TetrominoType::Z],
                    vec![TetrominoType::E, TetrominoType::E, TetrominoType::E],
                ]
            }
            TetrominoType::S => {
                vec![
                    vec![TetrominoType::E, TetrominoType::S, TetrominoType::S],
                    vec![TetrominoType::S, TetrominoType::S, TetrominoType::E],
                    vec![TetrominoType::E, TetrominoType::E, TetrominoType::E],
                ]
            }
            TetrominoType::T => {
                vec![
                    vec![TetrominoType::E, TetrominoType::T, TetrominoType::E],
                    vec![TetrominoType::T, TetrominoType::T, TetrominoType::T],
                    vec![TetrominoType::E, TetrominoType::E, TetrominoType::E],
                ]
            }
        };
        Self {
            shape,
            rotation: 0,
            pos: (0, 3),
            pieces,
        }
    }

    fn can_rotate(
        &self,
        map: [[TetrominoType; MAP_WIDTH]; TRUE_MAP_HEIGHT],
        round_order: [usize; 8],
    ) -> u8 {
        let mut swap_pos: usize;
        for i in 0..round_order.len() {
            swap_pos = round_order[(i + 2) % 8];
            if self.pieces[round_order[i] / 3][round_order[i] % 3] == TetrominoType::E {
                continue;
            }

            let next_x: i8 = self.pos.1 + (swap_pos % 3) as i8;
            let next_y: i8 = self.pos.0 + (swap_pos / 3) as i8;

            if next_y >= TRUE_MAP_HEIGHT as i8 {
                return 0;
            }
            if next_x < 0 || next_x >= MAP_WIDTH as i8 {
                return 0;
            }
            if map[next_y as usize][next_x as usize] != TetrominoType::E {
                return 0;
            }
        }
        1
    }

    fn rotate(
        &mut self,
        map: [[TetrominoType; MAP_WIDTH]; TRUE_MAP_HEIGHT],
        round_order: [usize; 8],
    ) {
        let mut swap: TetrominoType;
        for _ in 0..self.can_rotate(map, round_order) {
            for i in 0..round_order.len() - 2 {
                swap = self.pieces[round_order[i] / 3][round_order[i] % 3];
                self.pieces[round_order[i] / 3][round_order[i] % 3] =
                    self.pieces[round_order[i + 2] / 3][round_order[i + 2] % 3];
                self.pieces[round_order[i + 2] / 3][round_order[i + 2] % 3] = swap;
            }
        }
    }
    fn can_rotate_i(
        &self,
        map: [[TetrominoType; MAP_WIDTH]; TRUE_MAP_HEIGHT],
        round_order1: [usize; 12],
        round_order2: [usize; 4],
    ) -> u8 {
        let mut swap_pos: usize;
        for i in 0..round_order1.len() {
            swap_pos = round_order1[(i + 3) % 12];
            if self.pieces[round_order1[i] / 4][round_order1[i] % 4] == TetrominoType::E {
                continue;
            }

            let next_x: i8 = self.pos.1 + (swap_pos % 4) as i8;
            let next_y: i8 = self.pos.0 + (swap_pos / 4) as i8;

            if next_y >= TRUE_MAP_HEIGHT as i8 {
                return 0;
            }
            if next_x < 0 || next_x >= MAP_WIDTH as i8 {
                return 0;
            }
            if map[next_y as usize][next_x as usize] != TetrominoType::E {
                return 0;
            }
        }
        for i in 0..round_order2.len() {
            swap_pos = round_order2[(i + 1) % 4];
            if self.pieces[round_order2[i] / 4][round_order2[i] % 4] == TetrominoType::E {
                continue;
            }

            let next_x: i8 = self.pos.1 + (swap_pos % 4) as i8;
            let next_y: i8 = self.pos.0 + (swap_pos / 4) as i8;

            if next_y >= TRUE_MAP_HEIGHT as i8 {
                return 0;
            }
            if next_x < 0 || next_x >= MAP_WIDTH as i8 {
                return 0;
            }
            if map[next_y as usize][next_x as usize] != TetrominoType::E {
                return 0;
            }
        }
        1
    }

    fn rotate_i(
        &mut self,
        map: [[TetrominoType; MAP_WIDTH]; TRUE_MAP_HEIGHT],
        round_order1: [usize; 12],
        round_order2: [usize; 4],
    ) {
        let mut swap: TetrominoType;
        for _ in 0..self.can_rotate_i(map, round_order1, round_order2) {
            for i in 0..round_order1.len() - 3 {
                swap = self.pieces[round_order1[i] / 4][round_order1[i] % 4];
                self.pieces[round_order1[i] / 4][round_order1[i] % 4] =
                    self.pieces[round_order1[i + 3] / 4][round_order1[i + 3] % 4];
                self.pieces[round_order1[i + 3] / 4][round_order1[i + 3] % 4] = swap;
            }
            for i in 0..round_order2.len() - 1 {
                swap = self.pieces[round_order2[i] / 4][round_order2[i] % 4];
                self.pieces[round_order2[i] / 4][round_order2[i] % 4] =
                    self.pieces[round_order2[i + 1] / 4][round_order2[i + 1] % 4];
                self.pieces[round_order2[i + 1] / 4][round_order2[i + 1] % 4] = swap;
            }
        }
    }
}

pub struct Tetris {
    score: u32,
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
        for y in 0..self.current.pieces.len() {
            for x in 0..self.current.pieces[y].len() {
                if self.current.pieces[y][x] == TetrominoType::E {
                    continue;
                }
                self.map[(self.current.pos.0 + y as i8) as usize]
                    [(self.current.pos.1 + x as i8) as usize] = self.current.shape;
            }
        }
        self.current = Tetromino::new(self.bag.pop().unwrap_or(TetrominoType::E));

        /*if self.current.pieces.iter().enumerate().any(|(y, row)| {
            row.iter().enumerate().any(|(x, tile)| {
                if (tile == &TetrominoType::E) {
                    return false;
                }
                if (self.map[(self.current.pos.0 + y as i8) as usize]
                    [(self.current.pos.1 + x as i8) as usize]
                    != TetrominoType::E)
                {
                    return true;
                }
                false
            })
        }) {
            todo!("Game lost")
        }*/

        for y in 0..self.current.pieces.len() {
            for x in 0..self.current.pieces[y].len() {
                if self.current.pieces[y][x] == TetrominoType::E {
                    continue;
                }
                if self.map[(self.current.pos.0 + y as i8) as usize]
                    [(self.current.pos.1 + x as i8) as usize]
                    != TetrominoType::E
                {
                    self.is_lost = true;
                }
            }
        }

        if self.bag.len() == 0 {
            self.refill_bag();
        }
        self.check_lines();
        self.has_hold_this_round = false;
    }

    fn check_lines(&mut self) {
        'row: for y in (0..self.map.len()).rev() {
            for x in 0..self.map[y].len() {
                if self.map[y][x] == TetrominoType::E {
                    // If line has Empty space, it's not empty
                    continue 'row;
                }
            }

            self.delete_line(y);
            self.map[0] = [TetrominoType::E; MAP_WIDTH]; // Clear the top line as it won't be moved from the line -1
        }
    }

    fn delete_line(&mut self, line: usize) {
        for i in (0..line).rev() {
            // Move each line below
            self.map[i + 1] = self.map[i];
        }
        if self.map[line]
            .into_iter()
            .position(|x| x == TetrominoType::E)
            == None
        // If we just copied another full line, delete it again
        {
            self.delete_line(line);
        }
    }

    pub fn get_map(&self) -> Vec<Line> {
        let mut display_map: [[TetrominoType; MAP_WIDTH]; MAP_HEIGHT] =
            [[TetrominoType::E; MAP_WIDTH]; MAP_HEIGHT];
        display_map.copy_from_slice(&self.map[HIDDEN_ROWS..]);
        for y in 0..self.current.pieces.len() {
            for x in 0..self.current.pieces[y].len() {
                if self.current.pieces[y][x] != TetrominoType::E
                    && y as i8 + self.current.pos.0 >= HIDDEN_ROWS as i8
                {
                    display_map[(y as i8 + self.current.pos.0 - HIDDEN_ROWS as i8) as usize]
                        [(x as i8 + self.current.pos.1) as usize] =
                        self.current.pieces[y][x].clone();
                }
            }
        }

        display_map
            .iter()
            .map(|row| {
                Line::from(
                    row.iter()
                        .map(|col| Span::raw("  ").bg(col.get_color()))
                        .collect::<Vec<Span>>(),
                )
            })
            .collect()
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    pub fn rotate_counter_clockwise(&mut self) {
        self.current.rotate_counter_clockwise(self.map);
    }

    pub fn rotate_clockwise(&mut self) {
        self.current.rotate_clockwise(self.map);
    }

    fn can_move(&self, vector: [i8; 2]) -> bool {
        for (y, row) in self.current.pieces.as_slice().iter().enumerate() {
            for (x, piece) in row.iter().enumerate() {
                if *piece == TetrominoType::E {
                    continue;
                }
                let next_y: i8 = y as i8 + self.current.pos.0 + vector[0];
                let next_x: i8 = x as i8 + self.current.pos.1 + vector[1];

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
            self.current.pos.0 += vector[0];
            self.current.pos.1 += vector[1];
        }
    }

    pub fn is_lost(&self) -> bool {
        self.is_lost
    }

    pub fn hold(&mut self) {
        if self.has_hold_this_round {
            return;
        }
        let swap: TetrominoType = self.current.shape;
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
}
