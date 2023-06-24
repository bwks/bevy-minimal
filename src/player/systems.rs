use std::collections::HashSet;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;

use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::InputManagerBundle;

use crate::common::components::{AnimationIndices, AnimationTimer, EntityDeadLocation, Vitality};
use crate::common::resources::{GameAudio, GameTextures};
use crate::common::utils::{animate_sprite, animate_sprite_single};
use crate::common::{SCROLL_X_VELOCITY, SCROLL_Y_VELOCITY};

use crate::player::actions::ControlAction;
use crate::player::bundles::{PlayerBundle, PlayerDeadBundle};
use crate::player::components::{Fireball, Lives, Player, PlayerDead, PlayerVariant};
use crate::player::{
    PLAYER1_SPRITE, PLAYER2_SPRITE, PLAYER_FIREBALL_SCALE, PLAYER_FIREBALL_SIZE, PLAYER_SCALE,
    PLAYER_SIZE, PLAYER_SPEED,
};

use crate::enemy::components::{Enemy, EnemyDead, EnemyVariant};
use crate::enemy::ENEMY1_SPRITE;
use crate::score::resources::{PlayerOneScore, PlayerTwoScore};

use crate::common::components::{Movable, Velocity};
use crate::common::{BASE_SPEED, TIME_STEP};

pub fn player_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    // Use only the subset of sprites in the sheet that make up the run animation
    let animation_indices = AnimationIndices {
        first: 10,
        last: 13,
    };

    // Player 1
    commands.spawn(PlayerBundle {
        player: Player,
        variant: PlayerVariant::One,
        lives: Lives::default(),
        vitality: Vitality::Alive,
        animation_indices: animation_indices,
        animation_timer: AnimationTimer::default(),
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::input_map(PlayerVariant::One),
            ..Default::default()
        },
        sprite_sheet: SpriteSheetBundle {
            texture_atlas: game_textures.player_one.clone(),
            sprite: TextureAtlasSprite::new(11),
            transform: Transform {
                translation: Vec3::new(-window.width() / 4.0, 0.0, 10.0),
                scale: Vec3::splat(PLAYER1_SPRITE.scale),
                ..Default::default()
            },
            ..Default::default()
        },
    });

    // Player 2
    commands.spawn(PlayerBundle {
        player: Player,
        variant: PlayerVariant::Two,
        lives: Lives::default(),
        vitality: Vitality::Alive,
        animation_indices: animation_indices,
        animation_timer: AnimationTimer::default(),
        input_manager: InputManagerBundle {
            input_map: PlayerBundle::input_map(PlayerVariant::Two),
            ..Default::default()
        },
        sprite_sheet: SpriteSheetBundle {
            texture_atlas: game_textures.player_two.clone(),
            sprite: TextureAtlasSprite::new(11),
            transform: Transform {
                translation: Vec3::new(-window.width() / 4.0 + 50.0, 10.0, 10.0),
                scale: Vec3::splat(PLAYER2_SPRITE.scale),
                ..Default::default()
            },
            ..Default::default()
        },
    });
}

pub fn player_respawn_system(
    mut player_query: Query<
        (
            &PlayerVariant,
            &mut Vitality,
            &Lives,
            &ActionState<ControlAction>,
            &mut Handle<TextureAtlas>,
        ),
        With<Player>,
    >,
    keyboard_input: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
) {
    for (player, mut player_state, player_lives, controller_input, mut sprite_handle) in
        player_query.iter_mut()
    {
        if *player_state == Vitality::Dead && player_lives.count > 0 {
            if keyboard_input.just_pressed(KeyCode::R)
                || controller_input.just_pressed(ControlAction::Restart)
            {
                let player_sprite_atlas = match player {
                    PlayerVariant::One => game_textures.player_one.clone(),
                    PlayerVariant::Two => game_textures.player_two.clone(),
                };
                *player_state = Vitality::Alive;
                *sprite_handle = player_sprite_atlas;
            }
        }
    }
}

