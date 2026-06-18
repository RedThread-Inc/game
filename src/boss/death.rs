use crate::boss::{Boss, BossProjectile};
use bevy::prelude::*;

#[derive(Message)]
pub(crate) struct BossDiedEvent;

pub(crate) fn boss_death_system(
    mut commands: Commands,
    mut writer: MessageWriter<BossDiedEvent>,
    query: Query<(Entity, &Boss)>,
) {
    for (entity, boss) in query.iter() {
        if boss.health <= 0.0 {
            commands.entity(entity).despawn();
            writer.write(BossDiedEvent);
        }
    }
}