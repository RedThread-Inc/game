use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::core::collision_groups::enemy_projectile_membership;
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
            RigidBody::Dynamic,
            Collider::ball(ENEMY_PROJ_RADIUS),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
            Velocity::default(),
            enemy_projectile_membership(),
            CollidingEntities::default(),
        ));

        ranged.fire_cooldown.reset();
    }
}

pub(crate) fn move_enemy_projectiles_system(
    mut commands: Commands,
    mut proj_query: Query<(Entity, &Transform, &mut Velocity, &EnemyProjectile)>,
) {
    for (entity, transform, mut velocity, proj) in proj_query.iter_mut() {
        velocity.linear = proj.direction * proj.speed;

        if transform.translation.x.abs() > 1500.0 || transform.translation.y.abs() > 1500.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub(crate) fn enemy_projectile_hit_player_system(
    mut commands: Commands,
    proj_query: Query<(Entity, &EnemyProjectile, &CollidingEntities)>,
    mut player_query: Query<&mut Player>,
) {
    for (proj_entity, proj, colliding) in proj_query.iter() {
        for target_entity in colliding.iter() {
            let Ok(mut player) = player_query.get_mut(target_entity) else { continue };

            if player.damage_cooldown.is_finished() {
                player.health -= proj.damage;
                player.damage_cooldown.reset();
            }
            commands.entity(proj_entity).despawn();
            break;
        }
    }
}