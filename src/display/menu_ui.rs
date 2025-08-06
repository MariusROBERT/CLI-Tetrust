use crate::display::utils::center::center;
use crate::menu;
use crate::menu::Menu;
use ratatui::layout::{Alignment, Constraint, Flex, Layout};
use ratatui::style::{Color, Stylize};
use ratatui::text::Text;
use ratatui::widgets::{Block, BorderType, Borders};
use ratatui::{Frame, border};

pub fn draw(frame: &mut Frame, menu: &Menu) {
    let area = Layout::horizontal([Constraint::Fill(1)]).split(frame.area())[0];

    let horizontal_centered_layout = center(area, Constraint::Fill(1), Constraint::Fill(1));

    let vertical_chunks = Layout::vertical([Constraint::Min(1); menu::OPTION_LEN])
        .flex(Flex::Center)
        .margin(1)
        .split(horizontal_centered_layout);

    let block = Block::bordered()
        .title_alignment(Alignment::Center)
        .borders(border!(ALL))
        .border_type(BorderType::Rounded)
        .title("Tetrust");
    frame.render_widget(block, horizontal_centered_layout);

    for (id, option) in menu.get_options().iter().enumerate() {
        let mut text: Text = Text::from(option.as_str());
        if menu.get_hovered() == *option {
            text = text.bg(Color::Gray).fg(Color::White);
        }
        frame.render_widget(text.centered(), vertical_chunks[id]);
    }
}
