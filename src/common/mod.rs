pub mod components;
pub mod resources;
pub mod utils;

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const BASE_SPEED: f32 = 500.0;
pub const SCROLL_X_VELOCITY: f32 = 0.1;
pub const SCROLL_Y_VELOCITY: f32 = 0.1;
pub const DEFAULT_SPAWN_TIMER: f32 = 2.0;

pub struct SpriteSheet<'a> {
    pub file: &'a str,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub columns: usize,
    pub rows: usize,
}
