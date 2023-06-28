pub mod bundles;
pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

use crate::common::SpriteSheet;

use crate::item::resources::{DiamondPowerTimer, DiamondSpawnTimer};
use crate::item::systems::{
    diamond_power_timer_tick_system, diamond_spawn_system, diamond_spawn_timer_tick_system,
    power_up_animation_system,
};

use crate::game::states::GameState;

pub const DIAMOND_SPRITE: SpriteSheet = SpriteSheet {
    file: "diamond.png",
    width: 18.0,
    height: 14.0,
    scale: 3.0,
    columns: 10,
    rows: 1,
};

pub const DIAMOND_SPAWN_TIME: f32 = 10.0;
pub const DIAMOND_POWER_TIME: f32 = 60.0;

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub struct ItemSystemSet;
pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DiamondSpawnTimer>()
            .init_resource::<DiamondPowerTimer>()
            .configure_set(ItemSystemSet.run_if(in_state(GameState::Playing)))
            .add_systems(
                (
                    diamond_spawn_system,
                    diamond_spawn_timer_tick_system,
                    power_up_animation_system,
                    diamond_power_timer_tick_system,
                )
                    .in_set(ItemSystemSet),
            );
    }
}
