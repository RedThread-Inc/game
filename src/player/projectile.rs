use crate::enemy::Enemy;
use crate::exceptions::RTGException;
use crate::player::Player;
use bevy::prelude::*;

const PROJECTILE_SPEED: f32 = 300.0;
const PROJECTILE_RADIUS: f32 = 8.0;
const PROJECTILE_DAMAGE: f32 = 20.0;
const FIRE_RATE: f32 = 1.0;

#[derive(Component)]
pub(crate) struct Projectile {
    pub(crate) direction: Vec2,
    pub(crate) damage: f32,
}

#[derive(Component)]
pub(crate) struct FireCooldown(pub(crate) Timer);

pub(crate) fn attach_fire_cooldown(
    mut commands: Commands,
    query: Query<Entity, With<Player>>,
) {
    let Ok(entity) = query.single() else { return };
    let mut t = Timer::from_seconds(FIRE_RATE, TimerMode::Once);
    t.tick(std::time::Duration::from_secs_f32(FIRE_RATE)); // déjà prêt à tirer
    commands.entity(entity).insert(FireCooldown(t));
}

pub(crate) fn shoot_projectile_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(&Transform, &mut FireCooldown), With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    asset_server: Res<AssetServer>,
) -> Result<(), RTGException> {
    let Ok((player_transform, mut cooldown)) = player_query.single_mut() else {
        println!("DEBUG: player introuvable");
        return Err(RTGException::RTG_PLAYER_ANIMATION_CANT_LOAD);
    };

    cooldown.0.tick(time.delta());
    println!("DEBUG: cooldown elapsed={:.2} finished={}", cooldown.0.elapsed_secs(), cooldown.0.is_finished());

    if !cooldown.0.is_finished() {
        return Ok(());
    }

    let player_pos = player_transform.translation.truncate();
    println!("DEBUG: player_pos={:?}", player_pos);

    let closest = enemy_query
        .iter()
        .map(|t| t.translation.truncate())
        .min_by(|a, b| {
            a.distance(player_pos)
                .partial_cmp(&b.distance(player_pos))
                .unwrap()
        });

    let Some(enemy_pos) = closest else {
        println!("DEBUG: aucun ennemi trouvé");
        return Ok(());
    };

    println!("DEBUG: ennemi trouvé à {:?}, spawn projectile", enemy_pos);

    let direction = (enemy_pos - player_pos).normalize();

    commands.spawn((
        Sprite {
            image: asset_server.load("fireBall.png"),
            custom_size:  Some(Vec2::splat(30.0)),
            ..default()
        },
        Transform::from_translation(Vec3::new(
            player_transform.translation.x,
            player_transform.translation.y,
            25.0,
        )),
        Projectile {
            direction,
            damage: PROJECTILE_DAMAGE,
        },
    ));

    cooldown.0.reset();
    println!("DEBUG: projectile spawné !");
    Ok(())
}

pub(crate) fn move_projectiles_system(
    mut commands: Commands,
    time: Res<Time>,
    mut projectile_query: Query<(Entity, &mut Transform, &Projectile)>,
    mut enemy_query: Query<(Entity, &Transform, &mut Enemy), Without<Projectile>>,
) -> Result<(), RTGException> {
    for (proj_entity, mut proj_transform, projectile) in projectile_query.iter_mut() {
        let delta = projectile.direction * PROJECTILE_SPEED * time.delta_secs();
        proj_transform.translation.x += delta.x;
        proj_transform.translation.y += delta.y;

        for (enemy_entity, enemy_transform, mut enemy) in enemy_query.iter_mut() {
            let distance = proj_transform
                .translation
                .truncate()
                .distance(enemy_transform.translation.truncate());

            if distance <= PROJECTILE_RADIUS + 32.0 {
                enemy.health -= projectile.damage;
                println!("Ennemi touché ! HP restants: {}", enemy.health);
                commands.entity(proj_entity).despawn();
                break;
            }
        }
    }

    Ok(())
}