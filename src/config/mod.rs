use crate::exercise::ExerciseTemplate;
use dirs_next::config_dir;
use std::collections::HashMap;
use std::fs::{self, File, read_dir, read_to_string};
use std::path::PathBuf;
use toml::Value;

const CONFIG_FILE: &'static str = "solo-leveling/config.toml";
const DEFAULT_CONFIG: &'static str = "
    [categories]
    strength = 0
    speed = 0
    intelligence = 0
";

pub fn get_config() -> File {
    let mut config_path = config_dir().expect("No viable config directory found");
    config_path.push(CONFIG_FILE);

    if !config_path.exists() {
        create_default_config();
    }

    File::open(config_path).expect("Failed to open the config file")
}

pub fn get_config_dir() -> PathBuf {
    let mut config_path = config_dir().expect("No viable config directory found");
    config_path.push("solo-leveling");

    config_path
}

fn create_default_config() {
    let mut config_path = get_config_dir();
    fs::create_dir_all(&config_path).expect("Failed to create config directory");
    config_path.push(CONFIG_FILE);
    if !config_path.exists() {
        fs::write(config_path, DEFAULT_CONFIG).expect("Failed to write the config file");
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
