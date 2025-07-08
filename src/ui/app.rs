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
use crate::level::levelup_requirement;
use crate::ui::widget::tabs::Page;

#[derive(Debug)]
pub struct App {
    exit: bool,
    username: String,
    user_config: UserConfig,
    total_xp: i32,
    current_tab: Page,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            exit: false,
            username: whoami::realname(),
            user_config: load_user_config(),
            total_xp: 0,
            current_tab: Page::Dashboard,
        };
        app.update_stats();

        let custom_username = app.user_config.user.name.clone();
        if let Some(custom_username) = custom_username {
            app.username = custom_username;
        }

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

        let mut constraints = vec![Constraint::Max(1), Constraint::Length(5)];

        {
            for _category in self.user_config.categories.iter() {
                constraints.push(Constraint::Max(3));
            }
        }

        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints)
            .horizontal_margin(4)
            .split(area);

        frame.render_widget(
            Paragraph::new(self.current_tab.name()).bold().block(block),
            frame.area(),
        );

        frame.render_widget(
            Paragraph::new(format!("Welcome, {}", self.username))
                .style(Style::default().white())
                .centered(),
            chunks[0],
        );

        match self.current_tab {
            Page::Dashboard => {
                frame.render_widget(
                    Gauge::default()
                        .block(Block::bordered().title("LEVEL"))
                        .gauge_style(Style::new().red())
                        .label(format!(
                            "{}/{} XP",
                            self.total_xp,
                            levelup_requirement(self.user_config.user.level as usize)
                        ))
                        .percent(self.total_xp as u16),
                    chunks[1],
                );
                let mut i = 2;
                for (category, xp) in self.user_config.categories.iter() {
                    frame.render_widget(
                        Gauge::default()
                            .block(Block::bordered().title(category.to_uppercase()))
                            .gauge_style(Style::new().cyan().on_black())
                            .label(format!("{xp}/100 XP"))
                            .percent(*xp as u16),
                        chunks[i],
                    );

                    i += 1;
                }
            }
            _ => {}
        }
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Tab => {
                self.current_tab = match self.current_tab {
                    Page::Dashboard => Page::Workouts,
                    Page::Workouts => Page::Exercises,
                    Page::Exercises => Page::Dashboard,
                }
            }
            _ => {}
        }
    }

    fn update_stats(&mut self) {
        self.total_xp = 0;
        for (_category, xp) in self.user_config.categories.iter() {
            self.total_xp += xp;
        }
    }
}
