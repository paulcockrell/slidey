use bevy::prelude::*;

const PLAYER_SPEED: f32 = 10.0;

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

fn update_position(mut query: Query<(&mut Transform, &Moveable)>, time: Res<Time>) {
    let Ok((mut transform, moveable)) = query.get_single_mut() else {
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

// fn wall_collision_check(target_player_pos: Vec3, wall_translation: Vec3) -> bool {
//     let collision = collide(
//         target_player_pos,
//         Vec2::splat(TILE_SIZE * 0.9),
//         wall_translation,
//         Vec2::splat(TILE_SIZE),
//     );
//     collision.is_some()
// }
