use rand::Rng;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::common::components::AnimationIndices;
use crate::common::resources::GameTextures;

use crate::power_up::components::Diamond;
use crate::power_up::resources::DiamondSpawnTimer;

use crate::DIAMOND_SPRITE;

pub fn diamond_spawn_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    diamond_spawn_timer: Res<DiamondSpawnTimer>,
    game_textures: Res<GameTextures>,
) {
    if diamond_spawn_timer.timer.finished() {
        let window = window_query.get_single().unwrap();

        let (spawn_area_width_start, spawn_area_width_end) =
            (-window.width() / 2.0, window.width() / 2.0);
        let (spawn_area_height_start, spawn_area_height_end) =
            (-window.height() / 2.0, window.height() / 2.0);

        let mut rng = rand::thread_rng();

        let random_width = rng.gen_range(spawn_area_width_start..spawn_area_width_end);
        let random_height = rng.gen_range(spawn_area_height_start..spawn_area_height_end);

        let animation_indices = AnimationIndices { first: 0, last: 8 };

        commands.spawn((
            Diamond,
            SpriteSheetBundle {
                texture_atlas: game_textures.diamond.clone(),
                sprite: TextureAtlasSprite::new(animation_indices.first),
                // transform: Transform::from_scale(Vec3::splat(3.0)),
                transform: Transform {
                    translation: Vec3::new(random_width, random_height, 1.0),
                    scale: Vec3::splat(DIAMOND_SPRITE.scale),
                    ..Default::default()
                },
                ..Default::default()
            },
            animation_indices,
        ));
    }
}

pub fn diamond_spawn_timer_tick_system(
    mut diamond_spawn_timer: ResMut<DiamondSpawnTimer>,
    time: Res<Time>,
) {
    diamond_spawn_timer.timer.tick(time.delta());
}
