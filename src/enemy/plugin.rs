use crate::enemy::animate::animate_enemies_system;
use crate::enemy::death::enemy_death_system;
use crate::enemy::movement::move_enemy_towards_player_system;
use crate::enemy::projectile::{
    enemy_projectile_hit_player_system, enemy_shoot_system, move_enemy_projectiles_system,
};
use crate::enemy::spawn::spawn_enemies;
use crate::exceptions::log_rtg_exception;
use crate::player::fight::pickup_potion_system;
use crate::round::RoundStartedEvent;
use crate::{GameState, InGameState};
use bevy::prelude::*;

pub(crate) struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            spawn_enemies
                .run_if(on_message::<RoundStartedEvent>)
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(InGameState::Playing)),
        )
        .add_systems(
            Update,
            (
                animate_enemies_system,
                move_enemy_towards_player_system.pipe(log_rtg_exception),
                enemy_death_system,
                pickup_potion_system,
                enemy_shoot_system,
                enemy_projectile_hit_player_system,
                move_enemy_projectiles_system,
            )
                .run_if(in_state(GameState::InGame))
                .run_if(in_state(InGameState::Playing)),
        );
    }
}
