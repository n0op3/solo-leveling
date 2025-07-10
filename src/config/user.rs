use crate::config::{exercise::Category, get_config_dir};
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
    pub level: i32,
    pub xp: i32,
}

pub fn load_user_config() -> UserConfig {
    let config: UserConfig = toml::from_str(read_to_string(user_config_path()).unwrap().as_str())
        .expect("User config is invalid");

    config
}

pub fn user_config_path() -> PathBuf {
    let mut config_path = get_config_dir();
    config_path.push("user.toml");

    if !config_path.exists() {
        create_default_config();
    }

    config_path
}

fn create_default_config() {
    let mut config_path = get_config_dir();
    fs::create_dir_all(&config_path).expect("Failed to create config directory");
    config_path.push("user.toml");
    if !config_path.exists() {
        File::create(&config_path).expect("Failed to create the config file");
        fs::write(
            config_path,
            toml::ser::to_string(&UserConfig::default())
                .expect("Failed to create the default config"),
        )
        .expect("Failed to write the config file");
    }
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            user: User::default(),
            categories: HashMap::new(),
        }
    }
}
