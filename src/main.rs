use std::io;

use crossterm::event::{self, KeyCode};
use ratatui::text::Line;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};

mod widget;

use crate::widget::tabs::TabsState;

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}

#[derive(Debug)]
pub struct App<'a> {
    exit: bool,
    tabs: TabsState<'a>,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            exit: false,
            tabs: TabsState::new(vec!["Overview", "Workouts"]),
        }
    }
}

impl App<'_> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                self.handle_key(key);
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Tab => self.tabs.next(),
            _ => {}
        }
    }
}

impl Widget for &App<'_> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" The System ".bold());
        let instructions = Line::from(vec![
            " Quit ".into(),
            "<Q> ".blue().bold(),
            " Next page ".into(),
            "<Tab> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        self.tabs.render(area, buf);

        Paragraph::new("Welcome, Player")
            .centered()
            .block(block)
            .render(area, buf);
    }
}
