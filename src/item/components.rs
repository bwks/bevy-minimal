use bevy::prelude::Component;

/// When consumed, a mushroom makes the player
/// invincible for 30 seconds.
#[derive(Component)]
pub struct Mushroom;

#[derive(Component)]
pub struct Diamond;

#[derive(Component)]
pub struct PowerUp;

#[derive(Component)]
pub enum ItemVariant {
    Diamond,
    Mushroom,
}