pub fn player_fire_system(
    mut commands: Commands,
    player_query: Query<(&Transform, &ActionState<ControlAction>, &Vitality), With<Player>>,
    game_textures: Res<GameTextures>,
    game_audio: Res<GameAudio>,
    audio: Res<Audio>,
) {
    for (player_transform, player_fire_action, player_state) in player_query.iter() {
        if *player_state == Vitality::Alive && player_fire_action.just_pressed(ControlAction::Fire)
        {
            let (player_x, player_y) = (
                player_transform.translation.x,
                player_transform.translation.y,
            );
            let x_offset = PLAYER_SIZE.0 / 2.0 * PLAYER_SCALE + 10.0;

            commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: game_textures.bullet.clone(),
                    sprite: TextureAtlasSprite::new(15),
                    transform: Transform {
                        translation: Vec3::new(player_x + x_offset, player_y, 1.0),
                        scale: Vec3::splat(3.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Fireball {},
                Movable { auto_despawn: true },
                Velocity { x: 1.0, y: 0.0 },
            ));
            audio.play(game_audio.player_shoot.clone());
        }
    }
}

pub fn player_movement_system(
    mut player_query: Query<
        (
            &mut Transform,
            &ActionState<ControlAction>,
            &AnimationIndices,
            &mut AnimationTimer,
            &mut TextureAtlasSprite,
        ),
        With<Player>,
    >,

    time: Res<Time>,
) {
    for (mut player_transform, player_move_action, indices, mut timer, mut sprite) in
        player_query.iter_mut()
    {
        let mut direction = Vec3::ZERO;
        for input_direction in ControlAction::PLAYER_MOVE {
            if player_move_action.pressed(input_direction) {
                match input_direction {
                    ControlAction::Up => direction += Vec3::new(0.0, 1.0, 0.0),
                    ControlAction::Down => direction += Vec3::new(0.0, -1.0, 0.0),
                    ControlAction::Left => direction += Vec3::new(-1.0, 0.0, 0.0),
                    ControlAction::Right => direction += Vec3::new(1.0, 0.0, 0.0),
                    ControlAction::AxisMove => {
                        // Each action has a button-like state of its own that you can check
                        // We're working with gamepads, so we want to defensively ensure that we're using the clamped values

                        if let Some(axis_move) =
                            player_move_action.axis_pair(ControlAction::AxisMove)
                        {
                            direction += Vec3::new(axis_move.x(), axis_move.y(), 0.0);
                        }
                    }

                    _ => {}
                }
            }
        }

        if direction.length() > 0.0 {
            animate_sprite(&mut sprite, &indices, &mut timer, &time)
        } else {
            sprite.index = 11;
        }

        player_transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
    }
}

pub fn player_confinement_system(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    // if let Ok(mut player_transform) = player_query.get_single_mut() {
    for mut player_transform in player_query.iter_mut() {
        let window = window_query.get_single().unwrap();

        let left_window_edge = -window.width() / 2.0 + PLAYER_SIZE.0 + 5.0;
        let right_window_edge = window.width() / 2.0 - PLAYER_SIZE.0 - 5.0;
        let bottom_window_edge = -window.height() / 2.0 + PLAYER_SIZE.1 + 5.0;
        let top_window_edge = window.height() / 2.0 - PLAYER_SIZE.1 - 5.0;

        // let mut translation = player_transform.translation;
        let mut player_x = player_transform.translation.x;
        let mut player_y = player_transform.translation.y;

        // Bound the player x position
        if player_x < left_window_edge {
            player_x = left_window_edge;
        } else if player_x > right_window_edge {
            player_x = right_window_edge;
        }
        // Bound the players y position.
        if player_y < bottom_window_edge {
            player_y = bottom_window_edge;
        } else if player_y > top_window_edge {
            player_y = top_window_edge;
        }

        player_transform.translation.x = player_x;
        player_transform.translation.y = player_y;
    }
}

pub fn fireball_movement_system(
    mut commands: Commands,
    mut entity_query: Query<(Entity, &Velocity, &mut Transform, &Movable), With<Fireball>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();
    for (entity, velocity, mut transform, movable) in entity_query.iter_mut() {
        let translation = &mut transform.translation;
        translation.x += velocity.x * TIME_STEP * BASE_SPEED + 0.5;

        if movable.auto_despawn {
            // despawn when out of screen
            let window_margin = window.width() / 2.0;
            if translation.x > window_margin {
                commands.entity(entity).despawn();
            }
        }
    }
}

