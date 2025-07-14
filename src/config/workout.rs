use std::{
    collections::HashMap,
    fs::{self, read_dir, read_to_string},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

use crate::config::{exercise::Exercise, get_config_dir};

#[derive(Debug, Serialize, Deserialize)]
pub struct Workout {
    pub name: String,
    pub exercises: HashMap<String, i32>,
}

impl Workout {
    pub fn xp_bonus(&self, exercise_map: &HashMap<String, Exercise>) -> i32 {
        self.exercises
            .iter()
            .map(
                |(exercise_name, amount)| match exercise_map.get(exercise_name) {
                    Some(exercise) => exercise.xp(*amount),
                    None => 0,
                },
            )
            .sum()
    }
}

pub fn load_workouts() -> Vec<Workout> {
    if !fs::exists(workouts_path()).expect("Failed to check for the workouts directory") {
        return Vec::new();
    }

    let mut workouts = Vec::new();
    if let Ok(workouts_dir) = read_dir(workouts_path()) {
        for file in workouts_dir {
            if let Ok(workout) = toml::from_str::<Workout>(
                read_to_string(file.expect("Failed to read a file").path())
                    .unwrap()
                    .as_str(),
            ) {
                workouts.push(workout);
            }
        }
    }

    workouts
}

pub fn workouts_path() -> PathBuf {
    get_config_dir().join("workouts")
}
