use bevy::prelude::*;

mod ascii;
mod camera;
mod map;
mod movement;
mod view_port;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 12;
    pub const SCREEN_HEIGHT: i32 = 9;
}

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.8, 0.7);

use ascii::AsciiPlugin;
use camera::CameraPlugin;
use movement::MovementPlugin;
use view_port::ViewPortPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum Music {
    Off,
    On,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .insert_resource(Music::On)
        .add_plugins(ViewPortPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AsciiPlugin)
        .add_plugins((splash::SplashPlugin, menu::MenuPlugin, game::GamePlugin))
        // .add_plugins(MapPlugin)
        .add_plugins(MovementPlugin)
        .run();
}

mod splash {
    use bevy::prelude::*;

    use super::{despawn_screen, GameState};

    // This plugin will display a splash screen for 10 seconds before switch to the menu
    pub struct SplashPlugin;

    impl Plugin for SplashPlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(OnEnter(GameState::Splash), splash_setup)
                // While in this state run the countdown system
                .add_systems(Update, countdown.run_if(in_state(GameState::Splash)))
                .add_systems(OnExit(GameState::Splash), despawn_screen::<OnSplashScreen>);
        }
    }

    // Tag component used to tag entities add on the splash screen
    #[derive(Component)]
    struct OnSplashScreen;

    // New type to use as a timer for this screen as a resource
    #[derive(Resource, Deref, DerefMut)]
    struct SplashTimer(Timer);

    fn splash_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        let icon = asset_server.load("icon.png");
        // Display logo
        commands
            .spawn((
                NodeBundle {
                    style: Style {
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::Center,
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        ..default()
                    },
                    ..default()
                },
                OnSplashScreen,
            ))
            .with_children(|parent| {
                parent.spawn(ImageBundle {
                    style: Style {
                        width: Val::Px(200.0),
                        ..default()
                    },
                    image: UiImage::new(icon),
                    ..default()
                });
            });

        // Insert the timer as a resource
        commands.insert_resource(SplashTimer(Timer::from_seconds(5.0, TimerMode::Once)));
    }

    // Tick the timer, and change state when finished
    fn countdown(
        mut game_state: ResMut<NextState<GameState>>,
        time: Res<Time>,
        mut timer: ResMut<SplashTimer>,
    ) {
        if timer.tick(time.delta()).finished() {
            game_state.set(GameState::Menu);
        }
    }
}

mod menu {
    use bevy::{app::AppExit, prelude::*};

    use crate::Music;

    use super::{despawn_screen, GameState, TEXT_COLOR};

    pub struct MenuPlugin;

    impl Plugin for MenuPlugin {
        fn build(&self, app: &mut App) {
            app.add_state::<MenuState>()
                .add_systems(OnEnter(GameState::Menu), menu_setup)
                .add_systems(OnEnter(MenuState::Main), main_menu_setup)
                .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
                .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
                .add_systems(
                    OnExit(MenuState::Settings),
                    despawn_screen::<OnSettingsMenuScreen>,
                )
                .add_systems(OnEnter(MenuState::SettingsMusic), sound_settings_menu_setup)
                .add_systems(
                    OnExit(MenuState::SettingsMusic),
                    despawn_screen::<OnSoundSettingsMenuScreen>,
                )
                .add_systems(
                    Update,
                    (setting_button::<Music>.run_if(in_state(MenuState::SettingsMusic)),),
                )
                .add_systems(
                    Update,
                    (menu_action, button_system).run_if(in_state(GameState::Menu)),
                );
        }
    }

    #[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
    enum MenuState {
        Main,
        Settings,
        SettingsMusic,
        #[default]
        Disabled,
    }

    #[derive(Component)]
    struct OnMainMenuScreen;

    #[derive(Component)]
    struct OnSettingsMenuScreen;

    #[derive(Component)]
    struct OnSoundSettingsMenuScreen;

    const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
    const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
    const HOVERED_PRESSED_BUTTON: Color = Color::rgb(0.25, 0.65, 0.25);
    const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

    #[derive(Component)]
    struct SelectedOption;

    #[derive(Component)]
    enum MenuButtonAction {
        Play,
        Settings,
        SettingsMusic,
        BackToMainMenu,
        BackToSettings,
        Quit,
    }

