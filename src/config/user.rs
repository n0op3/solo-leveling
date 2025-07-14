use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs::{self, read_to_string},
    path::PathBuf,
};

use crate::config::{daily_quest::DailyQuest, get_config_dir};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserConfig {
    pub name: Option<String>,
    pub daily_quest: Option<DailyQuest>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct IntermediateUserConfig {
    pub name: Option<String>,
    pub daily_quest: Option<HashMap<String, i32>>,
}

pub fn load_user_config() -> UserConfig {
    if !fs::exists(user_config_path()).expect("Failed to check for the user config") {
        return UserConfig::default();
    }

    let config: IntermediateUserConfig =
        toml::from_str(read_to_string(user_config_path()).unwrap().as_str())
            .expect("User config is invalid");

    UserConfig::from(config)
}

pub fn user_config_path() -> PathBuf {
    let mut config_path = get_config_dir();
    config_path.push("user.toml");

    config_path
}

pub fn write_config(config: &UserConfig) {
    fs::write(
        user_config_path(),
        toml::ser::to_string(config).expect("Failed to serialize the user config"),
    )
    .expect("Failed to write the user config");
}

impl From<IntermediateUserConfig> for UserConfig {
    fn from(config: IntermediateUserConfig) -> Self {
        Self {
            name: config.name,
            daily_quest: config.daily_quest.map(DailyQuest::new),
        }
    }
}
