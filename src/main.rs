pub mod common;
pub mod enemy;
pub mod game;
pub mod player;
pub mod power_up;
pub mod score;
pub mod world;

use bevy::prelude::*;
use bevy::window::WindowMode;

use common::resources::{GameAudio, GameTextures};
use common::utils::{get_game_sound, get_texture_atlas};

use power_up::DIAMOND_SPRITE;

use enemy::EnemyPlugin;
use enemy::{
    ENEMY1_DEAD_SPRITE, ENEMY1_SPRITE, ENEMY2_DEAD_SPRITE, ENEMY2_SPRITE, ENEMY3_DEAD_SPRITE,
    ENEMY3_SPRITE,
};
use game::GamePlugin;
use player::{
    PlayerPlugin, BULLET_SPRITE, PLAYER1_DEAD_SPRITE, PLAYER1_GHOST_SPRITE, PLAYER1_SPRITE,
    PLAYER2_GHOST_SPRITE, PLAYER2_SPRITE,
};

use power_up::PowerUpPlugin;
use score::ScorePlugin;
use world::WorldPlugin;

pub fn setup_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    // Game Textures
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

    let enemy_goblin_texture_atlas = get_texture_atlas(ENEMY3_SPRITE, &asset_server);
    let enemy_goblin_texture_handle = texture_atlases.add(enemy_goblin_texture_atlas);

    let enemy_goblin_dead_texture_atlas = get_texture_atlas(ENEMY3_DEAD_SPRITE, &asset_server);
    let enemy_goblin_dead_texture_handle = texture_atlases.add(enemy_goblin_dead_texture_atlas);

    let bullet_texture_atlas = get_texture_atlas(BULLET_SPRITE, &asset_server);
    let bullet_texture_handle = texture_atlases.add(bullet_texture_atlas);

    let diamond_texture_atlas = get_texture_atlas(DIAMOND_SPRITE, &asset_server);
    let diamond_texture_handle = texture_atlases.add(diamond_texture_atlas);

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
        enemy_goblin: enemy_goblin_texture_handle,
        enemy_goblin_dead: enemy_goblin_dead_texture_handle,
        bullet: bullet_texture_handle,
        diamond: diamond_texture_handle,
    };
    commands.insert_resource(game_textures);

    // Game Sounds
    let player_dead_sound = get_game_sound("dead.ogg", &asset_server);
    let player_shoot_sound = get_game_sound("shoot.ogg", &asset_server);
    let enemy_dead_sound = get_game_sound("zombie-die.ogg", &asset_server);

    let game_sounds = GameAudio {
        player_dead: player_dead_sound,
        player_shoot: player_shoot_sound,
        enemy_dead: enemy_dead_sound,
    };
    commands.insert_resource(game_sounds);
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
        .add_plugin(PowerUpPlugin)
        .add_startup_system(setup_system)
        .run();
}
