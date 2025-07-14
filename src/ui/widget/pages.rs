use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Modifier, Style, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Gauge, Paragraph};

use crate::ui::app::App;

#[derive(Debug, Clone)]
pub enum Page {
    Overview,
    DailyQuest,
    Exercises(i32),
}

impl Page {
    pub fn name(&self) -> String {
        match self {
            Page::Overview => String::from("Overview"),
            Self::DailyQuest => String::from("Daily Quest"),
            Page::Exercises(_) => String::from("Exercises"),
        }
    }

    pub fn render(&self, app: &App, frame: &mut Frame, area: Rect) {
        match self {
            Page::Overview => {
                let mut constraints = vec![Constraint::Max(1), Constraint::Length(5)];

                for _category in app.user_data.categories.iter() {
                    constraints.push(Constraint::Max(3));
                }

                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints(constraints)
                    .split(area);

                frame.render_widget(
                    Paragraph::new(format!("Welcome, {}", app.username))
                        .style(Style::default().white())
                        .centered(),
                    chunks[0],
                );
                frame.render_widget(
                    Gauge::default()
                        .block(
                            Block::bordered()
                                .title(format!("LEVEL {}", app.user_data.level.level())),
                        )
                        .gauge_style(Style::new().red())
                        .label(app.user_data.level.xp_text())
                        .percent((app.user_data.level.percentage() * 100.0) as u16),
                    chunks[1],
                );

                for (i, (category_name, category)) in app.user_data.categories.iter().enumerate() {
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

                for (i, (exercise_name, exercise)) in app.exercises.iter().enumerate() {
                    let style = Style::new().fg(exercise.color());

                    let line = Line::styled(
                        format!(
                            "{} {}: {}",
                            if *index == i as i32 { ">>" } else { "  " },
                            exercise_name.to_uppercase(),
                            exercise.xp_text()
                        ),
                        if *index == i as i32 {
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
    }
}
