use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum ExerciseTemplate {
    Dynamic { xp: usize },
    Static { xp: usize },
}

#[derive(Debug)]
pub struct Category {
    name: String,
    xp: usize,
}

impl Category {
    pub fn new(name: String, xp: usize) -> Self {
        Self { name, xp }
    }
}
