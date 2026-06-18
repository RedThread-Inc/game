use crate::boss::attack::{
    boss_attack_system, cleanup_boss_projectiles_system, move_boss_projectiles_system,
};
use crate::boss::damage::{boss_contact_damage_system, boss_projectile_damage_system};
use crate::boss::death::{BossDiedEvent, boss_death_system};
use crate::boss::movement_boss::boss_movement_system;
use crate::boss::spawn::spawn_boss;
use crate::exceptions::log_rtg_exception;
use crate::round::RoundStartedEvent;
use crate::{GameState, InGameState};
use bevy::prelude::*;

pub(crate) struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BossDiedEvent>()
            .add_systems(
                Update,
                spawn_boss
                    .run_if(on_message::<RoundStartedEvent>)
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(InGameState::Playing)),
            )
            .add_systems(
                Update,
                (
                    boss_movement_system.pipe(log_rtg_exception),
                    boss_attack_system.pipe(log_rtg_exception),
                    move_boss_projectiles_system,
                    cleanup_boss_projectiles_system,
                    boss_contact_damage_system,
                    boss_projectile_damage_system,
                    boss_death_system,
                )
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(InGameState::Playing)),
            );
    }
}
