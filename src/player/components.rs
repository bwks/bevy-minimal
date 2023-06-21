use bevy::prelude::Component;

#[derive(Component, Clone, Debug)]
pub enum Player {
    One,
    Two,
}

#[derive(Component, Debug, PartialEq, Eq)]
pub enum PlayerState {
    Alive,
    Dead,
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
