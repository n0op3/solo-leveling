use std::{collections::HashMap, fs::read_to_string, path::PathBuf};

use crate::config::get_config_dir;

pub fn load_daily_quest() -> Option<HashMap<String, i32>> {
    let daily = match read_to_string(daily_quest_path()) {
        Ok(daily) => Some(toml::from_str(daily.as_str())),
        Err(_) => None,
    };

    let daily = match daily {
        Some(daily) => match daily {
            Ok(toml) => Some(toml),
            Err(_) => None,
        },
        None => None,
    };

    daily
}

pub fn daily_quest_path() -> PathBuf {
    get_config_dir().join("daily.toml")
}
