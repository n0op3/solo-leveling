use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

use crate::category::Category;
use crate::data::get_data_dir;
use crate::level::Level;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub level: Level,
    pub categories: HashMap<String, Category>,
}

pub fn load_user_data() -> UserData {
    if !fs::exists(user_data_path()).expect("Failed to check for the user data") {
        return UserData::default();
    }

    let data: UserData = toml::from_str(read_to_string(user_data_path()).unwrap().as_str())
        .expect("User data is invalid");

    data
}

pub fn user_data_path() -> PathBuf {
    let mut data_path = get_data_dir();
    data_path.push("user.toml");

    data_path
}

pub fn write_data(data: &UserData) {
    fs::write(
        user_data_path(),
        toml::ser::to_string(data).expect("Failed to serialize the user data"),
    )
    .expect("Failed to write the user data");
}

impl Default for UserData {
    fn default() -> Self {
        Self {
            level: Level::default(),
            categories: HashMap::new(),
        }
    }
}
