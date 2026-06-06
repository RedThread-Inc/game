use std::time::Duration;
use crate::player::animate::atlas_index_for;
use crate::player::*;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::core::collision_groups::player_membership;
use crate::exceptions::RTGException;
use crate::InGameEntity;

fn spawn_player(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> Result<(), RTGException> {

    let mut damage_cooldown = Timer::from_seconds(1.0, TimerMode::Once);
    damage_cooldown.tick(Duration::from_secs_f32(1.0));

    let mut attack_cooldown = Timer::from_seconds(0.5, TimerMode::Once);
    attack_cooldown.tick(Duration::from_secs_f32(0.5));

    let player = Player {
        health: 100.0,
        max_health: 100.0,
        damage: 25.0,
        damage_cooldown,
        attack_cooldown,
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
            TextureAtlas { layout, index: start_index },
        ),
        Transform::from_translation(Vec3::new(0.0, 0.0, PLAYER_Z)),
        player,
        AnimationState { facing, moving: false, was_moving: false },
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
        InGameEntity,
        RigidBody::Dynamic,
        Collider::cuboid(10.0, 8.0),
        LockedAxes::ROTATION_LOCKED,
        GravityScale(0.0),
        Velocity::default(),
        Damping { linear_damping: 50.0, angular_damping: 0.0 },
        player_membership(),
    ));

    Ok(())
}

pub(crate) fn spawn_player_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) -> Result<(), RTGException> {
    if let Err(e) = spawn_player(commands, asset_server, atlas_layouts) {
        println!("{}", e.to_string());
        return Err(e);
    } else {
        Ok(())
    }
}