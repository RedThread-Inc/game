use crate::enemy::Enemy;
use bevy::prelude::*;

pub(crate) fn enemy_death_system(
    mut commands: Commands,
    query: Query<(Entity, &Enemy)>,
) {
    for (entity, enemy) in query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}