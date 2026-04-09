use crate::audio::{tick_cooldowns, SoundCooldowns};
use crate::enemy::plugin::EnemyPlugin;
use crate::player::plugin::PlayerPlugin;
use crate::interface::plugin::InterfacePlugin;
use crate::menu::main_menu::MainMenuPlugin;
use crate::round::plugin::RoundPlugin;
use crate::upgrade::plugin::UpgradePlugin;
use crate::{GameState, InGameEntity, InGameState};
use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_procedural_tilemaps::prelude::*;
use crate::map::generate::{map_pixel_dimensions, setup_generator, TILE_SIZE};
use crate::menu::pause_menu::PauseMenuPlugin;
use crate::menu::death_menu::DeathMenuPlugin;

pub(crate) fn cleanup_game(mut commands: Commands, query: Query<Entity, With<InGameEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn_related::<Children>();
        commands.entity(entity).despawn();
    }

}

pub(crate) fn init_app() {

    //Initialize windows with 0px but it's evaluated after in setup generator function
    let grid_x = 0;
    let grid_y = 0;
    let map_size = map_pixel_dimensions(grid_x, grid_y);

    App::new()
        .insert_resource(ClearColor(Color::srgb(0.847, 0.769, 0.588)))
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
        .init_state::<GameState>()
        .add_sub_state::<InGameState>()
        .init_resource::<SoundCooldowns>()
        .add_systems(Update, tick_cooldowns)
        .add_plugins((PlayerPlugin, EnemyPlugin, InterfacePlugin, MainMenuPlugin, PauseMenuPlugin, DeathMenuPlugin, RoundPlugin, UpgradePlugin))
        .add_plugins(ProcGenSimplePlugin::<Cartesian3D, Sprite>::default())
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, |windows: Query<&Window, With<PrimaryWindow>>| {
                let window = windows.single().expect("Primary window must exist");
                let grid_x = (window.width() / TILE_SIZE).floor() as u32;
                let grid_y = (window.height() / TILE_SIZE).floor() as u32;
                let map_size = map_pixel_dimensions(grid_x, grid_y);
                println!("Map size: {:?}", map_size);
            },
        )
        .add_systems(OnEnter(GameState::InGame), setup_generator)
        .add_systems(OnExit(GameState::InGame), cleanup_game)
        .add_systems(OnEnter(GameState::Restarting), cleanup_game)
        .add_systems(OnEnter(GameState::Restarting),
                     |mut next: ResMut<NextState<GameState>>| {
                         next.set(GameState::InGame);
                     }
        )
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;