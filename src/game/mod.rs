pub mod components;
pub mod states;
pub mod systems;

use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use bevy::prelude::*;

use crate::game::systems::{
    controller_system, exit_game_system, spawn_camera_system, text_color_system, text_setup_system,
    text_update_system, toggle_game_state_system,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default())
            .add_startup_system(spawn_camera_system)
            .add_startup_system(text_setup_system)
            .add_system(text_update_system)
            .add_system(text_color_system)
            .add_system(toggle_game_state_system)
            .add_system(exit_game_system)
            .add_system(controller_system);
    }
}
