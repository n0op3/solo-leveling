#[derive(Debug, Clone)]
pub enum Page {
    Dashboard,
    Workouts,
    Exercises(i32),
}

impl Page {
    pub fn name(&self) -> String {
        match self {
            Page::Dashboard => String::from("Dashboard"),
            Page::Workouts => String::from("Workouts"),
            Page::Exercises(_) => String::from("Exercises"),
        }
    }
}
