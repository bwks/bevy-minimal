use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use crate::common::SpriteSheet;
use crate::enemy::resources::EnemySpawnTimer;
use crate::enemy::systems::{
    enemies_spawn_over_time_system, enemy_hit_player_system, enemy_movement_system,
    enemy_spawn_system, enemy_spawn_timer_tick_system, respawn_enemy_system,
};
use crate::game::states::GameState;

const ENEMY_SPRITE_1: SpriteSheet = SpriteSheet {
    file: "zombie-walk.png",
    width: 32.0,
    height: 32.0,
    scale: 3.0,
    columns: 11,
    rows: 4,
};

pub const ENEMY_SPAWN_TIME: f32 = 2.0;
// pub const ENEMY_SPRITE: &str = "zombie.png";
// pub const ENEMY_SPRITE: &str = "ZombieToast.png";
// pub const ENEMY_SPRITE_1: &str = "zombie-man-walk-96px.png";
pub const ENEMY_SPRITE_2: &str = "zombie-woman-walk-96px.png";
pub const ENEMY_SPRITE_3: &str = "zombie-wild-walk-96px.png";
pub const ENEMY_SIZE: (f32, f32) = (48.0, 92.0); // (x, y)
pub const ENEMY_SCALE: f32 = 1.0;
pub const NUMBER_OF_ENEMIES: u8 = 10;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct EnemySystemSet;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .configure_set(EnemySystemSet.run_if(in_state(GameState::Playing)))
            .add_startup_system(enemy_spawn_system)
            .add_systems(
                (
                    enemy_movement_system,
                    enemy_spawn_timer_tick_system,
                    enemies_spawn_over_time_system,
                    enemy_hit_player_system,
                )
                    .in_set(EnemySystemSet),
            )
            .add_system(respawn_enemy_system);
    }
}
