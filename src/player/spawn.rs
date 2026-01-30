use crate::player::animate::atlas_index_for;
use crate::player::*;
use bevy::prelude::*;
use crate::exceptions::RTGException;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> Result<(), RTGException> {
    let player = Player {
        name: "XenHom".to_string(),
    };

    let texture = asset_server.load("character-spritesheet.png");
    let layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        WALK_FRAMES as u32,
        12,
        None,
        None,
    ));

    let facing = Facing::Down;
    let start_index = atlas_index_for(facing, 0);

    commands.spawn((
        Sprite::from_atlas_image(
            texture,
            TextureAtlas {
                layout,
                index: start_index,
            },
        ),
        Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_Z)),
        player,
        AnimationState {
            facing,
            moving: false,
            was_moving: false,
        },
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
    ));

    Ok(())
}

pub(crate) fn spawn_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if let Err(e) = spawn_player(commands, asset_server, atlas_layouts) {
        println!("[ERROR] - Spawn player error : {:?}", e)
    }
}
