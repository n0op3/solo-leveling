use chrono::{Local, NaiveDate};
use crossterm::event;
use crossterm::event::KeyCode;
use ratatui::layout::Margin;
use ratatui::text::Line;
use ratatui::{
    DefaultTerminal, Frame,
    style::Stylize,
    symbols::border,
    widgets::{Block, Paragraph},
};
use std::collections::HashMap;
use std::io;

use crate::category::Category;
use crate::config::exercise::Exercise;
use crate::config::exercise::load_exercises;
use crate::config::user::{UserConfig, load_user_config};
use crate::config::workout::{Workout, load_workouts};
use crate::data;
use crate::data::user::{UserData, load_user_data};
use crate::ui::widget::pages::Page;
use crate::ui::widget::popup::Popup;
use crate::ui::widget::popup::bonus_xp_popup::BonusXPPopup;
use crate::ui::widget::popup::exercise_popup::ExercisePopup;
use crate::ui::widget::popup::popup_area;
use crate::ui::widget::popup::workout_popup::WorkoutPopup;
use crate::util::today;

pub struct App {
    exit: bool,
    pub username: String,
    pub user_config: UserConfig,
    pub user_data: UserData,
    pub exercises: HashMap<String, Exercise>,
    pub workouts: Vec<Workout>,
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
            workouts: load_workouts(),
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
        self.apply_penalty_for_daily_quests();
        self.user_data.last_login = today();

        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if let Some(key) = event::read()?.as_key_press_event() {
                self.handle_key(key);
            }
        }

        data::user::write_data(&self.user_data);
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
        self.page.render(self, frame, area);

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
            Page::Workouts(i) => match key.code {
                KeyCode::Enter => {
                    if let Some(workout) = self.workouts.get(*i as usize) {
                        self.popup = Some(Box::new(WorkoutPopup::new(workout)));
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
            KeyCode::Char('q') | KeyCode::Esc => self.exit = true,
            KeyCode::Tab => {
                self.page = match self.page {
                    Page::Overview => Page::DailyQuest,
                    Page::DailyQuest => Page::Workouts(0),
                    Page::Workouts(_) => Page::Exercises(0),
                    Page::Exercises(_) => Page::Overview,
                }
            }
            KeyCode::Char('1') => self.page = Page::Overview,
            KeyCode::Char('2') => self.page = Page::DailyQuest,
            KeyCode::Char('3') => match self.page {
                Page::Workouts(_) => {}
                _ => self.page = Page::Workouts(0),
            },
            KeyCode::Char('4') => match self.page {
                Page::Exercises(_) => {}
                _ => self.page = Page::Exercises(0),
            },
            _ => {}
        }
    }

    pub fn add_xp(&mut self, category: &String, xp: i32) {
        let category = match self.user_data.categories.get_mut(category) {
            None => {
                self.user_data
                    .categories
                    .insert(category.clone(), Category::default());
                self.user_data.categories.get_mut(category).unwrap()
            }
            Some(category) => category,
        };

        category.level.add_xp(xp);
        self.user_data.level.add_xp(xp);

        if let Some(daily) = &self.user_config.daily_quest {
            if daily.is_completed(&self.user_data.daily_quest_progress)
                && self
                    .user_data
                    .last_daily
                    .map(|last_daily| last_daily != today())
                    .unwrap_or(true)
            {
                self.user_data.last_daily = Some(today());
                self.add_general_xp(daily.xp_bonus(&self.exercises));
            }
        }

        data::user::write_data(&self.user_data);
    }

    pub fn add_general_xp(&mut self, xp: i32) {
        self.user_data.level.add_xp(xp);
    }

    fn apply_penalty_for_daily_quests(&mut self) {
        let last_login = NaiveDate::parse_from_str(
            self.user_data.last_login.date.unwrap().to_string().as_str(),
            "%Y-%m-%d",
        )
        .expect("Unable to parse the last login date");

        let days_passed = (Local::now()
            .date_naive()
            .signed_duration_since(last_login)
            .num_days()
            - 1)
        .clamp(0, 7);

        if let Some(daily_quest) = &self.user_config.daily_quest {
            for (exercise_name, amount) in daily_quest.exercises.iter() {
                let xp = match self.exercises.get(exercise_name) {
                    Some(exercise) => exercise.xp(*amount),
                    None => 0,
                } * days_passed as i32;

                if let Some(category) =
                    self.user_data
                        .categories
                        .get_mut(match &self.exercises.get(exercise_name) {
                            Some(exercise) => &exercise.category,
                            None => {
                                self.user_data.level.add_xp(-xp);
                                continue;
                            }
                        })
                {
                    category.level.add_xp(-xp / 2);
                }
            }
        } else {
            self.user_data.level.add_xp(-50 * days_passed as i32);
        }
    }
}
