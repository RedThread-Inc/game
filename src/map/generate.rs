use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::map::{
    assets::prepare_tilemap_handles,
    perlin::{HeightMap, TerrainZone},
    tilemap::TILEMAP,
};

const ASSETS_PATH: &str = "tile_layers";
const TILEMAP_FILE: &str = "tilemap.png";
pub(crate) const TILE_SIZE: f32 = 32.;

const DEBUG_SEED: u32 = 123456789;
const PERLIN_SCALE: f64 = 0.2;

pub(crate) fn map_pixel_dimensions(grid_x: u32, grid_y: u32) -> Vec2 {
    Vec2::new(TILE_SIZE * grid_x as f32, TILE_SIZE * grid_y as f32)
}

#[derive(Resource)]
pub struct TerrainHeightMap(pub HeightMap);

fn terrain_priority(zone: TerrainZone) -> u8 {
    match zone {
        TerrainZone::Water       => 3,
        TerrainZone::GreenGrass  => 2,
        TerrainZone::YellowGrass => 1,
        TerrainZone::Dirt        => 0,
    }
}

fn zone_prefix(zone: TerrainZone) -> &'static str {
    match zone {
        TerrainZone::Water       => "water",
        TerrainZone::Dirt        => "dirt",
        TerrainZone::GreenGrass  => "green_grass",
        TerrainZone::YellowGrass => "yellow_grass",
    }
}

pub(crate) fn setup_generator(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows
        .single()
        .expect("Primary window must exist");

    let grid_x = (window.width() / TILE_SIZE).floor() as u32;
    let grid_y = (window.height() / TILE_SIZE).floor() as u32;

    println!("Grid size: {} x {}", grid_x, grid_y);

    let height_map = HeightMap::generate(grid_x, grid_y, DEBUG_SEED, PERLIN_SCALE);
    let tilemap_handles =
        prepare_tilemap_handles(&asset_server, &mut atlas_layouts, ASSETS_PATH, TILEMAP_FILE);

    let origin_x = -(TILE_SIZE * grid_x as f32) / 2.0;
    let origin_y = -(TILE_SIZE * grid_y as f32) / 2.0;

    // --- Base layer (Z = 0) ---
    for y in 0..grid_y {
        for x in 0..grid_x {
            let zone = height_map.classify(x, y);

            let sprite_name = match zone {
                TerrainZone::Water       => "water",
                TerrainZone::Dirt        => "dirt",
                TerrainZone::GreenGrass  => "green_grass",
                TerrainZone::YellowGrass => "yellow_grass",
            };

            let atlas_index = TILEMAP
                .sprite_index(sprite_name)
                .unwrap_or_else(|| panic!("Unknown sprite: '{}'", sprite_name));

            let world_x = origin_x + x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
            let world_y = origin_y + y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

            commands.spawn((
                tilemap_handles.sprite(atlas_index),
                Transform::from_xyz(world_x, world_y, 0.0),
            ));
        }
    }

    // --- Transition layer (Z = 1) ---
    //
    // The edge sprites belong to the HIGH-priority tile and are drawn ON TOP of
    // that tile, facing outward toward lower-priority neighbours.
    //
    // Example: a Water tile next to GreenGrass draws "water_side_*" on itself.
    //          The GreenGrass tile draws nothing extra — plain base only.
    //
    // This ensures each edge is owned by exactly one tile — no duplicates.
    // --- Transition layer (Z = 1) ---
    // --- Transition layer (Z = 1.0) ---
    for y in 0..grid_y {
        for x in 0..grid_x {
            let center = height_map.classify(x, y);
            let cp = terrain_priority(center);

            // Helper to get priority of neighbors
            let get_zone = |dx: i32, dy: i32| -> TerrainZone {
                let nx = (x as i32 + dx).clamp(0, grid_x as i32 - 1) as u32;
                let ny = (y as i32 + dy).clamp(0, grid_y as i32 - 1) as u32;
                height_map.classify(nx, ny)
            };

            let get_priority = |dx: i32, dy: i32| -> u8 {
                terrain_priority(get_zone(dx, dy))
            };

            let world_x = origin_x + x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
            let world_y = origin_y + y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

            let mut spawn_edge = |sprite_name: String| {
                if let Some(idx) = TILEMAP.sprite_index(&sprite_name) {
                    commands.spawn((
                        tilemap_handles.sprite(idx),
                        Transform::from_xyz(world_x, world_y, 1.0),
                    ));
                }
            };

            let t = get_priority(0, 1);
            let b = get_priority(0, -1);
            let l = get_priority(-1, 0);
            let r = get_priority(1, 0);
            let tl = get_priority(-1, 1);
            let tr = get_priority(1, 1);
            let bl = get_priority(-1, -1);
            let br = get_priority(1, -1);

            // 1. CARDINAL SIDES
            // If neighbor is higher priority, draw THEIR transition on OUR tile.
            // Rule: _t means the neighbor is BELOW us, so if bottom > cp, we spawn _t.
            if b > cp { spawn_edge(format!("{}_side_t", zone_prefix(get_zone(0, -1)))); }
            if t > cp { spawn_edge(format!("{}_side_b", zone_prefix(get_zone(0, 1)))); }
            if r > cp { spawn_edge(format!("{}_side_l", zone_prefix(get_zone(1, 0)))); }
            if l > cp { spawn_edge(format!("{}_side_r", zone_prefix(get_zone(-1, 0)))); }

            // 2. INNER CORNERS (Concave)
            // Happens when two adjacent cardinal neighbors are higher priority.
            // If Bottom and Right neighbors are land, the joint is in our Bottom-Right.
            // Per your asset rule, this uses the _tl (Top-Left) corner sprite of the land.
            if b > cp && r > cp && b == r {
                spawn_edge(format!("{}_corner_in_br", zone_prefix(get_zone(0, -1))));
            }
            // Land at Bottom and Left -> spawn Bottom-Left inner corner
            if b > cp && l > cp && b == l {
                spawn_edge(format!("{}_corner_in_bl", zone_prefix(get_zone(0, -1))));
            }
            // Land at Top and Right -> spawn Top-Right inner corner
            if t > cp && r > cp && t == r {
                spawn_edge(format!("{}_corner_in_tr", zone_prefix(get_zone(0, 1))));
            }
            // Land at Top and Left -> spawn Top-Left inner corner
            if t > cp && l > cp && t == l {
                spawn_edge(format!("{}_corner_in_tl", zone_prefix(get_zone(0, 1))));
            }

            // 3. OUTER CORNERS (Convex)
            // Happens when only the diagonal neighbor is higher priority.
            // If the neighbor at Bottom-Right (br) is land, it pokes its "Bottom-Right" tip into us.
            // Per your asset rule, this uses the land's _br corner_out sprite.
            if br > cp && b <= cp && r <= cp {
                spawn_edge(format!("{}_corner_out_tl", zone_prefix(get_zone(1, -1))));
            }
            // Land at Bottom-Left (bl)
            if bl > cp && b <= cp && l <= cp {
                spawn_edge(format!("{}_corner_out_tr", zone_prefix(get_zone(-1, -1))));
            }
            // Land at Top-Right (tr)
            if tr > cp && t <= cp && r <= cp {
                spawn_edge(format!("{}_corner_out_bl", zone_prefix(get_zone(1, 1))));
            }
            // Land at Top-Left (tl)
            if tl > cp && t <= cp && l <= cp {
                spawn_edge(format!("{}_corner_out_br", zone_prefix(get_zone(-1, 1))));
            }
        }
    }

    commands.insert_resource(TerrainHeightMap(height_map));
}