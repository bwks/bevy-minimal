pub mod actions;
pub mod bundles;
pub mod components;
pub mod systems;

use bevy::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;

use crate::common::SpriteSheet;
use crate::game::states::GameState;
use crate::player::actions::ControlAction;
use crate::player::systems::{
    fireball_movement_system, player_confinement_system, player_fire_system,
    player_fireball_hit_enemy_system, player_movement_system, player_respawn_system,
    player_spawn_system,
};

const PLAYER1_SPRITE: SpriteSheet = SpriteSheet {
    file: "player1.png",
    width: 32.0,
    height: 32.0,
    scale: 3.0,
    columns: 4,
    rows: 4,
};

const PLAYER2_SPRITE: SpriteSheet = SpriteSheet {
    // file: "player2.png",
    file: "player2.png",
    width: 32.0,
    height: 32.0,
    scale: 3.0,
    columns: 4,
    rows: 4,
};

pub const PLAYER1_DEAD_SPRITE: SpriteSheet = SpriteSheet {
    file: "player1-ghost.png",
    width: 32.0,
    height: 32.0,
    scale: 3.0,
    columns: 4,
    rows: 4,
};

pub const PLAYER2_DEAD_SPRITE: SpriteSheet = SpriteSheet {
    file: "player2-ghost.png",
    width: 32.0,
    height: 32.0,
    scale: 3.0,
    columns: 4,
    rows: 4,
};

pub const EXPLOSION_SPRITE: SpriteSheet = SpriteSheet {
    file: "explosion.png",
    width: 32.0,
    height: 32.0,
    scale: 10.0,
    columns: 6,
    rows: 1,
};

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SIZE: (f32, f32) = (17.0, 30.0); // (x, y)
pub const PLAYER_SCALE: f32 = 2.0;
pub const PLAYER_FIRE_KEY: KeyCode = KeyCode::J;
pub const PLAYER_FIREBALL_SPRITE: &str = "fireball-right.png";
pub const PLAYER_FIREBALL_SIZE: (f32, f32) = (75.0, 47.0);
pub const PLAYER_FIREBALL_SCALE: f32 = 0.3;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<ControlAction>::default())
            .configure_set(PlayerSystemSet.run_if(in_state(GameState::Playing)))
            .add_startup_system(player_spawn_system)
            .add_systems(
                (
                    // player_spawn_system,
                    player_movement_system,
                    player_confinement_system,
                    player_fire_system,
                    fireball_movement_system,
                    player_fireball_hit_enemy_system,
                )
                    .in_set(PlayerSystemSet),
            )
            .add_system(player_respawn_system);
    }
}
