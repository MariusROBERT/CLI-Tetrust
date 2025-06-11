#[allow(dead_code)]

#[derive(Clone)]
enum Tetromino {
    EMPTY = 0,
    I = 1,
    L = 2,
    J = 3,
    O = 4,
    Z = 5,
    S = 6,
    T = 7,
}

impl Tetromino {
    fn from_u8(value: u8) -> Tetromino {
        match value {
            1 => Tetromino::I,
            2 => Tetromino::L,
            3 => Tetromino::J,
            4 => Tetromino::O,
            5 => Tetromino::Z,
            6 => Tetromino::S,
            7 => Tetromino::T,
            _ => Tetromino::EMPTY,
        }
    }
}


pub struct Tetris {
    score: u32,
    hold: u8,
    bag: Vec<Tetromino>,
    map: Vec<Vec<Tetromino>>,
}

impl Tetris {
    fn refill_bag(&mut self) {
        self.bag = vec![Tetromino::I, Tetromino::L, Tetromino::J, Tetromino::O, Tetromino::Z, Tetromino::S, Tetromino::T];
    }
    pub fn new() -> Self {
        let mut tmp =        Self {
            score: 0,
            hold: 0,
            bag: Vec::new(),
            map: vec![vec![Tetromino::EMPTY; 10]; 40]
        };
        tmp.refill_bag();
        return tmp
    }

    pub fn debug(&self) {
        println!("Score: {}", self.score);
        self.map.iter().for_each(|_| {});
    }
}