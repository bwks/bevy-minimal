use bevy::prelude::Component;

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
pub struct FpsText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
pub struct ColorText;

#[derive(Component)]
pub struct UiPlayerLives;
