pub mod components;
pub mod systems;

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const BASE_SPEED: f32 = 500.0;

pub struct SpriteSheet<'a> {
    pub file: &'a str,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub columns: usize,
    pub rows: usize,
}
