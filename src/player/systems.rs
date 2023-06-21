use std::collections::HashSet;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;

use leafwing_input_manager::prelude::ActionState;
use leafwing_input_manager::InputManagerBundle;

use crate::common::components::{AnimationIndices, AnimationTimer};
use crate::common::resources::GameTextures;
use crate::player::actions::ControlAction;
use crate::player::bundles::PlayerBundle;
use crate::player::components::{Fireball, Lives, Playable, Player, PlayerState};
use crate::player::{
    PLAYER1_SPRITE, PLAYER2_SPRITE, PLAYER_FIREBALL_SCALE, PLAYER_FIREBALL_SIZE,
    PLAYER_FIREBALL_SPRITE, PLAYER_SCALE, PLAYER_SIZE, PLAYER_SPEED,
};

use crate::enemy::components::Enemy;
use crate::enemy::ENEMY_SIZE;

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
    let animation_indices = AnimationIndices { first: 8, last: 11 };

    commands.spawn((
        PlayerBundle {
            player: Player::One,
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::input_map(Player::One),
                ..default()
            },
        },
        SpriteSheetBundle {
            texture_atlas: game_textures.player_one.clone(),
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform {
                translation: Vec3::new(-window.width() / 4.0, 0.0, 10.0),
                scale: Vec3::splat(PLAYER1_SPRITE.scale),
                ..Default::default()
            },
            ..Default::default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayerState::Alive,
        Playable {},
        Lives::default(),
    ));

    commands.spawn((
        PlayerBundle {
            player: Player::Two,
            input_manager: InputManagerBundle {
                input_map: PlayerBundle::input_map(Player::Two),
                ..default()
            },
        },
        SpriteSheetBundle {
            texture_atlas: game_textures.player_two.clone(),
            sprite: TextureAtlasSprite::new(animation_indices.first),
            transform: Transform {
                translation: Vec3::new(-window.width() / 4.0 + 50.0, 10.0, 10.0),
                scale: Vec3::splat(PLAYER2_SPRITE.scale),
                ..Default::default()
            },
            ..Default::default()
        },
        animation_indices,
        AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
        PlayerState::Alive,
        Playable {},
        Lives::default(),
    ));
}

pub fn player_respawn_system(
    mut player_query: Query<
        (
            &Player,
            &mut PlayerState,
            &Lives,
            &ActionState<ControlAction>,
            &mut Handle<TextureAtlas>,
        ),
        With<Playable>,
    >,
    keyboard_input: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
) {
    for (player, mut player_state, player_lives, controller_input, mut sprite_handle) in
        player_query.iter_mut()
    {
        if *player_state == PlayerState::Dead && player_lives.count > 0 {
            if keyboard_input.just_pressed(KeyCode::F1)
                || controller_input.just_pressed(ControlAction::Restart)
            {
                let player_sprite_atlas = match player {
                    Player::One => game_textures.player_one.clone(),
                    Player::Two => game_textures.player_two.clone(),
                };
                *player_state = PlayerState::Alive;
                *sprite_handle = player_sprite_atlas;
            }
        }
    }
}

pub fn player_fire_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<(&Transform, &ActionState<ControlAction>, &PlayerState), With<Playable>>,
    _game_textures: Res<GameTextures>,
    audio: Res<Audio>,
) {
    for (player_transform, player_fire_action, player_state) in player_query.iter() {
        if *player_state == PlayerState::Alive
            && player_fire_action.just_pressed(ControlAction::Fire)
        {
            let (player_x, player_y) = (
                player_transform.translation.x,
                player_transform.translation.y,
            );
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
            let fire_sound = asset_server.load("shoot.ogg");
            audio.play(fire_sound);
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
            timer.tick(time.delta());
            if timer.just_finished() {
                sprite.index = if sprite.index < indices.first || sprite.index == indices.last {
                    8
                } else {
                    sprite.index + 1
                };
            }
        } else {
            sprite.index = 0;
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
    mut enemy_query: Query<
        (
            Entity,
            &Transform,
            // &mut Handle<TextureAtlas>,
            // &mut AnimationIndices,
        ),
        With<Enemy>,
    >,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_one_score: ResMut<PlayerOneScore>,
    // asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let window = window_query.get_single().unwrap();
    let world_right_edge = window.width() / 2.0 - 20.0;

    let mut despawned_entities: HashSet<Entity> = HashSet::new();

    // iterate through the lasers
    for (fireball_entity, fireball_transform) in fireball_query.iter() {
        // iterate through the enemies
        // for (enemy_entity, enemy_transform, mut sprite_handle, mut animation_indicies) in enemy_query.iter_mut()
        for (enemy_entity, enemy_transform) in enemy_query.iter_mut() {
            if despawned_entities.contains(&enemy_entity)
                || despawned_entities.contains(&fireball_entity)
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
                Vec2::new(ENEMY_SIZE.0, ENEMY_SIZE.1),
            );

            // if enemy has entered the screen
            if enemy_transform.translation.x < world_right_edge {
                // perform collision
                if collision.is_some() {
                    // remove the enemy

                    // let texture_handle = asset_server.load(EPLOSION_SPRITE.file);
                    // let texture_atlas = TextureAtlas::from_grid(
                    //     texture_handle,
                    //     Vec2::new(EPLOSION_SPRITE.width, EPLOSION_SPRITE.height),
                    //     EPLOSION_SPRITE.columns,
                    //     EPLOSION_SPRITE.rows,
                    //     None,
                    //     None,
                    // );

                    // let enemy_dead_sprite_atlas = texture_atlases.add(texture_atlas);
                    // *animation_indicies = AnimationIndices { first: 0, last: 5 };
                    // *sprite_handle = enemy_dead_sprite_atlas;

                    let zombie_die_sound = asset_server.load("zombie-die.ogg");
                    audio.play(zombie_die_sound);

                    commands.entity(enemy_entity).despawn();
                    despawned_entities.insert(enemy_entity);

                    // remove the laser
                    commands.entity(fireball_entity).despawn();
                    despawned_entities.insert(fireball_entity);

                    // update score
                    player_one_score.value += 1;

                    break;
                }
            }
        }
    }
}
