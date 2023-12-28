use bevy::{app::AppExit, prelude::*};

use crate::button::{button_style, button_system, button_text_style, NORMAL_BUTTON};

use super::{despawn_screen, GameState, TEXT_COLOR};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<MenuState>()
            .add_systems(OnEnter(GameState::Menu), menu_setup)
            .add_systems(
                Update,
                (menu_action, button_system).run_if(in_state(GameState::Menu)),
            )
            .add_systems(OnEnter(MenuState::Main), main_menu_setup)
            .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
            .add_systems(OnEnter(MenuState::Credits), credits_screen_setup)
            .add_systems(
                OnExit(MenuState::Credits),
                despawn_screen::<OnCreditsScreen>,
            );
    }
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum MenuState {
    Main,
    Credits,
    #[default]
    Disabled,
}

#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnCreditsScreen;

#[derive(Component)]
struct OnGameScreen;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Credits,
    BackToMainMenu,
    Quit,
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Vh(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnMainMenuScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "Wizards Conundrum",
                            TextStyle {
                                font_size: 80.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        })
                        .with_text_alignment(TextAlignment::Center),
                    );

                    // Display three buttons for each action available from the main menu:
                    // - new game
                    // - settings
                    // - credits
                    // - quit
                    for (action, text) in [
                        (MenuButtonAction::Play, "New Game"),
                        (MenuButtonAction::Credits, "Credits"),
                        (MenuButtonAction::Quit, "Quit"),
                    ] {
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                action,
                            ))
                            .with_children(|parent| {
                                parent.spawn(TextBundle::from_section(text, button_text_style()));
                            });
                    }
                });
        });
}

fn credits_screen_setup(mut commands: Commands) {
    let button_style = Style {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = TextStyle {
        font_size: 40.0,
        color: TEXT_COLOR,
        ..default()
    };

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..default()
                },
                ..default()
            },
            OnCreditsScreen,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        height: Val::Vh(100.0),
                        width: Val::Vw(100.0),
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn(
                        TextBundle::from_section(
                            "Credits",
                            TextStyle {
                                font_size: 40.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        })
                        .with_text_alignment(TextAlignment::Center),
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            "Programming: Paul Cockrell",
                            TextStyle {
                                font_size: 20.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        })
                        .with_text_alignment(TextAlignment::Left),
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            "Graphics: Kenney (www.kenney.nl)",
                            TextStyle {
                                font_size: 20.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        })
                        .with_text_alignment(TextAlignment::Left),
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            "Music: Unknown",
                            TextStyle {
                                font_size: 20.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        })
                        .with_text_alignment(TextAlignment::Left),
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            "Logo font: textcraft.net",
                            TextStyle {
                                font_size: 20.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        })
                        .with_text_alignment(TextAlignment::Left),
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            "Source code: github.com/paulcockrell/slidey",
                            TextStyle {
                                font_size: 20.0,
                                color: TEXT_COLOR,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        })
                        .with_text_alignment(TextAlignment::Left),
                    );
                    // Display the back button to return to the main menu screen
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style,
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::BackToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section("Back", button_text_style));
                        });
                });
        });
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => app_exit_events.send(AppExit),
                MenuButtonAction::Play => {
                    game_state.set(GameState::GameSetup);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Credits => menu_state.set(MenuState::Credits),
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
            }
        }
    }
}
