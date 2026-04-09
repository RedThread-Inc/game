use bevy::prelude::*;

mod core;
mod enemy;
mod engine;
mod player;
mod map;
mod fight;
mod interface;
mod exceptions;
mod menu;
mod round;
mod upgrade;

fn main() {
    engine::init_app();
}

#[derive(Component)]
pub struct InGameEntity;

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
    Restarting,
    GameOver,
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