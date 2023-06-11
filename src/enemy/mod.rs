use bevy::prelude::*;

pub mod components;
pub mod resources;
pub mod systems;

use crate::enemy::resources::EnemySpawnTimer;
use crate::enemy::systems::{
    enemies_spawn_over_time_system, enemy_hit_player_system, enemy_movement_system,
    enemy_spawn_system, enemy_spawn_timer_tick_system,
};

pub const ENEMY_SPAWN_TIME: f32 = 2.0;
pub const ENEMY_SPRITE: &str = "zombie.png";
pub const ENEMY_SIZE: (f32, f32) = (48.0, 92.0); // (x, y)
pub const ENEMY_SCALE: f32 = 1.0;
pub const NUMBER_OF_ENEMIES: u8 = 10;

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<EnemySpawnTimer>()
            .add_startup_system(enemy_spawn_system)
            .add_system(enemy_movement_system)
            .add_system(enemy_spawn_timer_tick_system)
            .add_system(enemies_spawn_over_time_system)
            .add_system(enemy_hit_player_system);
    }
}
