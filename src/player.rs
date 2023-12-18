use bevy::prelude::*;

use crate::{
    ascii::{spawn_ascii_sprite, AsciiSheet},
    map::TILE_SIZE,
};

const PLAYER_SPEED: f32 = 10.0;

#[derive(Component, Debug)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, spawn_player)
            .add_systems(Update, player_movement_controlls);
    }
}

fn spawn_player(mut commands: Commands, ascii: Res<AsciiSheet>) {
    let player = spawn_ascii_sprite(
        &mut commands,
        &ascii,
        86,
        Vec3::new(2.0 * TILE_SIZE, -2.0 * TILE_SIZE, 1.0),
    );

    commands
        .entity(player)
        .insert(Name::new("Player"))
        .insert(Player);
}

fn player_movement_controlls(
    mut query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let Ok(mut transform) = query.get_single_mut() else {
        return;
    };

    let movement_amount = PLAYER_SPEED * time.delta_seconds();

    if input.pressed(KeyCode::Up) {
        transform.translation.y += movement_amount;
    }
    if input.pressed(KeyCode::Down) {
        transform.translation.y -= movement_amount;
    }
    if input.pressed(KeyCode::Left) {
        transform.translation.x -= movement_amount;
    }
    if input.pressed(KeyCode::Right) {
        transform.translation.x += movement_amount;
    }
}
