use bevy::prelude::{Camera2dBundle, Commands};

pub fn spawn_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
