use crossterm::event::KeyCode;
use ratatui::{layout::Constraint, prelude::Margin};
use ratatui::{layout::Direction, widgets::Paragraph};
use ratatui::{layout::Rect, widgets::Widget};
use ratatui::{prelude::Layout, style::Stylize};

use crate::config::exercise::Exercise;
use crate::ui::widget::popup::Popup;

pub struct ExercisePopup {
    title: String,
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
}

impl Popup for ExercisePopup {
    fn title(&self) -> &String {
        &self.title
    }

    fn render_content(&self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
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

    fn handle_key(&mut self, app: &mut crate::ui::app::App, key: KeyCode) -> bool {
        match key {
            KeyCode::Esc | KeyCode::Char('q') => return true,
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
                if !(key == KeyCode::Char('0') && self.input.is_empty()) && self.input.len() < 3 {
                    self.input.push(key.as_char().unwrap());
                }
            }
            KeyCode::Backspace => {
                if !self.input.is_empty() {
                    self.input.remove(self.input.len() - 1);
                }
            }
            KeyCode::Enter => {
                if let Ok(amount) = self.input.parse::<i32>() {
                    app.add_xp(
                        self.exercise.category.clone(),
                        self.exercise.xp(amount) as i32,
                    );
                    return true;
                }
            }
            _ => {}
        }

        return false;
    }
}
