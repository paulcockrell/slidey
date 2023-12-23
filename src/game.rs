use bevy::prelude::*;

use crate::map::{spawn_assets, spawn_map};

use super::{despawn_screen, GameState};

#[derive(Component)]
pub struct OnGameScreen;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::Game),
            (game_setup, spawn_map, spawn_assets).chain(),
        )
        .add_systems(Update, game.run_if(in_state(GameState::Game)))
        .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

fn game_setup() {
    println!("Game setup goes here, maybe setting the level...");
}

fn game() {}
