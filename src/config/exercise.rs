use crate::config::get_config_dir;
use crate::exercise::Exercise;
use std::collections::HashMap;
use std::fs::{read_dir, read_to_string};

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
