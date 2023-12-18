use bevy::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const TILE_SIZE: f32 = 16.0;

#[derive(Component, Debug, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    if let Ok(lines) = read_lines("assets/level1.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(line) = line {
                for (x, char) in line.chars().enumerate() {
                    let (texture, tile_type) = match char {
                        '#' => (asset_server.load("wall.png"), TileType::Wall),
                        _ => (asset_server.load("floor.png"), TileType::Floor),
                    };

                    commands.spawn((SpriteBundle {
                        texture,
                        transform: Transform::from_xyz(
                            -90.0 + (x as f32 * TILE_SIZE),
                            65.0 + (-(y as f32) * TILE_SIZE),
                            0.0,
                        ),
                        sprite: Sprite {
                            custom_size: Some(Vec2::splat(16.)),
                            ..default()
                        },
                        ..default()
                    },));
                }
            }
        }
    }
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
