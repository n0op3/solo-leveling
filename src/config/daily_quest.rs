use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::config::exercise::Exercise;

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyQuest {
    pub exercises: HashMap<String, (i32, i32)>,
}

impl DailyQuest {
    pub fn new(exercises: HashMap<String, i32>) -> Self {
        Self {
            exercises: exercises
                .iter()
                .map(|(exercise, amount)| (exercise.clone(), (*amount, 0)))
                .collect(),
        }
    }

    pub fn xp_bonus(&self, exercise_map: &HashMap<String, Exercise>) -> i32 {
        self.exercises
            .iter()
            .map(|(exercise_name, amount)| {
                let to_do = amount.0;
                let xp = match exercise_map.get(exercise_name) {
                    Some(exercise) => exercise.xp(to_do),
                    None => 0,
                };

                xp as i32
            })
            .sum()
    }

    pub fn exercise_done(&mut self, exercise_name: &String, amount: usize) {
        if let Some(exercise_to_do) = self.exercises.get_mut(exercise_name) {
            exercise_to_do.1 += amount as i32;
        }
    }

    pub fn is_done(&self) -> bool {
        for exercise in self.exercises.iter() {
            if exercise.1.1 < exercise.1.0 {
                return false;
            }
        }

        true
    }
}
