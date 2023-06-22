use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameTextures {
    pub player_one: Handle<TextureAtlas>,
    pub player_two: Handle<TextureAtlas>,
    pub player_one_dead: Handle<TextureAtlas>,
    pub player_two_dead: Handle<TextureAtlas>,
    pub player_fireball: Handle<Image>,
    pub enemy_zombie: Handle<TextureAtlas>,
    pub enemy_zombie_dead: Handle<TextureAtlas>,
    pub explosion: Handle<TextureAtlas>,
}
