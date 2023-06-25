use bevy::prelude::*;

use crate::common::components::{
    AnimationIndices, AnimationTimer, EntityLocation, Movable, Velocity, Vitality,
};
use crate::enemy::components::{Enemy, EnemyDead, EnemyDeadLocation, EnemyVariant};

#[derive(Bundle)]
pub struct EnemyDeadLocationBundle {
    pub entity: EnemyDeadLocation,
    pub variant: EnemyVariant,
    pub location: EntityLocation,
}

#[derive(Bundle)]
pub struct EnemyDeadBundle {
    pub entity: EnemyDead,
    pub variant: EnemyVariant,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub movable: Movable,
    pub velocity: Velocity,

    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}

#[derive(Bundle)]
pub struct EnemyBundle {
    pub entity: Enemy,
    pub variant: EnemyVariant,
    pub vitality: Vitality,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub movable: Movable,
    pub velocity: Velocity,

    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}
