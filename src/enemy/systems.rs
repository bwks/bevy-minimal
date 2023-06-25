use std::collections::HashSet;

use rand::Rng;

use crate::enemy::bundles::{EnemyBundle, EnemyDeadBundle};
use crate::enemy::components::{Enemy, EnemyDead, EnemyDeadLocation, EnemyVariant};
use crate::enemy::resources::EnemySpawnTimer;
use crate::enemy::{
    ENEMY1_DEAD_SPRITE, ENEMY1_SPRITE, ENEMY2_DEAD_SPRITE, ENEMY2_SPRITE, ENEMY3_DEAD_SPRITE,
    ENEMY3_SPRITE, NUMBER_OF_ENEMIES,
};
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::player::bundles::PlayerDeadLocationBundle;
use crate::player::components::{Lives, Player, PlayerDeadLocation, PlayerVariant};
use crate::player::PLAYER_SIZE;

use crate::score::resources::{PlayerOneScore, PlayerTwoScore};

use crate::common::components::{
    AnimationIndices, AnimationTimer, EntityLocation, Movable, Velocity, Vitality,
};
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

        #[allow(illegal_floating_point_literal_pattern)]
        let enemy_variant = match flip {
            0.0..=3.0 => EnemyVariant::Zombie,
            3.1..=7.0 => EnemyVariant::Skelton,
            7.1..=10.0 => EnemyVariant::Goblin,
            _ => EnemyVariant::Zombie,
        };

        let (enemy_sprite, enemy_texture, animation_indices, flip_x) = match enemy_variant {
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
            EnemyVariant::Goblin => (
                ENEMY3_SPRITE,
                game_textures.enemy_goblin.clone(),
                AnimationIndices { first: 0, last: 7 },
                true,
            ),
        };

        let animation_timer = Timer::from_seconds(10.0, TimerMode::Repeating);

        let random_width = rng.gen_range(spawn_area_width_start..spawn_area_width_end);
        let random_height = rng.gen_range(spawn_area_height_start..spawn_area_height_end);

        commands.spawn(EnemyBundle {
            entity: Enemy,
            variant: enemy_variant,
            vitality: Vitality::Alive,
            animation_indices: animation_indices,
            animation_timer: AnimationTimer(animation_timer.clone()),
            movable: Movable { auto_despawn: true },
            velocity: Velocity {
                x: rng.gen_range(0.01..0.1),
                y: rng.gen_range(0.01..0.1),
            },
            sprite_sheet: SpriteSheetBundle {
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
        });
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
                    let player_ghost_sprite_atlas = match player {
                        PlayerVariant::One => game_textures.player_one_ghost.clone(),
                        PlayerVariant::Two => game_textures.player_two_ghost.clone(),
                    };

                    audio.play(game_audio.player_dead.clone());
                    *player_vitality = Vitality::Dead;
                    player_lives.count -= 1;
                    *sprite_handle = player_ghost_sprite_atlas;

                    // Spawn dead body
                    commands.spawn(PlayerDeadLocationBundle {
                        entity: PlayerDeadLocation,
                        location: EntityLocation {
                            x: player_transform.translation.x,
                            y: player_transform.translation.y,
                            z: 0.0,
                        },
                    });

                    break;
                }
            }
        }
    }
}

pub fn enemy_dead_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<(Entity, &EnemyVariant, &EntityLocation), With<EnemyDeadLocation>>,
) {
    for (enemy_dead_entity, enemy_type, enemy_dead_location) in enemy_query.iter() {
        // spawn the dead enemy sprite

        let (
            enemy_sprite,
            enemy_texture,
            dead_enemy_variant,
            animation_indices,
            animation_timer,
            flip_x,
        ) = match enemy_type {
            EnemyVariant::Zombie => (
                ENEMY1_DEAD_SPRITE,
                game_textures.enemy_zombie_dead.clone(),
                EnemyVariant::Zombie,
                AnimationIndices {
                    first: 24,
                    last: 30,
                },
                AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
                false,
            ),
            EnemyVariant::Skelton => (
                ENEMY2_DEAD_SPRITE,
                game_textures.enemy_skeleton_dead.clone(),
                EnemyVariant::Skelton,
                AnimationIndices { first: 0, last: 14 },
                AnimationTimer(Timer::from_seconds(0.03, TimerMode::Repeating)),
                true,
            ),
            EnemyVariant::Goblin => (
                ENEMY3_DEAD_SPRITE,
                game_textures.enemy_goblin_dead.clone(),
                EnemyVariant::Goblin,
                AnimationIndices { first: 0, last: 3 },
                AnimationTimer::default(),
                true,
            ),
        };

        commands.spawn({
            EnemyDeadBundle {
                entity: EnemyDead,
                variant: dead_enemy_variant,
                animation_indices: animation_indices,
                animation_timer: animation_timer,
                movable: Movable { auto_despawn: true },
                velocity: Velocity {
                    x: SCROLL_X_VELOCITY,
                    y: SCROLL_Y_VELOCITY,
                },
                sprite_sheet: SpriteSheetBundle {
                    texture_atlas: enemy_texture,
                    sprite: TextureAtlasSprite::new(animation_indices.first),
                    transform: Transform {
                        translation: Vec3::new(
                            enemy_dead_location.x,
                            enemy_dead_location.y,
                            enemy_dead_location.z,
                        ),
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
            }
        });

        // despawn the EnemyDeadToSpawn
        commands.entity(enemy_dead_entity).despawn();
    }
}

pub fn enemy_dead_movement_system(
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
