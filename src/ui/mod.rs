pub mod components;
pub mod styles;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use crate::game::states::AppState;
use crate::ui::systems::{
    interact_with_play_button_system, interact_with_quit_button_system, main_menu_despawn_system,
    main_menu_spawn_system,
};

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(main_menu_spawn_system.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(main_menu_despawn_system.in_schedule(OnExit(AppState::MainMenu)))
            .add_systems(
                (
                    interact_with_play_button_system,
                    interact_with_quit_button_system,
                )
                    .in_set(OnUpdate(AppState::MainMenu)),
            );
    }
}
