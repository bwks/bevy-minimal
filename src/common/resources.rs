use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameTextures {
    pub player_one: Handle<TextureAtlas>,
    pub player_two: Handle<TextureAtlas>,
    pub player_one_dead: Handle<TextureAtlas>,
    pub player_two_dead: Handle<TextureAtlas>,
    pub player_one_ghost: Handle<TextureAtlas>,
    pub player_two_ghost: Handle<TextureAtlas>,
    pub enemy_zombie: Handle<TextureAtlas>,
    pub enemy_zombie_dead: Handle<TextureAtlas>,
    pub enemy_skeleton: Handle<TextureAtlas>,
    pub enemy_skeleton_dead: Handle<TextureAtlas>,
    pub enemy_goblin: Handle<TextureAtlas>,
    pub enemy_goblin_dead: Handle<TextureAtlas>,
    pub bullet: Handle<TextureAtlas>,
    pub diamond: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct GameAudio {
    pub player_dead: Handle<AudioSource>,
    pub player_shoot: Handle<AudioSource>,
    pub enemy_dead: Handle<AudioSource>,
}

// #[derive(Resource)]
// pub struct TimerRepeating {
//     pub timer: Timer,
// }

// impl Default for TimerRepeating {
//     fn default() -> Self {
//         Self {
//             timer: Timer::from_seconds(DEFAULT_SPAWN_TIMER, TimerMode::Repeating),
//         }
//     }
// }

// impl TimerRepeating {
//     pub fn new(seconds: f32) -> Self {
//         Self {
//             timer: Timer::from_seconds(seconds, TimerMode::Repeating),
//         }
//     }
// }
