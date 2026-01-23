use bevy_procedural_tilemaps::prelude::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::map::{
    assets::{load_assets, prepare_tilemap_handles},
    rules::build_world,
};

const ASSETS_PATH: &str = "tile_layers";
const TILEMAP_FILE: &str = "tilemap.png";
pub(crate) const TILE_SIZE: f32 = 32.;
const NODE_SIZE: Vec3 = Vec3::new(TILE_SIZE, TILE_SIZE, 1.);
const ASSETS_SCALE: Vec3 = Vec3::ONE;
const GRID_Z: u32 = 5;

pub(crate) fn map_pixel_dimensions(grid_x: u32, grid_y: u32) -> Vec2 {
    Vec2::new(TILE_SIZE * grid_x as f32, TILE_SIZE * grid_y as f32)
}

pub(crate) fn setup_generator(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    let window = windows
        .single()
        .expect("La fenêtre principale doit exister");

    let grid_x = (window.width() / TILE_SIZE).floor() as u32;
    let grid_y = (window.height() / TILE_SIZE).floor() as u32;

    println!("Grid size: {} x {}", grid_x, grid_y);

    let (assets_definitions, models, socket_collection) = build_world();

    let rules = RulesBuilder::new_cartesian_3d(models, socket_collection)
        .with_rotation_axis(Direction::ZForward)
        .build()
        .unwrap();

    let grid = CartesianGrid::new_cartesian_3d(grid_x, grid_y, GRID_Z, false, false, false);

    let gen_builder = GeneratorBuilder::new()
        .with_rules(rules)
        .with_grid(grid.clone())
        .with_rng(RngMode::RandomSeed)
        .with_node_heuristic(NodeSelectionHeuristic::MinimumRemainingValue)
        .with_model_heuristic(ModelSelectionHeuristic::WeightedProbability);

    let generator = gen_builder.build().unwrap();

    let tilemap_handles =
        prepare_tilemap_handles(&asset_server, &mut atlas_layouts, ASSETS_PATH, TILEMAP_FILE);
    let models_assets = load_assets(&tilemap_handles, assets_definitions);

    commands.spawn((
        Transform::from_translation(Vec3 {
            x: -TILE_SIZE * grid.size_x() as f32 / 2.0,
            y: -TILE_SIZE * grid.size_y() as f32 / 2.0,
            z: 0.0,
        }),
        grid,
        generator,
        NodesSpawner::new(models_assets, NODE_SIZE, ASSETS_SCALE).with_z_offset_from_y(true),
    ));
}
