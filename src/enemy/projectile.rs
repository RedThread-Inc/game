use bevy::prelude::*;
use crate::enemy::RangedEnemy;
use crate::player::Player;

const ENEMY_PROJ_SPEED: f32 = 150.0;
const ENEMY_PROJ_DAMAGE: f32 = 10.0;
const ENEMY_PROJ_RADIUS: f32 = 6.0;
pub(crate) const ENEMY_ATTACK_RANGE: f32 = 1000.0;

#[derive(Component)]
pub(crate) struct EnemyProjectile {
    pub(crate) direction: Vec2,
    pub(crate) speed: f32,
    pub(crate) damage: f32,
    pub(crate) radius: f32,
}

pub(crate) fn enemy_shoot_system(
    mut commands: Commands,
    time: Res<Time>,
    mut enemy_query: Query<(&Transform, &mut RangedEnemy), Without<Player>>,
    player_query: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let Ok(player_transform) = player_query.single() else { return };
    let player_pos = player_transform.translation.truncate();

    for (enemy_transform, mut ranged) in enemy_query.iter_mut() {
        let enemy_pos = enemy_transform.translation.truncate();

        if enemy_pos.distance(player_pos) > ranged.attack_range {
            continue;
        }

        ranged.fire_cooldown.tick(time.delta());
        if !ranged.fire_cooldown.just_finished() {
            continue;
        }

        let dir = (player_pos - enemy_pos).normalize_or_zero();
        if dir == Vec2::ZERO {
            continue;
        }

        commands.spawn((
            Sprite {
                image: asset_server.load("ennemyFireball.png"),
                custom_size: Some(Vec2::splat(20.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                enemy_transform.translation.x,
                enemy_transform.translation.y,
                24.0,
            )),
            EnemyProjectile {
                direction: dir,
                speed: ENEMY_PROJ_SPEED,
                damage: ENEMY_PROJ_DAMAGE,
                radius: ENEMY_PROJ_RADIUS,
            },
        ));

        ranged.fire_cooldown.reset();
    }
}

pub(crate) fn move_enemy_projectiles_system(
    mut commands: Commands,
    time: Res<Time>,
    mut proj_query: Query<(Entity, &mut Transform, &EnemyProjectile), Without<Player>>,
) {
    for (entity, mut transform, proj) in proj_query.iter_mut() {
        let delta = proj.direction * proj.speed * time.delta_secs();
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;

        // Despawn if off-screen
        if transform.translation.x.abs() > 1500.0 || transform.translation.y.abs() > 1500.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn enemy_projectile_hit_player_system(
    mut commands: Commands,
    mut player_query: Query<(&Transform, &mut Player)>,
    proj_query: Query<(Entity, &Transform, &EnemyProjectile)>,
) {
    let Ok((player_transform, mut player)) = player_query.single_mut() else { return };
    let player_pos = player_transform.translation.truncate();

    for (entity, proj_transform, proj) in proj_query.iter() {
        let proj_pos = proj_transform.translation.truncate();
        let distance = player_pos.distance(proj_pos);

        if distance < proj.radius + 16.0 {
            if player.damage_cooldown.is_finished() {
                player.health -= proj.damage;
                player.damage_cooldown.reset();
            }
            commands.entity(entity).despawn();
        }
    }
}