#[derive(Clone, Debug)]
pub enum Tetromino {
    E = 0, //Empty
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
            _ => Tetromino::E,
        }
    }
}

pub struct Tetris {
    score: u32,
    hold: Tetromino,
    bag: Vec<Tetromino>,
    map: Vec<Vec<Tetromino>>,
}

impl Tetris {
    fn refill_bag(&mut self) {
        self.bag = (1..8).map(Tetromino::from_u8).collect();
    }
    pub fn new() -> Self {
        Self {
            score: 0,
            hold: Tetromino::E,
            bag: (1..8).map(Tetromino::from_u8).collect(),
            map: vec![vec![Tetromino::E; 10]; 40],
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
