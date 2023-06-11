use std::collections::HashSet;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;

use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::InputManagerBundle;

use crate::player::actions::ControlAction;
use crate::player::bundles::PlayerBundle;
use crate::player::components::{Fireball, Playable, Player};
use crate::player::{
    PLAYER_FIREBALL_SCALE, PLAYER_FIREBALL_SIZE, PLAYER_FIREBALL_SPRITE, PLAYER_SCALE, PLAYER_SIZE,
    PLAYER_SPEED, PLAYER_SPRITE,
};

use crate::enemy::components::Enemy;
use crate::enemy::ENEMY_SIZE;

use crate::score::resources::Score;

use crate::common::components::{Movable, Velocity};
use crate::common::{BASE_SPEED, TIME_STEP};

pub fn player_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        PlayerBundle {
            player: Player,
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::default_input_map(),
                ..default()
            },
        },
        SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE),
            transform: Transform {
                scale: Vec3::new(PLAYER_SCALE, PLAYER_SCALE, 1.0),
                ..Default::default()
            },
            ..Default::default()
        },
        Playable {},
    ));
}

pub fn player_fire_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
    player_fire_query: Query<&ActionState<ControlAction>, With<Player>>,
) {
    if let Ok(player_tf) = player_query.get_single() {
        let player_fire_action = player_fire_query.single();

        if player_fire_action.just_pressed(ControlAction::Fire) {
            let (player_x, player_y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 / 2.0 * PLAYER_SCALE + 10.0;

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(PLAYER_FIREBALL_SPRITE),
                    transform: Transform {
                        scale: Vec3::new(PLAYER_FIREBALL_SCALE, PLAYER_FIREBALL_SCALE, 1.0),
                        translation: Vec3::new(player_x + x_offset, player_y, 1.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                Fireball {},
                Movable { auto_despawn: true },
                Velocity { x: 1.0, y: 0.0 },
            ));
        }
    }
}

pub fn player_movement_system(
    mut player_query: Query<&mut Transform, With<Player>>,
    player_move_query: Query<&ActionState<ControlAction>, With<Player>>,
    time: Res<Time>,
) {
    for player_move_action in player_move_query.iter() {
        if let Ok(mut transform) = player_query.get_single_mut() {
            let mut direction = Vec3::ZERO;

            for input_direction in ControlAction::PLAYER_MOVE_KEYS {
                if player_move_action.pressed(input_direction) {
                    match input_direction {
                        ControlAction::Up => direction += Vec3::new(0.0, 1.0, 0.0),
                        ControlAction::Down => direction += Vec3::new(0.0, -1.0, 0.0),
                        ControlAction::Left => direction += Vec3::new(-1.0, 0.0, 0.0),
                        ControlAction::Right => direction += Vec3::new(1.0, 0.0, 0.0),
                        ControlAction::AxisMove => {
                            // Each action has a button-like state of its own that you can check
                            // We're working with gamepads, so we want to defensively ensure that we're using the clamped values
                            if let Some(axis_pair) =
                                player_move_action.clamped_axis_pair(ControlAction::AxisMove)
                            {
                                match axis_pair.x() > 0.0 {
                                    true => direction += Vec3::new(1.0, 0.0, 0.0), // move right
                                    false => direction += Vec3::new(-1.0, 0.0, 0.0), // move left
                                }
                                match axis_pair.y() > 0.0 {
                                    true => direction += Vec3::new(0.0, 1.0, 0.0),   // up
                                    false => direction += Vec3::new(0.0, -1.0, 0.0), // down
                                }
                            }
                        }

                        _ => {}
                    }
                }
            }

            if direction.length() > 0.0 {
                direction = direction.normalize();
            }

            transform.translation += direction * PLAYER_SPEED * time.delta_seconds();
        }
    }
}

pub fn player_confinement_system(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = -window.width() / 2.0 + PLAYER_SIZE.0 + 5.0;
        let x_max = window.width() / 2.0 - PLAYER_SIZE.0 - 5.0;
        let y_min = -window.height() / 2.0 + PLAYER_SIZE.1 + 5.0;
        let y_max = window.height() / 2.0 - PLAYER_SIZE.1 - 5.0;

        let mut translation = player_transform.translation;

        // println!("player_position: {}", translation);

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }
        // Bound the players y position.
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
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
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut score: ResMut<Score>,
) {
    let window = window_query.get_single().unwrap();
    let world_right_edge = window.width() / 2.0 - 20.0;

    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // iterate through the lasers
    for (fireball_entity, fireball_tf) in fireball_query.iter() {
        if despawned_entities.contains(&fireball_entity) {
            continue;
        }

        // let laser_scale = fireball_tf.scale.xy();

        // iterate through the enemies
        for (enemy_entity, enemy_tf) in enemy_query.iter() {
            if despawned_entities.contains(&enemy_entity)
                || despawned_entities.contains(&fireball_entity)
            {
                continue;
            }

            // let enemy_scale = enemy_tf.scale.xy();

            // determine if collision
            let collision = collide(
                fireball_tf.translation,
                Vec2::new(
                    PLAYER_FIREBALL_SIZE.0 * PLAYER_FIREBALL_SCALE,
                    PLAYER_FIREBALL_SIZE.1 * PLAYER_FIREBALL_SCALE,
                ),
                enemy_tf.translation,
                Vec2::new(ENEMY_SIZE.0, ENEMY_SIZE.1),
            );

            // if enemy has entered the screen
            if enemy_tf.translation.x < world_right_edge {
                // perform collision
                if collision.is_some() {
                    // remove the enemy
                    commands.entity(enemy_entity).despawn();
                    despawned_entities.insert(enemy_entity);

                    // remove the laser
                    commands.entity(fireball_entity).despawn();
                    despawned_entities.insert(fireball_entity);

                    // update score
                    score.value += 1;

                    // spawn the explosionToSpawn
                    // commands.spawn(SpriteBundle {
                    //     // transform: Transform::from_xyz(window.width() / 4.0, window.height() / 4.0, 0.0),
                    //     texture: asset_server.load("explosion.png"),
                    //     transform: Transform {
                    //         scale: Vec3::new(0.5, 0.5, 1.0),
                    //         ..Default::default()
                    //     },
                    //     ..Default::default()
                    // });
                }
            }
        }
    }
}
