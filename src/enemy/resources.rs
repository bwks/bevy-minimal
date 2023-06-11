use bevy::prelude::Resource;
use bevy::time::{Timer, TimerMode};

use crate::enemy::ENEMY_SPAWN_TIME;

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
