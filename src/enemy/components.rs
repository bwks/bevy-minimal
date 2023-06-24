use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Default)]
pub enum EnemyVariant {
    #[default]
    Zombie,
    Skelton,
}

#[derive(Component)]
pub struct EnemyDead;

#[derive(Component)]
pub struct EnemyDeadTimer(pub Timer);

impl Default for EnemyDeadTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.07, TimerMode::Repeating))
    }
}
