use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{layout::Constraint, prelude::Margin};
use ratatui::{layout::Direction, widgets::Paragraph};
use ratatui::{
    layout::Rect,
    widgets::{Block, Clear, Widget},
};
use ratatui::{prelude::Layout, style::Stylize};

use crate::config::exercise::Exercise;

pub struct ExercisePopup {
    pub title: String,
    input: String,
    pub exercise: Exercise,
}

impl ExercisePopup {
    pub fn new(title: String, exercise: &Exercise) -> Self {
        Self {
            title,
            input: String::new(),
            exercise: exercise.clone(),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> PopupResult {
        match &key.code {
            KeyCode::Char('q') | KeyCode::Esc => return PopupResult::Exit,
            KeyCode::Char('0')
            | KeyCode::Char('1')
            | KeyCode::Char('2')
            | KeyCode::Char('3')
            | KeyCode::Char('4')
            | KeyCode::Char('5')
            | KeyCode::Char('6')
            | KeyCode::Char('7')
            | KeyCode::Char('8')
            | KeyCode::Char('9') => {
                if self.input.len() < 3 {
                    self.input.push(key.code.as_char().unwrap());
                }
            }
            KeyCode::Backspace => {
                if !self.input.is_empty() {
                    self.input.remove(self.input.len() - 1);
                }
            }
            KeyCode::Enter => {
                if let Ok(amount) = self.input.parse::<i32>() {
                    return PopupResult::AddXP {
                        category: self.exercise.category.clone(),
                        xp: self.exercise.xp(amount) as i32,
                    };
                }
            }
            _ => {}
        }

        PopupResult::None
    }
}

impl Widget for &ExercisePopup {
    fn render(self, area: Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let block = &Block::bordered().title(self.title.clone()).cyan();

        Clear::default().render(area, buf);
        block.render(area, buf);

        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ],
        )
        .split(area.inner(Margin::new(1, 1)));

        Paragraph::new(format!(
            "{}: {}",
            if self.exercise.time.is_some() {
                "Seconds"
            } else {
                "Reps"
            },
            if self.input.is_empty() {
                String::from("0")
            } else {
                self.input.clone()
            }
        ))
        .fg(self.exercise.color())
        .centered()
        .render(layout[0], buf);

        if self.input.parse::<i32>().is_err() {
            Paragraph::new("Please provide input")
                .red()
                .bold()
                .centered()
                .render(layout[1], buf);
        }

        Paragraph::new("Press Enter to confirm, q/Esc to cancel")
            .dark_gray()
            .centered()
            .render(layout[2], buf);
    }
}

pub enum PopupResult {
    None,
    Exit,
    AddXP { category: String, xp: i32 },
}
