use bevy::prelude::*;

use crate::common::components::{AnimationIndices, AnimationTimer, Movable, Velocity};

use crate::item::components::{ItemVariant, PowerUp};

#[derive(Bundle)]
pub struct PowerUpBundle {
    pub power_up: PowerUp,
    pub variant: ItemVariant,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub movable: Movable,
    pub velocity: Velocity,

    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}
