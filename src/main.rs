pub mod common;
pub mod enemy;
pub mod game;
pub mod player;
pub mod power_up;
pub mod score;
pub mod world;

use bevy::prelude::*;
use bevy::window::WindowMode;

use crate::common::resources::GameTextures;
use crate::common::utils::get_texture_atlas;

use enemy::EnemyPlugin;
use enemy::{ENEMY1_DEAD_SPRITE, ENEMY1_SPRITE, ENEMY2_DEAD_SPRITE, ENEMY2_SPRITE};
use game::GamePlugin;
use player::{
    PlayerPlugin, BULLET_SPRITE, PLAYER1_DEAD_SPRITE, PLAYER1_GHOST_SPRITE, PLAYER1_SPRITE,
    PLAYER2_GHOST_SPRITE, PLAYER2_SPRITE,
};

use score::ScorePlugin;
use world::WorldPlugin;

pub fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let player1_texture_atlas = get_texture_atlas(PLAYER1_SPRITE, &asset_server);
    let player1_texture_handle = texture_atlases.add(player1_texture_atlas);

    let player2_texture_atlas = get_texture_atlas(PLAYER2_SPRITE, &asset_server);
    let player2_texture_handle = texture_atlases.add(player2_texture_atlas);

    let player1_dead_texture_atlas = get_texture_atlas(PLAYER1_DEAD_SPRITE, &asset_server);
    let player1_dead_texture_handle = texture_atlases.add(player1_dead_texture_atlas);

    let player1_ghost_texture_atlas = get_texture_atlas(PLAYER1_GHOST_SPRITE, &asset_server);
    let player1_ghost_texture_handle = texture_atlases.add(player1_ghost_texture_atlas);

    let player2_ghost_texture_atlas = get_texture_atlas(PLAYER2_GHOST_SPRITE, &asset_server);
    let player2_ghost_texture_handle = texture_atlases.add(player2_ghost_texture_atlas);

    let enemy_zombie_texture_atlas = get_texture_atlas(ENEMY1_SPRITE, &asset_server);
    let enemy_zombie_texture_handle = texture_atlases.add(enemy_zombie_texture_atlas);

    let enemy_zombie_dead_texture_atlas = get_texture_atlas(ENEMY1_DEAD_SPRITE, &asset_server);
    let enemy_zombie_dead_texture_handle = texture_atlases.add(enemy_zombie_dead_texture_atlas);

    let enemy_skeleton_texture_atlas = get_texture_atlas(ENEMY2_SPRITE, &asset_server);
    let enemy_skeleton_texture_handle = texture_atlases.add(enemy_skeleton_texture_atlas);

    let enemy_skeleton_dead_texture_atlas = get_texture_atlas(ENEMY2_DEAD_SPRITE, &asset_server);
    let enemy_skeleton_dead_texture_handle = texture_atlases.add(enemy_skeleton_dead_texture_atlas);

    let bullet_texture_atlas = get_texture_atlas(BULLET_SPRITE, &asset_server);
    let bullet_texture_handle = texture_atlases.add(bullet_texture_atlas);

    let game_textures = GameTextures {
        player_one: player1_texture_handle,
        player_two: player2_texture_handle,
        player_one_dead: player1_dead_texture_handle.clone(),
        player_two_dead: player1_dead_texture_handle.clone(),
        player_one_ghost: player1_ghost_texture_handle,
        player_two_ghost: player2_ghost_texture_handle,
        enemy_zombie: enemy_zombie_texture_handle,
        enemy_zombie_dead: enemy_zombie_dead_texture_handle,
        enemy_skeleton: enemy_skeleton_texture_handle,
        enemy_skeleton_dead: enemy_skeleton_dead_texture_handle,
        bullet: bullet_texture_handle,
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
