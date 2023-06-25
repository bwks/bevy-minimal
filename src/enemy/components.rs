use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component, Default, Clone, Copy)]
pub enum EnemyVariant {
    #[default]
    Zombie,
    Skelton,
    Goblin,
}

#[derive(Component)]
pub struct EnemyDeadLocation;

#[derive(Component)]
pub struct EnemyDead;

#[derive(Component)]
pub struct EnemyDeadTimer(pub Timer);

impl Default for EnemyDeadTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.07, TimerMode::Repeating))
    }
}
