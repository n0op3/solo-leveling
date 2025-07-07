use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ExerciseTemplate {
    Dynamic { xp: usize },
    Static { xp: usize },
}

#[derive(Debug)]
struct Category {
    name: String,
    xp: usize,
}
