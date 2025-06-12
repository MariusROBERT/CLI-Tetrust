use std::io;

use crate::tetris::Tetris;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Default)]
pub struct Window {
    exit: bool,
    text: String,
}
impl Window {
    pub fn start(&mut self, terminal: &mut DefaultTerminal, game: &mut Tetris) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        ratatui::restore();
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            _ => self.text += &key_event.code.to_string(),
        }
    }
}

impl Widget for &Window {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" App ".bold());
        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::ROUNDED);

        Paragraph::new(Text::from(self.text.clone()))
            .centered()
            .block(block)
            .render(area, buf);
    }
}
