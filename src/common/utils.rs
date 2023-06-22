use bevy::prelude::*;

use crate::common::SpriteSheet;

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
