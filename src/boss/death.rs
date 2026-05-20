use crate::boss::{Boss, BossProjectile};
use bevy::prelude::*;

#[derive(Event)]
pub(crate) struct BossDiedEvent;

pub(crate) fn boss_death_system(
    mut commands: Commands,
    query: Query<(Entity, &Boss)>,
    projectiles: Query<Entity, With<BossProjectile>>,
) {
    for (entity, boss) in query.iter() {
        if boss.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}