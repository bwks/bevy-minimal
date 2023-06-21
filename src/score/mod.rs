pub mod resources;

use bevy::prelude::*;

use crate::score::resources::{PlayerOneScore, PlayerTwoScore};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerOneScore>();
        app.init_resource::<PlayerTwoScore>();
    }
}
