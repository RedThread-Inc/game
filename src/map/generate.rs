use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::{Collider, RigidBody};
use rand::Rng;
use crate::core::collision_groups::world_membership;
use crate::map::{
    assets::{prepare_tilemap_handles, TilemapHandles},
    perlin::{HeightMap, TerrainZone},
    props::spawn_all_props,
    tilemap::TILEMAP,
};
use crate::exceptions::RTGException;
use crate::InGameEntity;

const ASSETS_PATH: &str = "tile_layers";
const TILEMAP_FILE: &str = "tilemap.png";
pub(crate) const TILE_SIZE: f32 = 32.;

pub(crate) const DEBUG_SEED: u32 = 123456789;
const PERLIN_SCALE: f64 = 0.2;

#[derive(Resource)]
pub struct TerrainHeightMap(pub HeightMap);

fn terrain_priority(zone: TerrainZone) -> u8 {
    match zone {
        TerrainZone::Water       => 3,
        TerrainZone::GreenGrass  => 2,
        TerrainZone::Dirt        => 0,
    }
}

fn zone_prefix(zone: TerrainZone) -> &'static str {
    match zone {
        TerrainZone::Water       => "water",
        TerrainZone::Dirt        => "dirt",
        TerrainZone::GreenGrass  => "green_grass",
    }
}

pub(crate) fn setup_generator(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) -> Result<(), RTGException> {
    let window = windows
        .single()
        .map_err(|_| RTGException::REDTHREAD_FAILED_TO_GENERATE_MAP_MISSING_GAME_WINDOW)?;

    let grid_x = (window.width() / TILE_SIZE).floor() as u32;
    let grid_y = (window.height() / TILE_SIZE).floor() as u32;

    let seed: u32 = rand::rng().random();
    let height_map = HeightMap::generate(grid_x, grid_y, seed, PERLIN_SCALE);
    let handles =
        prepare_tilemap_handles(&asset_server, &mut atlas_layouts, ASSETS_PATH, TILEMAP_FILE);

    let origin_x = -(TILE_SIZE * grid_x as f32) / 2.0;
    let origin_y = -(TILE_SIZE * grid_y as f32) / 2.0;

    for y in 0..grid_y {
        for x in 0..grid_x {
            let world_x = origin_x + x as f32 * TILE_SIZE + TILE_SIZE / 2.0;
            let world_y = origin_y + y as f32 * TILE_SIZE + TILE_SIZE / 2.0;

            let zone = height_map.classify(x, y);
            let base_sprite_name = zone_prefix(zone);
            let base_sprite_id = TILEMAP.sprite_index(base_sprite_name).unwrap_or_else(|| panic!("Unknown sprite: '{}'", base_sprite_name));

            let borders_water = is_tile_water_border(&height_map, x, y, grid_x, grid_y);

            let mut entity = commands.spawn((
                handles.sprite(base_sprite_id),
                Transform::from_xyz(world_x, world_y, 0.0),
                InGameEntity,
            ));

            if zone == TerrainZone::Water {
                entity.insert((RigidBody::Fixed, Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0), world_membership()));
            }

            else if borders_water {
                entity.insert((RigidBody::Fixed, Collider::cuboid(TILE_SIZE / 2.0, TILE_SIZE / 2.0), world_membership()));
            }


            spawn_transition(&mut commands, &handles, &height_map, grid_x, grid_y, x, y, world_x, world_y);
        }
    }

    spawn_all_props(&mut commands, &handles, &height_map, grid_x, grid_y, origin_x, origin_y);

    commands.insert_resource(TerrainHeightMap(height_map));

    let map_w = grid_x as f32 * TILE_SIZE;
    let map_h = grid_y as f32 * TILE_SIZE;
    let thickness = TILE_SIZE;

    let borders = [
        (0.0,                    map_h / 2.0,   map_w / 2.0 + thickness, thickness / 2.0), // haut
        (0.0,                   -map_h / 2.0,   map_w / 2.0 + thickness, thickness / 2.0), // bas
        (-map_w / 2.0,           0.0,           thickness / 2.0,          map_h / 2.0),     // gauche
        ( map_w / 2.0,           0.0,           thickness / 2.0,          map_h / 2.0),     // droite
    ];

    for (x, y, hw, hh) in borders {
        commands.spawn((
            Transform::from_xyz(x, y, 0.0),
            RigidBody::Fixed,
            Collider::cuboid(hw, hh),
            world_membership(),
            InGameEntity,
        ));
    }

    Ok(())
}


