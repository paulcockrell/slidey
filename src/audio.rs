use bevy::audio::PlaybackMode;
use bevy::prelude::*;

use crate::asset_loader::AudioAssets;
use crate::movement::PlayerState;

pub struct AudioPlugin;

impl Plugin for AudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, pause)
            .add_systems(OnEnter(PlayerState::Teleport), play_teleport_sfx)
            .add_systems(OnEnter(PlayerState::CollectPotion), play_collect_potion_sfx);
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
