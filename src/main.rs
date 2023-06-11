use std::collections::HashSet;

use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::window::PrimaryWindow;

use leafwing_input_manager::orientation::Direction;
use leafwing_input_manager::prelude::*;

use rand::Rng;

const TIME_STEP: f32 = 1.0 / 60.0;
const BASE_SPEED: f32 = 500.0;

pub const ENEMY_SPAWN_TIME: f32 = 2.0;
pub const ENEMY_SPRITE: &str = "zombie.png";
pub const ENEMY_SIZE: (f32, f32) = (48.0, 92.0); // (x, y)
pub const ENEMY_SCALE: f32 = 1.0;
pub const NUMBER_OF_ENEMIES: u8 = 10;

pub const PLAYER_SPEED: f32 = 500.0;
pub const PLAYER_SPRITE: &str = "purplecloakwizard.png";
pub const PLAYER_SIZE: (f32, f32) = (17.0, 30.0); // (x, y)
pub const PLAYER_SCALE: f32 = 2.0;

pub const PLAYER_FIRE_KEY: KeyCode = KeyCode::J;
pub const FIREBALL_SPRITE: &str = "fireball-right.png";
pub const FIREBALL_SIZE: (f32, f32) = (75.0, 47.0);
pub const FIREBALL_SCALE: f32 = 0.3;

#[derive(Actionlike, PartialEq, Eq, Clone, Copy, Hash, Debug)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    AxisMove,
    Fire,
}

impl Action {
    // Lists like this can be very useful for quickly matching subsets of actions
    pub const PLAYER_MOVE_KEYS: [Self; 5] = [
        Self::Left,
        Self::Right,
        Self::Up,
        Self::Down,
        Self::AxisMove,
    ];

    pub fn direction(self) -> Option<Direction> {
        match self {
            Self::Up => Some(Direction::NORTH),
            Self::Down => Some(Direction::SOUTH),
            Self::Left => Some(Direction::WEST),
            Self::Right => Some(Direction::EAST),
            _ => None,
        }
    }
}

#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    // This bundle must be added to your player entity
    // (or whatever else you wish to control)
    #[bundle]
    input_manager: InputManagerBundle<Action>,
}

impl PlayerBundle {
    fn default_input_map() -> InputMap<Action> {
        // This allows us to replace `Action::Up` with `Up`,
        // significantly reducing boilerplate
        let mut input_map = InputMap::default();

        // Axis joystick
        input_map.insert(DualAxis::left_stick(), Action::AxisMove);

        // D-Pad
        input_map.insert(GamepadButtonType::DPadUp, Action::Up);
        input_map.insert(GamepadButtonType::DPadDown, Action::Down);
        input_map.insert(GamepadButtonType::DPadLeft, Action::Left);
        input_map.insert(GamepadButtonType::DPadRight, Action::Right);

        // Keyboard
        input_map.insert(KeyCode::Up, Action::Up);
        input_map.insert(KeyCode::W, Action::Up);
        input_map.insert(KeyCode::Down, Action::Down);
        input_map.insert(KeyCode::S, Action::Down);
        input_map.insert(KeyCode::Left, Action::Left);
        input_map.insert(KeyCode::A, Action::Left);
        input_map.insert(KeyCode::Right, Action::Right);
        input_map.insert(KeyCode::D, Action::Right);

        // Abilities
        input_map.insert(KeyCode::J, Action::Fire);
        input_map.insert(GamepadButtonType::South, Action::Fire);
        input_map.insert(MouseButton::Left, Action::Fire);

        input_map
    }
}

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Playable;

#[derive(Component)]
pub struct Fireball;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct Movable {
    pub auto_despawn: bool,
}

#[derive(Resource)]
pub struct EnemySpawnTimer {
    pub timer: Timer,
}

impl Default for EnemySpawnTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(ENEMY_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

pub fn spawn_camera(
    mut commands: Commands,
    // _window_query: Query<&Window, With<PrimaryWindow>>
) {
    // let window = window_query.get_single().unwrap();

    // commands.spawn(Camera2dBundle {
    //     transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
    //     ..default()
    // });
    commands.spawn(Camera2dBundle::default());
}

fn enemy_spawn_system(
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

pub fn enemy_spawn_timer_tick(mut enemy_spawn_timer: ResMut<EnemySpawnTimer>, time: Res<Time>) {
    enemy_spawn_timer.timer.tick(time.delta());
}

pub fn enemies_spawn_over_time(
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

pub fn enemy_hit_player(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
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
            }
        }
    }
}

fn player_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
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

fn player_fire_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    player_query: Query<&Transform, With<Player>>,
    player_fire_query: Query<&ActionState<Action>, With<Player>>,
) {
    if let Ok(player_tf) = player_query.get_single() {
        let payer_fire_action = player_fire_query.single();

        if payer_fire_action.just_pressed(Action::Fire) {
            let (player_x, player_y) = (player_tf.translation.x, player_tf.translation.y);
            let x_offset = PLAYER_SIZE.0 / 2.0 * PLAYER_SCALE + 10.0;

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(FIREBALL_SPRITE),
                    transform: Transform {
                        scale: Vec3::new(FIREBALL_SCALE, FIREBALL_SCALE, 1.0),
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
    player_move_query: Query<&ActionState<Action>, With<Player>>,
    time: Res<Time>,
) {
    for player_move_action in player_move_query.iter() {
        if let Ok(mut transform) = player_query.get_single_mut() {
            let mut direction = Vec3::ZERO;

            for input_direction in Action::PLAYER_MOVE_KEYS {
                if player_move_action.pressed(input_direction) {
                    match input_direction {
                        Action::Up => direction += Vec3::new(0.0, 1.0, 0.0),
                        Action::Down => direction += Vec3::new(0.0, -1.0, 0.0),
                        Action::Left => direction += Vec3::new(-1.0, 0.0, 0.0),
                        Action::Right => direction += Vec3::new(1.0, 0.0, 0.0),
                        Action::AxisMove => {
                            // Each action has a button-like state of its own that you can check
                            // We're working with gamepads, so we want to defensively ensure that we're using the clamped values
                            if let Some(axis_pair) =
                                player_move_action.clamped_axis_pair(Action::AxisMove)
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

fn player_fireball_hit_enemy_system(
    mut commands: Commands,
    fireball_query: Query<(Entity, &Transform), With<Fireball>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
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
                    FIREBALL_SIZE.0 * FIREBALL_SCALE,
                    FIREBALL_SIZE.1 * FIREBALL_SCALE,
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
                    // enemy_count.0 -= 1;

                    // remove the laser
                    commands.entity(fireball_entity).despawn();
                    despawned_entities.insert(fireball_entity);

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

fn main() {
    App::new()
        .init_resource::<EnemySpawnTimer>()
        .add_plugins(DefaultPlugins)
        .add_plugin(InputManagerPlugin::<Action>::default())
        .add_startup_system(spawn_camera)
        .add_startup_system(player_spawn_system)
        .add_startup_system(enemy_spawn_system)
        .add_system(player_movement_system)
        .add_system(player_confinement_system)
        .add_system(player_fire_system)
        .add_system(player_fireball_hit_enemy_system)
        .add_system(fireball_movement_system)
        .add_system(enemy_movement_system)
        .add_system(enemy_spawn_timer_tick)
        .add_system(enemies_spawn_over_time)
        .add_system(enemy_hit_player)
        .run();
}