fn is_tile_water_border(
    height_map: &HeightMap,
    x: u32, y: u32,
    grid_x: u32, grid_y: u32,
) -> bool {
    // This tile itself must not be water
    if height_map.classify(x, y) == TerrainZone::Water {
        return false;
    }

    for dx in -1i32..=1 {
        for dy in -1i32..=1 {
            if dx == 0 && dy == 0 { continue; }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= grid_x as i32 || ny < 0 || ny >= grid_y as i32 {
                continue;
            }
            if height_map.classify(nx as u32, ny as u32) == TerrainZone::Water {
                return true;
            }
        }
    }
    false
}

fn spawn_transition(commands: &mut Commands, handles: &TilemapHandles, height_map: &HeightMap, grid_x: u32, grid_y: u32, x: u32, y: u32, world_x: f32, world_y: f32) {

    let center = height_map.classify(x, y);
    let cp = terrain_priority(center);
    let cp = terrain_priority(center);

    let get_zone = |dx: i32, dy: i32| -> TerrainZone {
        let nx = (x as i32 + dx).clamp(0, grid_x as i32 - 1) as u32;
        let ny = (y as i32 + dy).clamp(0, grid_y as i32 - 1) as u32;
        height_map.classify(nx, ny)
    };

    let prio = |dx: i32, dy: i32| -> u8 { terrain_priority(get_zone(dx, dy)) };

    let mut spawn_edge = |sprite_name: String| {
        if let Some(idx) = TILEMAP.sprite_index(&sprite_name) {
            commands.spawn((
                handles.sprite(idx),
                Transform::from_xyz(world_x, world_y, 1.0),
                InGameEntity,
            ));
        }
    };

    let (t, b, l, r) = (prio(0,1), prio(0,-1), prio(-1,0), prio(1,0));
    let (tl, tr, bl, br) = (prio(-1,1), prio(1,1), prio(-1,-1), prio(1,-1));

    if b > cp { spawn_edge(format!("{}_side_t", zone_prefix(get_zone(0, -1)))); }
    if t > cp { spawn_edge(format!("{}_side_b", zone_prefix(get_zone(0, 1)))); }
    if r > cp { spawn_edge(format!("{}_side_l", zone_prefix(get_zone(1, 0)))); }
    if l > cp { spawn_edge(format!("{}_side_r", zone_prefix(get_zone(-1, 0)))); }

    if b > cp && r > cp && b == r { spawn_edge(format!("{}_corner_in_br", zone_prefix(get_zone(0, -1)))); }
    if b > cp && l > cp && b == l { spawn_edge(format!("{}_corner_in_bl", zone_prefix(get_zone(0, -1)))); }
    if t > cp && r > cp && t == r { spawn_edge(format!("{}_corner_in_tr", zone_prefix(get_zone(0, 1)))); }
    if t > cp && l > cp && t == l { spawn_edge(format!("{}_corner_in_tl", zone_prefix(get_zone(0, 1)))); }

    if br > cp && b <= cp && r <= cp { spawn_edge(format!("{}_corner_out_tl", zone_prefix(get_zone(1, -1)))); }
    if bl > cp && b <= cp && l <= cp { spawn_edge(format!("{}_corner_out_tr", zone_prefix(get_zone(-1, -1)))); }
    if tr > cp && t <= cp && r <= cp { spawn_edge(format!("{}_corner_out_bl", zone_prefix(get_zone(1, 1)))); }
    if tl > cp && t <= cp && l <= cp { spawn_edge(format!("{}_corner_out_br", zone_prefix(get_zone(-1, 1)))); }
}