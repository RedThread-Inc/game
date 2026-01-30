use crate::enemy::{AnimationState, Enemy, MOVE_SPEED as ENEMY_SPEED};
use crate::player::Player;
use bevy::prelude::*;
use crate::exceptions::RTGException;

fn move_enemy_towards_player_enemy(
    time: &Time,
    player_transform: &Transform,
    enemy_transform: &mut Transform,
    anim: &mut AnimationState,
) -> Result<(), RTGException> {
    let direction =
        (player_transform.translation - enemy_transform.translation).truncate();

    if direction.length() <= 2.0 {
        anim.moving = false;
        return Ok(());
    }

    if direction.length_squared() == 0.0 {
        return Err(RTGException::RTG_ENEMY_MOVEMENT_ZERO_DIRECTION_VECTOR);
    }

    let delta = direction.normalize() * ENEMY_SPEED * time.delta_secs();

    enemy_transform.translation.x += delta.x;
    enemy_transform.translation.y += delta.y;

    anim.moving = true;
    anim.facing = if delta.x.abs() > delta.y.abs() {
        if delta.x > 0.0 {
            crate::enemy::Facing::Right
        } else {
            crate::enemy::Facing::Left
        }
    } else {
        if delta.y > 0.0 {
            crate::enemy::Facing::Up
        } else {
            crate::enemy::Facing::Down
        }
    };

    Ok(())
}

pub(crate) fn move_enemy_towards_player_system(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &mut AnimationState), (With<Enemy>, Without<Player>)>,
) {
    let player_transform = match player_query.single() {
        Ok(t) => t,
        Err(bevy::ecs::query::QuerySingleError::NoEntities(_)) => {
            println!("[ERROR] - move_enemy_towards_player: no Player entity found");
            return;
        }
        Err(bevy::ecs::query::QuerySingleError::MultipleEntities(_)) => {
            println!("[ERROR] - move_enemy_towards_player: multiple Player entities found");
            return;
        }
    };

    for (mut transform, mut anim) in enemy_query.iter_mut() {
        if let Err(e) =
            move_enemy_towards_player_enemy(&time, player_transform, &mut transform, &mut anim)
        {
            println!("[ERROR] - enemy movement skipped: {:?}", e);
        }
    }
}


