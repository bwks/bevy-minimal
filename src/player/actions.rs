use leafwing_input_manager::orientation::Direction;
use leafwing_input_manager::Actionlike;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    AxisMove,
    Fire,
}

impl Action {
    // Lists like this can be very useful for quickly matching subsets of actions
    pub const PLAYER_MOVE_KEYS: [Self; 5] = [
        Self::Left,
        Self::Right,
        Self::Up,
        Self::Down,
        Self::AxisMove,
    ];

    pub fn direction(self) -> Option<Direction> {
        match self {
            Self::Up => Some(Direction::NORTH),
            Self::Down => Some(Direction::SOUTH),
            Self::Left => Some(Direction::WEST),
            Self::Right => Some(Direction::EAST),
            _ => None,
        }
    }
}
