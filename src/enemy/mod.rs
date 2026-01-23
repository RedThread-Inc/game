pub(crate) mod spawn;
pub(crate) mod animate;
pub(crate) mod plugin;
pub(crate) mod movement;

use bevy::prelude::*;

const TILE_SIZE: u32 = 64;
const WALK_FRAMES: usize = 9;
const ANIM_DT: f32 = 0.1;

pub(crate) const MOVE_SPEED: f32 = 100.0;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Facing {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Component, Deref, DerefMut)]
pub(crate) struct AnimationTimer(pub Timer);

#[derive(Component)]
pub(crate) struct AnimationState {
    pub facing: Facing,
    pub moving: bool,
    pub was_moving: bool,
}

#[derive(Component)]
pub(crate) struct Enemy;
