use dirs_next::config_dir;
use std::{fs, path::PathBuf};

pub mod exercise;
pub mod user;

pub fn get_config_dir() -> PathBuf {
    let mut config_path = config_dir().expect("No viable config directory found");
    config_path.push("solo-leveling");

    // NOTE: Does nothing if the directory already exists
    fs::create_dir_all(&config_path).expect("Failed to create the config directory");

    config_path
}
