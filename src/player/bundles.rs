use bevy::prelude::*;

use leafwing_input_manager::prelude::*;

use crate::common::components::{AnimationIndices, AnimationTimer, Movable, Velocity, Vitality};
use crate::player::actions::ControlAction;
use crate::player::components::{Lives, Player, PlayerDead, PlayerVariant};

#[derive(Bundle)]
pub struct PlayerDeadBundle {
    pub player_dead: PlayerDead,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,
    pub movable: Movable,
    pub velocity: Velocity,

    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    pub variant: PlayerVariant,
    pub lives: Lives,
    pub vitality: Vitality,
    pub animation_indices: AnimationIndices,
    pub animation_timer: AnimationTimer,

    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    pub input_manager: InputManagerBundle<ControlAction>,

    #[bundle]
    pub sprite_sheet: SpriteSheetBundle,
}

impl PlayerBundle {
    pub fn input_map(player_variant: PlayerVariant) -> InputMap<ControlAction> {
        let mut input_map = match player_variant {
            PlayerVariant::One => InputMap::new([
                (KeyCode::W, ControlAction::Up),
                (KeyCode::S, ControlAction::Down),
                (KeyCode::A, ControlAction::Left),
                (KeyCode::D, ControlAction::Right),
                (KeyCode::B, ControlAction::Fire),
            ])
            .set_gamepad(Gamepad { id: 0 })
            .build(),
            PlayerVariant::Two => InputMap::new([
                (KeyCode::Up, ControlAction::Up),
                (KeyCode::Down, ControlAction::Down),
                (KeyCode::Left, ControlAction::Left),
                (KeyCode::Right, ControlAction::Right),
                (KeyCode::J, ControlAction::Fire),
            ])
            .set_gamepad(Gamepad { id: 1 })
            .build(),
        };

        input_map.insert(DualAxis::left_stick(), ControlAction::AxisMove);

        input_map.insert_multiple([
            (GamepadButtonType::DPadUp, ControlAction::Up),
            (GamepadButtonType::DPadDown, ControlAction::Down),
            (GamepadButtonType::DPadLeft, ControlAction::Left),
            (GamepadButtonType::DPadRight, ControlAction::Right),
            (GamepadButtonType::South, ControlAction::Fire),
            (GamepadButtonType::Select, ControlAction::Pause),
            (GamepadButtonType::Start, ControlAction::Restart),
        ]);

        input_map
    }
}
