use crate::enemy::Enemy;
use crate::player::{AnimationState, Facing, MOVE_SPEED, Player};
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{KeyCode, Res, Single, Time, Transform, With, Without};
use bevy_rapier2d::prelude::Velocity;
use crate::exceptions::RTGException;

pub(crate) fn move_player_system(
    input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Velocity, &mut AnimationState), (With<Player>, Without<Enemy>)>,
) -> Result<(), RTGException> {
    let (mut velocity, mut anim) = player.into_inner();

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

    *velocity = if anim.moving {
        Velocity::linear(direction.normalize() * MOVE_SPEED)
    } else {
        Velocity::zero()
    };


    Ok(())
}

