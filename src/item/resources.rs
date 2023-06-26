use bevy::prelude::*;

use crate::item::DIAMOND_POWER_TIME;
use crate::item::DIAMOND_SPAWN_TIME;

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

#[derive(Resource)]
pub struct DiamondPowerTimer {
    pub timer: Timer,
}

impl Default for DiamondPowerTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(DIAMOND_POWER_TIME, TimerMode::Once),
        }
    }
}
