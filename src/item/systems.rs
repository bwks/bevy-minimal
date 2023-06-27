use rand::Rng;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::common::components::{AnimationIndices, AnimationTimer, Movable, Velocity};
use crate::common::resources::GameTextures;
use crate::common::utils::animate_sprite;

use crate::item::bundles::PowerUpBundle;
use crate::item::components::{ItemPower, ItemVariant, PowerUp};
use crate::item::resources::{DiamondPowerTimer, DiamondSpawnTimer};

use crate::player::components::Player;
use crate::DIAMOND_SPRITE;

pub fn diamond_spawn_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    diamond_spawn_timer: Res<DiamondSpawnTimer>,
    player_query: Query<&ItemPower, With<Player>>,
    diamond_qeury: Query<&ItemVariant, With<PowerUp>>,
    game_textures: Res<GameTextures>,
) {
    let mut player_powered = false;

    for item_power in player_query.iter() {
        if item_power.diamond {
            player_powered = true;
        }
    }

    let diamond_count = diamond_qeury.iter().count();

    if !player_powered && diamond_count == 0 {
        if diamond_spawn_timer.timer.finished() {
            let window = window_query.get_single().unwrap();

            let (spawn_area_width_start, spawn_area_width_end) =
                (-window.width() / 2.0 + 20.0, window.width() / 2.0 - 20.0);
            let (spawn_area_height_start, spawn_area_height_end) =
                (-window.height() / 2.0 + 20.0, window.height() / 2.0 - 20.0);

            let mut rng = rand::thread_rng();

            let random_width = rng.gen_range(spawn_area_width_start..spawn_area_width_end);
            let random_height = rng.gen_range(spawn_area_height_start..spawn_area_height_end);

            let animation_indices = AnimationIndices { first: 0, last: 9 };

            commands.spawn(PowerUpBundle {
                power_up: PowerUp,
                variant: ItemVariant::Diamond,
                animation_indices: animation_indices,
                animation_timer: AnimationTimer::default(),
                movable: Movable::default(),
                velocity: Velocity::default(),
                sprite_sheet: SpriteSheetBundle {
                    texture_atlas: game_textures.diamond.clone(),
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    transform: Transform {
                        translation: Vec3::new(random_width, random_height, 1.0),
                        scale: Vec3::splat(DIAMOND_SPRITE.scale),
                        ..Default::default()
                    },
                    ..Default::default()
                },
            });
        }
    }
}

pub fn diamond_spawn_timer_tick_system(
    mut diamond_spawn_timer: ResMut<DiamondSpawnTimer>,
    time: Res<Time>,
) {
    diamond_spawn_timer.timer.tick(time.delta());
}

pub fn power_up_animation_system(
    time: Res<Time>,
    mut power_up_query: Query<
        (
            &mut AnimationTimer,
            &AnimationIndices,
            &mut TextureAtlasSprite,
        ),
        With<PowerUp>,
    >,
) {
    for (mut power_up_animation_timer, power_up_animation_indices, mut power_up_sprite) in
        power_up_query.iter_mut()
    {
        animate_sprite(
            &mut power_up_sprite,
            &power_up_animation_indices,
            &mut power_up_animation_timer,
            &time,
        )
    }
}

pub fn diamond_power_timer_tick_system(
    mut diamond_power_timer: ResMut<DiamondPowerTimer>,
    time: Res<Time>,
) {
    diamond_power_timer.timer.tick(time.delta());
}
