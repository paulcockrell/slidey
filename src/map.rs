use bevy::prelude::*;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use crate::ascii::{spawn_ascii_sprite, AsciiSheet};

pub const TILE_SIZE: f32 = 16.0;

#[derive(Component, Debug, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
    Potion,
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

fn spawn_map(mut commands: Commands, ascii: Res<AsciiSheet>) {
    if let Ok(lines) = read_lines("assets/level1.txt") {
        for (y, line) in lines.enumerate() {
            if let Ok(line) = line {
                for (x, char) in line.chars().enumerate() {
                    let (index, tile_type, tile_name) = match char {
                        '#' => (40, TileType::Wall, Name::new("Wall")),
                        'o' => (115, TileType::Potion, Name::new("Potion")),
                        't' => (60, TileType::Potion, Name::new("Transporter")),
                        _ => (48, TileType::Floor, Name::new("Floor")),
                    };

                    let sprite = spawn_ascii_sprite(
                        &mut commands,
                        &ascii,
                        index,
                        Vec3::new(
                            85.0 + -(x as f32 * TILE_SIZE),
                            65.0 + -(y as f32 * TILE_SIZE),
                            0.0,
                        ),
                    );

                    commands.entity(sprite).insert(tile_name).insert(tile_type);
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
