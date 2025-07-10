use crate::{
    config::{exercise::Category, get_config_dir},
    level::Level,
};
use std::{
    collections::HashMap,
    fs::{self, File, read_to_string},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserConfig {
    pub user: User,
    pub categories: HashMap<String, Category>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    pub name: Option<String>,
    pub level: Level,
}

pub fn load_user_config() -> UserConfig {
    if !fs::exists(user_config_path()).expect("Failed to check for the user config") {
        create_default_config();
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

fn create_default_config() {
    File::create(&user_config_path()).expect("Failed to create the config file");
    fs::write(
        user_config_path(),
        toml::ser::to_string(&UserConfig::default()).expect("Failed to create the default config"),
    )
    .expect("Failed to write the config file");
}

pub fn write_config(config: &UserConfig) {
    fs::write(
        user_config_path(),
        toml::ser::to_string(config).expect("Failed to serialize the user config"),
    )
    .expect("Failed to write the user config");
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            user: User::default(),
            categories: HashMap::new(),
        }
    }
}
