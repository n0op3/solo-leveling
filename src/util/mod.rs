use chrono::{Datelike, Utc};
use toml::value::{Date, Datetime};

pub fn today() -> Datetime {
    Datetime {
        date: Some(Date {
            year: Utc::now().year() as u16,
            month: Utc::now().month() as u8,
            day: Utc::now().day() as u8,
        }),
        time: None,
        offset: None,
    }
}
