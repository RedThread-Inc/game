use bevy::prelude::*;
use rand::{Rng, SeedableRng};
use rand::rngs::SmallRng;
use std::collections::HashSet;
use bevy_rapier2d::prelude::{Collider, RigidBody};
use crate::core::collision_groups::world_membership;
use crate::map::{
    assets::TilemapHandles,
    generate::{TILE_SIZE, DEBUG_SEED},
    perlin::{HeightMap, TerrainZone},
    tilemap::TILEMAP,
};

struct PropDef {
    bottom: &'static str,
    top: Option<&'static str>,
    allowed_zones: &'static [TerrainZone],
    avoid_water_border: bool,
    height_in_tiles: u32,
    density: f32,
    collider: Option<fn() -> Collider>,
}

const PROPS: &[PropDef] = &[
    PropDef { bottom: "plant_1",      top: None, allowed_zones: &[TerrainZone::GreenGrass], avoid_water_border: false, height_in_tiles: 1, density: 0.005, collider: None },
    PropDef { bottom: "plant_2",      top: None, allowed_zones: &[TerrainZone::GreenGrass], avoid_water_border: false, height_in_tiles: 1, density: 0.005, collider: None },
    PropDef { bottom: "plant_3",      top: None, allowed_zones: &[TerrainZone::GreenGrass], avoid_water_border: false, height_in_tiles: 1, density: 0.005, collider: None },
    PropDef { bottom: "plant_4",      top: None, allowed_zones: &[TerrainZone::GreenGrass], avoid_water_border: false, height_in_tiles: 1, density: 0.005, collider: Some(|| Collider::ball(9.0)) },

    PropDef { bottom: "rock_1",       top: None, allowed_zones: &[TerrainZone::GreenGrass], avoid_water_border: true,  height_in_tiles: 1, density: 0.005, collider: None },
    PropDef { bottom: "rock_2",       top: None, allowed_zones: &[TerrainZone::GreenGrass], avoid_water_border: true,  height_in_tiles: 1, density: 0.005, collider: Some(|| Collider::cuboid(10.0, 10.0)) },
    PropDef { bottom: "rock_3",       top: None, allowed_zones: &[TerrainZone::GreenGrass], avoid_water_border: true,  height_in_tiles: 1, density: 0.005, collider: Some(|| Collider::cuboid(9.0, 8.0)) },
    PropDef { bottom: "rock_4",       top: None, allowed_zones: &[TerrainZone::GreenGrass], avoid_water_border: true,  height_in_tiles: 1, density: 0.005, collider: Some(|| Collider::cuboid(9.0, 7.0)) },

    PropDef { bottom: "tree_stump_1", top: None, allowed_zones: &[TerrainZone::GreenGrass], avoid_water_border: false, height_in_tiles: 1, density: 0.005, collider: Some(|| Collider::ball(8.0)) },
    PropDef { bottom: "tree_stump_2", top: None, allowed_zones: &[TerrainZone::GreenGrass], avoid_water_border: false, height_in_tiles: 1, density: 0.005, collider: Some(|| Collider::ball(8.0)) },
    PropDef { bottom: "tree_stump_3", top: None, allowed_zones: &[TerrainZone::GreenGrass], avoid_water_border: false, height_in_tiles: 1, density: 0.005, collider: Some(|| Collider::ball(8.0)) },

    PropDef { bottom: "small_tree_bottom", top: Some("small_tree_top"), allowed_zones: &[TerrainZone::GreenGrass], avoid_water_border: false, height_in_tiles: 2, density: 0.015, collider: Some(|| Collider::capsule_y(7.0, 0.5)) },
];

const BIG_TREE_DENSITY: f32 = 0.015;

fn is_near_water(height_map: &HeightMap, x: u32, y: u32, grid_x: u32, grid_y: u32) -> bool {
    if height_map.classify(x, y) == TerrainZone::Water {
        return true;
    }
    [(0i32, 1i32), (0, -1), (1, 0), (-1, 0)].iter().any(|&(dx, dy)| {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx < 0 || nx >= grid_x as i32 || ny < 0 || ny >= grid_y as i32 {
            return false;
        }
        height_map.classify(nx as u32, ny as u32) == TerrainZone::Water
    })
}

pub(crate) fn spawn_all_props(
    commands: &mut Commands,
    handles: &TilemapHandles,
    height_map: &HeightMap,
    grid_x: u32,
    grid_y: u32,
    origin_x: f32,
    origin_y: f32,
) {
    let mut occupied: HashSet<(u32, u32)> = HashSet::new();

    spawn_big_trees(commands, handles, height_map, grid_x, grid_y, origin_x, origin_y, &mut occupied);
    spawn_props(commands, handles, height_map, grid_x, grid_y, origin_x, origin_y, &mut occupied);
}

