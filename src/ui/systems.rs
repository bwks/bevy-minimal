use bevy::app::AppExit;
use bevy::prelude::*;

use crate::game::states::{AppState, GameState};
use crate::ui::components::{MainMenu, PlayButton, QuitButton, HUD};
use crate::ui::styles::{
    BUTTON_STYLE, HOVERED_BUTTON_COLOR, IMAGE_STYLE, MAIN_MENU_STYLE, NORMAL_BUTTON_COLOR,
    PRESSED_BUTTON_COLOR, TITLE_STYLE,
};
use crate::ui::utils::get_title_text_style;

pub fn main_menu_spawn_system(mut commands: Commands, asset_server: Res<AssetServer>) {
    let _main_menu_entity = build_main_menu(&mut commands, &asset_server);
}

pub fn main_menu_despawn_system(
    mut commands: Commands,
    main_menu_query: Query<Entity, With<MainMenu>>,
) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive()
    }
}

pub fn build_main_menu(commands: &mut Commands, asset_server: &Res<AssetServer>) -> Entity {
    let main_menu_entity = commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..Default::default()
            },
            MainMenu,
        ))
        .with_children(|parent| {
            // === Title ===
            parent
                .spawn(NodeBundle {
                    style: TITLE_STYLE,
                    ..default()
                })
                .with_children(|parent| {
                    // Image 1
                    // parent.spawn(ImageBundle {
                    //     style: IMAGE_STYLE,
                    //     image: asset_server.load("zombie.png").into(),
                    //     ..default()
                    // });
                    // Text
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Monster Apocalypse",
                                get_title_text_style(&asset_server),
                            )],
                            alignment: TextAlignment::Center,
                            ..default()
                        },
                        ..default()
                    });
                    // Image 2
                    // parent.spawn(ImageBundle {
                    //     style: IMAGE_STYLE,
                    //     image: asset_server.load("zombie.png").into(),
                    //     ..default()
                    // });
                });

            // Play
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..Default::default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Play",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                    ..Default::default()
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
            // Quit
            parent
                .spawn((
                    ButtonBundle {
                        style: BUTTON_STYLE,
                        background_color: NORMAL_BUTTON_COLOR.into(),
                        ..Default::default()
                    },
                    QuitButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text {
                            sections: vec![TextSection::new(
                                "Quit",
                                TextStyle {
                                    font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                    font_size: 32.0,
                                    color: Color::WHITE,
                                    ..Default::default()
                                },
                            )],
                            alignment: TextAlignment::Center,
                            ..Default::default()
                        },
                        ..Default::default()
                    });
                });
        })
        .id();

    main_menu_entity
}

pub fn interact_with_play_button_system(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<PlayButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_state_next_state.set(AppState::InGame);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

pub fn interact_with_quit_button_system(
    mut app_exit_event_writer: EventWriter<AppExit>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<QuitButton>),
    >,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Clicked => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            Interaction::Hovered => {
                *background_color = HOVERED_BUTTON_COLOR.into();
            }
            Interaction::None => {
                *background_color = NORMAL_BUTTON_COLOR.into();
            }
        }
    }
}

// HUD
pub fn hud_spawn(
    mut commands: Commands,
    _asset_server: Res<AssetServer>,
    _game_state: Res<State<GameState>>,
) {
    commands.spawn((
        NodeBundle {
            background_color: Color::RED.into(),
            style: Style {
                // flex_direction: FlexDirection::Column,
                // justify_content: JustifyContent::Center,
                // align_items: AlignItems::Center,
                size: Size::new(Val::Percent(100.0), Val::Percent(10.0)),
                // gap: Size::new(Val::Px(8.0), Val::Px(8.0)),
                ..Style::DEFAULT
            },
            ..Default::default()
        },
        HUD,
    ));
    // .with_children(|parent| {
    //     // === Title ===
    //     parent
    //         .spawn(NodeBundle {
    //             style: TITLE_STYLE,
    //             ..default()
    //         })
    //         .with_children(|parent| {
    //             // Image 1
    //             // parent.spawn(ImageBundle {
    //             //     style: IMAGE_STYLE,
    //             //     image: asset_server.load("zombie.png").into(),
    //             //     ..default()
    //             // });
    //             // Text
    //             parent.spawn(TextBundle {
    //                 text: Text {
    //                     sections: vec![TextSection::new(
    //                         "Monster Apocalypse",
    //                         get_title_text_style(&asset_server),
    //                     )],
    //                     alignment: TextAlignment::Center,
    //                     ..default()
    //                 },
    //                 ..default()
    //             });
    //             // Image 2
    //             // parent.spawn(ImageBundle {
    //             //     style: IMAGE_STYLE,
    //             //     image: asset_server.load("zombie.png").into(),
    //             //     ..default()
    //             // });
    //         });

    //     // Play
    //     parent
    //         .spawn((
    //             ButtonBundle {
    //                 style: BUTTON_STYLE,
    //                 background_color: NORMAL_BUTTON_COLOR.into(),
    //                 ..Default::default()
    //             },
    //             PlayButton,
    //         ))
    //         .with_children(|parent| {
    //             parent.spawn(TextBundle {
    //                 text: Text {
    //                     sections: vec![TextSection::new(
    //                         if game_state.0 != GameState::GameOver {
    //                             "Play"
    //                         } else {
    //                             "Replay"
    //                         },
    //                         TextStyle {
    //                             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //                             font_size: 32.0,
    //                             color: Color::WHITE,
    //                             ..Default::default()
    //                         },
    //                     )],
    //                     alignment: TextAlignment::Center,
    //                     ..Default::default()
    //                 },
    //                 ..Default::default()
    //             });
    //         });
    //     // Quit
    //     parent
    //         .spawn((
    //             ButtonBundle {
    //                 style: BUTTON_STYLE,
    //                 background_color: NORMAL_BUTTON_COLOR.into(),
    //                 ..Default::default()
    //             },
    //             QuitButton,
    //         ))
    //         .with_children(|parent| {
    //             parent.spawn(TextBundle {
    //                 text: Text {
    //                     sections: vec![TextSection::new(
    //                         "Quit",
    //                         TextStyle {
    //                             font: asset_server.load("fonts/FiraSans-Bold.ttf"),
    //                             font_size: 32.0,
    //                             color: Color::WHITE,
    //                             ..Default::default()
    //                         },
    //                     )],
    //                     alignment: TextAlignment::Center,
    //                     ..Default::default()
    //                 },
    //                 ..Default::default()
    //             });
    //         });
    // });
}
