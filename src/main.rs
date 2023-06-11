pub mod common;
pub mod enemy;
pub mod game;
pub mod player;
pub mod power_up;
pub mod score;

use bevy::prelude::*;

use enemy::EnemyPlugin;
use game::GamePlugin;
use player::PlayerPlugin;
use score::ScorePlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .run();
}
