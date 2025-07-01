use ratatui::style::{Color, Stylize};
use std::cmp::PartialEq;

use rand::seq::SliceRandom;
use ratatui::prelude::{Line, Span};

pub enum Rotation {
    Up,
    Right,
    Down,
    Left,
}

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
            TetrominoType::O => Color::LightGreen,
            TetrominoType::Z => Color::Red,
            TetrominoType::S => Color::Green,
            TetrominoType::T => Color::Magenta,
            TetrominoType::E => Color::White,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tetromino {
    shape: TetrominoType,
    pos: (u8, u8),
    pieces: Vec<Vec<TetrominoType>>,
}

pub trait TetrominoTrait {
    fn rotate_clockwise(&mut self);
    fn rotate_counter_clockwise(&mut self);
}

// impl TetrominoTrait for Tetromino {}

impl Tetromino {
    pub fn new(shape: TetrominoType) -> Self {
        if (shape == TetrominoType::E) {
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
            pos: (0, 3),
            pieces,
        }
    }

    pub fn get_color(&self) -> Color {
        TetrominoType::get_color(&self.shape)
    }
}

pub struct Tetris {
    score: u32,
    hold: TetrominoType,
    bag: Vec<TetrominoType>,
    map: [[TetrominoType; 10]; 22],
    current: Tetromino,
    current_rotation: u8,
}

impl Tetris {
    fn refill_bag(&mut self) {
        let mut bag: Vec<TetrominoType> = (1..8).map(TetrominoType::from_u8).collect();
        bag.shuffle(&mut rand::rng());
        self.bag = bag;
    }

    pub fn new() -> Self {
        let mut bag: Vec<TetrominoType> = (1..8).map(TetrominoType::from_u8).collect();
        bag.shuffle(&mut rand::rng());
        let current = Tetromino::new(bag.pop().unwrap_or(TetrominoType::E));
        Self {
            score: 0,
            hold: TetrominoType::E,
            bag,
            map: [[TetrominoType::E; 10]; 22],
            // map: vec![vec![TetrominoType::E; 10]; 22],
            current,
            current_rotation: 0,
        }
    }

    pub fn debug(&self) {
        println!("Score: {}", self.score);
        println!("Bag: {:?}", &self.bag);
        self.map.iter().for_each(|row| {
            println!("{:?}", row);
        });
    }

    pub fn get_case(&self, x: u8, y: u8) -> &TetrominoType {
        &self.map[y as usize][x as usize]
    }

    pub fn get_map(&self) -> Vec<Line> {
        let mut display_map: [[TetrominoType; 10]; 20] = [[TetrominoType::E; 10]; 20];
        display_map.copy_from_slice(&self.map[2..]);
        for i in 0..self.current.pieces.len() {
            for j in 0..self.current.pieces[i].len() {
                if self.current.pieces[i][j] != TetrominoType::E {
                    display_map[i + self.current.pos.0 as usize][j + self.current.pos.1 as usize] =
                        self.current.pieces[i][j].clone();
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

    pub fn get_current(&self) -> &Tetromino {
        &self.current
    }
}
