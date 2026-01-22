use bevy::prelude::{App, Plugin, Startup, Update};
use crate::Player::animate::animate_player;
use crate::Player::movement::move_player;
use crate::Player::spawn::spawn_player;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, (move_player, animate_player));
    }
}