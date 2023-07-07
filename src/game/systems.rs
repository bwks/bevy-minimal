use bevy::app::AppExit;
use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::common::components::Vitality;
use crate::common::resources::GameTextures;
use crate::enemy::components::Enemy;
use crate::game::components::ColorText;
use crate::game::states::{AppState, GameState};
use crate::player::actions::ControlAction;
use crate::player::components::{Lives, Player, PlayerVariant, Score};

use super::components::UiPlayerLives;

pub fn camera_spawn_system(mut commands: Commands) {
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
        _player_vitality,
        _player_lives,
        _player_score,
        _player_sprite,
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

pub fn player_lives_spawn_system(
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    player_query: Query<(&Lives, &PlayerVariant), With<Player>>,
    ui_player_lives: Query<&PlayerVariant, With<UiPlayerLives>>,
) {
    let mut player1_ui_lives = 0;
    let mut player2_ui_lives = 0;

    for player_variant in ui_player_lives.iter() {
        match &player_variant {
            PlayerVariant::One => player1_ui_lives += 1,
            PlayerVariant::Two => player2_ui_lives += 1,
        }
    }

    for (player_lives, player_variant) in player_query.iter() {
        let (player_ui_lives, player_ui_sprite) = match &player_variant {
            PlayerVariant::One => (player1_ui_lives, game_textures.player_one_static.clone()),
            PlayerVariant::Two => (player2_ui_lives, game_textures.player_two_static.clone()),
        };

        if player_ui_lives < player_lives.count {
            let mut count = 0.0;
            for _ in 0..player_lives.count {
                let position = match &player_variant {
                    PlayerVariant::One => UiRect {
                        top: Val::Px(0.0),
                        left: Val::Px(count),
                        ..default()
                    },
                    PlayerVariant::Two => UiRect {
                        top: Val::Px(0.0),
                        right: Val::Px(count),
                        ..default()
                    },
                };
                commands.spawn((
                    ImageBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            size: Size::new(Val::Px(48.0), Val::Px(48.0)),
                            margin: UiRect {
                                left: Val::Px(2.0),
                                right: Val::Px(2.0),
                                top: Val::Px(65.0),
                                bottom: Val::Px(8.0),
                            },
                            position: position,
                            ..Style::DEFAULT
                        },
                        image: player_ui_sprite.clone().into(),
                        ..default()
                    },
                    UiPlayerLives,
                    player_variant.clone(),
                ));
                count += 40.0;
            }
        }
    }
}

pub fn player_lives_despawn_system(
    mut commands: Commands,
    player_query: Query<(&Lives, &PlayerVariant), With<Player>>,
    mut ui_player_lives: Query<(Entity, &PlayerVariant), With<UiPlayerLives>>,
) {
    let mut player1_ui_lives = 0;
    let mut player2_ui_lives = 0;

    for (_ui_playery_lives_entity, player_variant) in ui_player_lives.iter() {
        match &player_variant {
            PlayerVariant::One => player1_ui_lives += 1,
            PlayerVariant::Two => player2_ui_lives += 1,
        }
    }

    for (player_lives, player_variant) in player_query.iter() {
        let player_ui_lives = match &player_variant {
            PlayerVariant::One => player1_ui_lives,
            PlayerVariant::Two => player2_ui_lives,
        };

        if player_ui_lives != player_lives.count {
            for (ui_player_lives_entity, _player_variant) in ui_player_lives.iter_mut() {
                commands.entity(ui_player_lives_entity).despawn()
            }
        }
    }
}

pub fn score_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 0

    commands.spawn((
        TextBundle {
            text: Text {
                sections: vec![TextSection::new(
                    "Player 1 Score",
                    TextStyle {
                        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                        font_size: 50.0,
                        color: Color::WHITE,
                    },
                )],
                alignment: TextAlignment::Center,
                ..default()
            },
            style: Style {
                position_type: PositionType::Absolute,
                margin: UiRect::new(Val::Px(20.0), Val::Px(20.0), Val::Px(10.0), Val::Px(10.0)),
                position: UiRect {
                    top: Val::Px(0.0),
                    left: Val::Px(0.0),
                    ..default()
                },
                ..default()
            },
            ..default()
        },
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
}

pub fn score_update_system(
    player_query: Query<(&PlayerVariant, &Score), With<Player>>,
    mut query: Query<(&mut Text, &PlayerVariant), With<ColorText>>,
) {
    for (mut text, score_player_variant) in &mut query.iter_mut() {
        for (player_variant, player_score) in player_query.iter() {
            if player_variant == score_player_variant {
                text.sections[0].value = format!("{}", player_score.value);
            }
        }
    }
}
