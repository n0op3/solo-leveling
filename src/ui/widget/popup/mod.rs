pub mod exercise_popup;

use crossterm::event::KeyCode;
use ratatui::{
    buffer::Buffer,
    layout::{Margin, Rect},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, Widget},
};

use crate::ui::app::App;

pub trait Popup {
    fn render_content(&self, area: Rect, buf: &mut Buffer);
    fn title(&self) -> &String;
    fn handle_key(&mut self, app: &mut App, key: KeyCode) -> bool;

    fn render(&self, area: Rect, buf: &mut Buffer) {
        let popup_area = area.inner(Margin::new(1, 1));

        let block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::White))
            .title(self.title().clone());

        Clear.render(popup_area, buf);
        block.clone().render(popup_area, buf);

        let inner_area = block.inner(popup_area);
        self.render_content(inner_area, buf);
    }
}
