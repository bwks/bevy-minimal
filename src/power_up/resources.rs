use bevy::prelude::*;

use crate::power_up::DIAMOND_SPAWN_TIME;

#[derive(Resource)]
pub struct DiamondSpawnTimer {
    pub timer: Timer,
}

impl Default for DiamondSpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(DIAMOND_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}
