use crate::enemy::animate::atlas_index_for;
use crate::enemy::*;
use bevy::prelude::*;
use crate::player::Player;

pub(crate) fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture = asset_server.load("skeleton-spritesheet.png");

    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        WALK_FRAMES as u32,
        12,
        None,
        None,
    ));

    let facing = Facing::Down;
    let start_index = atlas_index_for(facing, 0);

    let enemy = Enemy {
        health: 100.0,
        damage: 10.0,
    };

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: start_index,
            },
        ),
        Transform::from_translation(Vec3::new(250.0, 50.0, 20.0)),
        enemy,
        AnimationState {
            facing,
            moving: true,
            was_moving: false,
        },
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
    ));
}
