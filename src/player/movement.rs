use crate::enemy::Enemy;
use crate::player::{AnimationState, Facing, MOVE_SPEED, Player};
use crate::settings::GameSettings;
use bevy::input::ButtonInput;
use bevy::math::Vec2;
use bevy::prelude::{KeyCode, Res, Single, Time, Transform, With, Without};
use bevy_rapier2d::prelude::Velocity;
use crate::exceptions::RTGException;

pub(crate) fn move_player_system(
    input: Res<ButtonInput<KeyCode>>,
    settings: Res<GameSettings>,
    player: Single<(&mut Velocity, &mut AnimationState), (With<Player>, Without<Enemy>)>,
) -> Result<(), RTGException> {
    let (mut velocity, mut anim) = player.into_inner();

    let mut direction = Vec2::ZERO;

    if input.pressed(settings.key_left) || input.pressed(KeyCode::ArrowLeft) {
        direction.x -= 1.0;
        anim.facing = Facing::Left;
    }
    if input.pressed(settings.key_right) || input.pressed(KeyCode::ArrowRight) {
        direction.x += 1.0;
        anim.facing = Facing::Right;
    }
    if input.pressed(settings.key_up) || input.pressed(KeyCode::ArrowUp) {
        direction.y += 1.0;
        anim.facing = Facing::Up;
    }
    if input.pressed(settings.key_down) || input.pressed(KeyCode::ArrowDown) {
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

