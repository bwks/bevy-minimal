use rand::Rng;

use crate::enemy::components::Enemy;
use crate::enemy::resources::EnemySpawnTimer;
use crate::enemy::{ENEMY_SIZE, ENEMY_SPRITE_1, NUMBER_OF_ENEMIES};
use crate::game::states::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::player::components::{Playable, Player};
use crate::player::PLAYER_SIZE;

use crate::score::resources::Score;

use crate::common::components::{AnimationIndices, AnimationTimer, Movable, Velocity};
use crate::common::{BASE_SPEED, TIME_STEP};

pub fn enemy_spawn_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = window_query.get_single().unwrap();

    let (spawn_area_width_start, spawn_area_width_end) =
        (window.width() / 2.0, window.width() - window.width() / 8.0);
    let (spawn_area_height_start, spawn_area_height_end) =
        (-window.height() / 2.0 + 50.0, window.height() / 2.0 - 50.0);

    for _ in 0..NUMBER_OF_ENEMIES {
        let mut rng = rand::thread_rng();

        let texture_handle = asset_server.load(ENEMY_SPRITE_1.file);
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle,
            Vec2::new(ENEMY_SPRITE_1.width, ENEMY_SPRITE_1.height),
            ENEMY_SPRITE_1.columns,
            ENEMY_SPRITE_1.rows,
            None,
            None,
        );
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        // Use only the subset of sprites in the sheet that make up the run animation
        let animation_indices = AnimationIndices {
            first: 33,
            last: 42,
        };

        let animation_timer = Timer::from_seconds(10.0, TimerMode::Repeating);

        let random_width = rng.gen_range(spawn_area_width_start..spawn_area_width_end);
        let random_height = rng.gen_range(spawn_area_height_start..spawn_area_height_end);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: texture_atlas_handle.clone(),
                sprite: TextureAtlasSprite::new(animation_indices.first),
                // transform: Transform::from_scale(Vec3::splat(3.0)),
                transform: Transform {
                    translation: Vec3::new(random_width, random_height, 10.0),
                    // rotation: Quat::from_rotation_y(std::f32::consts::PI),
                    scale: Vec3::splat(ENEMY_SPRITE_1.scale),
                    ..Default::default()
                },
                ..Default::default()
            },
            animation_indices,
            AnimationTimer(animation_timer.clone()),
            Enemy {},
            Movable { auto_despawn: true },
            Velocity {
                x: rng.gen_range(0.01..0.1),
                y: rng.gen_range(0.01..0.1),
            },
        ));
    }
}

pub fn respawn_enemy_system(
    commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        enemy_spawn_system(commands, window_query, asset_server, texture_atlases)
    }
}

pub fn enemy_spawn_timer_tick_system(
    mut enemy_spawn_timer: ResMut<EnemySpawnTimer>,
    time: Res<Time>,
) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn enemies_spawn_over_time_system(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    if enemy_spawn_timer.timer.finished() {
        enemy_spawn_system(commands, window_query, asset_server, texture_atlases)
    }
}

pub fn enemy_movement_system(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Velocity, &mut Transform, &Movable), With<Enemy>>,
    player_query: Query<(&Transform, &Playable), Without<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut score: ResMut<Score>,
    mut enemy_animate_query: Query<
        (
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<Enemy>,
    >,
    time: Res<Time>,
) {
    let window = window_query.get_single().unwrap();

    for (player_transform, _playable) in player_query.iter() {
        let player_y = player_transform.translation.y;

        for (enemy_entity, velocity, mut enemy_transform, movable) in enemy_query.iter_mut() {
            let enemy_translation = &mut enemy_transform.translation;
            enemy_translation.x -= velocity.x * TIME_STEP * BASE_SPEED / 2.0 + 1.0;

            let mut rng = rand::thread_rng();
            let flip = rng.gen_range(0.0..10.0);

            if flip > 5.0 {
                match player_y > enemy_translation.y {
                    true => enemy_translation.y += velocity.y * TIME_STEP * BASE_SPEED / 2.0,
                    false => enemy_translation.y -= velocity.y * TIME_STEP * BASE_SPEED / 2.0,
                }
            }
            for (indices, mut timer, mut sprite) in &mut enemy_animate_query {
                timer.tick(time.delta());
                if timer.just_finished() {
                    sprite.index = if sprite.index == indices.last {
                        indices.first
                    } else {
                        sprite.index + 1
                    };
                }
            }
            if movable.auto_despawn {
                // despawn when out of screen
                let window_margin = -window.width() / 2.0 - 20.0;
                if enemy_translation.x < window_margin {
                    commands.entity(enemy_entity).despawn();
                    if score.value > 0 {
                        score.value -= 1;
                    }
                }
            }
        }
    }
}

pub fn enemy_hit_player_system(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    score: Res<Score>,
) {
    if let Ok((player_entity, player_transform)) = player_query.get_single() {
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE.0 / 2.0;
            let enemy_radius = ENEMY_SIZE.0 / 2.0;
            if distance < player_radius + enemy_radius {
                commands.insert_resource(NextState(Some(GameState::Paused)));
                commands.entity(player_entity).despawn();
                println!("Final Score: {}", score.value);
            }
        }
    }
}
