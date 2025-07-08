#[derive(Debug, Clone)]
pub enum Page {
    Dashboard,
    Workouts,
    Exercises,
}

impl Page {
    pub fn name(&self) -> String {
        match self {
            Page::Dashboard => String::from("Dashboard"),
            Page::Workouts => String::from("Workouts"),
            Page::Exercises => String::from("Exercises"),
        }
    }
}
