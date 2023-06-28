use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

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

pub fn get_game_sound(sound: &str, asset_server: &Res<AssetServer>) -> Handle<AudioSource> {
    asset_server.load(sound)
}

use crate::common::components::{AnimationIndices, AnimationTimer};

/// Continuously animate a sprite sheet
pub fn animate_sprite(
    sprite: &mut TextureAtlasSprite,
    animation_indices: &AnimationIndices,
    animation_timer: &mut AnimationTimer,
    time: &Res<Time>,
) {
    animation_timer.tick(time.delta());
    if animation_timer.just_finished() {
        if sprite.index < animation_indices.first || sprite.index == animation_indices.last {
            sprite.index = animation_indices.first;
        } else {
            sprite.index += 1;
        };
    }
}

/// Single pass through an animated sprite sheet
pub fn animate_sprite_single(
    sprite: &mut TextureAtlasSprite,
    animation_indices: &AnimationIndices,
    animation_timer: &mut AnimationTimer,
    time: &Res<Time>,
) {
    if sprite.index < animation_indices.last {
        animation_timer.0.tick(time.delta());
        if animation_timer.0.finished() {
            sprite.index += 1;
        }
    }
}
