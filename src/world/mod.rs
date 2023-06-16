pub mod components;
pub mod resources;
pub mod systems;

use crate::{
    world::systems::{
        tree_movement_system, tree_spawn_system, tree_spawn_system_init,
        tree_spawn_timer_tick_system, trees_spawn_over_time_system,
    },
    AppState,
};
use bevy::prelude::*;

use self::resources::TreeSpawnTimer;

pub const TREE_SPAWN_TIME: f32 = 5.0;
pub const NUMBER_OF_TREES: u8 = 4;
// pub const TREE_SPRITE: &str = "tree.png";
// pub const TREE_SPRITE: &str = "grass.png";
// pub const TREE_SPRITE: &str = "grass2_32x32.png";
pub const TREE_SPRITE: &str = "Bush-001.png";
pub const TREE_SIZE: (f32, f32) = (32.0, 16.0); // (x, y)
pub const TREE_SCALE: f32 = 0.5;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TreeSpawnTimer>()
            .add_startup_system(tree_spawn_system_init)
            .add_startup_system(tree_spawn_system)
            .add_system(tree_movement_system.run_if(in_state(AppState::InGame)))
            .add_system(tree_spawn_timer_tick_system)
            .add_system(trees_spawn_over_time_system);
    }
}
