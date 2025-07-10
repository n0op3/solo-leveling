use crate::exercise::Exercise;
use dirs_next::config_dir;
use std::collections::HashMap;
use std::fs::{self, read_dir, read_to_string};
use std::path::PathBuf;

pub mod user;

pub fn get_config_dir() -> PathBuf {
    let mut config_path = config_dir().expect("No viable config directory found");
    config_path.push("solo-leveling");
    fs::create_dir_all(&config_path).expect("Failed to create the config directory");

    config_path
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
