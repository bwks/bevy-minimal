use bevy::prelude::Resource;
use bevy::time::{Timer, TimerMode};

use crate::world::TREE_SPAWN_TIME;

#[derive(Resource)]
pub struct TreeSpawnTimer {
    pub timer: Timer,
}

impl Default for TreeSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(TREE_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
