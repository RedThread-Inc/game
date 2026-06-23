use crate::enemy::pathfinding::{seek_direction, AStarPath};
use crate::enemy::{AnimationState, Enemy, Facing, RangedEnemy, MOVE_SPEED as ENEMY_SPEED};
use crate::exceptions::RTGException;
use crate::map::generate::TerrainHeightMap;
use crate::player::Player;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

const SEPARATION_RADIUS: f32 = 48.0;
const SEPARATION_STRENGTH: f32 = 1.8;
const STOP_DISTANCE: f32 = 32.0;
const RANGED_KEEP_DISTANCE: f32 = 220.0;

fn compute_separation(my_pos: Vec2, all_positions: &[Vec2]) -> Vec2 {
    let mut separation = Vec2::ZERO;
    for &other_pos in all_positions {
        let diff = my_pos - other_pos;
        let dist = diff.length();
        if dist > 0.0 && dist < SEPARATION_RADIUS {
            separation += diff.normalize() * (1.0 - dist / SEPARATION_RADIUS);
        }
    }
    separation
}

fn facing_from_direction(delta: Vec2) -> Facing {
    if delta.x.abs() > delta.y.abs() {
        if delta.x > 0.0 { Facing::Right } else { Facing::Left }
    } else {
        if delta.y > 0.0 { Facing::Up } else { Facing::Down }
    }
}

pub(crate) fn move_enemy_towards_player_system(
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    terrain: Option<Res<TerrainHeightMap>>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&Transform, &mut Velocity, &mut AnimationState, &mut AStarPath), (With<Enemy>, Without<Player>, Without<RangedEnemy>)>
) -> Result<(), RTGException> {
    let player_transform = match player_query.single() {
        Ok(t) => t,
        Err(bevy::ecs::query::QuerySingleError::NoEntities(_)) => {
            return Err(RTGException::RTG_ENEMY_MOVE_TOWARDS_PLAYER_NOT_FOUND);
        }
        Err(bevy::ecs::query::QuerySingleError::MultipleEntities(_)) => {
            return Err(RTGException::RTG_ENEMY_MOVE_TOWARDS_PLAYER_MULTIPLE_PLAYER_FOUND);
        }
    };

    let player_pos = player_transform.translation.truncate();

    let Ok(window) = windows.single() else { return Ok(()); };
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;

    let positions: Vec<Vec2> = enemy_query
        .iter()
        .map(|(t, _, _, _)| t.translation.truncate())
        .collect();

    for (i, (transform, mut velocity, mut anim, mut path)) in enemy_query.iter_mut().enumerate() {
        let my_pos = transform.translation.truncate();
        let dist_to_player = my_pos.distance(player_pos);

        if dist_to_player <= STOP_DISTANCE {
            velocity.linear = Vec2::ZERO;
            anim.moving = false;
            continue;
        }

        let seek = seek_direction(
            &mut path, &time, my_pos, player_pos, terrain.as_deref(), half_w, half_h,
        );

        let other_positions: Vec<Vec2> = positions
            .iter()
            .enumerate()
            .filter(|(j, _)| *j != i)
            .map(|(_, p)| *p)
            .collect();
        let separation = compute_separation(my_pos, &other_positions);

        let combined = (seek + separation * SEPARATION_STRENGTH).normalize_or_zero();
        velocity.linear = combined * ENEMY_SPEED;

        anim.moving = true;
        anim.facing = facing_from_direction(combined);
    }

    Ok(())
}

pub(crate) fn move_ranged_enemy_system(
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    terrain: Option<Res<TerrainHeightMap>>,
    player_query: Query<&Transform, With<Player>>,
    mut ranged_query: Query<(&Transform, &mut Velocity, &mut AnimationState, &mut AStarPath), (With<Enemy>, With<RangedEnemy>, Without<Player>)>,
    all_enemy_query: Query<&Transform, (With<Enemy>, Without<Player>)>,
) -> Result<(), RTGException> {
    let player_transform = match player_query.single() {
        Ok(t) => t,
        Err(bevy::ecs::query::QuerySingleError::NoEntities(_)) => {
            return Err(RTGException::RTG_ENEMY_MOVE_TOWARDS_PLAYER_NOT_FOUND);
        }
        Err(bevy::ecs::query::QuerySingleError::MultipleEntities(_)) => {
            return Err(RTGException::RTG_ENEMY_MOVE_TOWARDS_PLAYER_MULTIPLE_PLAYER_FOUND);
        }
    };

    let player_pos = player_transform.translation.truncate();

    let Ok(window) = windows.single() else { return Ok(()); };
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;

    let all_positions: Vec<Vec2> = all_enemy_query
        .iter()
        .map(|t| t.translation.truncate())
        .collect();

    for (transform, mut velocity, mut anim, mut path) in ranged_query.iter_mut() {
        let my_pos = transform.translation.truncate();
        let to_player = player_pos - my_pos;
        let dist = to_player.length();

        let separation = compute_separation(my_pos, &all_positions);

        let desired = if dist > RANGED_KEEP_DISTANCE {
            seek_direction(&mut path, &time, my_pos, player_pos, terrain.as_deref(), half_w, half_h)
        } else if dist < RANGED_KEEP_DISTANCE * 0.6 {
            -to_player.normalize_or_zero()
        } else {
            Vec2::ZERO
        };

        let combined = (desired + separation * SEPARATION_STRENGTH).normalize_or_zero();

        if combined.length_squared() < 0.01 {
            velocity.linear = Vec2::ZERO;
            anim.moving = false;
        } else {
            velocity.linear = combined * ENEMY_SPEED * 0.75;
            anim.moving = true;
            anim.facing = facing_from_direction(combined);
        }
    }

    Ok(())
}