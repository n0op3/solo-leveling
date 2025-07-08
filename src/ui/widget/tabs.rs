#[derive(Debug, Clone)]
pub enum Tab {
    Dashboard,
    Workouts,
}

impl Tab {
    pub fn name(&self) -> String {
        match self {
            Tab::Dashboard => String::from("Dashboard"),
            Tab::Workouts => String::from("Workouts"),
        }
    }
}
