#[derive(Debug, Clone)]
pub enum Page {
    Overview,
    Workouts,
    Exercises(i32),
}

impl Page {
    pub fn name(&self) -> String {
        match self {
            Page::Overview => String::from("Overview"),
            Page::Workouts => String::from("Workouts"),
            Page::Exercises(_) => String::from("Exercises"),
        }
    }
}
