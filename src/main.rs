pub mod common;
pub mod enemy;
pub mod game;
pub mod player;
pub mod power_up;
pub mod score;
pub mod world;

use bevy::app::AppExit;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::window::{PrimaryWindow, WindowMode};

use enemy::components::Enemy;
use enemy::systems::enemy_spawn_system;
use enemy::EnemyPlugin;
use game::GamePlugin;
use player::components::Player;
use player::systems::player_spawn_system;
use player::PlayerPlugin;
use score::resources::Score;
use score::ScorePlugin;
use world::WorldPlugin;

pub fn exit_game(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::rgb_u8(120, 200, 102)))
        .insert_resource(ClearColor(Color::rgb_u8(85, 128, 0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                mode: WindowMode::BorderlessFullscreen,
                ..Default::default()
            }),
            ..default()
        }))
        .add_state::<AppState>()
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_startup_system(setup)
        // .add_startup_system(setup_animate)
        .add_plugin(GamePlugin)
        .add_plugin(WorldPlugin)
        .add_plugin(ScorePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(EnemyPlugin)
        .add_system(exit_game)
        .add_system(text_update_system)
        .add_system(text_color_system)
        // .add_system(animate_sprite)
        .add_system(toggle_game_state)
        .add_system(restart_game)
        .add_system(respawn_player)
        .add_system(respawn_enemy)
        .run();
}

pub fn restart_game(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::F2) {
        for entity in player_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in enemy_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn respawn_player(
    commands: Commands,
    player_query: Query<Entity, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if keyboard_input.just_pressed(KeyCode::F1) {
        if !player_query.get_single().is_ok() {
            player_spawn_system(commands, asset_server, texture_atlases, window_query);
        }
    }
}

pub fn respawn_enemy(
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

#[derive(States, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    #[default]
    Paused,
    InGame,
    // MainMenu,
}

pub fn toggle_game_state(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    game_state: Res<State<AppState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match game_state.0 {
            AppState::InGame => {
                commands.insert_resource(NextState(Some(AppState::Paused)));
                println!("PAUSING");
            }
            AppState::Paused => {
                commands.insert_resource(NextState(Some(AppState::InGame)));
                println!("RUNNING");
            }
        }
    }
}

// A unit struct to help identify the FPS UI component, since there may be many Text components
#[derive(Component)]
struct FpsText;

// A unit struct to help identify the color-changing Text component
#[derive(Component)]
struct ColorText;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // UI camera
    // commands.spawn(Camera2dBundle::default());
    // Text with one section

    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "Score: 0",
            TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 50.0,
                color: Color::WHITE,
            },
        ) // Set the alignment of the Text
        .with_text_alignment(TextAlignment::Center)
        // Set the style of the TextBundle itself.
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(70.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
    ));

    // Text with multiple sections
    commands.spawn((
        // Create a TextBundle that has a Text with a list of sections.
        TextBundle::from_sections([
            TextSection::new(
                "FPS: ",
                TextStyle {
                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 30.0,
                    color: Color::WHITE,
                },
            ),
            TextSection::from_style(TextStyle {
                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 30.0,
                color: Color::GOLD,
            }),
        ]),
        FpsText,
    ));
}

fn text_color_system(
    score: Res<Score>,
    time: Res<Time>,
    mut query: Query<&mut Text, With<ColorText>>,
) {
    for mut text in &mut query {
        let seconds = time.elapsed_seconds();

        // Update the color of the first and only section.
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };
        if score.is_changed() {
            text.sections[0].value = format!("Score {}", score.value)
        }
    }
}

fn text_update_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}
