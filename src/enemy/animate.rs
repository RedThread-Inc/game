use crate::enemy::*;
use bevy::prelude::*;
use crate::exceptions::RTGException;

pub(crate) fn animate_enemies(
    time: &Time,
    anim: &mut AnimationState,
    timer: &mut AnimationTimer,
    sprite: &mut Sprite,
) -> Result<(), RTGException> {
    let atlas = sprite
        .texture_atlas
        .as_mut()
        .ok_or(RTGException::RTG_ENEMY_ANIMATION_TEXTURE_ATLAS_CANT_LOAD)?;

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

    Ok(())
}

pub(crate) fn animate_enemies_system(
    time: Res<Time>,
    mut query: Query<(&mut AnimationState, &mut AnimationTimer, &mut Sprite), With<Enemy>>,
) -> Result<(), RTGException> {
    let (mut anim, mut timer, mut sprite) = match query.single_mut() {
        Ok(v) => v,
        Err(bevy::ecs::query::QuerySingleError::NoEntities(_)) => {
            println!("{}", RTGException::RTG_ENEMY_IMPOSSIBLE_TO_ANIMATE_ENEMY_NOT_FOUND.to_string());
            return Err(RTGException::RTG_ENEMY_IMPOSSIBLE_TO_ANIMATE_ENEMY_NOT_FOUND);
        }
        // TODO: Seems weird because we will have multiple enemies
        Err(bevy::ecs::query::QuerySingleError::MultipleEntities(_)) => {
            println!("{}", RTGException::RTG_ENEMY_IMPOSSIBLE_TO_ANIMATE_MULTIPLE_ENEMIES_FOUND.to_string());
            return Err(RTGException::RTG_ENEMY_IMPOSSIBLE_TO_ANIMATE_MULTIPLE_ENEMIES_FOUND);
        }
    };

    if let Err(e) = animate_enemies(&time, &mut anim, &mut timer, &mut sprite) {
        println!("{}", e.to_string());
        return Err(e);
    } else {
        return Ok(())
    }
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
