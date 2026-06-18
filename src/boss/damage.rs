use crate::boss::{Boss, BossProjectile, BOSS_SCALE, BOSS_TILE_SIZE};
use crate::player::Player;
use bevy::prelude::*;

pub(crate) const BOSS_HITBOX: Vec2 = Vec2::new(
    BOSS_TILE_SIZE as f32 * BOSS_SCALE * 0.6,
    BOSS_TILE_SIZE as f32 * BOSS_SCALE * 0.6,
);
const PLAYER_HITBOX: Vec2 = Vec2::new(20.0, 20.0);
const PROJECTILE_HITBOX: Vec2 = Vec2::new(16.0, 16.0);

fn aabb_overlap(pos_a: Vec2, size_a: Vec2, pos_b: Vec2, size_b: Vec2) -> bool {
    let half_a = size_a / 2.0;
    let half_b = size_b / 2.0;
    (pos_a.x - half_a.x) < (pos_b.x + half_b.x)
        && (pos_a.x + half_a.x) > (pos_b.x - half_b.x)
        && (pos_a.y - half_a.y) < (pos_b.y + half_b.y)
        && (pos_a.y + half_a.y) > (pos_b.y - half_b.y)
}

/// Dégâts au contact direct du boss
pub(crate) fn boss_contact_damage_system(
    time: Res<Time>,
    mut player_query: Query<(&Transform, &mut Player)>,
    boss_query: Query<&Transform, With<Boss>>,
) {
    let Ok((player_transform, mut player)) = player_query.single_mut() else {
        return;
    };

    if !player.damage_cooldown.just_finished() {
        return;
    }

    let player_pos = player_transform.translation.truncate();

    for boss_transform in boss_query.iter() {
        let boss_pos = boss_transform.translation.truncate();
        let dist = player_pos.distance(boss_pos);
        println!("distance joueur-boss: {}", dist);
        if aabb_overlap(player_pos, PLAYER_HITBOX, boss_pos, BOSS_HITBOX) {
            player.health -= 20.0;
            player.damage_cooldown.reset();
            return;
        }
    }
}

/// Dégâts des projectiles du boss
pub(crate) fn boss_projectile_damage_system(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player)>,
    projectile_query: Query<(Entity, &Transform, &BossProjectile)>,
) {
    let Ok((player_transform, mut player)) = player_query.single_mut() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();

    for (entity, proj_transform, projectile) in projectile_query.iter() {
        let proj_pos = proj_transform.translation.truncate();

        if aabb_overlap(player_pos, PLAYER_HITBOX, proj_pos, PROJECTILE_HITBOX) {
            player.health -= projectile.damage;
            commands.entity(entity).despawn();
        }
    }
}