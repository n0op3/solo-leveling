use dirs_next::data_dir;
use std::{fs, path::PathBuf};

pub mod user;

pub fn get_data_dir() -> PathBuf {
    let mut data_path = data_dir().expect("No viable data directory found");
    data_path.push("solo-leveling");

    // NOTE: Does nothing if the directory already exists
    fs::create_dir_all(&data_path).expect("Failed to create the data directory");

    data_path
}
