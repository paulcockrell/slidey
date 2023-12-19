use bevy::prelude::*;

mod ascii;
mod camera;
mod map;
mod player;
mod view_port;

mod prelude {
    pub const SCREEN_WIDTH: i32 = 12;
    pub const SCREEN_HEIGHT: i32 = 10;
}

use ascii::AsciiPlugin;
use camera::CameraPlugin;
use map::MapPlugin;
use player::PlayerPlugin;
use view_port::ViewPortPlugin;

fn main() {
    App::new()
        .add_plugins(ViewPortPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(AsciiPlugin)
        .add_plugins(PlayerPlugin)
        .add_plugins(MapPlugin)
        .run();
}
