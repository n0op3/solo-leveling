use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Direction, Layout, Margin};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::Line;
use ratatui::widgets::Gauge;
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    symbols::border,
    widgets::{Block, Paragraph},
};
use std::collections::HashMap;
use std::io;

use crate::config::{UserConfig, load_exercises, load_user_config};
use crate::exercise::{Difficulty, Exercise};
use crate::level::levelup_requirement;
use crate::ui::widget::tabs::Page;

#[derive(Debug)]
pub struct App {
    exit: bool,
    username: String,
    user_config: UserConfig,
    exercises: HashMap<String, Exercise>,
    total_xp: i32,
    page: Page,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            exit: false,
            username: whoami::realname(),
            user_config: load_user_config(),
            exercises: load_exercises(),
            total_xp: 0,
            page: Page::Dashboard,
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

        let area = frame.area().inner(Margin::new(1, 2));
        frame.render_widget(
            Paragraph::new(self.page.name()).bold().block(block),
            frame.area(),
        );

        let area = area.inner(Margin::new(4, 0));
        match self.page {
            Page::Dashboard => {
                let mut constraints = vec![Constraint::Max(1), Constraint::Length(5)];

                for _category in self.user_config.categories.iter() {
                    constraints.push(Constraint::Max(3));
                }

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(constraints)
                    .split(area);

                frame.render_widget(
                    Paragraph::new(format!("Welcome, {}", self.username))
                        .style(Style::default().white())
                        .centered(),
                    chunks[0],
                );
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

                for (i, (category, xp)) in self.user_config.categories.iter().enumerate() {
                    frame.render_widget(
                        Gauge::default()
                            .block(Block::bordered().title(category.to_uppercase()))
                            .gauge_style(Style::new().cyan())
                            .label(format!("{xp}/100 XP"))
                            .percent(*xp as u16),
                        chunks[i + 2],
                    );
                }
            }
            Page::Exercises(index) => {
                let mut lines = Vec::new();

                for (i, (exercise_name, exercise)) in self.exercises.iter().enumerate() {
                    let style = Style::new().fg(match exercise.difficulty() {
                        Difficulty::Easy => Color::Green,
                        Difficulty::Normal => Color::Yellow,
                        Difficulty::Difficult => Color::Red,
                        Difficulty::Extreme => Color::Magenta,
                    });

                    let line = Line::styled(
                        format!(
                            "{} {}: {}",
                            if index == i as i32 { ">>" } else { "  " },
                            exercise_name.to_uppercase(),
                            exercise.xp_text()
                        ),
                        if index == i as i32 {
                            style.add_modifier(Modifier::BOLD)
                        } else {
                            style
                        },
                    );

                    lines.push(line);
                }

                let paragraph = Paragraph::new(lines).block(Block::new());

                frame.render_widget(paragraph, area);
            }
            _ => {}
        }
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Tab => {
                self.page = match self.page {
                    Page::Dashboard => Page::Workouts,
                    Page::Workouts => Page::Exercises(0),
                    Page::Exercises(_) => Page::Dashboard,
                }
            }
            KeyCode::Down | KeyCode::Char('j') => match &mut self.page {
                Page::Exercises(i) => {
                    if *i == self.exercises.len() as i32 {
                        *i = 0;
                    } else {
                        *i += 1;
                    }
                }
                _ => {}
            },
            KeyCode::Up | KeyCode::Char('k') => match &mut self.page {
                Page::Exercises(i) => {
                    if *i == -1 {
                        *i = self.exercises.len() as i32 - 1;
                    } else {
                        *i -= 1;
                    }
                }
                _ => {}
            },
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
