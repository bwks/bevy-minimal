pub mod actions;
pub mod bundles;
pub mod components;
pub mod systems;

use bevy::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;

use crate::player::actions::ControlAction;
use crate::player::systems::{
    fireball_movement_system, player_confinement_system, player_fire_system,
    player_fireball_hit_enemy_system, player_movement_system, player_spawn_system,
};

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SPRITE: &str = "purplecloakwizard.png";
pub const PLAYER_SIZE: (f32, f32) = (17.0, 30.0); // (x, y)
pub const PLAYER_SCALE: f32 = 2.0;
pub const PLAYER_FIRE_KEY: KeyCode = KeyCode::J;
pub const PLAYER_FIREBALL_SPRITE: &str = "fireball-right.png";
pub const PLAYER_FIREBALL_SIZE: (f32, f32) = (75.0, 47.0);
pub const PLAYER_FIREBALL_SCALE: f32 = 0.3;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<ControlAction>::default())
            .add_startup_system(player_spawn_system)
            .add_system(player_movement_system)
            .add_system(player_confinement_system)
            .add_system(player_fire_system)
            .add_system(fireball_movement_system)
            .add_system(player_fireball_hit_enemy_system);
    }
}
