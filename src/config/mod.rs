use dirs_next::config_dir;
use std::fs;
use std::path::PathBuf;

pub fn get_config_path() -> PathBuf {
    let mut path = config_dir().expect("No config directory found");
    path.push("solo-leveling");
    fs::create_dir_all(&path).expect("Failed to create config directory");
    path
}
