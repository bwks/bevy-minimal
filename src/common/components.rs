use bevy::prelude::*;
use bevy::time::Timer;

#[derive(Component, Debug, PartialEq, Eq, Default)]
pub enum Vitality {
    #[default]
    Alive,
    Dead,
}

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

impl Default for Velocity {
    fn default() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

impl Default for Movable {
    fn default() -> Self {
        Self {
            auto_despawn: false,
        }
    }
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
pub struct EntityLocation {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
