use crate::enemy::animate::atlas_index_for;
use crate::enemy::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_rapier2d::prelude::*;
use crate::InGameEntity;
use crate::round::RoundState;
use rand::Rng;
use crate::core::collision_groups::enemy_membership;
use crate::enemy::projectile::ENEMY_ATTACK_RANGE;

pub(crate) fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut round: ResMut<RoundState>,
) {
    let Ok(window) = windows.single() else { return };
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;

    let mut rng = rand::rng();
    let player_pos = Vec2::ZERO;

    // --- Melee enemies ---
    let melee_texture = asset_server.load("skeleton-spritesheet.png");
    let melee_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        WALK_FRAMES as u32,
        12,
        None,
        None,
    ));

    for _ in 0..round.enemy_count() {
        let pos = find_spawn_position(
            &mut rng,
            half_w,
            half_h,
            player_pos,
            round.min_distance_from_player,
        );

        let facing = Facing::Down;
        let start_index = atlas_index_for(facing, 0);

        commands.spawn((
            Sprite::from_atlas_image(
                melee_texture.clone(),
                TextureAtlas {
                    layout: melee_layout.clone(),
                    index: start_index,
                },
            ),
            Transform::from_translation(Vec3::new(pos.x, pos.y, 20.0)),
            Enemy {
                health: 100.0,
                damage: 10.0,
            },
            AnimationState {
                facing,
                moving: true,
                was_moving: false,
            },
            AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
            InGameEntity,
            RigidBody::Dynamic,
            Collider::cuboid(16.0, 16.0),
            LockedAxes::ROTATION_LOCKED,
            GravityScale(0.0),
            Velocity::default(),
            Damping { linear_damping: 10.0, angular_damping: 0.0 },
            enemy_membership(),
        ));
    }

    // --- Ranged enemies ---
    let ranged_texture = asset_server.load("rangedEnemy.png");
    let ranged_layout = atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::splat(TILE_SIZE),
        WALK_FRAMES as u32,
        12,
        None,
        None,
    ));

    let ranged_count = (round.enemy_count() / 3).max(1);
    for _ in 0..ranged_count {
        let pos = find_spawn_position(
            &mut rng,
            half_w,
            half_h,
            player_pos,
            round.min_distance_from_player,
        );

        let facing = Facing::Down;
        let start_index = atlas_index_for(facing, 0);

        commands.spawn((
            Sprite::from_atlas_image(
                ranged_texture.clone(),
                TextureAtlas {
                    layout: ranged_layout.clone(),
                    index: start_index,
                },
            ),
            Transform::from_translation(Vec3::new(pos.x, pos.y, 20.0)),
            Enemy {
                health: 80.0,
                damage: 0.0,
            },
            RangedEnemy {
                fire_cooldown: Timer::from_seconds(2.0, TimerMode::Repeating),
                attack_range: ENEMY_ATTACK_RANGE,
            },
            AnimationState {
                facing,
                moving: true,
                was_moving: false,
            },
            AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
            InGameEntity,
            RigidBody::Dynamic,
            Collider::cuboid(16.0, 16.0),
            LockedAxes::ROTATION_LOCKED,
            GravityScale(0.0),
            Velocity::default(),
            Damping { linear_damping: 10.0, angular_damping: 0.0 },
        ));
    }

    round.mark_spawned();
}

fn find_spawn_position(
    rng: &mut impl Rng,
    half_w: f32,
    half_h: f32,
    player_pos: Vec2,
    min_distance: f32,
) -> Vec2 {
    for _ in 0..100 {
        let x = rng.random_range(-half_w..half_w);
        let y = rng.random_range(-half_h..half_h);
        let candidate = Vec2::new(x, y);
        if candidate.distance(player_pos) >= min_distance {
            return candidate;
        }
    }
    Vec2::new(half_w * 0.8, half_h * 0.8)
}