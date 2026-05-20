pub(crate) mod animate;
pub(crate) mod movement;
pub(crate) mod plugin;
pub(crate) mod spawn;
pub(crate) mod death;

use bevy::prelude::*;

pub(crate) const TILE_SIZE: u32 = 64;
pub(crate) const WALK_FRAMES: usize = 9;
pub(crate) const ANIM_DT: f32 = 0.1;

pub(crate) const MOVE_SPEED: f32 = 100.0;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Facing {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Component, Deref, DerefMut)]
pub(crate) struct AnimationTimer(pub(crate) Timer);

#[derive(Component)]
pub(crate) struct AnimationState {
    pub(crate) facing: Facing,
    pub(crate) moving: bool,
    pub(crate) was_moving: bool,
}

#[derive(Component)]
pub(crate) struct Enemy{
    pub(crate) health: f32,
    pub(crate) damage: f32
}
