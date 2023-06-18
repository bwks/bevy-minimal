pub mod common;
pub mod enemy;
pub mod game;
pub mod player;
pub mod power_up;
pub mod score;
pub mod world;

use bevy::prelude::*;
use bevy::window::WindowMode;

use enemy::EnemyPlugin;
use game::GamePlugin;
use player::PlayerPlugin;

use score::ScorePlugin;
use world::WorldPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb_u8(85, 128, 0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen,
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_plugin(GamePlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .run();
}
