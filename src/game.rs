use bevy::prelude::*;

use crate::{
    map::{spawn_assets, spawn_map, AssetMap, TileMap},
    Level,
};

use super::{despawn_screen, GameState};

#[derive(Component, Debug)]
pub struct OnLevelCard;

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
        .add_systems(OnExit(GameState::GamePlay), despawn_screen::<TileMap>)
        .add_systems(OnExit(GameState::GamePlay), despawn_screen::<AssetMap>)
        .add_systems(OnExit(GameState::GamePlay), game_levels_next)
        .add_systems(OnEnter(GameState::GameCompleted), game_levels_completed)
        .add_systems(OnEnter(GameState::Menu), game_reset);
    }
}

fn game_levels_next(mut level: ResMut<Level>, mut game_state: ResMut<NextState<GameState>>) {
    level.number += 1;
    if level.number <= 10 {
        println!("Loading level {:?}", level.number);
    } else {
        game_state.set(GameState::GameCompleted);
    }
}

fn game(keyboard_input: Res<Input<KeyCode>>, mut game_state: ResMut<NextState<GameState>>) {
    if keyboard_input.just_pressed(KeyCode::Q) || keyboard_input.just_pressed(KeyCode::Escape) {
        game_state.set(GameState::Menu);
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
                    background_color: BackgroundColor(Color::CRIMSON),
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
                    parent.spawn(
                        TextBundle::from_section(
                            "Press Q to return to the main menu",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(50.0)),
                            ..default()
                        }),
                    );
                });
        });

    // Insert the timer as a resource
    commands.insert_resource(LevelCardTimer(Timer::from_seconds(3.0, TimerMode::Once)));
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
