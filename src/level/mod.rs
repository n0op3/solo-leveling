use std::fmt::Display;

use serde::{Deserialize, Serialize};

const BASE_XP_LEVEL: i32 = 100;

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Level {
    level: i32,
    xp: i32,
}

impl Level {
    pub fn add_xp(&mut self, mut xp: i32) {
        if xp < 0 {
            while -xp > self.xp {
                self.level -= 1;
                self.xp = self.levelup_requirement() - 1;
            }

            self.xp += xp;
            self.level = self.level.max(0);
            self.xp = self.xp.max(0);
            return;
        }

        while xp >= self.levelup_requirement() - self.xp() {
            xp -= self.levelup_requirement() - self.xp();
            self.level_up();
        }

        self.xp += xp;

        self.level = self.level.max(0);
        self.xp = self.xp.max(0);
    }

    pub fn level_up(&mut self) {
        self.level += 1;
        self.xp = 0;
    }

    pub fn xp_text(&self) -> String {
        format!("{}/{} XP", self.xp, self.levelup_requirement())
    }

    pub fn levelup_requirement(&self) -> i32 {
        let current_level = self.level.max(0);
        ((1.0 + ((current_level + 1) as f32 * 4.0).sqrt() * 5.0) * BASE_XP_LEVEL as f32).floor()
            as i32
    }

    pub fn percentage(&self) -> f32 {
        self.xp as f32 / self.levelup_requirement() as f32
    }

    pub fn xp_to_level_up(&self) -> i32 {
        self.levelup_requirement() - self.xp
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn xp(&self) -> i32 {
        self.xp
    }
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.level.to_string().as_str())
    }
}
