use crate::boss::attack::{
    boss_attack_system, cleanup_boss_projectiles_system, move_boss_projectiles_system,
};
use crate::boss::damage::{boss_contact_damage_system, boss_projectile_damage_system};
use crate::boss::death::{boss_death_system, BossDiedEvent};
use crate::boss::movement::boss_movement_system;
use crate::boss::spawn::spawn_boss;
use crate::exceptions::log_rtg_exception;
use crate::{GameState, InGameState};
use bevy::prelude::*;

pub(crate) struct BossPlugin;

impl Plugin for BossPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(InGameState::Playing), spawn_boss)
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