use ratatui::style::Color;

pub enum Rotation {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone, Debug)]
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

#[derive(Clone, Debug)]
pub struct Tetromino {
    shape: TetrominoType,
}

pub trait TetrominoTrait {
    fn rotate_clockwise(&mut self);
    fn rotate_counter_clockwise(&mut self);
}

// impl TetrominoTrait for Tetromino {}

impl Tetromino {
    pub fn new(shape: TetrominoType) -> Self {
        Self { shape }
    }

    fn from_u8(value: u8) -> Tetromino {
        match value {
            1 => Self {
                shape: TetrominoType::I,
            },
            2 => Self {
                shape: TetrominoType::L,
            },
            3 => Self {
                shape: TetrominoType::J,
            },
            4 => Self {
                shape: TetrominoType::O,
            },
            5 => Self {
                shape: TetrominoType::Z,
            },
            6 => Self {
                shape: TetrominoType::S,
            },
            7 => Self {
                shape: TetrominoType::T,
            },
            _ => Self {
                shape: TetrominoType::E,
            },
        }
    }

    pub fn get_color(&self) -> Color {
        match self.shape {
            TetrominoType::I => Color::Cyan,
            TetrominoType::L => Color::Yellow,
            TetrominoType::J => Color::Blue,
            TetrominoType::O => Color::LightGreen,
            TetrominoType::Z => Color::Red,
            TetrominoType::S => Color::Green,
            TetrominoType::T => Color::Magenta,
            TetrominoType::E => Color::Cyan,
        }
    }
}

pub struct Tetris {
    score: u32,
    hold: TetrominoType,
    bag: Vec<Tetromino>,
    map: Vec<Vec<Tetromino>>,
    current_rotation: u8,
}

impl Tetris {
    fn refill_bag(&mut self) {
        self.bag = (1..8).map(Tetromino::from_u8).collect();
    }

    pub fn new() -> Self {
        Self {
            score: 0,
            hold: TetrominoType::E,
            bag: (1..8).map(Tetromino::from_u8).collect(),
            map: vec![vec![Tetromino::new(TetrominoType::E); 10]; 40],
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

    pub fn get_case(&self, x: u8, y: u8) -> &Tetromino {
        &self.map[y as usize][x as usize]
    }

    pub fn get_map(&self) -> &Vec<Vec<Tetromino>> {
        &self.map
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }
}
