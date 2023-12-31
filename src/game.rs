use bevy::prelude::*;

use crate::{
    button::{button_style, button_system, button_text_style, NORMAL_BUTTON},
    map::{spawn_assets, spawn_map, AssetMap, TileMap},
    menu::MenuButtonAction,
    Level,
};

use super::{despawn_screen, GameState};

#[derive(Component, Debug)]
pub struct OnLevelCard;

#[derive(Component, Debug)]
pub struct OnGameCompleted;

// New type to use as a timer for the level card as a resource
#[derive(Resource, Deref, DerefMut)]
struct LevelCardTimer(Timer);

#[derive(Component, Debug)]
pub struct OnGameScreen;

#[derive(Component, Debug)]
pub struct OnGameLoad;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameSetup),
            (spawn_map, spawn_assets, game_setup_complete).chain(),
        )
        .add_systems(Update, countdown.run_if(in_state(GameState::GameSetup)))
        .add_systems(OnExit(GameState::GameSetup), despawn_screen::<OnLevelCard>)
        .add_systems(Update, game.run_if(in_state(GameState::GamePlay)))
        .add_systems(
            OnExit(GameState::GamePlay),
            (
                despawn_screen::<TileMap>,
                despawn_screen::<AssetMap>,
                game_levels_next,
            ),
        )
        .add_systems(OnEnter(GameState::GameCompleted), game_levels_completed)
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(GameState::GameCompleted)),
        )
        .add_systems(
            OnExit(GameState::GameCompleted),
            despawn_screen::<OnGameCompleted>,
        )
        .add_systems(OnEnter(GameState::Menu), game_reset);
    }
}

fn game_levels_next(mut level: ResMut<Level>) {
    level.number += 1;
    println!("Loading level {:?}", level.number);
}

fn game(
    keyboard_input: Res<Input<KeyCode>>,
    mut game_state: ResMut<NextState<GameState>>,
    mut level: ResMut<Level>,
) {
    if keyboard_input.just_pressed(KeyCode::Q) || keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
    }
    if keyboard_input.just_pressed(KeyCode::R) {
        level.number -= 1;
        game_state.set(GameState::GameSetup);
    }
}

fn game_reset(mut level: ResMut<Level>) {
    level.number = 1;
}

fn game_setup_complete(mut commands: Commands, level: Res<Level>) {
    let level_text = format!("Level {}", level.number);

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
                background_color: BackgroundColor(Color::rgba(0.0, 0.0, 0.0, 0.8)),
                ..default()
            },
            OnLevelCard,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: BackgroundColor(Color::NONE),
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            level_text,
                            TextStyle {
                                font_size: 80.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        }),
                    );
                    parent.spawn(
                        TextBundle::from_section(
                            "Collect all the potions to complete level",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            justify_content: JustifyContent::Center,
                            ..default()
                        }),
                    );
                });
        });

    // Insert the timer as a resource
    commands.insert_resource(LevelCardTimer(Timer::from_seconds(3.0, TimerMode::Once)));
}

fn game_levels_completed(mut commands: Commands) {
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
            OnGameCompleted,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
                    parent.spawn(
                        TextBundle::from_section(
                            "You have completed the game!",
                            TextStyle {
                                font_size: 40.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                    parent
                        .spawn((
                            ButtonBundle {
                                style: button_style(),
                                background_color: NORMAL_BUTTON.into(),
                                ..default()
                            },
                            MenuButtonAction::BackToMainMenu,
                        ))
                        .with_children(|parent| {
                            parent.spawn(TextBundle::from_section(
                                "Back to menu",
                                button_text_style(),
                            ));
                        });
                });
        });
}

// Tick the timer, and change state when finished
fn countdown(
    mut game_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut timer: ResMut<LevelCardTimer>,
) {
    if timer.tick(time.delta()).finished() {
        game_state.set(GameState::GamePlay);
    }
}

fn menu_action(
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_query {
        if *interaction == Interaction::Pressed {
            game_state.set(GameState::Menu);
        }
    }
}
