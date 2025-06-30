use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ExerciseList {}

#[derive(Debug, Deserialize)]
pub enum ExerciseTemplate {
    RepExercise { xp: usize },
    TimedExercise { xp: usize },
}
