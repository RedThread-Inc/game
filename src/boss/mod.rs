pub(crate) mod attack;
pub(crate) mod death;
pub(crate) mod movement;
pub(crate) mod plugin;
pub(crate) mod spawn;
pub(crate) mod damage;

use bevy::prelude::*;

// Même spritesheet que les ennemis (skeleton-spritesheet.png)
pub(crate) const BOSS_TILE_SIZE: u32 = 64;
pub(crate) const BOSS_WALK_FRAMES: usize = 9;
pub(crate) const BOSS_ANIM_DT: f32 = 0.1;
pub(crate) const BOSS_SCALE: f32 = 2.5;

pub(crate) const BOSS_MOVE_SPEED: f32 = 45.0;
pub(crate) const BOSS_BASE_HP: f32 = 500.0;
pub(crate) const BOSS_BASE_PROJECTILE_COUNT: u32 = 8;
pub(crate) const BOSS_BASE_PROJECTILE_SPEED: f32 = 120.0;
pub(crate) const BOSS_BASE_FIRE_RATE: f32 = 2.0;

#[derive(Component)]
pub(crate) struct Boss {
    pub(crate) health: f32,
    pub(crate) round: u32,
}

#[derive(Component)]
pub(crate) struct BossProjectile {
    pub(crate) velocity: Vec2,
    pub(crate) damage: f32,
}

#[derive(Component, Deref, DerefMut)]
pub(crate) struct BossAttackTimer(pub(crate) Timer);

pub(crate) struct BossStats {
    pub(crate) hp: f32,
    pub(crate) projectile_count: u32,
    pub(crate) projectile_speed: f32,
    pub(crate) fire_rate: f32,
}

impl BossStats {
    pub(crate) fn for_round(round: u32) -> Self {
        let tier = (round / 5).max(1);
        let scale = tier as f32;
        Self {
            hp: BOSS_BASE_HP * scale * 1.5,
            projectile_count: BOSS_BASE_PROJECTILE_COUNT + (tier - 1) * 4,
            projectile_speed: BOSS_BASE_PROJECTILE_SPEED + (tier - 1) as f32 * 30.0,
            fire_rate: BOSS_BASE_FIRE_RATE,
        }
    }
}