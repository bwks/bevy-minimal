use bevy::prelude::*;

use crate::common::components::{AnimationIndices, AnimationTimer, Movable, Velocity};

use crate::power_up::components::{PowerUp, PowerUpVariant};

#[derive(Bundle)]
pub struct PowerUpBundle {
    pub power_up: PowerUp,
    pub variant: PowerUpVariant,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub movable: Movable,
    pub velocity: Velocity,

    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}
