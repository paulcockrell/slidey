use bevy::audio::PlaybackMode;
use bevy::prelude::*;

use crate::movement::PlayerState;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AudioAssets>()
            .add_systems(Startup, (load_audio_assets, setup).chain())
            .add_systems(Update, pause)
            .add_systems(OnEnter(PlayerState::Teleport), play_teleport_sfx)
            .add_systems(OnEnter(PlayerState::CollectPotion), play_collect_potion_sfx);
    }
}

#[derive(Resource, Debug, Default)]
pub struct AudioAssets {
    pub music: Handle<AudioSource>,
    pub teleport: Handle<AudioSource>,
    pub potion_collect: Handle<AudioSource>,
}

fn load_audio_assets(mut audio_assets: ResMut<AudioAssets>, asset_server: Res<AssetServer>) {
    *audio_assets = AudioAssets {
        music: asset_server.load("dungeon-level.ogg"),
        teleport: asset_server.load("teleport.ogg"),
        potion_collect: asset_server.load("potion-collect.ogg"),
    }
}

fn pause(keyboard_input: Res<Input<KeyCode>>, music_controller: Query<&AudioSink, With<Music>>) {
    if keyboard_input.just_pressed(KeyCode::M) {
        if let Ok(sink) = music_controller.get_single() {
            sink.toggle();
        }
    }
}

fn setup(audio_assets: Res<AudioAssets>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: audio_assets.music.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Loop,
                ..default()
            },
        },
        Music,
    ));
}

fn play_teleport_sfx(audio_assets: Res<AudioAssets>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: audio_assets.teleport.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                ..default()
            },
        },
        Sfx,
    ));
}

fn play_collect_potion_sfx(audio_assets: Res<AudioAssets>, mut commands: Commands) {
    commands.spawn((
        AudioBundle {
            source: audio_assets.potion_collect.clone(),
            settings: PlaybackSettings {
                mode: PlaybackMode::Once,
                ..default()
            },
        },
        Sfx,
    ));
}

#[derive(Component, Debug)]
pub struct Music;

#[derive(Component, Debug)]
pub struct Sfx;
