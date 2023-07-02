use std::collections::HashSet;

use bevy::app::AppExit;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::common::components::Vitality;
use crate::common::resources::GameTextures;
use crate::enemy::components::Enemy;
use crate::game::components::{ColorText, FpsText};
use crate::game::states::{AppState, GameState};
use crate::player::actions::ControlAction;
use crate::player::components::{Lives, Player, PlayerVariant, Score};

pub fn spawn_camera_system(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn toggle_game_state_system(
    keyboard_input: Res<Input<KeyCode>>,
    controller_query: Query<&ActionState<ControlAction>>,
    app_state: Res<State<AppState>>,
    game_state: Res<State<GameState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    if game_state.0 != GameState::GameOver {
        for controller_input in controller_query.iter() {
            if keyboard_input.just_pressed(KeyCode::Space)
                || controller_input.just_pressed(ControlAction::Pause)
            {
                match app_state.0 {
                    AppState::InGame => {
                        game_state_next_state.set(GameState::Paused);
                        app_state_next_state.set(AppState::MainMenu);
                    }
                    AppState::MainMenu => {
                        app_state_next_state.set(AppState::InGame);
                        game_state_next_state.set(GameState::Playing);
                    }
                }
            }
        }
    }
}

pub fn game_over_system(
    player_query: Query<(&Vitality, &Lives), With<Player>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
) {
    let mut dead_players = 0;

    for (player_vitality, player_lives) in player_query.iter() {
        if *player_vitality == Vitality::Dead && player_lives.count == 0 {
            dead_players += 1;
        }
    }

    if dead_players == 2 {
        game_state_next_state.set(GameState::GameOver);
        app_state_next_state.set(AppState::MainMenu);
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
    mut player_query: Query<
        (
            &PlayerVariant,
            &mut Vitality,
            &mut Lives,
            &mut Score,
            &mut Handle<TextureAtlas>,
            &ActionState<ControlAction>,
        ),
        With<Player>,
    >,
    enemy_query: Query<Entity, With<Enemy>>,
    keyboard_input: Res<Input<KeyCode>>,
    game_textures: Res<GameTextures>,
    mut game_state_next_state: ResMut<NextState<GameState>>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    let mut restart_game = false;
    for (
        _player_variant,
        mut _player_vitality,
        mut _player_lives,
        mut _player_score,
        mut _player_sprite,
        controller_input,
    ) in player_query.iter()
    {
        if controller_input.just_pressed(ControlAction::Restart)
            || keyboard_input.just_pressed(KeyCode::R)
        {
            restart_game = true;
            break;
        }
    }

    if restart_game {
        for enemy_entity in enemy_query.iter() {
            commands.entity(enemy_entity).despawn();
        }

        for (
            player_variant,
            mut player_vitality,
            mut player_lives,
            mut player_score,
            mut player_sprite,
            _controller_input,
        ) in player_query.iter_mut()
        {
            let player_sprite_atlas = match player_variant {
                PlayerVariant::One => game_textures.player_one.clone(),
                PlayerVariant::Two => game_textures.player_two.clone(),
            };

            *player_sprite = player_sprite_atlas;

            *player_vitality = Vitality::Alive;
            player_lives.count = 3;
            player_score.value = 0;
            game_state_next_state.set(GameState::Playing);
            app_state_next_state.set(AppState::InGame);
        }
    }
}

pub fn text_setup_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 0
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "player1_score",
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
                top: Val::Px(15.0),
                left: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
        PlayerVariant::One,
    ));

    // 1
    commands.spawn((
        // Create a TextBundle that has a Text with a single section.
        TextBundle::from_section(
            // Accepts a `String` or any type that converts into a `String`, such as `&str`
            "player2_score",
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
                top: Val::Px(15.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
        PlayerVariant::Two,
    ));

    // Text with multiple sections
    // commands.spawn((
    //     // Create a TextBundle that has a Text with a list of sections.
    //     TextBundle::from_sections([
    //         TextSection::new(
    //             "FPS: ",
    //             TextStyle {
    //                 font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //                 font_size: 30.0,
    //                 color: Color::WHITE,
    //             },
    //         ),
    //         TextSection::from_style(TextStyle {
    //             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //             font_size: 30.0,
    //             color: Color::GOLD,
    //         }),
    //     ])
    //     .with_style(Style {
    //         position_type: PositionType::Absolute,
    //         position: UiRect {
    //             top: Val::Px(220.0),
    //             left: Val::Px(15.0),
    //             ..default()
    //         },
    //         ..default()
    //     }),
    //     FpsText,
    // ));
}

pub fn text_color_system(
    player_query: Query<(&PlayerVariant, &Score, &Lives), With<Player>>,
    time: Res<Time>,
    mut query: Query<(&mut Text, &PlayerVariant), With<ColorText>>,
) {
    for (mut text, score_player_variant) in &mut query.iter_mut() {
        let seconds = time.elapsed_seconds();

        // Update the color of the first and only section.
        text.sections[0].style.color = Color::Rgba {
            red: (1.25 * seconds).sin() / 2.0 + 0.5,
            green: (0.75 * seconds).sin() / 2.0 + 0.5,
            blue: (0.50 * seconds).sin() / 2.0 + 0.5,
            alpha: 1.0,
        };

        for (player_variant, player_score, player_lives) in player_query.iter() {
            if player_variant == score_player_variant {
                let lives = match player_lives.count {
                    3 => "* * *",
                    2 => "* *",
                    1 => "*",
                    _ => "",
                };
                text.sections[0].value = format!(
                    "Player {}\n {}\n {}",
                    player_variant, player_score.value, lives
                );
            }
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
