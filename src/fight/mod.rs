use bevy::prelude::*;
use crate::enemy::Enemy;
use crate::enemy::projectile::EnemyProjectile;
use crate::player::Player;

pub(crate) fn enemy_hits_player_system(
    time: Res<Time>,
    mut player_query: Query<(&Transform, &mut Player)>,
    enemy_query: Query<(&Transform, &Enemy)>,
) {
    let Ok((player_transform, mut player)) = player_query.single_mut() else {
        return;
    };

    player.damage_cooldown.tick(time.delta());

    if player.damage_cooldown.elapsed_secs() < player.damage_cooldown.duration().as_secs_f32() {
        return;
    }

    for (enemy_transform, enemy) in enemy_query.iter() {
        if player_transform.translation.distance(enemy_transform.translation) < 28.0 {
            player.health -= enemy.damage;
            player.damage_cooldown.reset();
            break;
        }
    }
}

pub(crate) fn enemy_projectile_hits_player_system(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player)>,
    proj_query: Query<(Entity, &Transform, &EnemyProjectile)>,
) {
    let Ok((player_transform, mut player)) = player_query.single_mut() else {
        return;
    };

    for (entity, proj_transform, proj) in proj_query.iter() {
        let distance = proj_transform
            .translation
            .truncate()
            .distance(player_transform.translation.truncate());

        if distance <= proj.radius + 16.0 {
            player.health -= proj.damage;
            commands.entity(entity).despawn();
        }
    }
}