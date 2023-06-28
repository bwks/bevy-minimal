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
    fireball_movement_system, player_confinement_system, player_dead_movement_system,
    player_dead_spawn_system, player_diamond_power_system, player_fire_system,
    player_fireball_hit_enemy_system, player_hit_power_up_system, player_movement_system,
    player_respawn_system, player_spawn_system,
};

pub const PLAYER1_SPRITE: SpriteSheet = SpriteSheet {
    // file: "player1.png",
    file: "player1.png",
    width: 32.0,
    height: 32.0,
    scale: 3.0,
    columns: 5,
    rows: 4,
};

pub const PLAYER2_SPRITE: SpriteSheet = SpriteSheet {
    // file: "player2.png",
    file: "player2.png",
    width: 32.0,
    height: 32.0,
    scale: 3.0,
    columns: 5,
    rows: 4,
};

pub const PLAYER1_IDLE_SPRITE: SpriteSheet = SpriteSheet {
    file: "player1-idle.png",
    width: 32.0,
    height: 32.0,
    scale: 3.0,
    columns: 3,
    rows: 4,
};

pub const PLAYER1_GHOST_SPRITE: SpriteSheet = SpriteSheet {
    file: "player1-ghost.png",
    width: 32.0,
    height: 32.0,
    scale: 3.0,
    columns: 5,
    rows: 4,
};

pub const PLAYER2_GHOST_SPRITE: SpriteSheet = SpriteSheet {
    file: "player2-ghost.png",
    width: 32.0,
    height: 32.0,
    scale: 3.0,
    columns: 5,
    rows: 4,
};

pub const PLAYER1_DEAD_SPRITE: SpriteSheet = SpriteSheet {
    file: "player1-dead.png",
    width: 32.0,
    height: 32.0,
    scale: 3.0,
    columns: 5,
    rows: 4,
};

pub const PLAYER_DIAMOND_SPRITE: SpriteSheet = SpriteSheet {
    // file: "player1.png",
    file: "player1-powerup.png",
    width: 32.0,
    height: 32.0,
    scale: 3.0,
    columns: 5,
    rows: 4,
};

pub const BULLET_SPRITE: SpriteSheet = SpriteSheet {
    file: "bullet.png",
    width: 32.0,
    height: 32.0,
    scale: 1.0,
    columns: 5,
    rows: 5,
};

pub const PLAYER_SPEED: f32 = 500.0;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct PlayerSystemSet;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(InputManagerPlugin::<ControlAction>::default())
            .configure_set(PlayerSystemSet.run_if(in_state(GameState::Playing)))
            .add_startup_system(player_spawn_system.in_base_set(StartupSet::PostStartup))
            .add_systems(
                (
                    player_movement_system,
                    player_confinement_system,
                    player_fire_system,
                    fireball_movement_system,
                    player_fireball_hit_enemy_system,
                    player_dead_spawn_system,
                    player_dead_movement_system,
                    player_hit_power_up_system,
                    player_diamond_power_system,
                )
                    .in_set(PlayerSystemSet),
            )
            .add_system(player_respawn_system);
    }
}
