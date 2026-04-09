use crate::enemy::animate::animate_enemies_system;
use crate::enemy::death::enemy_death_system; // ajouter
use crate::enemy::movement::move_enemy_towards_player_system;
use crate::enemy::spawn::spawn_enemies;
use crate::exceptions::log_rtg_exception;
use bevy::prelude::*;
use crate::{GameState, InGameState};

pub(crate) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), spawn_enemies)
            .add_systems(Update, (
                animate_enemies_system.pipe(log_rtg_exception),
                move_enemy_towards_player_system.pipe(log_rtg_exception),
                enemy_death_system,
            )
            .run_if(in_state(GameState::InGame))
            .run_if(in_state(InGameState::Playing)));
    }
}
