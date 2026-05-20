pub(crate) mod animate;
pub(crate) mod movement;
pub(crate) mod plugin;
pub(crate) mod spawn;
pub(crate) mod death;
pub(crate) mod projectile;

use bevy::prelude::*;

pub(crate) const TILE_SIZE: u32 = 64;
pub(crate) const WALK_FRAMES: usize = 9;
pub(crate) const ANIM_DT: f32 = 0.1;

pub(crate) const MOVE_SPEED: f32 = 100.0;

// Stats de base des ennemis
const MELEE_BASE_HEALTH: f32  = 100.0;
const MELEE_BASE_DAMAGE: f32  = 10.0;
const RANGED_BASE_HEALTH: f32 = 80.0;
const RANGED_BASE_DAMAGE: f32 = 8.0;

// +15% vie et +10% dégâts par manche
const HEALTH_SCALE_PER_ROUND: f32 = 0.15;
const DAMAGE_SCALE_PER_ROUND: f32 = 0.10;

pub(crate) fn scaled_health(base: f32, round: u32) -> f32 {
    base * (1.0 + HEALTH_SCALE_PER_ROUND * (round.saturating_sub(1)) as f32)
}

pub(crate) fn scaled_damage(base: f32, round: u32) -> f32 {
    base * (1.0 + DAMAGE_SCALE_PER_ROUND * (round.saturating_sub(1)) as f32)
}

pub(crate) fn melee_health(round: u32) -> f32  { scaled_health(MELEE_BASE_HEALTH, round) }
pub(crate) fn melee_damage(round: u32) -> f32  { scaled_damage(MELEE_BASE_DAMAGE, round) }
pub(crate) fn ranged_health(round: u32) -> f32 { scaled_health(RANGED_BASE_HEALTH, round) }
pub(crate) fn ranged_damage(round: u32) -> f32 { scaled_damage(RANGED_BASE_DAMAGE, round) }

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
pub(crate) struct Enemy {
    pub(crate) health: f32,
    pub(crate) damage: f32,
}

#[derive(Component)]
pub(crate) struct RangedEnemy {
    pub(crate) fire_cooldown: Timer,
    pub(crate) attack_range: f32,
}

#[derive(Component)]
pub(crate) struct HitFlash {
    pub(crate) timer: Timer,
}