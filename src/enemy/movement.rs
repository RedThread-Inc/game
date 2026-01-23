use crate::enemy::{AnimationState, Enemy, MOVE_SPEED as ENEMY_SPEED};
use crate::player::Player;
use bevy::prelude::*;

pub(crate) fn move_enemy_towards_player(
    time: Res<Time>,
    player_query: Query<&Transform, With<Player>>,
    mut enemy_query: Query<(&mut Transform, &mut AnimationState), (With<Enemy>, Without<Player>)>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    for (mut transform, mut anim) in enemy_query.iter_mut() {
        let direction = (player_transform.translation - transform.translation).truncate();

        if direction.length() > 2.0 {
            let delta = direction.normalize() * ENEMY_SPEED * time.delta().as_secs_f32();
            transform.translation.x += delta.x;
            transform.translation.y += delta.y;

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
        } else {
            anim.moving = false;
        }
    }
}
