use bevy::prelude::*;

use leafwing_input_manager::prelude::*;

use crate::player::actions::Action;
use crate::player::components::Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub player: Player,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    pub input_manager: InputManagerBundle<Action>,
}

impl PlayerBundle {
    pub fn default_input_map() -> InputMap<Action> {
        // This allows us to replace `Action::Up` with `Up`,
        // significantly reducing boilerplate
        let mut input_map = InputMap::default();

        // Axis joystick
        input_map.insert(DualAxis::left_stick(), Action::AxisMove);

        // D-Pad
        input_map.insert(GamepadButtonType::DPadUp, Action::Up);
        input_map.insert(GamepadButtonType::DPadDown, Action::Down);
        input_map.insert(GamepadButtonType::DPadLeft, Action::Left);
        input_map.insert(GamepadButtonType::DPadRight, Action::Right);

        // Keyboard
        input_map.insert(KeyCode::Up, Action::Up);
        input_map.insert(KeyCode::W, Action::Up);
        input_map.insert(KeyCode::Down, Action::Down);
        input_map.insert(KeyCode::S, Action::Down);
        input_map.insert(KeyCode::Left, Action::Left);
        input_map.insert(KeyCode::A, Action::Left);
        input_map.insert(KeyCode::Right, Action::Right);
        input_map.insert(KeyCode::D, Action::Right);

        // Abilities
        input_map.insert(KeyCode::J, Action::Fire);
        input_map.insert(GamepadButtonType::South, Action::Fire);
        input_map.insert(MouseButton::Left, Action::Fire);

        input_map
    }
}
