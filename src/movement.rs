use crate::ascii::TILE_SIZE;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;

use crate::map::TileCollider;

const PLAYER_SPEED: f32 = 85.0;

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

    // Only allow new movement when player is stopped
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

    let movement_amount = moveable.speed * time.delta_seconds();

    match moveable.direction {
        Direction::Up => transform.translation.y += movement_amount,
        Direction::Down => transform.translation.y -= movement_amount,
        Direction::Left => transform.translation.x -= movement_amount,
        Direction::Right => transform.translation.x += movement_amount,
        _ => (),
    }

    for wall in wall_query.iter() {
        if let Some(collision) = collide(
            transform.translation,
            Vec2::splat(TILE_SIZE),
            wall.0.translation,
            Vec2::splat(TILE_SIZE),
        ) {
            // Moving left, collided with right side of wall
            if matches!(moveable.direction, Direction::Left)
                && matches!(collision, Collision::Right)
            {
                moveable.speed = 0.0;
                moveable.direction = Direction::Stopped;
                // Ensure we don't move in to the wall, as the collision may occur
                // after we have moved 'into' it (as translation is a vec3 of f32s)
                transform.translation.x = wall.0.translation.x + TILE_SIZE;
            };

            // Moving right, collided with left side of wall
            if matches!(moveable.direction, Direction::Right)
                && matches!(collision, Collision::Left)
            {
                moveable.speed = 0.0;
                moveable.direction = Direction::Stopped;
                // Ensure we don't move in to the wall, as the collision may occur
                // after we have moved 'into' it (as translation is a vec3 of f32s)
                transform.translation.x = wall.0.translation.x - TILE_SIZE;
            };

            // Moving up, collided with bottom side of wall
            if matches!(moveable.direction, Direction::Up) && matches!(collision, Collision::Bottom)
            {
                moveable.speed = 0.0;
                moveable.direction = Direction::Stopped;
                // Ensure we don't move in to the wall, as the collision may occur
                // after we have moved 'into' it (as translation is a vec3 of f32s)
                transform.translation.y = wall.0.translation.y - TILE_SIZE;
            };

            // Moving down, collided with top side of wall
            if matches!(moveable.direction, Direction::Down) && matches!(collision, Collision::Top)
            {
                moveable.speed = 0.0;
                moveable.direction = Direction::Stopped;
                // Ensure we don't move in to the wall, as the collision may occur
                // after we have moved 'into' it (as translation is a vec3 of f32s)
                transform.translation.y = wall.0.translation.y + TILE_SIZE;
            };
        }
    }
}
