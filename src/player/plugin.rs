use crate::player::animate::animate_player_system;
use crate::player::movement::move_player_system;
use crate::player::projectile::{attach_fire_cooldown, move_projectiles_system, shoot_projectile_system};
use crate::player::spawn::spawn_player_system;
use bevy::prelude::*;
use crate::exceptions::log_rtg_exception;

pub(crate) struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (
            spawn_player_system.pipe(log_rtg_exception),
            attach_fire_cooldown,
        ).chain())
            .add_systems(Update, (
                move_player_system.pipe(log_rtg_exception),
                animate_player_system.pipe(log_rtg_exception),
                shoot_projectile_system.pipe(log_rtg_exception),
                move_projectiles_system.pipe(log_rtg_exception),
            ));
    }
}