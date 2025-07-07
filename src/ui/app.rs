use std::collections::HashMap;
use std::io;

use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout, Margin};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    symbols::border,
    widgets::{Block, Gauge, Paragraph},
};

use crate::config::load_exercises;
use crate::exercise::ExerciseTemplate;
use crate::ui::widget::tabs::Tab;

#[derive(Debug)]
pub struct App {
    exercises: HashMap<String, ExerciseTemplate>,
    exit: bool,
    username: String,
    current_tab: Tab,
}

impl Default for App {
    fn default() -> Self {
        Self {
            exercises: load_exercises(),
            exit: false,
            username: whoami::realname(),
            current_tab: Tab::Dashboard,
        }
    }
}

impl App {
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

        let area = frame.area().inner(Margin::new(1, 1));

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(2),
                Constraint::Percentage(2),
                Constraint::Percentage(4),
            ])
            .horizontal_margin(4)
            .split(area);

        frame.render_widget(Paragraph::new("").block(block), frame.area()); // Render the outline

        frame.render_widget(
            Paragraph::new(format!("Welcome, {}", self.username))
                .style(Style::default().white())
                .centered(),
            chunks[0],
        );

        frame.render_widget(
            Paragraph::new("LEVEL 412")
                .style(Style::default().white())
                .centered(),
            chunks[1],
        );

        match self.current_tab {
            Tab::Dashboard => frame.render_widget(
                Gauge::default()
                    .block(Block::new())
                    .gauge_style(Style::new().cyan().on_black())
                    .label("69/420 XP")
                    .percent(40),
                chunks[2],
            ),
            Tab::Workouts => {}
        }
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Tab => {
                self.current_tab = match self.current_tab {
                    Tab::Dashboard => Tab::Workouts,
                    Tab::Workouts => Tab::Dashboard,
                }
            }
            _ => {}
        }
    }
}
