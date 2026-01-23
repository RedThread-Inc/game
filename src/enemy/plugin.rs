use bevy::prelude::*;
use crate::enemy::spawn::spawn_enemies;
use crate::enemy::animate::animate_enemies;
use crate::enemy::movement::move_enemy_towards_player;

pub(crate) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, (animate_enemies, move_enemy_towards_player));
    }
}
