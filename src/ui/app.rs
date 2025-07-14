use crossterm::event;
use crossterm::event::KeyCode;
use ratatui::layout::{Constraint, Direction, Layout, Margin};
use ratatui::style::{Modifier, Style};
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    symbols::border,
    widgets::{Block, Paragraph},
};
use ratatui::{text::Line, widgets::Gauge};
use std::collections::HashMap;
use std::io;

use crate::category::Category;
use crate::config::exercise::Exercise;
use crate::config::exercise::load_exercises;
use crate::config::user::{UserConfig, load_user_config};
use crate::data;
use crate::data::user::{UserData, load_user_data};
use crate::ui::widget::pages::Page;
use crate::ui::widget::popup::Popup;
use crate::ui::widget::popup::bonus_xp_popup::BonusXPPopup;
use crate::ui::widget::popup::exercise_popup::ExercisePopup;
use crate::ui::widget::popup::popup_area;

pub struct App {
    exit: bool,
    username: String,
    user_config: UserConfig,
    user_data: UserData,
    exercises: HashMap<String, Exercise>,
    popup: Option<Box<dyn Popup>>,
    page: Page,
}

impl Default for App {
    fn default() -> Self {
        let mut app = Self {
            exit: false,
            username: whoami::realname(),
            user_config: load_user_config(),
            user_data: load_user_data(),
            exercises: load_exercises(),
            popup: None,
            page: Page::Overview,
        };

        let custom_username = app.user_config.name.clone();
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
        let mut instructions = vec![
            " Next page ".into(),
            "<Tab> ".blue().bold(),
            " Quit ".into(),
            "<q> ".blue().bold(),
        ];

        instructions.append(&mut match self.page {
            Page::Exercises(_) => {
                vec![" Select ".into(), "<Enter> ".blue().bold()]
            }
            Page::Overview => {
                vec![" Add Bonus XP ".into(), "<b> ".blue().bold()]
            }
            _ => Vec::new(),
        });

        let instructions = Line::from(instructions);

        let outline = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::ROUNDED);

        let area = frame.area().inner(Margin::new(1, 2));
        frame.render_widget(
            Paragraph::new(self.page.name()).bold().block(outline),
            frame.area(),
        );

        let area = area.inner(Margin::new(4, 0));
        match self.page {
            Page::Overview => {
                let mut constraints = vec![Constraint::Max(1), Constraint::Length(5)];

                for _category in self.user_data.categories.iter() {
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
                        .block(
                            Block::bordered()
                                .title(format!("LEVEL {}", self.user_data.level.level())),
                        )
                        .gauge_style(Style::new().red())
                        .label(self.user_data.level.xp_text())
                        .percent((self.user_data.level.percentage() * 100.0) as u16),
                    chunks[1],
                );

                for (i, (category_name, category)) in self.user_data.categories.iter().enumerate() {
                    frame.render_widget(
                        Gauge::default()
                            .block(Block::bordered().title(format!(
                                "{} LVL {}",
                                category_name.to_uppercase(),
                                category.level.level()
                            )))
                            .gauge_style(Style::new().cyan())
                            .label(format!(
                                "{}/{} XP",
                                category.level.xp(),
                                category.level.levelup_requirement()
                            ))
                            .percent((category.level.percentage() * 100.0) as u16),
                        chunks[i + 2],
                    );
                }
            }
            Page::Exercises(index) => {
                let mut lines = Vec::new();

                for (i, (exercise_name, exercise)) in self.exercises.iter().enumerate() {
                    let style = Style::new().fg(exercise.color());

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

                let paragraph = Paragraph::new(lines)
                    .block(Block::new())
                    .scroll(((index - area.height as i32 / 2).max(0) as u16, 0));

                frame.render_widget(paragraph, area);
            }
            _ => {}
        }

        if let Some(popup) = &self.popup {
            let area = popup_area(area, 60, 40);
            popup.render(area, frame.buffer_mut());
        }
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        if let Some(mut popup) = self.popup.take() {
            if !popup.handle_key(self, key.code) {
                self.popup = Some(popup);
            }
            return;
        }

        match &mut self.page {
            Page::Exercises(i) => match key.code {
                KeyCode::Enter => {
                    if let Some((name, exercise)) = self.exercises.iter().nth(*i as usize) {
                        self.popup =
                            Some(Box::new(ExercisePopup::new(name.to_uppercase(), exercise)))
                    }
                }
                KeyCode::Down | KeyCode::Char('j') => {
                    if *i == self.exercises.len() as i32 {
                        *i = 0;
                    } else {
                        *i += 1;
                    }
                }
                KeyCode::Up | KeyCode::Char('k') => {
                    if *i == -1 {
                        *i = self.exercises.len() as i32 - 1;
                    } else {
                        *i -= 1;
                    }
                }
                _ => {}
            },
            Page::Overview => match key.code {
                KeyCode::Esc => self.popup = None,
                KeyCode::Char('b') => {
                    self.popup = Some(Box::new(BonusXPPopup::default()));
                }
                _ => {}
            },
            _ => {}
        }

        match key.code {
            KeyCode::Char('q') => self.exit = true,
            KeyCode::Tab => {
                self.page = match self.page {
                    Page::Overview => Page::Workouts,
                    Page::Workouts => Page::Exercises(0),
                    Page::Exercises(_) => Page::Overview,
                }
            }
            _ => {}
        }
    }

    pub fn add_xp(&mut self, category: String, xp: i32) {
        let category = match self.user_data.categories.get_mut(&category) {
            None => {
                self.user_data
                    .categories
                    .insert(category.clone(), Category::default());
                self.user_data.categories.get_mut(&category).unwrap()
            }
            Some(category) => category,
        };

        category.level.add_xp(xp);
        self.user_data.level.add_xp(xp);
        data::user::write_data(&self.user_data);
    }
}
