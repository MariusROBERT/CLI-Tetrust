use ratatui::prelude::{Color, Line, Span};
use ratatui::style::Stylize;

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
    pub fn from_u8(value: u8) -> TetrominoType {
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
        //TODO try to center O and I tetromino by moving them 1 char to the right
        (match self {
            TetrominoType::E => {
                vec![vec![]]
            }
            TetrominoType::I => {
                vec![vec![], vec![TetrominoType::I; 4]]
            }
            TetrominoType::L => {
                vec![
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
                    vec![TetrominoType::E, TetrominoType::O, TetrominoType::O],
                    vec![TetrominoType::E, TetrominoType::O, TetrominoType::O],
                ]
            }
            TetrominoType::Z => {
                vec![
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
                    let mut before = vec![Span::raw(" ")];
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
