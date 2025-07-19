use crate::ui::widget::popup::{MAX_INPUT_LENGTH, Popup};
use crossterm::event::KeyCode;
use ratatui::style::Color;
use ratatui::widgets::Widget;
use ratatui::{layout::Constraint, prelude::Margin};
use ratatui::{layout::Direction, widgets::Paragraph};
use ratatui::{prelude::Layout, style::Stylize};

#[derive(Debug)]
pub struct BonusXPPopup {
    title: String,
    xp_input: String,
    category_input: String,
    input_focused: bool,
}

impl Default for BonusXPPopup {
    fn default() -> Self {
        Self {
            title: String::from("Bonus XP"),
            xp_input: String::new(),
            category_input: String::new(),
            input_focused: false,
        }
    }
}

impl BonusXPPopup {
    fn is_input_valid(&self) -> bool {
        self.xp_input.parse::<i32>().is_ok() && !self.category_input.is_empty()
    }
}

impl Popup for BonusXPPopup {
    fn render_content(&self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(2),
                Constraint::Fill(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ],
        )
        .split(area.inner(Margin::new(1, 1)));

        Paragraph::new(format!(
            "{} bonus XP",
            if self.xp_input.is_empty() {
                "0".to_string()
            } else {
                self.xp_input.to_string()
            }
        ))
        .fg(if self.input_focused {
            Color::Blue
        } else {
            Color::Gray
        })
        .centered()
        .render(layout[0], buf);

        Paragraph::new(format!("Category: {}", self.category_input))
            .fg(if !self.input_focused {
                Color::Blue
            } else {
                Color::Gray
            })
            .centered()
            .render(layout[1], buf);

        if !self.is_input_valid() {
            Paragraph::new("Please provide correct input for the XP and the category")
                .red()
                .bold()
                .centered()
                .render(layout[2], buf);
        }

        Paragraph::new("Press Enter to confirm, q/Esc to cancel")
            .dark_gray()
            .centered()
            .render(layout[3], buf);
    }

    fn title(&self) -> &String {
        &self.title
    }

    fn handle_key(
        &mut self,
        app: &mut crate::ui::app::App,
        key: crossterm::event::KeyCode,
    ) -> bool {
        let input = if self.input_focused {
            &mut self.category_input
        } else {
            &mut self.xp_input
        };

        match key {
            KeyCode::Esc => return true,
            KeyCode::Backspace => {
                if !input.is_empty() {
                    input.remove(input.len() - 1);
                }
            }
            KeyCode::Tab => {
                self.input_focused = !self.input_focused;
            }
            KeyCode::Enter => {
                if self.is_input_valid() {
                    app.add_xp(
                        &self.category_input.to_lowercase(),
                        self.xp_input.parse().unwrap(),
                    );
                    return true;
                }
            }
            _ => {
                if self.input_focused
                    && let Some(character) = key.as_char()
                {
                    input.push(character);
                } else {
                    match key {
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
                            if !(key == KeyCode::Char('0') && self.xp_input.is_empty())
                                && self.xp_input.len() < MAX_INPUT_LENGTH
                            {
                                self.xp_input.push(key.as_char().unwrap());
                            }
                        }
                        KeyCode::Char('-') => {
                            if !self.input_focused && self.xp_input.is_empty() {
                                self.xp_input.push('-');
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        false
    }
}
