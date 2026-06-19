use crate::boss::{Boss, BOSS_MOVE_SPEED};
use crate::enemy::pathfinding::{seek_direction, AStarPath};
use crate::enemy::Enemy;
use crate::exceptions::RTGException;
use crate::map::generate::TerrainHeightMap;
use crate::player::Player;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;

const BOSS_STOP_DISTANCE: f32 = 80.0;
const BOSS_SEPARATION_RADIUS: f32 = 80.0;
const BOSS_SEPARATION_STRENGTH: f32 = 1.5;

pub(crate) fn boss_movement_system(
    time: Res<Time>,
    windows: Query<&Window, With<PrimaryWindow>>,
    terrain: Option<Res<TerrainHeightMap>>,
    player_query: Query<&Transform, With<Player>>,
    mut boss_query: Query<(&Transform, &mut Velocity, &mut AStarPath), (With<Boss>, Without<Player>)>,
    enemy_query: Query<&Transform, (With<Enemy>, Without<Boss>, Without<Player>)>,
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

    let enemy_positions: Vec<Vec2> = enemy_query
        .iter()
        .map(|t| t.translation.truncate())
        .collect();

    for (boss_transform, mut velocity, mut path) in boss_query.iter_mut() {
        let my_pos = boss_transform.translation.truncate();
        let dist = my_pos.distance(player_pos);

        if dist <= BOSS_STOP_DISTANCE {
            velocity.linear = Vec2::ZERO;
            continue;
        }

        let seek = seek_direction(
            &mut path, &time, my_pos, player_pos, terrain.as_deref(), half_w, half_h,
        );

        let mut separation = Vec2::ZERO;
        for &ep in &enemy_positions {
            let diff = my_pos - ep;
            let d = diff.length();
            if d > 0.0 && d < BOSS_SEPARATION_RADIUS {
                separation += diff.normalize() * (1.0 - d / BOSS_SEPARATION_RADIUS);
            }
        }

        let combined = (seek + separation * BOSS_SEPARATION_STRENGTH).normalize_or_zero();
        velocity.linear = combined * BOSS_MOVE_SPEED;
    }

    Ok(())
}