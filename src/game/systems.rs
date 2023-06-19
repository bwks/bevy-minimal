use bevy::app::AppExit;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::enemy::components::Enemy;
use crate::game::components::{ColorText, FpsText};
use crate::game::states::GameState;
use crate::player::actions::ControlAction;
use crate::player::components::Player;
use crate::score::resources::Score;

pub fn spawn_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn toggle_game_state_system(
    player_query: Query<Entity, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
    controller_query: Query<&ActionState<ControlAction>>,
    game_state: Res<State<GameState>>,
    mut next_app_state: ResMut<NextState<GameState>>,
) {
    if let Ok(_player) = player_query.get_single() {
        let controller_input = controller_query.single();
        if keyboard_input.just_pressed(KeyCode::Space)
            || controller_input.just_pressed(ControlAction::Pause)
        {
            match game_state.0 {
                GameState::Playing => {
                    next_app_state.set(GameState::Paused);
                }
                GameState::Paused => {
                    next_app_state.set(GameState::Playing);
                }
            }
        }
    }
}

pub fn text_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
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

pub fn text_color_system(
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

pub fn text_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    for mut text in &mut query {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(value) = fps.smoothed() {
                // Update the value of the second section
                text.sections[1].value = format!("{value:.2}");
            }
        }
    }
}

pub fn exit_game_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        app_exit_event_writer.send(AppExit);
    }
}

pub fn restart_game_system(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
    enemy_query: Query<Entity, With<Enemy>>,
    keyboard_input: Res<Input<KeyCode>>,
    controller_query: Query<&ActionState<ControlAction>>,
) {
    if let Ok(controller_input) = controller_query.get_single() {
        if controller_input.just_pressed(ControlAction::Restart) {
            if let Ok(player) = player_query.get_single() {
                commands.entity(player).despawn();
            }
            for entity in enemy_query.iter() {
                commands.entity(entity).despawn();
            }
        }
    };
    if keyboard_input.just_pressed(KeyCode::F2) {
        for entity in player_query.iter() {
            commands.entity(entity).despawn();
        }
        for entity in enemy_query.iter() {
            commands.entity(entity).despawn();
        }
    }
}
