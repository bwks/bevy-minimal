use std::collections::HashSet;

use rand::Rng;

use crate::enemy::components::{Enemy, EnemyDead, EnemyDeadTimer, EnemyDeadToSpawn, EnemyVariant};
use crate::enemy::resources::EnemySpawnTimer;
use crate::enemy::{
    ENEMY1_DEAD_SPRITE, ENEMY1_SPRITE, ENEMY2_DEAD_SPRITE, ENEMY2_SPRITE, NUMBER_OF_ENEMIES,
};
use crate::game;
use crate::game::states::GameState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::player::components::{Lives, Playable, Player, PlayerDeadToSpawn, PlayerVariant};
use crate::player::PLAYER_SIZE;

use crate::score::resources::{PlayerOneScore, PlayerTwoScore};

use crate::common::components::{AnimationIndices, AnimationTimer, Movable, Velocity, Vitality};
use crate::common::resources::{GameAudio, GameTextures};
use crate::common::utils::{animate_sprite, animate_sprite_single};
use crate::common::{BASE_SPEED, SCROLL_X_VELOCITY, SCROLL_Y_VELOCITY, TIME_STEP};

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

        let flip = rng.gen_range(0.0..10.0);
        let enemy_type = match flip > 5.0 {
            true => EnemyVariant::Zombie,
            false => EnemyVariant::Skelton,
        };

        let (enemy_sprite, enemy_texture, animation_indices, flip_x) = match enemy_type {
            EnemyVariant::Zombie => (
                ENEMY1_SPRITE,
                game_textures.enemy_zombie.clone(),
                AnimationIndices {
                    first: 33,
                    last: 42,
                },
                false,
            ),
            EnemyVariant::Skelton => (
                ENEMY2_SPRITE,
                game_textures.enemy_skeleton.clone(),
                AnimationIndices { first: 0, last: 12 },
                true,
            ),
        };

        let animation_timer = Timer::from_seconds(10.0, TimerMode::Repeating);

        let random_width = rng.gen_range(spawn_area_width_start..spawn_area_width_end);
        let random_height = rng.gen_range(spawn_area_height_start..spawn_area_height_end);

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: enemy_texture,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                // transform: Transform::from_scale(Vec3::splat(3.0)),
                transform: Transform {
                    translation: Vec3::new(random_width, random_height, 10.0),
                    rotation: if flip_x {
                        Quat::from_rotation_y(std::f32::consts::PI)
                    } else {
                        Quat::IDENTITY
                    },
                    scale: Vec3::splat(enemy_sprite.scale),
                    ..Default::default()
                },
                ..Default::default()
            },
            Enemy,
            enemy_type,
            animation_indices,
            AnimationTimer(animation_timer.clone()),
            Vitality::Alive,
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
    mut enemy_query: Query<
        (Entity, &Velocity, &mut Transform, &Movable),
        (With<Enemy>, Without<EnemyDead>),
    >,
    player_query: Query<&Transform, (With<Player>, Without<Enemy>)>,
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

    for player_transform in player_query.iter() {
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
                animate_sprite(&mut sprite, &indices, &mut timer, &time)
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
            &PlayerVariant,
            &mut Vitality,
            &mut Lives,
            &Transform,
            &mut Handle<TextureAtlas>,
        ),
        (With<Player>, Without<Enemy>),
    >,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
    game_textures: Res<GameTextures>,
    // score: Res<Score>,
    game_audio: Res<GameAudio>,
    audio: Res<Audio>,
) {
    for enemy_transform in enemy_query.iter() {
        for (player, mut player_vitality, mut player_lives, player_transform, mut sprite_handle) in
            player_query.iter_mut()
        {
            if *player_vitality == Vitality::Alive {
                let distance = player_transform
                    .translation
                    .distance(enemy_transform.translation);
                let player_radius = PLAYER_SIZE.0 / 2.0;
                let enemy_radius = ENEMY1_SPRITE.width / 2.0;
                if distance < player_radius + enemy_radius {
                    // Spawn dead body

                    let player_ghost_sprite_atlas = match player {
                        PlayerVariant::One => game_textures.player_one_ghost.clone(),
                        PlayerVariant::Two => game_textures.player_two_ghost.clone(),
                    };

                    commands.spawn(PlayerDeadToSpawn(Vec3::new(
                        player_transform.translation.x,
                        player_transform.translation.y,
                        0.0,
                    )));

                    *player_vitality = Vitality::Dead;
                    player_lives.count -= 1;
                    *sprite_handle = player_ghost_sprite_atlas;
                    audio.play(game_audio.player_dead.clone());
                    break;
                }
            }
        }
    }
}

pub fn enemy_dead_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<(Entity, &EnemyVariant, &EnemyDeadToSpawn), With<EnemyDead>>,
) {
    for (enemy_dead_entity, enemy_type, enemy_dead_location) in enemy_query.iter() {
        // spawn the dead enemy sprite

        let (enemy_sprite, enemy_texture, dead_enemy_spawn_type, animation_indices, flip_x) =
            match enemy_type {
                EnemyVariant::Zombie => (
                    ENEMY1_DEAD_SPRITE,
                    game_textures.enemy_zombie_dead.clone(),
                    EnemyVariant::Zombie,
                    AnimationIndices {
                        first: 24,
                        last: 30,
                    },
                    false,
                ),
                EnemyVariant::Skelton => (
                    ENEMY2_DEAD_SPRITE,
                    game_textures.enemy_skeleton_dead.clone(),
                    EnemyVariant::Skelton,
                    AnimationIndices { first: 0, last: 14 },
                    true,
                ),
            };

        commands.spawn((
            SpriteSheetBundle {
                texture_atlas: enemy_texture,
                sprite: TextureAtlasSprite::new(animation_indices.first),
                transform: Transform {
                    translation: enemy_dead_location.0,
                    scale: Vec3::splat(enemy_sprite.scale),
                    rotation: if flip_x {
                        Quat::from_rotation_y(std::f32::consts::PI)
                    } else {
                        Quat::IDENTITY
                    },
                    ..Default::default()
                },
                ..Default::default()
            },
            dead_enemy_spawn_type,
            Vitality::Dead,
            EnemyDead,
            AnimationTimer::default(),
            animation_indices,
            Movable { auto_despawn: true },
            Velocity {
                x: SCROLL_X_VELOCITY,
                y: SCROLL_Y_VELOCITY,
            },
        ));

        // despawn the EnemyDeadToSpawn
        commands.entity(enemy_dead_entity).despawn();
    }
}

pub fn enemy_dead_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<
        (
            Entity,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
            &AnimationIndices,
            &Velocity,
            &mut Transform,
            &Movable,
        ),
        With<EnemyDead>,
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (
        dead_enemy_entity,
        mut timer,
        mut sprite,
        animation_indices,
        velocity,
        mut enemy_transform,
        movable,
    ) in enemy_query.iter_mut()
    {
        animate_sprite_single(&mut sprite, &animation_indices, &mut timer, &time);

        let enemy_translation = &mut enemy_transform.translation;
        enemy_translation.x -= velocity.x * TIME_STEP * BASE_SPEED / 2.0 + 1.0;

        if movable.auto_despawn {
            // despawn when out of screen
            let window_margin = -window.width() / 2.0 - 20.0;
            if enemy_translation.x < window_margin {
                commands.entity(dead_enemy_entity).despawn();
            }
        }
    }
}
