use crate::ascii::*;
use crate::movement::Moveable;
use crate::prelude::*;
use bevy::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Component, Debug)]
pub struct TileCollider;

#[derive(Component, Clone, Copy, Debug)]
pub enum TileType {
    Wall,
    Floor,
    Potion,
    Player,
    Teleport,
}

pub struct Map {
    tiles: Vec<TileType>,
}

impl Map {
    pub fn new(num_tiles: usize) -> Self {
        Self {
            tiles: vec![TileType::Floor; num_tiles],
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (spawn_map, spawn_assets));
    }
}

// Builds the non-interactive map, i.e floor and walls
fn spawn_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    println!("Spawning map...");

    let mut map = Map::new(NUM_TILES);

    if let Ok(lines) = read_lines("assets/level1.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(line) = line {
                for (x, char) in line.chars().enumerate() {
                    let idx = map_idx(x as i32, y as i32);
                    match char {
                        '#' => map.tiles[idx] = TileType::Wall,
                        _ => map.tiles[idx] = TileType::Floor,
                    }
                }
            }
        }

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = map_idx(x, y);
                let tile_type = map.tiles[index];
                let (sprite_idx, z_idx) = match tile_type {
                    TileType::Wall => (40, 1.0),
                    _ => (48, 0.0),
                };

                let sprite = spawn_ascii_sprite(
                    &mut commands,
                    &ascii,
                    sprite_idx,
                    Vec3::new(
                        -80.0 + (x as f32 * TILE_SIZE),
                        65.0 + -(y as f32 * TILE_SIZE),
                        z_idx,
                    ),
                );

                if matches!(tile_type, TileType::Wall) {
                    commands.entity(sprite).insert((tile_type, TileCollider));
                } else {
                    commands.entity(sprite).insert(tile_type);
                }
            }
        }
    }

    println!("Spawn map done");
}

// Builds the assets, i.e Player and Potions
fn spawn_assets(mut commands: Commands, ascii: Res<AsciiSheet>) {
    println!("Spawning assets...");

    let mut map = Map::new(NUM_TILES);

    if let Ok(lines) = read_lines("assets/level1.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(line) = line {
                for (x, char) in line.chars().enumerate() {
                    let idx = map_idx(x as i32, y as i32);
                    match char {
                        'o' => map.tiles[idx] = TileType::Potion,
                        'p' => map.tiles[idx] = TileType::Player,
                        't' => map.tiles[idx] = TileType::Teleport,
                        _ => (),
                    }
                }
            }
        }

        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                let index = map_idx(x, y);
                let tile_type = map.tiles[index];
                if let Some((sprite_idx, z_idx)) = match tile_type {
                    TileType::Potion => Some((115, 1.0)),
                    TileType::Player => Some((84, 2.0)),
                    TileType::Teleport => Some((60, 1.0)),
                    _ => None,
                } {
                    let sprite = spawn_ascii_sprite(
                        &mut commands,
                        &ascii,
                        sprite_idx,
                        Vec3::new(
                            -80.0 + (x as f32 * TILE_SIZE),
                            65.0 + -(y as f32 * TILE_SIZE),
                            z_idx,
                        ),
                    );

                    if matches!(tile_type, TileType::Player) {
                        commands.entity(sprite).insert((tile_type, Moveable::new()));
                    } else {
                        commands.entity(sprite).insert(tile_type);
                    }
                }
            }
        }
    }

    println!("Spawn assets done");
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
