use serde::{Deserialize, Serialize};

use crate::level::Level;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Category {
    pub level: Level,
}
