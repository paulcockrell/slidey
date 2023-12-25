use bevy::prelude::*;

use crate::{despawn_screen, GameState};

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

fn spawn_hud(mut commands: Commands) {
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
                        flex_direction: FlexDirection::Row,
                        align_items: AlignItems::End,
                        justify_content: JustifyContent::Center,
                        ..default()
                    },
                    ..default()
                })
                .with_children(|parent| {
                    // Display the game name
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
