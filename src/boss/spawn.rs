use crate::boss::*;
use crate::enemy::{AnimationState, AnimationTimer, Facing, ANIM_DT, TILE_SIZE, WALK_FRAMES};
use crate::enemy::animate::atlas_index_for;
use crate::InGameEntity;
use crate::round::RoundState;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub(crate) fn spawn_boss(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    round: Res<RoundState>,
) {

    if round.current == 0 || round.current % 5 != 0 {
        return;
    }

    let Ok(window) = windows.single() else { return };
    let half_w = window.width() / 2.0;

    let stats = BossStats::for_round(round.current);

    // Même spritesheet que skeleton, même layout 9x12
    let texture = asset_server.load("boss.png");
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
        Transform {
            translation: Vec3::new(half_w - 120.0, 0.0, 20.0),
            scale: Vec3::splat(BOSS_SCALE),
            ..default()
        },
        Boss {
            health: stats.hp,
            round: round.current,
        },
        AnimationState {
            facing,
            moving: true,
            was_moving: false,
        },
        AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
        BossAttackTimer(Timer::from_seconds(stats.fire_rate, TimerMode::Repeating)),
        InGameEntity,
    ));
}