use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;

pub struct ViewPortPlugin;

impl Plugin for ViewPortPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AssetMetaCheck::Never).add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Slidey".into(),
                        resolution: (640.0, 480.0).into(),
                        resizable: false,
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        );
    }
}
