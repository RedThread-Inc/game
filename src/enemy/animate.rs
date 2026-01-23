use crate::enemy::*;
use bevy::prelude::*;

pub(crate) fn animate_enemies(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut AnimationTimer, &mut Sprite), With<Enemy>>,
) {
    let Ok((mut anim, mut timer, mut sprite)) = query.single_mut() else {
        return;
    };

    let atlas = match sprite.texture_atlas.as_mut() {
        Some(a) => a,
        None => return,
    };

    let target_row = row_zero_based(anim.facing);
    let current_col = atlas.index % WALK_FRAMES;
    let current_row = atlas.index / WALK_FRAMES;

    if current_row != target_row {
        atlas.index = row_start_index(anim.facing);
        timer.reset();
    }

    timer.tick(time.delta());
    if timer.just_finished() {
        let row_start = row_start_index(anim.facing);
        let next_col = (current_col + 1) % WALK_FRAMES;
        atlas.index = row_start + next_col;
    }

    anim.was_moving = anim.moving;
}

fn row_start_index(facing: Facing) -> usize {
    row_zero_based(facing) * WALK_FRAMES
}

pub(crate) fn atlas_index_for(facing: Facing, frame: usize) -> usize {
    row_start_index(facing) + frame.min(WALK_FRAMES - 1)
}

fn row_zero_based(facing: Facing) -> usize {
    match facing {
        Facing::Up => 8,
        Facing::Left => 9,
        Facing::Down => 10,
        Facing::Right => 11,
    }
}
