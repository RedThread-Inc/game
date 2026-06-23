use crate::boss::{Boss, BossAttackTimer, BossProjectile, BossStats};
use crate::exceptions::RTGException;
use crate::InGameEntity;
use bevy::prelude::*;
use std::f32::consts::TAU;

pub(crate) fn move_boss_projectiles_system(
    time: Res<Time>,
    mut query: Query<(&BossProjectile, &mut Transform)>,
) {
    for (proj, mut transform) in query.iter_mut() {
        let delta = proj.velocity * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    }
}

pub(crate) fn cleanup_boss_projectiles_system(
    mut commands: Commands,
    query: Query<(Entity, &Transform), With<BossProjectile>>,
) {
    for (entity, transform) in query.iter() {
        if transform.translation.truncate().length() > 1200.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn fire_radial_burst(
    commands: &mut Commands,
    asset_server: &AssetServer,
    boss_pos: Vec3,
    stats: &BossStats,
) {
    let angle_step = TAU / stats.projectile_count as f32;
    let texture = asset_server.load("bossSpell.png");

    for i in 0..stats.projectile_count {
        let angle = angle_step * i as f32;
        let direction = Vec2::new(angle.cos(), angle.sin());
        let velocity = direction * stats.projectile_speed;

        // Rotation du sprite pour qu'il pointe dans la bonne direction
        let rotation = Quat::from_rotation_z(angle);

        commands.spawn((
            Sprite {
                image: texture.clone(),
                custom_size: Some(Vec2::splat(24.0)), // taille du sprite fireball
                ..default()
            },
            Transform {
                translation: boss_pos + Vec3::new(0.0, 0.0, 15.0),
                rotation,
                ..default()
            },
            BossProjectile {
                velocity,
                damage: 15.0,
            },
            InGameEntity,
        ));
    }
}

pub(crate) fn boss_attack_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
    mut boss_query: Query<(&Boss, &Transform, &mut BossAttackTimer)>,
) -> Result<(), RTGException> {
    for (boss, transform, mut timer) in boss_query.iter_mut() {
        timer.tick(time.delta());

        if timer.just_finished() {
            let stats = BossStats::for_round(boss.round);
            fire_radial_burst(
                &mut commands,
                &asset_server,
                transform.translation,
                &stats,
            );
        }
    }

    Ok(())
}