use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, Debug)]
pub enum PlayerVariant {
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
pub struct Fireball;

#[derive(Component)]
pub struct PlayerDead;

#[derive(Component)]
pub struct PlayerDeadLocation;
