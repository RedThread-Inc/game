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
use crate::map::generate::{TerrainHeightMap, TILE_SIZE as MAP_TILE_SIZE};
use crate::map::perlin::TerrainZone;


pub(crate) fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    windows: Query<&Window, With<PrimaryWindow>>,
    mut round: ResMut<RoundState>,
    terrain: Option<Res<TerrainHeightMap>>,
) {
    let Ok(window) = windows.single() else { return };
    let half_w = window.width() / 2.0;
    let half_h = window.height() / 2.0;

    let mut rng = rand::rng();
    let player_pos = Vec2::ZERO;
    let current_round = round.current;

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
            terrain.as_deref(),
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
                health: melee_health(current_round),
                damage: melee_damage(current_round),
            },
            AnimationState {
                facing,
                moving: true,
                was_moving: false,
            },
            AnimationTimer(Timer::from_seconds(ANIM_DT, TimerMode::Repeating)),
            InGameEntity,
            RigidBody::Dynamic,
            Collider::cuboid(10.0, 8.0),
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
            terrain.as_deref(),
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
                health: ranged_health(current_round),
                damage: ranged_damage(current_round),
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
            Collider::cuboid(12.0, 8.0),
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
    terrain: Option<&TerrainHeightMap>,
) -> Vec2 {
    for _ in 0..200 {
        let x = rng.random_range(-half_w..half_w);
        let y = rng.random_range(-half_h..half_h);
        let candidate = Vec2::new(x, y);

        if candidate.distance(player_pos) < min_distance {
            continue;
        }

        if let Some(t) = terrain {
            let tile_x = ((x + half_w) / MAP_TILE_SIZE) as u32;
            let tile_y = ((y + half_h) / MAP_TILE_SIZE) as u32;
            let tx = tile_x.min(t.0.width.saturating_sub(1));
            let ty = tile_y.min(t.0.height.saturating_sub(1));

            let zone = t.0.classify(tx, ty);
            if zone == TerrainZone::Water {
                continue;
            }

            // Reject water-adjacent tiles (the visual border)
            if is_near_water_tile(&t.0, tx, ty) {
                continue;
            }
        }


        return candidate;
    }
    Vec2::new(half_w * 0.5, half_h * 0.5)
}

fn is_near_water_tile(height_map: &crate::map::perlin::HeightMap, x: u32, y: u32) -> bool {
    for dx in -1i32..=1 {
        for dy in -1i32..=1 {
            if dx == 0 && dy == 0 { continue; }
            let nx = x as i32 + dx;
            let ny = y as i32 + dy;
            if nx < 0 || nx >= height_map.width as i32 || ny < 0 || ny >= height_map.height as i32 {
                continue;
            }
            if height_map.classify(nx as u32, ny as u32) == TerrainZone::Water {
                return true;
            }
        }
    }
    false
}