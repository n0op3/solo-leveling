use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::config::exercise::Exercise;

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyQuest {
    pub exercises: HashMap<String, i32>,
}

impl DailyQuest {
    pub fn new(exercises: HashMap<String, i32>) -> Self {
        Self { exercises }
    }

    pub fn xp_bonus(&self, exercise_map: &HashMap<String, Exercise>) -> i32 {
        self.exercises
            .iter()
            .map(|(exercise_name, amount)| {
                let xp = match exercise_map.get(exercise_name) {
                    Some(exercise) => exercise.xp(*amount),
                    None => 0,
                };

                xp as i32
            })
            .sum()
    }

    pub fn is_completed(&self, exercises_done: HashMap<String, i32>) -> bool {
        for exercise in self.exercises.iter() {
            match exercises_done.get(exercise.0) {
                Some(amount) => {
                    if amount < exercise.1 {
                        return false;
                    }
                }
                None => return false,
            };
        }

        true
    }
}
