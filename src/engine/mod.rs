use crate::enemy::plugin::EnemyPlugin;
use crate::player::plugin::PlayerPlugin;
use crate::interface::plugin::InterfacePlugin;
use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};
use bevy::window::{WindowMode};
use bevy_procedural_tilemaps::prelude::*;
use crate::map::generate::{setup_generator};

pub(crate) fn init_app() {
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
                        resolution: WindowResolution::new(1920, 1080),
                        resizable: true,
                        mode: WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((PlayerPlugin, EnemyPlugin, InterfacePlugin))
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
