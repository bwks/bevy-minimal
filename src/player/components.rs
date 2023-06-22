use bevy::prelude::*;

#[derive(Component, Clone, Debug)]
pub enum Player {
    One,
    Two,
}

#[derive(Component)]
pub struct Lives {
    pub count: u8,
}

impl Default for Lives {
    fn default() -> Self {
        Self { count: 3 }
    }
}

#[derive(Component)]
pub struct Playable;

#[derive(Component)]
pub struct Fireball;

#[derive(Component)]
pub struct PlayerDead;

#[derive(Component)]
pub struct PlayerDeadToSpawn(pub Vec3);

#[derive(Component)]
pub struct PlayerDeadTimer(pub Timer);

impl Default for PlayerDeadTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}
