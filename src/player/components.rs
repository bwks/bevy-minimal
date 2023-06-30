use std::fmt;

use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone, PartialEq, Eq)]
pub enum PlayerVariant {
    One,
    Two,
}

impl fmt::Display for PlayerVariant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            PlayerVariant::One => write!(f, "1"),
            PlayerVariant::Two => write!(f, "2"),
        }
    }
}

#[derive(Component, Debug)]
pub struct Lives {
    pub count: u8,
}

impl Default for Lives {
    fn default() -> Self {
        Self { count: 3 }
    }
}

#[derive(Component)]
pub struct Score {
    pub value: u32,
}

impl Default for Score {
    fn default() -> Self {
        Self { value: 0 }
    }
}

#[derive(Component)]
pub struct Fireball;

#[derive(Component)]
pub struct PlayerDead;

#[derive(Component)]
pub struct PlayerDeadLocation;
