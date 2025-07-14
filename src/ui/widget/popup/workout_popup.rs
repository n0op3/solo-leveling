use crossterm::event::KeyCode;
use ratatui::{layout::Constraint, prelude::Margin};
use ratatui::{layout::Direction, widgets::Paragraph};
use ratatui::{layout::Rect, widgets::Widget};
use ratatui::{prelude::Layout, style::Stylize};

use crate::config::workout::Workout;
use crate::ui::widget::popup::Popup;

pub struct WorkoutPopup {
    title: String,
    pub workout: Workout,
}

impl WorkoutPopup {
    pub fn new(workout: &Workout) -> Self {
        Self {
            title: format!("Apply {}", workout.name),
            workout: workout.clone(),
        }
    }
}

impl Popup for WorkoutPopup {
    fn title(&self) -> &String {
        &self.title
    }

    fn render_content(&self, area: Rect, buf: &mut ratatui::prelude::Buffer) {
        let mut constraints = vec![Constraint::Length(1), Constraint::Length(1)];

        for _ in self.workout.exercises.iter() {
            constraints.push(Constraint::Length(1));
        }

        let layout =
            Layout::new(Direction::Vertical, constraints).split(area.inner(Margin::new(1, 1)));

        for (i, exercise) in self.workout.exercises.iter().enumerate() {
            Paragraph::new(format!("{}: {}", exercise.0.to_uppercase(), exercise.1))
                .centered()
                .render(layout[i], buf);
        }

        Paragraph::new("Press Enter to confirm, q/Esc to cancel")
            .dark_gray()
            .centered()
            .render(layout[2], buf);
    }

    fn handle_key(&mut self, app: &mut crate::ui::app::App, key: KeyCode) -> bool {
        match key {
            KeyCode::Enter => {
                for exercise in self.workout.exercises.iter() {
                    let amount = exercise.1;
                    let exercise = app.exercises.get(exercise.0);

                    if exercise.is_some() {
                        let exercise = exercise.unwrap().clone();
                        app.add_xp(&exercise.category, exercise.xp(*amount));
                        app.add_general_xp(self.workout.xp_bonus(&app.exercises));
                    }
                }

                return true;
            }
            KeyCode::Esc | KeyCode::Char('q') => {
                return true;
            }
            _ => {}
        }

        false
    }
}
