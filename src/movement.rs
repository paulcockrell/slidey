use crate::ascii::TILE_SIZE;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;

use crate::map::TileCollider;

const PLAYER_SPEED: f32 = 25.0;

#[derive(Debug)]
enum Direction {
    Stopped,
    Up,
    Down,
    Left,
    Right,
}

#[derive(Component, Debug)]
pub struct Moveable {
    speed: f32,
    direction: Direction,
}

impl Moveable {
    pub fn new() -> Self {
        Self {
            speed: 0.0,
            direction: Direction::Stopped,
        }
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (movement_controlls, update_position).chain());
    }
}

fn movement_controlls(mut query: Query<&mut Moveable>, input: Res<Input<KeyCode>>) {
    let Ok(mut moveable) = query.get_single_mut() else {
        return;
    };

    if !matches!(moveable.direction, Direction::Stopped) {
        return;
    }

    if input.pressed(KeyCode::Up) {
        moveable.direction = Direction::Up;
        moveable.speed = PLAYER_SPEED;
    }
    if input.pressed(KeyCode::Down) {
        moveable.direction = Direction::Down;
        moveable.speed = PLAYER_SPEED;
    }
    if input.pressed(KeyCode::Left) {
        moveable.direction = Direction::Left;
        moveable.speed = PLAYER_SPEED;
    }
    if input.pressed(KeyCode::Right) {
        moveable.direction = Direction::Right;
        moveable.speed = PLAYER_SPEED;
    }
}

fn update_position(
    mut player_query: Query<(&mut Transform, &mut Moveable), Without<TileCollider>>,
    wall_query: Query<(&Transform, With<TileCollider>)>,
    time: Res<Time>,
) {
    let Ok((mut transform, mut moveable)) = player_query.get_single_mut() else {
        return;
    };

    if matches!(moveable.direction, Direction::Stopped) {
        return;
    }

    for wall in wall_query.iter() {
        if let Some(collision) = collide(
            transform.translation,
            Vec2::splat(TILE_SIZE),
            wall.0.translation,
            Vec2::splat(TILE_SIZE),
        ) {
            if matches!(moveable.direction, Direction::Left)
                && matches!(collision, Collision::Right)
            {
                moveable.speed = 0.0;
                moveable.direction = Direction::Stopped;
            };
            if matches!(moveable.direction, Direction::Left)
                && matches!(collision, Collision::Right)
            {
                moveable.speed = 0.0;
                moveable.direction = Direction::Stopped;
            };
            if matches!(moveable.direction, Direction::Up) && matches!(collision, Collision::Bottom)
            {
                moveable.speed = 0.0;
                moveable.direction = Direction::Stopped;
            };
            if matches!(moveable.direction, Direction::Down) && matches!(collision, Collision::Top)
            {
                moveable.speed = 0.0;
                moveable.direction = Direction::Stopped;
            };
        }
    }

    let movement_amount = moveable.speed * time.delta_seconds();

    match moveable.direction {
        Direction::Up => transform.translation.y += movement_amount,
        Direction::Down => transform.translation.y -= movement_amount,
        Direction::Left => transform.translation.x -= movement_amount,
        Direction::Right => transform.translation.x += movement_amount,
        _ => (),
    }
}
