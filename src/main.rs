use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod ascii;
mod audio;
mod button;
mod camera;
mod game;
mod hud;
mod levels;
mod map;
mod menu;
mod movement;
mod splash;
mod view_port;

mod prelude {
    pub const MAP_NUM_TILES_WIDE: i32 = 12;
    pub const MAP_NUM_TILES_HIGH: i32 = 9;
    pub const MAP_NUM_TILES: i32 = MAP_NUM_TILES_WIDE * MAP_NUM_TILES_HIGH;
}

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.8, 0.7);

use ascii::AsciiPlugin;
use audio::AudioPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use hud::HudPlugin;
use levels::LevelPlugin;
use menu::MenuPlugin;
use movement::{MovementPlugin, PlayerState};
use splash::SplashPlugin;
use view_port::ViewPortPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    GameSetup,
    GamePlay,
    GameCompleted,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
pub struct Level {
    number: u8,
}

impl Level {
    pub fn new() -> Self {
        Self { number: 9 }
    }
}

impl Default for Level {
    fn default() -> Self {
        Self::new()
    }
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .add_state::<PlayerState>()
        .insert_resource(Level::default())
        .add_plugins(ViewPortPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AsciiPlugin)
        .add_plugins((SplashPlugin, MenuPlugin, GamePlugin))
        .add_plugins(MovementPlugin)
        .add_plugins(AudioPlugin)
        .add_plugins(HudPlugin)
        .add_plugins(LevelPlugin)
        // .add_plugins(WorldInspectorPlugin::new())
        .run();
}

// Generic system that takes a component as a parameter, and will despawn all entites with that
// component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
