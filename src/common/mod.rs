pub mod components;
pub mod resources;
pub mod systems;

use bevy::prelude::*;

pub const TIME_STEP: f32 = 1.0 / 60.0;
pub const BASE_SPEED: f32 = 500.0;

pub struct SpriteSheet<'a> {
    pub file: &'a str,
    pub width: f32,
    pub height: f32,
    pub scale: f32,
    pub columns: usize,
    pub rows: usize,
}

pub fn get_texture_atlas(sprite: SpriteSheet, asset_server: &Res<AssetServer>) -> TextureAtlas {
    let texture_handle = asset_server.load(sprite.file);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(sprite.width, sprite.height),
        sprite.columns,
        sprite.rows,
        None,
        None,
    );
    texture_atlas
}

// pub fn get_game_textures() {
//     let game_textures = GameTextures {
//         player_one: get_game_texture(PLAYER1_SPRITE, &asset_server, texture_atlases),
//         player_two: get_game_texture(PLAYER2_SPRITE, &asset_server, texture_atlases),
//         player_one_dead: get_game_texture(PLAYER1_DEAD_SPRITE, &asset_server, texture_atlases),
//         player_two_dead: get_game_texture(PLAYER2_DEAD_SPRITE, &asset_server, texture_atlases),
//         player_fireball: asset_server.load("fireball-right.png"),
//         enemy_zombie: get_game_texture(ENEMY1_SPRITE, &asset_server, texture_atlases),
//         explosion: asset_server.load("explosion.png"),
//     };
// }
