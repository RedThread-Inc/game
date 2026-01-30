use bevy::prelude::*;
use crate::fight::enemy_hits_player_system;
use crate::interface::player_health_bar::*;

pub struct InterfacePlugin;

impl Plugin for InterfacePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_health_bar);
        app.add_systems(Update, update_player_health_bar);
        app.add_systems(Update, enemy_hits_player_system);
    }
}
