use crate::display::utils::center::center;
use crate::tetris::Tetris;
use ratatui::layout::{Alignment, Constraint, Flex, Layout, Rect};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::{Frame, border, symbols};

pub fn draw(frame: &mut Frame, game: &mut Tetris) {
    let chunks = Layout::horizontal([
        Constraint::Fill(1),
        Constraint::Length((10 * 2) + 2),
        Constraint::Fill(1),
    ])
    .split(frame.area());
    draw_left(frame, game, chunks[0]);
    draw_game(frame, game, chunks[1]);
    draw_right(frame, game, chunks[2]);
}

fn draw_game(frame: &mut Frame, game: &mut Tetris, area: Rect) {
    let block = Block::bordered()
        .title_alignment(Alignment::Center)
        .border_set(symbols::border::Set {
            top_left: symbols::line::NORMAL.horizontal_down,
            top_right: symbols::line::NORMAL.horizontal_down,
            bottom_left: symbols::line::NORMAL.horizontal_up,
            bottom_right: symbols::line::NORMAL.horizontal_up,
            ..symbols::border::PLAIN
        })
        .title("Tetrust");

    let game_display = Text::from(game.get_map());
    let layout = center(area, Constraint::Length(20), Constraint::Length(20));
    frame.render_widget(game_display, layout);
    frame.render_widget(block, area);
}

fn draw_left(frame: &mut Frame, game: &mut Tetris, area: Rect) {
    let horizontal_centered_layout = center(area, Constraint::Fill(1), Constraint::Fill(1));
    let vertical_chunks = Layout::vertical([Constraint::Fill(1), Constraint::Fill(1)])
        .flex(Flex::Center)
        .margin(1)
        .split(horizontal_centered_layout);

    let block = Block::bordered()
        .title_alignment(Alignment::Center)
        .borders(border!(TOP, BOTTOM, LEFT))
        .border_type(BorderType::Rounded)
        .title("Score");
    frame.render_widget(block, horizontal_centered_layout);

    frame.render_widget(
        Block::bordered()
            .title_alignment(Alignment::Center)
            .border_type(BorderType::Rounded)
            .title(" Hold "),
        center(
            vertical_chunks[0],
            Constraint::Length(12),
            Constraint::Length(6),
        ),
    );

    frame.render_widget(
        Paragraph::new(format!("Score: {}", game.get_score())).centered(), // .block(Block::bordered().border_type(BorderType::Rounded) )
        center(vertical_chunks[1], Constraint::Fill(1), Constraint::Fill(1)),
    );
}

fn draw_right(frame: &mut Frame, game: &mut Tetris, area: Rect) {
    let horizontal_layout = Layout::horizontal([Constraint::Fill(1)])
        .flex(Flex::Center)
        .split(area)[0];
    let vertical_layout = Layout::vertical([Constraint::Fill(1)])
        // .flex(Flex::Center)
        .split(horizontal_layout)[0];
    let block = Block::bordered()
        .title_alignment(Alignment::Center)
        .borders(border!(TOP, BOTTOM, RIGHT))
        .title("Next");
    frame.render_widget(block, vertical_layout);
}
