use bevy::prelude::*;
use crate::enemy::plugin::EnemyPlugin;
use crate::player::movement::move_player;
use crate::enemy::movement::move_enemy_towards_player;
use crate::player::plugin::PlayerPlugin;

pub(crate) fn init_app() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(
            DefaultPlugins.set(AssetPlugin {
                file_path: "src/assets".into(),
                ..default()
            }),
        )
        .add_plugins((PlayerPlugin, EnemyPlugin))
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;