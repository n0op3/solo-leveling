use crate::config::get_config_dir;
use ratatui::style::Color;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};

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

pub fn load_exercises() -> HashMap<String, Exercise> {
    let mut exercises = HashMap::new();

    let mut exercise_path = get_config_dir();
    exercise_path.push("exercises");

    for file in read_dir(exercise_path).unwrap() {
        let file = file.unwrap();
        if file.path().is_file() && file.path().extension().unwrap_or_default() == "toml" {
            let contents = read_to_string(file.path()).unwrap();
            let exercises_list: HashMap<String, Exercise> =
                toml::from_str(contents.as_str()).unwrap();

            for (name, exercise) in exercises_list.iter() {
                exercises.insert(name.clone(), exercise.clone());
            }
        }
    }

    exercises
}

impl Exercise {
    pub fn xp(&self, reps_or_seconds: i32) -> usize {
        (match self.time {
            Some(time) => reps_or_seconds / time,
            None => reps_or_seconds,
        }) as usize
            * self.xp
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
