const BASE_XP_LEVEL: i32 = 100;

pub fn levelup_requirement(current_level: i32) -> i32 {
    ((1.0 + (current_level as f32 * 8.0).sqrt() * 4.0) * BASE_XP_LEVEL as f32).floor() as i32
}
