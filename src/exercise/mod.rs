use ratatui::style::Color;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ExerciseType {
    Dynamic,
    Static,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Exercise {
    pub xp: usize,
    pub category: String,
    pub time: Option<i32>,
}

#[derive(Debug)]
pub enum Difficulty {
    Easy,
    Normal,
    Difficult,
    Extreme,
}

impl Exercise {
    pub fn xp(&self, reps_or_seconds: usize) -> usize {
        (match self.time {
            Some(time) => reps_or_seconds / time as usize,
            None => reps_or_seconds,
        }) * self.xp
    }

    pub fn xp_text(&self) -> String {
        format!(
            "{} XP {}",
            self.xp,
            match self.time {
                Some(time) => format!("per {time} seconds"),
                None => "per rep".to_string(),
            }
        )
    }

    pub fn absolute_xp(&self) -> usize {
        match self.time {
            Some(time) => self.xp / time as usize,
            None => self.xp,
        }
    }

    pub fn difficulty(&self) -> Difficulty {
        match self.absolute_xp() {
            0..=2 => Difficulty::Easy,
            3..=5 => Difficulty::Normal,
            6..=10 => Difficulty::Difficult,
            _ => Difficulty::Extreme,
        }
    }

    pub fn color(&self) -> Color {
        match self.difficulty() {
            Difficulty::Easy => Color::Green,
            Difficulty::Normal => Color::Yellow,
            Difficulty::Difficult => Color::Red,
            Difficulty::Extreme => Color::Magenta,
        }
    }
}
