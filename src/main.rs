use bevy::prelude::*;

mod ascii;
mod camera;
mod game;
mod map;
mod menu;
mod movement;
mod splash;
mod view_port;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 12;
    pub const SCREEN_HEIGHT: i32 = 9;
}

pub const TEXT_COLOR: Color = Color::rgb(0.9, 0.8, 0.7);

use ascii::AsciiPlugin;
use camera::CameraPlugin;
use game::GamePlugin;
use menu::MenuPlugin;
use movement::MovementPlugin;
use splash::SplashPlugin;
use view_port::ViewPortPlugin;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    Splash,
    Menu,
    Game,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum Music {
    Off,
    On,
}

fn main() {
    App::new()
        .add_state::<GameState>()
        .insert_resource(Music::On)
        .add_plugins(ViewPortPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AsciiPlugin)
        .add_plugins((SplashPlugin, MenuPlugin, GamePlugin))
        // .add_plugins(MapPlugin)
        .add_plugins(MovementPlugin)
        .run();
}

// Generic system that takes a component as a parameter, and will despawn all entites with that
// component
fn despawn_screen<T: Component>(to_despan: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despan {
        commands.entity(entity).despawn_recursive();
    }
}
