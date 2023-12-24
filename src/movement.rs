use crate::ascii::TILE_SIZE;
use crate::map::Collectable;
use crate::map::Teleporter;
use crate::GameState;
use crate::Level;
use bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy::sprite::collide_aabb::Collision;

use crate::map::TileCollider;

const PLAYER_SPEED: f32 = 100.0;

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

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum PlayerState {
    #[default]
    Idle,
    Moving,
    Teleport,
    CollectPotion,
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                movement_controlls,
                update_position,
                check_wall,
                check_potion,
            )
                .chain()
                .run_if(in_state(GameState::GamePlay)),
        )
        .add_systems(OnEnter(PlayerState::Teleport), player_set_idle);
    }
}

fn movement_controlls(
    mut player_state: ResMut<NextState<PlayerState>>,
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
        player_state.set(PlayerState::Moving);
    }
    if input.just_released(KeyCode::Down) {
        moveable.direction = Direction::Down;
        moveable.speed = PLAYER_SPEED;
        player_state.set(PlayerState::Moving);
    }
    if input.just_released(KeyCode::Left) {
        moveable.direction = Direction::Left;
        moveable.speed = PLAYER_SPEED;
        player_state.set(PlayerState::Moving);
    }
    if input.just_released(KeyCode::Right) {
        moveable.direction = Direction::Right;
        moveable.speed = PLAYER_SPEED;
        player_state.set(PlayerState::Moving);
    }
    if input.just_released(KeyCode::Space) {
        moveable.direction = Direction::Stopped;
        moveable.speed = 0.0;
        let teleporter_translation = teleporter_transform.translation;
        let moveable_translation = moveable_transform.translation;

        moveable_transform.translation = teleporter_translation;
        teleporter_transform.translation = moveable_translation;

        player_state.set(PlayerState::Teleport);
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
    mut player_state: ResMut<NextState<PlayerState>>,
    mut game_state: ResMut<NextState<GameState>>,
    potion_query: Query<(Entity, &Transform), With<Collectable>>,
    level: ResMut<Level>,
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
            player_state.set(PlayerState::CollectPotion);
        }
    }

    if potion_query.is_empty() {
        println!("Level {} complete!", level.number);
        game_state.set(GameState::GameSetup);
    }
}

fn check_wall(
    mut player_query: Query<(&mut Transform, &mut Moveable), Without<TileCollider>>,
    mut player_state: ResMut<NextState<PlayerState>>,
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
                player_state.set(PlayerState::Idle);
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
                player_state.set(PlayerState::Idle);
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
                player_state.set(PlayerState::Idle);
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
                player_state.set(PlayerState::Idle);
            };
        }
    }
}

fn player_set_idle(mut player_state: ResMut<NextState<PlayerState>>) {
    player_state.set(PlayerState::Idle);
}
