use crate::player::animate::animate_player_system;
use crate::player::movement::move_player_system;
use crate::player::spawn::spawn_player_system;
use bevy::prelude::*;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_system)
            .add_systems(Update, (move_player_system, animate_player_system));
    }
}
