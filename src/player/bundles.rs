use bevy::prelude::*;

use leafwing_input_manager::prelude::*;

use crate::player::actions::ControlAction;
use crate::player::components::Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    pub input_manager: InputManagerBundle<ControlAction>,
}

impl PlayerBundle {
    pub fn default_input_map() -> InputMap<ControlAction> {
        // This allows us to replace `Action::Up` with `Up`,
        // significantly reducing boilerplate
        let mut input_map = InputMap::default();

        // Axis joystick
        input_map.insert(DualAxis::left_stick(), ControlAction::AxisMove);

        // D-Pad
        input_map.insert(GamepadButtonType::DPadUp, ControlAction::Up);
        input_map.insert(GamepadButtonType::DPadDown, ControlAction::Down);
        input_map.insert(GamepadButtonType::DPadLeft, ControlAction::Left);
        input_map.insert(GamepadButtonType::DPadRight, ControlAction::Right);

        // Keyboard
        input_map.insert(KeyCode::Up, ControlAction::Up);
        input_map.insert(KeyCode::W, ControlAction::Up);
        input_map.insert(KeyCode::Down, ControlAction::Down);
        input_map.insert(KeyCode::S, ControlAction::Down);
        input_map.insert(KeyCode::Left, ControlAction::Left);
        input_map.insert(KeyCode::A, ControlAction::Left);
        input_map.insert(KeyCode::Right, ControlAction::Right);
        input_map.insert(KeyCode::D, ControlAction::Right);

        // Abilities
        input_map.insert(KeyCode::J, ControlAction::Fire);
        input_map.insert(GamepadButtonType::South, ControlAction::Fire);
        input_map.insert(MouseButton::Left, ControlAction::Fire);

        input_map
    }
}
