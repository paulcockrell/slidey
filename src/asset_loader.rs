use bevy::prelude::*;

#[derive(Resource, Debug, Default)]
pub struct AudioAssets {
    pub music: Handle<AudioSource>,
    pub teleport: Handle<AudioSource>,
    pub potion_collect: Handle<AudioSource>,
}

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioAssets>()
            .add_systems(Startup, load_audio_assets);
    }
}

fn load_audio_assets(mut audio_assets: ResMut<AudioAssets>, asset_server: Res<AssetServer>) {
    *audio_assets = AudioAssets {
        music: asset_server.load("dungeon-level.ogg"),
        teleport: asset_server.load("teleport.ogg"),
        potion_collect: asset_server.load("potion-collect.ogg"),
    }
}
