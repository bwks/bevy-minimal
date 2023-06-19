use bevy::prelude::Component;

#[derive(Component)]
pub enum Player {
    One,
    Two,
}

#[derive(Component)]
pub struct Playable;

#[derive(Component)]
pub struct Fireball;
