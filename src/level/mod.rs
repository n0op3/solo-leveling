const BASE_XP_LEVEL: usize = 100;

pub fn levelup_requirement(current_level: usize) -> usize {
    ((1.0 + (current_level as f32 * 8.0).sqrt() * 4.0) * BASE_XP_LEVEL as f32).floor() as usize
}