pub fn player_fireball_hit_enemy_system(
    mut commands: Commands,
    fireball_query: Query<(Entity, &Transform), With<Fireball>>,
    mut enemy_query: Query<(Entity, &EnemyVariant, &Vitality, &Transform), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_one_score: ResMut<PlayerOneScore>,
    game_audio: Res<GameAudio>,
    audio: Res<Audio>,
) {
    let window = window_query.get_single().unwrap();
    let world_right_edge = window.width() / 2.0 - 20.0;

    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // iterate through the lasers
    for (fireball_entity, fireball_transform) in fireball_query.iter() {
        // iterate through the enemies
        for (enemy_entity, enemy_type, enemy_vitality, enemy_transform) in enemy_query.iter_mut() {
            if despawned_entities.contains(&enemy_entity)
                || despawned_entities.contains(&fireball_entity)
                || enemy_vitality == &Vitality::Dead
            {
                continue;
            }

            // let enemy_scale = enemy_transform.scale.xy();

            // determine if collision
            let collision = collide(
                fireball_transform.translation,
                Vec2::new(
                    PLAYER_FIREBALL_SIZE.0 * PLAYER_FIREBALL_SCALE,
                    PLAYER_FIREBALL_SIZE.1 * PLAYER_FIREBALL_SCALE,
                ),
                enemy_transform.translation,
                Vec2::new(ENEMY1_SPRITE.width, ENEMY1_SPRITE.height),
            );

            // if enemy has entered the screen
            if enemy_transform.translation.x < world_right_edge {
                // perform collision
                if collision.is_some() {
                    // remove the enemy

                    audio.play(game_audio.enemy_dead.clone());

                    commands.entity(enemy_entity).despawn();
                    despawned_entities.insert(enemy_entity);

                    // remove the laser
                    commands.entity(fireball_entity).despawn();
                    despawned_entities.insert(fireball_entity);

                    let dead_enemy_spawn_type = match enemy_type {
                        &EnemyVariant::Skelton => EnemyVariant::Skelton,
                        &EnemyVariant::Zombie => EnemyVariant::Zombie,
                    };

                    commands.spawn((
                        EntityDeadLocation(Vec3::new(
                            enemy_transform.translation.x,
                            enemy_transform.translation.y,
                            0.0,
                        )),
                        EnemyDead,
                        dead_enemy_spawn_type,
                    ));

                    // update score
                    player_one_score.value += 1;

                    break;
                }
            }
        }
    }
}

pub fn player_dead_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    enemy_query: Query<(Entity, &EntityDeadLocation)>,
) {
    for (player_dead_entity, player_dead_location) in enemy_query.iter() {
        // spawn the dead enemy sprite
        commands.spawn(PlayerDeadBundle {
            player_dead: PlayerDead,
            animation_indices: AnimationIndices {
                first: 10,
                last: 13,
            },
            animation_timer: AnimationTimer::default(),
            movable: Movable { auto_despawn: true },
            velocity: Velocity {
                x: SCROLL_X_VELOCITY,
                y: SCROLL_Y_VELOCITY,
            },
            sprite_sheet: SpriteSheetBundle {
                texture_atlas: game_textures.player_one_dead.clone(),
                sprite: TextureAtlasSprite::new(11),
                transform: Transform {
                    translation: player_dead_location.0,
                    scale: Vec3::splat(3.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        });
        // despawn the PlayerDeadToSpawn
        commands.entity(player_dead_entity).despawn();
    }
}

pub fn player_dead_animation_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player_dead_query: Query<
        (
            Entity,
            &mut AnimationTimer,
            &AnimationIndices,
            &mut TextureAtlasSprite,
            &Velocity,
            &mut Transform,
            &Movable,
        ),
        With<PlayerDead>,
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (
        player_dead_entity,
        mut player_dead_animation_timer,
        player_dead_animation_indices,
        mut player_dead_sprite,
        player_dead_velocity,
        mut player_dead_transform,
        player_dead_movable,
    ) in player_dead_query.iter_mut()
    {
        if player_dead_sprite.index < player_dead_animation_indices.last {
            animate_sprite_single(
                &mut player_dead_sprite,
                &player_dead_animation_indices,
                &mut player_dead_animation_timer,
                &time,
            )
        }
        let player_dead_translation = &mut player_dead_transform.translation;
        player_dead_translation.x -= player_dead_velocity.x * TIME_STEP * BASE_SPEED / 2.0 + 1.0;

        if player_dead_movable.auto_despawn {
            // despawn when out of screen
            let window_margin = -window.width() / 2.0 - 20.0;
            if player_dead_translation.x < window_margin {
                commands.entity(player_dead_entity).despawn();
            }
        }
    }
}
