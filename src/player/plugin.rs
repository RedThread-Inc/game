use crate::player::animate::animate_player_system;
use crate::player::movement::move_player_system;
use crate::player::spawn::spawn_player_system;
use bevy::prelude::*;
use crate::GameState;
use crate::exceptions::log_rtg_exception;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player_system.pipe(log_rtg_exception))
            .add_systems(Update, (move_player_system.pipe(log_rtg_exception), animate_player_system.pipe(log_rtg_exception)).run_if(in_state(GameState::InGame)));
    }
}
