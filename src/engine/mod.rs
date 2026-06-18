use crate::audio::{tick_cooldowns, SoundCooldowns};
use crate::enemy::plugin::EnemyPlugin;
use crate::player::plugin::PlayerPlugin;
use crate::interface::plugin::InterfacePlugin;
use crate::menu::main_menu::MainMenuPlugin;
use crate::menu::settings_menu::SettingsMenuPlugin;
use crate::round::plugin::RoundPlugin;
use crate::settings::GameSettings;
use crate::upgrade::plugin::UpgradePlugin;
use crate::{GameState, InGameEntity, InGameState};
use bevy::{
    prelude::*,
    window::{Window, WindowPlugin, WindowResolution},
};
use bevy::window::{PrimaryWindow, WindowMode};
use bevy_procedural_tilemaps::prelude::*;
use crate::map::generate::{setup_generator, TILE_SIZE};
use crate::menu::pause_menu::PauseMenuPlugin;
use crate::menu::death_menu::DeathMenuPlugin;
use crate::boss::plugin::BossPlugin;
use bevy_rapier2d::prelude::*;
use crate::exceptions::log_rtg_exception;

#[derive(Resource, Clone)]
pub(crate) struct GameFont(pub(crate) Handle<Font>);

impl FromWorld for GameFont {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        GameFont(asset_server.load("fonts/Ubuntu-R.ttf"))
    }
}

#[derive(Resource)]
struct FrameLimiter(std::time::Instant);

impl Default for FrameLimiter {
    fn default() -> Self {
        Self(std::time::Instant::now())
    }
}

fn apply_frame_limit(settings: Res<GameSettings>, mut limiter: ResMut<FrameLimiter>) {
    if let Some(target_secs) = settings.fps_limit.target_secs() {
        let target = std::time::Duration::from_secs_f64(target_secs);
        let elapsed = limiter.0.elapsed();
        if elapsed < target {
            std::thread::sleep(target - elapsed);
        }
    }
    limiter.0 = std::time::Instant::now();
}

pub(crate) fn cleanup_game(mut commands: Commands, query: Query<Entity, With<InGameEntity>>) {
    for entity in &query {
        commands.entity(entity).despawn_related::<Children>();
        commands.entity(entity).despawn();
    }

}

pub(crate) fn init_app() {
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
                        resolution: WindowResolution::new(1920, 1080),
                        resizable: true,
                        mode: WindowMode::Windowed,
                        ..default()
                    }),
                    ..default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.0),
        ))
        .init_state::<GameState>()
        .add_sub_state::<InGameState>()
        .init_resource::<SoundCooldowns>()
        .init_resource::<GameSettings>()
        .init_resource::<FrameLimiter>()
        .init_resource::<GameFont>()
        .add_systems(Update, tick_cooldowns)
        .add_systems(Last, apply_frame_limit)
        .add_plugins((PlayerPlugin, EnemyPlugin, InterfacePlugin, MainMenuPlugin, PauseMenuPlugin, DeathMenuPlugin, SettingsMenuPlugin, RoundPlugin, UpgradePlugin, BossPlugin))
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::InGame), setup_generator.pipe(log_rtg_exception))
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