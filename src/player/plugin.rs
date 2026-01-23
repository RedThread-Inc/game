use crate::player::animate::animate_player;
use crate::player::movement::move_player;
use crate::player::spawn::spawn_player;
use bevy::prelude::*;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, animate_player));
    }
}
