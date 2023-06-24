use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct GameTextures {
    pub player_one: Handle<TextureAtlas>,
    pub player_two: Handle<TextureAtlas>,
    pub player_one_dead: Handle<TextureAtlas>,
    pub player_two_dead: Handle<TextureAtlas>,
    pub player_one_ghost: Handle<TextureAtlas>,
    pub player_two_ghost: Handle<TextureAtlas>,
    pub enemy_zombie: Handle<TextureAtlas>,
    pub enemy_zombie_dead: Handle<TextureAtlas>,
    pub enemy_skeleton: Handle<TextureAtlas>,
    pub enemy_skeleton_dead: Handle<TextureAtlas>,
    pub bullet: Handle<TextureAtlas>,
}

#[derive(Resource)]
pub struct GameAudio {
    pub player_dead: Handle<AudioSource>,
    pub player_shoot: Handle<AudioSource>,
    pub enemy_dead: Handle<AudioSource>,
}
