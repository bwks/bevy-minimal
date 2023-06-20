use bevy::prelude::Component;

#[derive(Component, Clone)]
pub enum Player {
    One,
    Two,
}

#[derive(Component)]
pub struct Playable;

#[derive(Component)]
pub struct Fireball;
