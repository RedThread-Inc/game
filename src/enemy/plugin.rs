use crate::enemy::animate::animate_enemies_system;
use crate::enemy::movement::move_enemy_towards_player_system;
use crate::enemy::spawn::spawn_enemies;
use bevy::prelude::*;

pub(crate) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, (animate_enemies_system, move_enemy_towards_player_system));
    }
}
