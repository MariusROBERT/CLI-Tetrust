use crate::tetromino_type::TetrominoType;

#[derive(Clone, Debug)]
pub struct Tetromino {
    shape: TetrominoType,
    rotation: u8,
    pos: (i8, i8),
    pieces: Vec<Vec<TetrominoType>>,
}

pub trait TetrominoTrait {
    fn rotate_clockwise(
        &mut self,
        map: [[TetrominoType; crate::tetris::MAP_WIDTH]; crate::tetris::TRUE_MAP_HEIGHT],
    );
    fn rotate_counter_clockwise(
        &mut self,
        map: [[TetrominoType; crate::tetris::MAP_WIDTH]; crate::tetris::TRUE_MAP_HEIGHT],
    );
}

impl TetrominoTrait for Tetromino {
    fn rotate_clockwise(
        &mut self,
        map: [[TetrominoType; crate::tetris::MAP_WIDTH]; crate::tetris::TRUE_MAP_HEIGHT],
    ) {
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

    fn rotate_counter_clockwise(
        &mut self,
        map: [[TetrominoType; crate::tetris::MAP_WIDTH]; crate::tetris::TRUE_MAP_HEIGHT],
    ) {
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
        map: [[TetrominoType; crate::tetris::MAP_WIDTH]; crate::tetris::TRUE_MAP_HEIGHT],
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

            if next_y >= crate::tetris::TRUE_MAP_HEIGHT as i8 {
                return 0;
            }
            if next_x < 0 || next_x >= crate::tetris::MAP_WIDTH as i8 {
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
        map: [[TetrominoType; crate::tetris::MAP_WIDTH]; crate::tetris::TRUE_MAP_HEIGHT],
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
        map: [[TetrominoType; crate::tetris::MAP_WIDTH]; crate::tetris::TRUE_MAP_HEIGHT],
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

            if next_y >= crate::tetris::TRUE_MAP_HEIGHT as i8 {
                return 0;
            }
            if next_x < 0 || next_x >= crate::tetris::MAP_WIDTH as i8 {
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

            if next_y >= crate::tetris::TRUE_MAP_HEIGHT as i8 {
                return 0;
            }
            if next_x < 0 || next_x >= crate::tetris::MAP_WIDTH as i8 {
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
        map: [[TetrominoType; crate::tetris::MAP_WIDTH]; crate::tetris::TRUE_MAP_HEIGHT],
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

    pub fn pos(&self) -> (i8, i8) {
        self.pos
    }

    pub fn pieces(&self) -> &Vec<Vec<TetrominoType>> {
        &self.pieces
    }

    pub fn shape(&self) -> TetrominoType {
        self.shape
    }

    pub fn r#move(&mut self, vector: [i8; 2]) {
        self.pos.0 += vector[0];
        self.pos.1 += vector[1];
    }
}
