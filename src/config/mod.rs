use crate::exercise::ExerciseTemplate;
use dirs_next::config_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, File, read_dir, read_to_string};
use std::path::PathBuf;
use toml::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub user: User,
    pub categories: HashMap<String, i32>,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            user: User::default(),
            categories: [
                (String::from("strength"), 0),
                (String::from("speed"), 0),
                (String::from("intelligence"), 0),
                (String::from("social skills"), 0),
            ]
            .into(),
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    pub level: i32,
}

pub fn load_user_config() -> UserConfig {
    let config: UserConfig = toml::from_str(read_to_string(user_config_path()).unwrap().as_str())
        .expect("User config is invalid");

    config
}

pub fn user_config_path() -> PathBuf {
    let mut config_path = get_config_dir();
    config_path.push("config.toml");

    if !config_path.exists() {
        create_default_config();
    }

    config_path
}

pub fn get_config_dir() -> PathBuf {
    let mut config_path = config_dir().expect("No viable config directory found");
    config_path.push("solo-leveling");
    fs::create_dir_all(&config_path).expect("Failed to create the config directory");

    config_path
}

fn create_default_config() {
    let mut config_path = get_config_dir();
    fs::create_dir_all(&config_path).expect("Failed to create config directory");
    config_path.push("config.toml");
    if !config_path.exists() {
        File::create(&config_path).expect("Failed to create the config file");
        fs::write(
            config_path,
            toml::ser::to_string(&UserConfig::default())
                .expect("Failed to create the default config"),
        )
        .expect("Failed to write the config file");
    }
}

pub fn load_exercises() -> HashMap<String, ExerciseTemplate> {
    let mut exercises = HashMap::new();

    let mut exercise_path = get_config_dir();
    exercise_path.push("exercises");

    for file in read_dir(exercise_path).unwrap() {
        let file = file.unwrap();
        if file.path().is_file() && file.path().extension().unwrap_or_default() == "toml" {
            let contents = read_to_string(file.path()).unwrap();
            let exercises_list: Value = toml::from_str(contents.as_str()).unwrap();
            for (exercise_name, value) in exercises_list
                .as_table()
                .expect("config file is not a table of exercises")
            {
                let xp = value.get("xp").unwrap_or(&Value::Integer(1));
                let rep = Value::String(String::from("rep"));
                let exercise_type = value.get("type").unwrap_or(&rep);

                let exercise = match exercise_type.as_str().unwrap() {
                    "rep" => Some(ExerciseTemplate::Dynamic {
                        xp: xp.as_integer().unwrap() as usize,
                    }),
                    "timed" => Some(ExerciseTemplate::Static {
                        xp: xp.as_integer().unwrap() as usize,
                    }),
                    _ => {
                        println!("Unknown exercise type: {exercise_type}");
                        None
                    }
                };

                if let Some(exercise) = exercise {
                    exercises.insert(exercise_name.clone(), exercise);
                }
            }
        }
    }

    exercises
}
