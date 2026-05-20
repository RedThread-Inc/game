use crate::boss::{Boss, BOSS_MOVE_SPEED};
use crate::exceptions::RTGException;
use crate::player::Player;
use bevy::prelude::*;

fn move_boss_towards_player(
    time: &Time,
    player_pos: Vec3,
    boss_transform: &mut Transform,
) -> Result<(), RTGException> {
    let direction = (player_pos - boss_transform.translation).truncate();

    if direction.length() <= 80.0 {
        // Le boss garde une distance minimale : il n'écrase pas le joueur
        return Ok(());
    }

    if direction.length_squared() == 0.0 {
        return Err(RTGException::RTG_ENEMY_MOVEMENT_ZERO_DIRECTION_VECTOR);
    }

    let delta = direction.normalize() * BOSS_MOVE_SPEED * time.delta_secs();
    boss_transform.translation.x += delta.x;
    boss_transform.translation.y += delta.y;

    Ok(())
}

pub(crate) fn boss_movement_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut boss_query: Query<&mut Transform, (With<Boss>, Without<Player>)>,
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

    for mut transform in boss_query.iter_mut() {
        if let Err(e) = move_boss_towards_player(&time, player_transform.translation, &mut transform) {
            println!("{}", e.to_string());
        }
    }

    Ok(())
}