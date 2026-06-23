use crate::exceptions::log_rtg_exception;
use crate::player::Player;
use crate::player::animate::animate_player_system;
use crate::player::fight::{PlayerDamageSoundState, player_fight_system, tick_player_cooldowns};
use crate::player::movement::move_player_system;
use crate::player::projectile::{
    attach_fire_cooldown, move_projectiles_system, player_projectile_hit_system,
    shoot_projectile_system,
};
use crate::player::spawn::spawn_player_system;
use crate::{GameState, InGameState};
use bevy::prelude::*;

pub(crate) struct PlayerPlugin;

pub fn check_player_death_system(
    player_query: Query<&Player>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if let Ok(player) = player_query.single() {
        if player.health <= 0.0 {
            next_state.set(GameState::GameOver);
        }
    }
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerDamageSoundState>()
            .add_systems(
                OnEnter(GameState::InGame),
                (
                    spawn_player_system.pipe(log_rtg_exception),
                    attach_fire_cooldown,
                )
                    .chain(),
            )
            .add_systems(
                Update,
                (
                    tick_player_cooldowns,
                    (
                        move_player_system.pipe(log_rtg_exception),
                        animate_player_system.pipe(log_rtg_exception),
                        player_fight_system.pipe(log_rtg_exception),
                    ),
                    check_player_death_system,
                    shoot_projectile_system.pipe(log_rtg_exception),
                    move_projectiles_system.pipe(log_rtg_exception),
                    player_projectile_hit_system,
                )
                    .chain()
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(InGameState::Playing)),
            );
    }
}