fn spawn_big_trees(
    commands: &mut Commands,
    handles: &TilemapHandles,
    height_map: &HeightMap,
    grid_x: u32,
    grid_y: u32,
    origin_x: f32,
    origin_y: f32,
    occupied: &mut HashSet<(u32, u32)>,
) {
    let mut rng = SmallRng::seed_from_u64(DEBUG_SEED as u64 ^ 0xC0FFEE);

    let trees = [
        ("big_tree_1_bl", "big_tree_1_br", "big_tree_1_tl", "big_tree_1_tr"),
        ("big_tree_2_bl", "big_tree_2_br", "big_tree_2_tl", "big_tree_2_tr"),
    ];

    for y in 0..grid_y.saturating_sub(1) {
        for x in 0..grid_x.saturating_sub(1) {
            if rng.random::<f32>() >= BIG_TREE_DENSITY {
                continue;
            }

            let footprint = [(x, y), (x + 1, y), (x, y + 1), (x + 1, y + 1)];

            let clear = footprint.iter().all(|&(fx, fy)| {
                height_map.classify(fx, fy) == TerrainZone::GreenGrass
                    && !occupied.contains(&(fx, fy))
                    && !is_near_water(height_map, fx, fy, grid_x, grid_y)
            });

            if !clear {
                continue;
            }

            for &(fx, fy) in &footprint {
                occupied.insert((fx, fy));
            }

            let tree_idx = rng.random_range(0..trees.len());
            let (bl, br, tl, tr) = trees[tree_idx];

            let wx = origin_x + x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
            let wy = origin_y + y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

            commands.spawn((
                Transform::from_xyz(wx + TILE_SIZE / 2.0, wy - TILE_SIZE / 2.0, 0.0),
                GlobalTransform::default(),
                RigidBody::Fixed,
                Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
            ));

            for (name, dx, dy) in [
                (bl, 0.0,       0.0      ),
                (br, TILE_SIZE, 0.0      ),
                (tl, 0.0,       TILE_SIZE),
                (tr, TILE_SIZE, TILE_SIZE),
            ] {
                if let Some(idx) = TILEMAP.sprite_index(name) {
                    commands.spawn((
                        handles.sprite(idx),
                        Transform::from_xyz(wx + dx, wy + dy, 2.0),
                    ));
                }
            }
        }
    }
}

fn spawn_props(
    commands: &mut Commands,
    handles: &TilemapHandles,
    height_map: &HeightMap,
    grid_x: u32,
    grid_y: u32,
    origin_x: f32,
    origin_y: f32,
    occupied: &mut HashSet<(u32, u32)>,
) {
    let mut rng = SmallRng::seed_from_u64(DEBUG_SEED as u64 ^ 0xDEAD_BEEF);

    for y in 0..grid_y {
        for x in 0..grid_x {
            if occupied.contains(&(x, y)) {
                continue;
            }

            let zone = height_map.classify(x, y);
            let near_water = is_near_water(height_map, x, y, grid_x, grid_y);

            for prop in PROPS {
                if !prop.allowed_zones.contains(&zone) { continue; }
                if prop.avoid_water_border && near_water { continue; }
                if prop.height_in_tiles > 1
                    && (y + 1 >= grid_y || occupied.contains(&(x, y + 1))) {
                    continue;
                }

                if rng.random::<f32>() >= prop.density { continue; }

                occupied.insert((x, y));
                if prop.height_in_tiles > 1 {
                    occupied.insert((x, y + 1));
                }

                let wx = origin_x + x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
                let wy = origin_y + y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

                if let Some(idx) = TILEMAP.sprite_index(prop.bottom) {
                    let mut entity = commands.spawn((
                        handles.sprite(idx),
                        Transform::from_xyz(wx, wy, 2.0),
                    ));

                    if let Some(collider_fn) = prop.collider {
                        entity.insert((
                            RigidBody::Fixed,
                            collider_fn(),
                            world_membership(),
                        ));
                    }
                }
                if let Some(top_name) = prop.top {
                    if let Some(idx) = TILEMAP.sprite_index(top_name) {
                        commands.spawn((
                            handles.sprite(idx),
                            Transform::from_xyz(wx, wy + TILE_SIZE, 2.0),
                        ));
                    }
                }

                break;
            }
        }
    }
}