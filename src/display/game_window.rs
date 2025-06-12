use std::io;

use crate::tetris::Tetris;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::style::{Color, Style};
use ratatui::text::Text;
use ratatui::widgets::Paragraph;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    layout::{Constraint, Layout},
    prelude::Direction,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Widget},
};

pub struct GameWindow {
    game: Tetris,
}

impl GameWindow {
    pub fn new(game: Tetris) -> Self {
        Self { game }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Fill(1)])
            .split(area)[0];

        self.render_column(frame, chunks);
    }

    fn render_column(&self, frame: &mut Frame, area: Rect) {
        let constraints = vec![Constraint::Length(10); 10];
        let rows = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .split(area);

        /*for j in 0..40 {
            let mut line = String::new();
            for i in 0..10 {
                line += &*self.game.get_case(i, j).to_string();
            }
            frame.render_widget(Paragraph::new(self.game.get_map()[i]));
        }*/
        // for (i, text) in self.game.iter().enumerate() {
        let para = Paragraph::new("test".clone());
        frame.render_widget(para, rows[0]);
        // }
    }
}
