use bevy::prelude::*;

use crate::{despawn_screen, GameState, Level};

#[derive(Component, Debug)]
struct OnHud;

#[derive(Component, Debug)]
pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::GameSetup), spawn_hud)
            .add_systems(OnExit(GameState::GamePlay), despawn_screen::<OnHud>);
    }
}

fn spawn_hud(mut commands: Commands, level: Res<Level>) {
    let level_text = format!("Level {} of 10", level.number);

    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Vw(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                ..default()
            },
            OnHud,
        ))
        .with_children(|parent| {
            parent
                .spawn(NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        width: Val::Vw(100.0),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        justify_content: JustifyContent::SpaceBetween,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Display level info
                    parent.spawn(
                        TextBundle::from_section(
                            level_text,
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        }),
                    );
                    // Display controlls
                    parent.spawn(
                        TextBundle::from_section(
                            "Move: ARROW KEYS, Teleport: SPACEBAR, Music toggle: M, Quit: Q",
                            TextStyle {
                                font_size: 20.0,
                                color: Color::WHITE,
                                ..default()
                            },
                        )
                        .with_style(Style {
                            margin: UiRect::all(Val::Px(10.0)),
                            ..default()
                        }),
                    );
                });
        });
}
