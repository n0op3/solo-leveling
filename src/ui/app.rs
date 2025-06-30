use std::collections::HashMap;
use std::io;

use crate::ui::widget::tabs::TabsState;
use crossterm::event::{self, KeyCode};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};

use crate::config::load_exercises;
use crate::exercise::ExerciseTemplate;

#[derive(Debug)]
pub struct App<'a> {
    exercises: HashMap<String, ExerciseTemplate>,
    exit: bool,
    tabs: TabsState<'a>,
}

impl Default for App<'_> {
    fn default() -> Self {
        Self {
            exercises: load_exercises(),
            exit: false,
            tabs: TabsState::new(vec!["Overview", "Workouts"]),
        }
    }
}

impl App<'_> {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        println!("{:?}", self.exercises);
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

        Paragraph::new("Welcome, Player")
            .style(Style::default().white())
            .centered()
            .block(block)
            .render(area, buf);

        self.tabs.render(area, buf);
    }
}
