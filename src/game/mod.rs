pub mod systems;

use bevy::prelude::*;

use crate::game::systems::spawn_camera_system;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_camera_system);
    }
}
