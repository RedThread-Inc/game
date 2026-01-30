
use crate::player::*;
use bevy::prelude::*;
use crate::exceptions::RTGException;

pub(crate) fn animate_player(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut AnimationTimer, &mut Sprite), With<Player>>,
) -> Result<(), RTGException> {
    let Ok((mut anim, mut timer, mut sprite)) = query.single_mut() else {
        return Err(RTGException::RTG_PLAYER_ANIMATION_CANT_LOAD);
    };

    let atlasw: std::result::Result<&mut _, RTGException> = match sprite.texture_atlas.as_mut() {
        Some(a) => Ok(a),
        None => return Err(RTGException::RTG_PLAYER_ANIMATION_TEXTURE_ATLAS_CANT_LOAD),
    };

    let atlas = atlasw.unwrap();

    let target_row = row_zero_based(anim.facing);
    let mut current_col = atlas.index % WALK_FRAMES;
    let mut current_row = atlas.index / WALK_FRAMES;

    if current_row != target_row {
        atlas.index = row_start_index(anim.facing);
        current_col = 0;
        current_row = target_row;
        timer.reset();
    }

    let just_started = anim.moving && !anim.was_moving;
    let just_stopped = !anim.moving && anim.was_moving;

    if anim.moving {
        if just_started {
            let row_start = row_start_index(anim.facing);
            let next_col = (current_col + 1) % WALK_FRAMES;
            atlas.index = row_start + next_col;
            timer.reset();
        } else {
            timer.tick(time.delta());
            if timer.just_finished() {
                let row_start = row_start_index(anim.facing);
                let next_col = (current_col + 1) % WALK_FRAMES;
                atlas.index = row_start + next_col;
            }
        }
    } else if just_stopped {
        timer.reset();
    }

    anim.was_moving = anim.moving;
    Ok(())
}

pub(crate) fn animate_player_system(
    time: Res<Time>,
    query: Query<(&mut AnimationState, &mut AnimationTimer, &mut Sprite), With<Player>>,
) {
    if let Err(e) = animate_player(time, query) {
        println!("[ERROR] - Animation error: {:?}", e);
    }
}

fn row_start_index(facing: Facing) -> usize {
    row_zero_based(facing) * WALK_FRAMES
}

pub(crate) fn atlas_index_for(facing: Facing, frame_in_row: usize) -> usize {
    row_start_index(facing) + frame_in_row.min(WALK_FRAMES - 1)
}

fn row_zero_based(facing: Facing) -> usize {
    match facing {
        Facing::Up => 8,
        Facing::Left => 9,
        Facing::Down => 10,
        Facing::Right => 11,
    }
}
