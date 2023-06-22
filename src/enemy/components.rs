use bevy::prelude::*;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemyDead;

#[derive(Component)]
pub struct EnemyDeadToSpawn(pub Vec3);

#[derive(Component)]
pub struct EnemyDeadTimer(pub Timer);

impl Default for EnemyDeadTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.05, TimerMode::Repeating))
    }
}
