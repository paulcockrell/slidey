use bevy::prelude::*;

use crate::{
    map::{spawn_assets, spawn_map, AssetMap, TileMap},
    Level,
};

use super::{despawn_screen, GameState};

#[derive(Component)]
pub struct OnGameScreen;

#[derive(Component)]
pub struct OnGameLoad;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameState::GameSetup),
            (spawn_map, spawn_assets, game_setup_complete).chain(),
        )
        // .add_systems(Update, game.run_if(in_state(GameState::GamePlay)))
        .add_systems(OnExit(GameState::GamePlay), despawn_screen::<TileMap>)
        .add_systems(OnExit(GameState::GamePlay), despawn_screen::<AssetMap>)
        .add_systems(OnExit(GameState::GamePlay), game_setup);
    }
}

fn game_setup(mut commands: Commands, level: Res<Level>) {
    let next_level = level.next();
    // TODO: This doesn't appear to be setting (overwritting) existing level ?
    if next_level.to_number() < 10 {
        println!("Loading level {:?}", next_level);
        commands.insert_resource(level.next());
    }
    // commands.remove_resource::<Level>();
    // commands.insert_resource(next_level);
}

fn game_setup_complete(mut game_state: ResMut<NextState<GameState>>) {
    game_state.set(GameState::GamePlay);
}

// fn game() {}
