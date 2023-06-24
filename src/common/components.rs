use bevy::prelude::*;
use bevy::time::Timer;

#[derive(Component, Debug, PartialEq, Eq)]
pub enum Vitality {
    Alive,
    Dead,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Component, Clone, Copy)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

impl Default for AnimationTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(0.1, TimerMode::Repeating))
    }
}

#[derive(Component)]
pub struct EntityDeadLocation(pub Vec3);
