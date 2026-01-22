use bevy::prelude::*;
use crate::Player::movement::move_player;
use crate::Player::spawn::spawn_player;
use crate::Player::plugin::PlayerPlugin;

pub(crate) fn init_app() {
    App::new()
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            file_path: "src/assets".into(),
            ..default()
        }),)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, move_player)
        .add_plugins(PlayerPlugin)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Person;