    // This system handles changing all buttons color based on mouse interaction
    fn button_system(
        mut interaction_query: Query<
            (&Interaction, &mut BackgroundColor, Option<&SelectedOption>),
            (Changed<Interaction>, With<Button>),
        >,
    ) {
        for (interaction, mut color, selected) in &mut interaction_query {
            *color = match (*interaction, selected) {
                (Interaction::Pressed, _) | (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
                (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
                (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
                (Interaction::None, None) => NORMAL_BUTTON.into(),
            }
        }
    }

    // This system updates the settings when a new value for a setting is selected, and marks the
    // button as the one currently selected
    fn setting_button<T: Resource + Component + PartialEq + Copy>(
        interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
        mut selected_query: Query<(Entity, &mut BackgroundColor), With<SelectedOption>>,
        mut commands: Commands,
        mut setting: ResMut<T>,
    ) {
        for (interaction, button_setting, entity) in &interaction_query {
            if *interaction == Interaction::Pressed && *setting != *button_setting {
                let (previous_button, mut previous_color) = selected_query.single_mut();
                *previous_color = NORMAL_BUTTON.into();
                commands.entity(previous_button).remove::<SelectedOption>();
                commands.entity(entity).insert(SelectedOption);
                *setting = *button_setting;
            }
        }
    }

    fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
        menu_state.set(MenuState::Main);
    }

    fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
        // Common style for all buttons
        let button_style = Style {
            width: Val::Px(250.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let button_icon_style = Style {
            width: Val::Px(30.0),
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
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
                OnMainMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::CRIMSON.into(),
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
                            }),
                        );

                        // Display three buttons for each action available from the main menu:
                        // - new game
                        // - settings
                        // - quit
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::Play,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/right.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "New Game",
                                    button_text_style.clone(),
                                ));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style.clone(),
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::Settings,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/wrench.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style.clone(),
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section(
                                    "Settings",
                                    button_text_style.clone(),
                                ));
                            });
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style,
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::Quit,
                            ))
                            .with_children(|parent| {
                                let icon = asset_server.load("textures/Game Icons/exitRight.png");
                                parent.spawn(ImageBundle {
                                    style: button_icon_style,
                                    image: UiImage::new(icon),
                                    ..default()
                                });
                                parent.spawn(TextBundle::from_section("Quit", button_text_style));
                            });
                    });
            });
    }

    fn settings_menu_setup(mut commands: Commands) {
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
                OnSettingsMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::CRIMSON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        for (action, text) in [
                            (MenuButtonAction::SettingsMusic, "Sound"),
                            (MenuButtonAction::BackToMainMenu, "Back"),
                        ] {
                            parent
                                .spawn((
                                    ButtonBundle {
                                        style: button_style.clone(),
                                        background_color: NORMAL_BUTTON.into(),
                                        ..default()
                                    },
                                    action,
                                ))
                                .with_children(|parent| {
                                    parent.spawn(TextBundle::from_section(
                                        text,
                                        button_text_style.clone(),
                                    ));
                                });
                        }
                    });
            });
    }

    fn sound_settings_menu_setup(mut commands: Commands, sound_level: Res<Music>) {
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
                OnSoundSettingsMenuScreen,
            ))
            .with_children(|parent| {
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            flex_direction: FlexDirection::Column,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: Color::CRIMSON.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        // Create a new `NodeBundle`, this time not setting its `flex_direction`. It will
                        // use the default value, `FlexDirection::Row`, from left to right.
                        parent
                            .spawn(NodeBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    ..default()
                                },
                                background_color: Color::CRIMSON.into(),
                                ..default()
                            })
                            .with_children(|parent| {
                                // Display a label for the current setting
                                parent.spawn(TextBundle::from_section(
                                    "Music",
                                    button_text_style.clone(),
                                ));
                                // Display a button for each possible value
                                for music_setting in [Music::Off, Music::On] {
                                    let mut entity = parent.spawn((
                                        ButtonBundle {
                                            style: Style {
                                                width: Val::Px(150.0),
                                                height: Val::Px(65.0),
                                                ..button_style.clone()
                                            },
                                            background_color: NORMAL_BUTTON.into(),
                                            ..default()
                                        },
                                        music_setting,
                                    ));
                                    entity.with_children(|parent| {
                                        parent.spawn(TextBundle::from_section(
                                            format!("{music_setting:?}"),
                                            button_text_style.clone(),
                                        ));
                                    });
                                    if *sound_level == music_setting {
                                        entity.insert(SelectedOption);
                                    }
                                }
                            });
                        // Display the back button to return to the settings screen
                        parent
                            .spawn((
                                ButtonBundle {
                                    style: button_style,
                                    background_color: NORMAL_BUTTON.into(),
                                    ..default()
                                },
                                MenuButtonAction::BackToSettings,
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
                        game_state.set(GameState::Game);
                        menu_state.set(MenuState::Disabled);
                    }
                    MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                    MenuButtonAction::SettingsMusic => {
                        menu_state.set(MenuState::SettingsMusic);
                    }
                    MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                    MenuButtonAction::BackToSettings => {
                        menu_state.set(MenuState::Settings);
                    }
                }
            }
        }
    }
}

mod game {
    use bevy::prelude::*;

    use crate::map::{spawn_assets, spawn_map};

    use super::{despawn_screen, GameState};

    #[derive(Component)]
    pub struct OnGameScreen;

    pub struct GamePlugin;

    impl Plugin for GamePlugin {
        fn build(&self, app: &mut App) {
            app.add_systems(
                OnEnter(GameState::Game),
                (game_setup, spawn_map, spawn_assets).chain(),
            )
            .add_systems(Update, game.run_if(in_state(GameState::Game)))
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
        }
    }

    fn game_setup() {
        todo!("Game setup goes here, maybe setting the level...");
    }

    fn game() {}
}

// Generic system that takes a component as a parameter, and will despawn all entites with that
// component
fn despawn_screen<T: Component>(to_despan: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despan {
        commands.entity(entity).despawn_recursive();
    }
}
