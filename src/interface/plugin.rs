use bevy::prelude::*;
use crate::GameState;
use crate::interface::player_health_bar::*;

pub(crate) struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_player_health_bar);
        app.add_systems(Update, update_player_health_bar.run_if(in_state(GameState::InGame)));
    }
}
