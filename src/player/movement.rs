use crate::enemy::Enemy;
use crate::player::{AnimationState, Facing, MOVE_SPEED, Player};
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{KeyCode, Res, Single, Time, Transform, With, Without};
use crate::exceptions::RTGException;

pub(crate) fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(&mut Transform, &mut AnimationState), (With<Player>, Without<Enemy>)>,
) -> Result<(), RTGException> {
    let (mut transform, mut anim) = player.into_inner();

    let mut direction = Vec2::ZERO;

    if input.pressed(KeyCode::ArrowLeft) || input.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
        anim.facing = Facing::Left;
    }
    if input.pressed(KeyCode::ArrowRight) || input.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
        anim.facing = Facing::Right;
    }
    if input.pressed(KeyCode::ArrowUp) || input.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
        anim.facing = Facing::Up;
    }
    if input.pressed(KeyCode::ArrowDown) || input.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
        anim.facing = Facing::Down;
    }

    anim.moving = direction != Vec2::ZERO;

    if anim.moving {
        let delta = direction.normalize() * MOVE_SPEED * time.delta_secs();
        if time.delta_secs() <= 0.0 {
            return Err(RTGException::RTG_PLAYER_MOVEMENT_DELTA_TIME_INVALID);
        }
        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    }
    Ok(())
}

pub(crate) fn move_player_system(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    player: Single<(&mut Transform, &mut AnimationState), (With<Player>, Without<Enemy>)>,
) {
     if let Err(e) = move_player(input, time, player) {
         println!("[ERROR] - Player movement error: {:?}", e)
     }
}


