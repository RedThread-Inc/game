use crate::enemy::animate::animate_enemies;
use crate::enemy::movement::move_enemy_towards_player;
use crate::enemy::spawn::spawn_enemies;
use bevy::prelude::*;

pub(crate) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_enemies)
            .add_systems(Update, (animate_enemies, move_enemy_towards_player));
    }
}
