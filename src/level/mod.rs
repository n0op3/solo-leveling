const BASE_XP_LEVEL: i32 = 100;

pub fn levelup_requirement(current_level: i32) -> i32 {
    let current_level = current_level.max(0);
    ((1.0 + ((current_level + 1) as f32 * 4.0).sqrt() * 5.0) * BASE_XP_LEVEL as f32).floor() as i32
}
