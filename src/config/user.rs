use serde::{Deserialize, Serialize};
use std::{
    fs::{self, read_to_string},
    path::PathBuf,
};

use crate::config::get_config_dir;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UserConfig {
    pub name: Option<String>,
}

pub fn load_user_config() -> UserConfig {
    if !fs::exists(user_config_path()).expect("Failed to check for the user config") {
        return UserConfig::default();
    }

    let config: UserConfig = toml::from_str(read_to_string(user_config_path()).unwrap().as_str())
        .expect("User config is invalid");

    config
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
