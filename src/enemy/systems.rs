use rand::Rng;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::enemy::components::Enemy;
use crate::enemy::resources::EnemySpawnTimer;
use crate::enemy::{ENEMY_SCALE, ENEMY_SIZE, ENEMY_SPRITE, NUMBER_OF_ENEMIES};

use crate::player::components::{Playable, Player};
use crate::player::PLAYER_SIZE;

use crate::score::resources::Score;

use crate::{Movable, Velocity};
use crate::{BASE_SPEED, TIME_STEP};

pub fn enemy_spawn_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let (spawn_area_width_start, spawn_area_width_end) =
        (window.width() / 2.0, window.width() - window.width() / 8.0);
    let (spawn_area_height_start, spawn_area_height_end) =
        (-window.height() / 2.0 + 50.0, window.height() / 2.0 - 50.0);

    // println!("spawn_area_width_start: {}", spawn_area_width_start);
    // println!("spawn_area_width_end: {}", spawn_area_width_end);

    for _ in 0..NUMBER_OF_ENEMIES {
        let mut rng = rand::thread_rng();
        let random_width = rng.gen_range(spawn_area_width_start..spawn_area_width_end);
        let random_height = rng.gen_range(spawn_area_height_start..spawn_area_height_end);
        // println!("window_width {}", window.width());
        // println!("window_height {}", window.height());
        // println!("random_width {random_width}");
        // println!("random_height {random_height}");
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(ENEMY_SPRITE),
                transform: Transform {
                    translation: Vec3::new(random_width, random_height, 0.0),
                    scale: Vec3::new(ENEMY_SCALE, ENEMY_SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            Enemy {},
            Movable { auto_despawn: true },
            Velocity {
                x: rng.gen_range(0.01..0.1),
                y: rng.gen_range(0.01..0.1),
            },
        ));
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
) {
    if enemy_spawn_timer.timer.finished() {
        enemy_spawn_system(commands, window_query, asset_server)
    }
}

pub fn enemy_movement_system(
    mut commands: Commands,
    mut enemy_query: Query<(Entity, &Velocity, &mut Transform, &Movable), With<Enemy>>,
    player_query: Query<(&Transform, &Playable), Without<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
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

            if movable.auto_despawn {
                // despawn when out of screen
                let window_margin = -window.width() / 2.0 - 20.0;
                if enemy_translation.x < window_margin {
                    commands.entity(enemy_entity).despawn();
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
        // println!("player_entity: {}", player_entity.index());
        for enemy_transform in enemy_query.iter() {
            let distance = player_transform
                .translation
                .distance(enemy_transform.translation);
            let player_radius = PLAYER_SIZE.0 / 2.0;
            let enemy_radius = ENEMY_SIZE.0 / 2.0;
            if distance < player_radius + enemy_radius {
                // println!("Enemy hit player! Game Over!");
                commands.entity(player_entity).despawn();
                println!("Final Score: {}", score.value);
            }
        }
    }
}
