use std::collections::HashSet;

use rand::Rng;

use crate::enemy::components::{Enemy, EnemyDead, EnemyDeadTimer, EnemyDeadToSpawn};
use crate::enemy::resources::EnemySpawnTimer;
use crate::enemy::{ENEMY1_SPRITE, ENEMY_SIZE, NUMBER_OF_ENEMIES};
use crate::game::states::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::player::components::{Lives, Playable, Player, PlayerState};
use crate::player::PLAYER_SIZE;

use crate::score::resources::{PlayerOneScore, PlayerTwoScore};

use crate::common::components::{AnimationIndices, AnimationTimer, Movable, Velocity};
use crate::common::resources::GameTextures;
use crate::common::{BASE_SPEED, TIME_STEP};

pub fn enemy_spawn_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_textures: Res<GameTextures>,
) {
    let window = window_query.get_single().unwrap();

    let (spawn_area_width_start, spawn_area_width_end) =
        (window.width() / 2.0, window.width() - window.width() / 8.0);
    let (spawn_area_height_start, spawn_area_height_end) =
        (-window.height() / 2.0 + 50.0, window.height() / 2.0 - 50.0);

    for _ in 0..NUMBER_OF_ENEMIES {
        let mut rng = rand::thread_rng();

        let animation_indices = AnimationIndices {
            first: 33,
            last: 42,
        };

        let animation_timer = Timer::from_seconds(10.0, TimerMode::Repeating);

        let random_width = rng.gen_range(spawn_area_width_start..spawn_area_width_end);
        let random_height = rng.gen_range(spawn_area_height_start..spawn_area_height_end);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: game_textures.enemy_zombie.clone(),
                sprite: TextureAtlasSprite::new(animation_indices.first),
                // transform: Transform::from_scale(Vec3::splat(3.0)),
                transform: Transform {
                    translation: Vec3::new(random_width, random_height, 10.0),
                    // rotation: Quat::from_rotation_y(std::f32::consts::PI),
                    scale: Vec3::splat(ENEMY1_SPRITE.scale),
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
    window_query: Query<&Window, With<PrimaryWindow>>,
    game_textures: Res<GameTextures>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        enemy_spawn_system(commands, window_query, game_textures)
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
    enemy_spawn_timer: Res<EnemySpawnTimer>,
    game_textures: Res<GameTextures>,
) {
    if enemy_spawn_timer.timer.finished() {
        enemy_spawn_system(commands, window_query, game_textures)
    }
}

pub fn enemy_movement_system(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Velocity, &mut Transform, &Movable), With<Enemy>>,
    player_query: Query<(&Transform, &Playable), Without<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_one_score: ResMut<PlayerOneScore>,
    mut player_two_score: ResMut<PlayerTwoScore>,
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
    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    for (player_transform, _playable) in player_query.iter() {
        let player_y = player_transform.translation.y;

        for (enemy_entity, velocity, mut enemy_transform, movable) in enemy_query.iter_mut() {
            if despawned_entities.contains(&enemy_entity) {
                continue;
            }

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
                    despawned_entities.insert(enemy_entity);

                    if player_one_score.value > 0 {
                        player_one_score.value -= 1;
                    }
                    if player_two_score.value > 0 {
                        player_two_score.value -= 1;
                    }
                }
            }
        }
    }
}

pub fn enemy_hit_player_system(
    mut commands: Commands,
    mut player_query: Query<
        (
            &Player,
            &mut PlayerState,
            &mut Lives,
            &Transform,
            &mut Handle<TextureAtlas>,
        ),
        With<Playable>,
    >,
    enemy_query: Query<&Transform, With<Enemy>>,
    game_textures: Res<GameTextures>,
    // score: Res<Score>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if player_query.iter().len() == 0 {
        commands.insert_resource(NextState(Some(GameState::Paused)));
    }

    for (player, mut player_state, mut player_lives, player_transform, mut sprite_handle) in
        player_query.iter_mut()
    {
        if *player_state == PlayerState::Alive {
            let player_dead_sprite_atlas = match player {
                Player::One => game_textures.player_one_dead.clone(),
                Player::Two => game_textures.player_two_dead.clone(),
            };

            for enemy_transform in enemy_query.iter() {
                let distance = player_transform
                    .translation
                    .distance(enemy_transform.translation);
                let player_radius = PLAYER_SIZE.0 / 2.0;
                let enemy_radius = ENEMY_SIZE.0 / 2.0;
                if distance < player_radius + enemy_radius {
                    *player_state = PlayerState::Dead;
                    player_lives.count -= 1;
                    *sprite_handle = player_dead_sprite_atlas;
                    let dead_sound = asset_server.load("dead.ogg");
                    audio.play(dead_sound);
                    break;
                }
            }
        }
    }
}

pub fn enemy_dead_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    query: Query<(Entity, &EnemyDeadToSpawn)>,
) {
    for (explosion_spawn_entity, explosion_to_spawn) in query.iter() {
        // spawn the explosion sprite
        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: game_textures.enemy_zombie_dead.clone(),
                sprite: TextureAtlasSprite::new(24),
                transform: Transform {
                    translation: explosion_to_spawn.0,
                    scale: Vec3::splat(3.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            EnemyDead,
            EnemyDeadTimer::default(),
            Movable { auto_despawn: true },
            Velocity { x: 0.01, y: 0.01 },
        ));

        // despawn the EnemyDeadToSpawn
        commands.entity(explosion_spawn_entity).despawn();
    }
}

pub fn enemy_dead_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut EnemyDeadTimer,
            &mut TextureAtlasSprite,
            &Velocity,
            &mut Transform,
            &Movable,
        ),
        With<EnemyDead>,
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (entity, mut timer, mut sprite, velocity, mut enemy_transform, movable) in query.iter_mut()
    {
        timer.0.tick(time.delta());
        if timer.0.finished() {
            sprite.index += 1; // move to next sprite cell
            if sprite.index >= 30 {
                sprite.index = 30;
                // commands.entity(entity).despawn()
            }
        }
        let enemy_translation = &mut enemy_transform.translation;
        enemy_translation.x -= velocity.x * TIME_STEP * BASE_SPEED / 2.0 + 1.0;

        if movable.auto_despawn {
            // despawn when out of screen
            let window_margin = -window.width() / 2.0 - 20.0;
            if enemy_translation.x < window_margin {
                commands.entity(entity).despawn();
            }
        }
    }
}
