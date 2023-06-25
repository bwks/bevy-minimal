pub mod components;
pub mod systems;

use bevy::prelude::*;

use crate::common::SpriteSheet;

use crate::power_up::systems::diamond_spawn_system;

pub const DIAMOND_SPRITE: SpriteSheet = SpriteSheet {
    file: "diamond.png",
    width: 18.0,
    height: 14.0,
    scale: 3.0,
    columns: 10,
    rows: 1,
};

pub struct PowerUpPlugin;

impl Plugin for PowerUpPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(diamond_spawn_system);
    }
}
