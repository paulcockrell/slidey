use crate::ascii::TILE_SIZE;
use crate::map::Collectable;
use crate::map::Teleporter;
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
        app.add_systems(
            Update,
            (movement_controlls, update_position, check_wall).chain(),
        )
        .add_systems(Update, check_potion);
    }
}

fn movement_controlls(
    mut moveable_query: Query<(&mut Moveable, &mut Transform), Without<Teleporter>>,
    mut teleporter_query: Query<&mut Transform, With<Teleporter>>,
    input: Res<Input<KeyCode>>,
) {
    let Ok((mut moveable, mut moveable_transform)) = moveable_query.get_single_mut() else {
        return;
    };

    let Ok(mut teleporter_transform) = teleporter_query.get_single_mut() else {
        return;
    };

    // Only allow new movement when player is stopped
    if !matches!(moveable.direction, Direction::Stopped) {
        return;
    }

    if input.just_released(KeyCode::Up) {
        moveable.direction = Direction::Up;
        moveable.speed = PLAYER_SPEED;
    }
    if input.just_released(KeyCode::Down) {
        moveable.direction = Direction::Down;
        moveable.speed = PLAYER_SPEED;
    }
    if input.just_released(KeyCode::Left) {
        moveable.direction = Direction::Left;
        moveable.speed = PLAYER_SPEED;
    }
    if input.just_released(KeyCode::Right) {
        moveable.direction = Direction::Right;
        moveable.speed = PLAYER_SPEED;
    }
    if input.just_released(KeyCode::Space) {
        moveable.direction = Direction::Stopped;
        moveable.speed = 0.0;
        let teleporter_translation = teleporter_transform.translation;
        let moveable_translation = moveable_transform.translation;

        moveable_transform.translation = teleporter_translation;
        teleporter_transform.translation = moveable_translation;
    }
}

fn update_position(mut player_query: Query<(&mut Transform, &Moveable)>, time: Res<Time>) {
    let Ok((mut transform, moveable)) = player_query.get_single_mut() else {
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
}

fn check_potion(
    mut commands: Commands,
    mut player_query: Query<&mut Transform, (With<Moveable>, Without<Collectable>)>,
    potion_query: Query<(Entity, &Transform), With<Collectable>>,
) {
    let Ok(player_transform) = player_query.get_single_mut() else {
        return;
    };

    for (potion_entity, potion_transform) in potion_query.iter() {
        let hit = collide(
            player_transform.translation,
            Vec2::splat(TILE_SIZE),
            potion_transform.translation,
            Vec2::splat(TILE_SIZE),
        )
        .is_some();

        if hit {
            commands.entity(potion_entity).despawn_recursive();
        }
    }
}

fn check_wall(
    mut player_query: Query<(&mut Transform, &mut Moveable), Without<TileCollider>>,
    wall_query: Query<&Transform, With<TileCollider>>,
) {
    let Ok((mut player_transform, mut player_moveable)) = player_query.get_single_mut() else {
        return;
    };

    for wall_transform in wall_query.iter() {
        if let Some(collision) = collide(
            player_transform.translation,
            Vec2::splat(TILE_SIZE),
            wall_transform.translation,
            Vec2::splat(TILE_SIZE),
        ) {
            // Moving left, collided with right side of wall
            if matches!(player_moveable.direction, Direction::Left)
                && matches!(collision, Collision::Right)
            {
                player_moveable.speed = 0.0;
                player_moveable.direction = Direction::Stopped;
                // Ensure we don't move in to the wall, as the collision may occur
                // after we have moved 'into' it (as translation is a vec3 of f32s)
                player_transform.translation.x = wall_transform.translation.x + TILE_SIZE;
            };

            // Moving right, collided with left side of wall
            if matches!(player_moveable.direction, Direction::Right)
                && matches!(collision, Collision::Left)
            {
                player_moveable.speed = 0.0;
                player_moveable.direction = Direction::Stopped;
                // Ensure we don't move in to the wall, as the collision may occur
                // after we have moved 'into' it (as translation is a vec3 of f32s)
                player_transform.translation.x = wall_transform.translation.x - TILE_SIZE;
            };

            // Moving up, collided with bottom side of wall
            if matches!(player_moveable.direction, Direction::Up)
                && matches!(collision, Collision::Bottom)
            {
                player_moveable.speed = 0.0;
                player_moveable.direction = Direction::Stopped;
                // Ensure we don't move in to the wall, as the collision may occur
                // after we have moved 'into' it (as translation is a vec3 of f32s)
                player_transform.translation.y = wall_transform.translation.y - TILE_SIZE;
            };

            // Moving down, collided with top side of wall
            if matches!(player_moveable.direction, Direction::Down)
                && matches!(collision, Collision::Top)
            {
                player_moveable.speed = 0.0;
                player_moveable.direction = Direction::Stopped;
                // Ensure we don't move in to the wall, as the collision may occur
                // after we have moved 'into' it (as translation is a vec3 of f32s)
                player_transform.translation.y = wall_transform.translation.y + TILE_SIZE;
            };
        }
    }
}
