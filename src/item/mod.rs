pub mod bundles;
pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

use crate::common::SpriteSheet;

use crate::item::resources::DiamondSpawnTimer;
use crate::item::systems::{
    diamond_spawn_system, diamond_spawn_timer_tick_system, power_up_animation_system,
};

pub const DIAMOND_SPRITE: SpriteSheet = SpriteSheet {
    file: "diamond.png",
    width: 18.0,
    height: 14.0,
    scale: 3.0,
    columns: 10,
    rows: 1,
};

pub const DIAMOND_SPAWN_TIME: f32 = 2.0;

pub struct PowerUpPlugin;

impl Plugin for PowerUpPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiamondSpawnTimer>().add_systems((
            diamond_spawn_system,
            diamond_spawn_timer_tick_system,
            power_up_animation_system,
        ));
    }
}
