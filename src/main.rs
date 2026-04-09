use bevy::prelude::States;

mod core;
mod enemy;
mod engine;
mod player;
mod map;
mod fight;
mod interface;
mod exceptions;
mod menu;

fn main() {
    engine::init_app();
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    MainMenu,
    InGame,
}