pub(crate) mod animate;
pub(crate) mod movement;
pub(crate) mod plugin;
pub(crate) mod spawn;

use bevy::prelude::{Component, Deref, DerefMut, Timer};

const TILE_SIZE: u32 = 64;
const WALK_FRAMES: usize = 9;
const MOVE_SPEED: f32 = 140.0;
const ANIM_DT: f32 = 0.1;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
enum Facing {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

#[derive(Component)]
pub(crate) struct AnimationState {
    facing: Facing,
    moving: bool,
    was_moving: bool,
}

#[derive(Component, Clone)]
pub(crate) struct Player {
    pub(crate) name: String,
}
