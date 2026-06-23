use bevy::prelude::*;

mod audio;
mod core;
mod enemy;
mod engine;
mod player;
mod map;
mod interface;
mod exceptions;
mod menu;
mod locale;
mod round;
mod settings;
mod upgrade;
mod boss;

fn main() {
    engine::init_app();
}

#[derive(Component)]
pub struct InGameEntity;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    NameEntry,
    Lore,
    InGame,
    Restarting,
    GameOver,
    Settings,
}

#[derive(SubStates, Debug, Clone, PartialEq, Eq, Hash, Default)]
#[source(GameState = GameState::InGame)]
pub enum InGameState {
    #[default]
    Playing,
    Paused,
    Restarting,
    ChoosingUpgrade,
}

#[derive(Resource, Default)]
pub struct PlayerName(pub String);