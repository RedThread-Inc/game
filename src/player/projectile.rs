use crate::audio::{maybe_play, SoundCooldowns};
use crate::enemy::Enemy;
use crate::exceptions::RTGException;
use crate::player::Player;
use crate::upgrade::PlayerUpgrades;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::boss::Boss;
use crate::core::collision_groups::player_projectile_membership;

const PROJECTILE_SPEED: f32 = 300.0;
const PROJECTILE_RADIUS: f32 = 8.0;
const PROJECTILE_DAMAGE: f32 = 20.0;
const FIRE_RATE: f32 = 1.0;
const SPREAD_ANGLE: f32 = 20.0;

#[derive(Component)]
pub(crate) struct Projectile {
    pub(crate) direction: Vec2,
    pub(crate) damage: f32,
    pub(crate) speed: f32,
    pub(crate) radius: f32,
}

#[derive(Component)]
pub(crate) struct FireCooldown(pub(crate) Timer);

pub(crate) fn attach_fire_cooldown(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
) {
    let Ok(entity) = query.single() else { return };
    let mut t = Timer::from_seconds(FIRE_RATE, TimerMode::Once);
    t.tick(std::time::Duration::from_secs_f32(FIRE_RATE));
    commands.entity(entity).insert(FireCooldown(t));
}

pub(crate) fn shoot_projectile_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(&Transform, &mut FireCooldown), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    mut boss_query: Query<(&Transform, &mut Boss)>,
    asset_server: Res<AssetServer>,
    upgrades: Res<PlayerUpgrades>,
) -> Result<(), RTGException> {
    let Ok((player_transform, mut cooldown)) = player_query.single_mut() else {
        return Err(RTGException::RTG_PLAYER_FIGHT_CANT_LOAD_PLAYER);
    };

    cooldown.0.tick(time.delta());
    if !cooldown.0.is_finished() {
        return Ok(());
    }

    let player_pos = player_transform.translation.truncate();

    let closest_enemy = enemy_query
        .iter()
        .map(|t| t.translation.truncate())
        .min_by(|a, b| {
            a.distance(player_pos)
                .partial_cmp(&b.distance(player_pos))
                .unwrap()
        });

    // Get boss position if it exists
    let boss_pos = boss_query.iter().next().map(|(t, _)| t.translation.truncate());

    // Determine which is closer
    let target_pos = match (closest_enemy, boss_pos) {
        (Some(enemy), Some(boss)) => {
            if enemy.distance(player_pos) <= boss.distance(player_pos) {
                enemy
            } else {
                boss
            }
        }
        (Some(enemy), None) => enemy,
        (None, Some(boss)) => boss,
        (None, None) => return Ok(()),
    };

    let base_dir = (target_pos - player_pos).normalize_or_zero();
    if base_dir == Vec2::ZERO {
        return Ok(());
    }

    let total = 1 + upgrades.extra_projectiles;
    let spread = SPREAD_ANGLE.to_radians();

    maybe_play(&mut commands, &asset_server, "player_attack.ogg", 0.2);

    for i in 0..total {
        let angle_offset = if total == 1 {
            0.0
        } else {
            (i as f32 - (total as f32 - 1.0) / 2.0) * spread
        };
        let dir = rotate_vec2(base_dir, angle_offset);

        commands.spawn((
            Sprite {
                image: asset_server.load("fireball.png"),
                custom_size: Some(Vec2::splat(30.0)),
                ..default()
            },
            Transform::from_translation(Vec3::new(
                player_transform.translation.x,
                player_transform.translation.y,
                25.0,
            )),
            Projectile {
                direction: dir,
                damage: PROJECTILE_DAMAGE * upgrades.damage_multiplier(),
                speed: PROJECTILE_SPEED * upgrades.speed_multiplier(),
                radius: PROJECTILE_RADIUS + upgrades.radius_bonus(),
            },
            RigidBody::Dynamic,
            Collider::ball(PROJECTILE_RADIUS),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
            GravityScale(0.0),
            LockedAxes::ROTATION_LOCKED,
            Velocity::default(),
            player_projectile_membership(),
            CollidingEntities::default(),
        ));
    }

    let new_duration = std::time::Duration::from_secs_f32(
        (FIRE_RATE * upgrades.fire_rate_multiplier()).max(0.1),
    );
    cooldown.0.set_duration(new_duration);
    cooldown.0.reset();

    Ok(())
}

pub(crate) fn move_projectiles_system(
    mut commands: Commands,
    mut projectile_query: Query<(Entity, &Transform, &mut Velocity, &Projectile)>,
) -> Result<(), RTGException> {
    for (entity, transform, mut velocity, projectile) in projectile_query.iter_mut() {
        velocity.linear = projectile.direction * projectile.speed;

        if transform.translation.x.abs() > 1500.0 || transform.translation.y.abs() > 1500.0 {
            commands.entity(entity).despawn();
        }
    }
    Ok(())
}

pub(crate) fn player_projectile_hit_system(
    mut commands: Commands,
    projectile_query: Query<(Entity, &Projectile, &CollidingEntities)>,
    mut enemy_query: Query<&mut Enemy>,
    mut boss_query: Query<&mut Boss>,
    asset_server: Res<AssetServer>,
    mut sound_cooldowns: ResMut<SoundCooldowns>,
) {
    for (proj_entity, projectile, colliding) in projectile_query.iter() {
        for target_entity in colliding.iter() {
            if let Ok(mut enemy) = enemy_query.get_mut(target_entity) {
                enemy.health -= projectile.damage;
                commands.entity(proj_entity).despawn();
                if sound_cooldowns.enemy_hit.is_finished() {
                    if maybe_play(&mut commands, &asset_server, "squelette_triso_damage.ogg", 0.20) {
                        sound_cooldowns.enemy_hit.reset();
                    }
                }
                break;
            } else if let Ok(mut boss) = boss_query.get_mut(target_entity) {
                boss.health -= projectile.damage;
                commands.entity(proj_entity).despawn();
                if sound_cooldowns.enemy_hit.is_finished() {
                    if maybe_play(&mut commands, &asset_server, "squelette_triso_damage.ogg", 0.20) {
                        sound_cooldowns.enemy_hit.reset();
                    }
                }
                break;
            }
        }
    }
}

fn rotate_vec2(v: Vec2, angle: f32) -> Vec2 {
    let (sin, cos) = angle.sin_cos();
    Vec2::new(v.x * cos - v.y * sin, v.x * sin + v.y * cos)
}
