use crate::enemy::movement::move_enemy_towards_player_system;
use crate::enemy::plugin::EnemyPlugin;
use crate::player::plugin::PlayerPlugin;
use crate::interface::plugin::InterfacePlugin;
use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_procedural_tilemaps::prelude::*;
use crate::map::generate::{map_pixel_dimensions, setup_generator, TILE_SIZE};

pub(crate) fn init_app() {

    //Initialize windows with 0px but it's evaluated after in setup generator function
    let grid_x = 0;
    let grid_y = 0;
    let map_size = map_pixel_dimensions(grid_x, grid_y);

    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    file_path: "src/assets".into(),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(map_size.x as u32, map_size.y as u32),
                        resizable: false,
                        mode: WindowMode::Fullscreen(MonitorSelection::Primary, VideoModeSelection::Current),
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((PlayerPlugin, EnemyPlugin, InterfacePlugin))
        .add_systems(Startup, |windows: Query<&Window, With<PrimaryWindow>>| {
            let window = windows.single().expect("Primary window must exist");
            let grid_x = (window.width() / TILE_SIZE).floor() as u32;
            let grid_y = (window.height() / TILE_SIZE).floor() as u32;
            let map_size = map_pixel_dimensions(grid_x, grid_y);
            println!("Map size: {:?}", map_size);
        })
        .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Sprite>::default())
        .add_systems(Startup, (setup_camera, setup_generator))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;
