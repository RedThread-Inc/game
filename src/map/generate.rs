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

    commands.insert_resource(TerrainHeightMap(height_map));
}