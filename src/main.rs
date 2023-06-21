pub mod common;
pub mod enemy;
pub mod game;
pub mod player;
pub mod power_up;
pub mod score;
pub mod world;

use bevy::prelude::*;
use bevy::window::WindowMode;

use crate::common::get_game_texture;
use crate::common::resources::GameTextures;

use enemy::EnemyPlugin;
use enemy::ENEMY1_SPRITE;
use game::GamePlugin;
use player::{
    PlayerPlugin, PLAYER1_DEAD_SPRITE, PLAYER1_SPRITE, PLAYER2_DEAD_SPRITE, PLAYER2_SPRITE,
};

use score::ScorePlugin;
use world::WorldPlugin;

pub fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player1_texture_atlas = get_game_texture(PLAYER1_SPRITE, &asset_server);
    let player1_texture_handle = texture_atlases.add(player1_texture_atlas);

    let player2_texture_atlas = get_game_texture(PLAYER2_SPRITE, &asset_server);
    let player2_texture_handle = texture_atlases.add(player2_texture_atlas);

    let player1_dead_texture_atlas = get_game_texture(PLAYER1_DEAD_SPRITE, &asset_server);
    let player1_dead_texture_handle = texture_atlases.add(player1_dead_texture_atlas);

    let player2_dead_texture_atlas = get_game_texture(PLAYER2_DEAD_SPRITE, &asset_server);
    let player2_dead_texture_handle = texture_atlases.add(player2_dead_texture_atlas);

    let enemy1_texture_atlas = get_game_texture(ENEMY1_SPRITE, &asset_server);
    let enemy1_texture_handle = texture_atlases.add(enemy1_texture_atlas);

    let game_textures = GameTextures {
        player_one: player1_texture_handle,
        player_two: player2_texture_handle,
        player_one_dead: player1_dead_texture_handle,
        player_two_dead: player2_dead_texture_handle,
        player_fireball: asset_server.load("fireball-right.png"),
        enemy_zombie: enemy1_texture_handle,
        explosion: asset_server.load("explosion.png"),
    };
    commands.insert_resource(game_textures);
}

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
        .add_startup_system(setup_system)
        .run();
}
