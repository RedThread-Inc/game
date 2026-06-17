use crate::boss::*;
use crate::enemy::{AnimationState, AnimationTimer, Facing, ANIM_DT, TILE_SIZE, WALK_FRAMES};
use crate::enemy::animate::atlas_index_for;
use crate::InGameEntity;
use crate::round::RoundState;
use crate::map::generate::{TerrainHeightMap, TILE_SIZE as MAP_TILE_SIZE};
use crate::map::perlin::TerrainZone;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use crate::core::collision_groups::boss_membership;
use rand::Rng;

pub(crate) fn spawn_boss(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    round: Res<RoundState>,
    terrain: Option<Res<TerrainHeightMap>>,
) {
    if round.current == 0 || round.current % 5 != 0 {
        return;
    }

    let Ok(window) = windows.single() else { return };
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;

    let stats = BossStats::for_round(round.current);

    let spawn_pos = find_boss_spawn(half_w, half_h, terrain.as_deref());

    let texture = asset_server.load("boss.png");
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        WALK_FRAMES as u32,
        12,
        None,
        None,
    ));

    let facing = Facing::Down;
    let start_index = atlas_index_for(facing, 0);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: start_index,
            },
        ),
        Transform {
            translation: Vec3::new(spawn_pos.x, spawn_pos.y, 20.0),
            scale: Vec3::splat(BOSS_SCALE),
            ..default()
        },
        Boss {
            health: stats.hp,
            round: round.current,
        },
        AnimationState {
            facing,
            moving: true,
            was_moving: false,
        },
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
        BossAttackTimer(Timer::from_seconds(stats.fire_rate, TimerMode::Repeating)),
        InGameEntity,
        RigidBody::Dynamic,
        Collider::cuboid(16.0, 16.0),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.0),
        Velocity::default(),
        Damping { linear_damping: 10.0, angular_damping: 0.0 },
        boss_membership(),
    ));
}

fn find_boss_spawn(half_w: f32, half_h: f32, terrain: Option<&TerrainHeightMap>) -> Vec2 {
    let candidates = [
        Vec2::new(half_w - 120.0, 0.0),
        Vec2::new(-(half_w - 120.0), 0.0),
        Vec2::new(0.0, half_h - 120.0),
        Vec2::new(0.0, -(half_h - 120.0)),
    ];

    let Some(t) = terrain else {
        return candidates[0];
    };

    for pos in candidates {
        let tile_x = ((pos.x + half_w) / MAP_TILE_SIZE) as u32;
        let tile_y = ((pos.y + half_h) / MAP_TILE_SIZE) as u32;
        let tx = tile_x.min(t.0.width.saturating_sub(1));
        let ty = tile_y.min(t.0.height.saturating_sub(1));

        if t.0.classify(tx, ty) != TerrainZone::Water && !is_near_water_tile(&t.0, tx, ty) {
            return pos;
        }
    }

    // Fallback : scan brut
    let mut rng = rand::rng();
    for _ in 0..200 {
        let x = rng.random_range(-half_w..half_w);
        let y = rng.random_range(-half_h..half_h);
        let tile_x = ((x + half_w) / MAP_TILE_SIZE) as u32;
        let tile_y = ((y + half_h) / MAP_TILE_SIZE) as u32;
        let tx = tile_x.min(t.0.width.saturating_sub(1));
        let ty = tile_y.min(t.0.height.saturating_sub(1));
        if t.0.classify(tx, ty) != TerrainZone::Water && !is_near_water_tile(&t.0, tx, ty) {
            return Vec2::new(x, y);
        }
    }

    Vec2::ZERO
}

fn is_near_water_tile(height_map: &crate::map::perlin::HeightMap, x: u32, y: u32) -> bool {
    for dx in -1i32..=1 {
        for dy in -1i32..=1 {
            if dx == 0 && dy == 0 { continue; }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= height_map.width as i32 || ny < 0 || ny >= height_map.height as i32 {
                continue;
            }
            if height_map.classify(nx as u32, ny as u32) == TerrainZone::Water {
                return true;
            }
        }
    }
    false
}