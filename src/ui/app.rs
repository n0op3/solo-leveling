use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout, Margin};
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::Gauge;
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    symbols::border,
    widgets::{Block, Paragraph},
};
use std::io;

use crate::config::{UserConfig, load_user_config};
use crate::ui::widget::tabs::Tab;

#[derive(Debug)]
pub struct App {
    exit: bool,
    username: String,
    user_config: UserConfig,
    total_xp: usize,
    current_tab: Tab,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            exit: false,
            username: whoami::realname(),
            user_config: load_user_config(),
            total_xp: 0,
            current_tab: Tab::Dashboard,
        };
        app.update_stats();

        app
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
            .constraints([Constraint::Percentage(2), Constraint::Percentage(4)])
            .horizontal_margin(4)
            .split(area);

        frame.render_widget(Paragraph::new("").block(block), frame.area()); // Render the outline

        frame.render_widget(
            Paragraph::new(format!("Welcome, {}", self.username))
                .style(Style::default().white())
                .centered(),
            chunks[0],
        );

        match self.current_tab {
            Tab::Dashboard => frame.render_widget(
                Gauge::default()
                    .block(Block::bordered().title("LEVEL"))
                    .gauge_style(Style::new().cyan().on_black())
                    .label(format!("{}/100 XP", self.total_xp))
                    .percent(self.total_xp as u16),
                chunks[1],
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

    fn update_stats(&mut self) {
        self.total_xp = 0;
        for category in self.user_config.categories.iter() {
            self.total_xp += category.xp;
        }
    }
}
