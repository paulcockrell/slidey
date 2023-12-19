use crate::ascii::*;
use crate::prelude::*;
use bevy::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

pub const TILE_SIZE: f32 = 16.0;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Component, Clone, Copy, Debug)]
pub enum TileType {
    Wall,
    Floor,
    Potion,
    Player,
    Teleport,
}

#[derive(Resource, Debug)]
pub struct Map {
    tiles: Vec<TileType>,
}

impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, build_map);
        app.add_systems(Startup, spawn_map);
    }
}

fn build_map(mut commands: Commands) {
    let mut map = Map::new();

    if let Ok(lines) = read_lines("assets/level1.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(line) = line {
                for (x, char) in line.chars().enumerate() {
                    let idx = map_idx(x as i32, y as i32);
                    match char {
                        '#' => map.tiles[idx] = TileType::Wall,
                        'o' => map.tiles[idx] = TileType::Potion,
                        'p' => map.tiles[idx] = TileType::Player,
                        't' => map.tiles[idx] = TileType::Teleport,
                        _ => map.tiles[idx] = TileType::Floor,
                    }
                }
            }
        }
    }

    commands.insert_resource(map);
}

pub fn spawn_map(mut commands: Commands, map: Res<Map>, ascii: Res<AsciiSheet>) {
    for y in 0..SCREEN_HEIGHT {
        for x in 0..SCREEN_WIDTH {
            let index = map_idx(x, y);
            let tile_type = map.tiles[index];
            let sprite_idx = match tile_type {
                TileType::Wall => 40,
                TileType::Floor => 48,
                TileType::Potion => 115,
                TileType::Player => 84,
                TileType::Teleport => 60,
            };

            let sprite = spawn_ascii_sprite(
                &mut commands,
                &ascii,
                sprite_idx,
                Vec3::new(
                    85.0 + -(x as f32 * TILE_SIZE),
                    65.0 + -(y as f32 * TILE_SIZE),
                    0.0,
                ),
            );

            commands.entity(sprite).insert(tile_type);
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

pub fn map_idx(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
