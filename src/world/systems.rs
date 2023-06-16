use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::Rng;

use crate::common::components::{Movable, Velocity};
use crate::common::{BASE_SPEED, TIME_STEP};
use crate::world::components::Tree;
use crate::world::resources::TreeSpawnTimer;
use crate::world::{NUMBER_OF_TREES, TREE_SCALE, TREE_SPRITE};

pub fn tree_spawn_system_init(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();

    let (spawn_area_width_start, spawn_area_width_end) =
        (-window.width() + 5.0, window.width() / 2.0 - 5.0);
    let (spawn_area_height_start, spawn_area_height_end) =
        (-window.height() + 5.0, window.height() / 2.0 - 5.0);

    // println!("spawn_area_width_start: {}", spawn_area_width_start);
    // println!("spawn_area_width_end: {}", spawn_area_width_end);

    for _ in 0..40 {
        let mut rng = rand::thread_rng();
        let random_width = rng.gen_range(spawn_area_width_start..spawn_area_width_end);
        let random_height = rng.gen_range(spawn_area_height_start..spawn_area_height_end);
        // println!("window_width {}", window.width());
        // println!("window_height {}", window.height());
        // println!("random_width {random_width}");sdwwvj
        // println!("random_height {random_height}");
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(TREE_SPRITE),
                transform: Transform {
                    translation: Vec3::new(random_width, random_height, 0.0),
                    scale: Vec3::new(TREE_SCALE, TREE_SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            Tree {},
            Movable { auto_despawn: true },
            Velocity { x: 0.01, y: 0.01 },
        ));
    }
}

pub fn tree_spawn_system(
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

    for _ in 0..NUMBER_OF_TREES {
        let mut rng = rand::thread_rng();
        let random_width = rng.gen_range(spawn_area_width_start..spawn_area_width_end);
        let random_height = rng.gen_range(spawn_area_height_start..spawn_area_height_end);
        // println!("window_width {}", window.width());
        // println!("window_height {}", window.height());
        // println!("random_width {random_width}");
        // println!("random_height {random_height}");
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load(TREE_SPRITE),
                transform: Transform {
                    translation: Vec3::new(random_width, random_height, 0.0),
                    scale: Vec3::new(TREE_SCALE, TREE_SCALE, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            Tree {},
            Movable { auto_despawn: true },
            Velocity { x: 0.01, y: 0.01 },
        ));
    }
}

pub fn tree_spawn_timer_tick_system(mut tree_spawn_timer: ResMut<TreeSpawnTimer>, time: Res<Time>) {
    tree_spawn_timer.timer.tick(time.delta());
}

pub fn trees_spawn_over_time_system(
    commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    tree_spawn_timer: Res<TreeSpawnTimer>,
) {
    if tree_spawn_timer.timer.finished() {
        tree_spawn_system(commands, window_query, asset_server)
    }
}

pub fn tree_movement_system(
    mut commands: Commands,
    mut tree_query: Query<(Entity, &Velocity, &mut Transform, &Movable), With<Tree>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let window = window_query.get_single().unwrap();

    for (tree_entity, velocity, mut enemy_transform, movable) in tree_query.iter_mut() {
        let enemy_translation = &mut enemy_transform.translation;
        enemy_translation.x -= velocity.x * TIME_STEP * BASE_SPEED / 2.0 + 1.0;

        if movable.auto_despawn {
            // despawn when out of screen
            let window_margin = -window.width() / 2.0 - 20.0;
            if enemy_translation.x < window_margin {
                commands.entity(tree_entity).despawn();
            }
        }
    }
}
