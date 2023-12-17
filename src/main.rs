use bevy::prelude::*;

mod camera;
mod player;
mod view_port;

use camera::CameraPlugin;
use player::PlayerPlugin;
use view_port::ViewPortPlugin;

fn main() {
    App::new()
        .add_plugins(ViewPortPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(PlayerPlugin)
        .run();
}

