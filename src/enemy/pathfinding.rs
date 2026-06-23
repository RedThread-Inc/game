use crate::map::generate::{TerrainHeightMap, TILE_SIZE as MAP_TILE_SIZE};
use crate::map::perlin::TerrainZone;
use bevy::prelude::*;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

const WAYPOINT_REACHED_DISTANCE: f32 = 12.0;

// ── Tile coordinate ────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub(crate) struct TilePos {
    pub(crate) x: i32,
    pub(crate) y: i32,
}

// ── A* node ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq)]
struct AStarNode {
    tile: TilePos,
    f: f32,
}

impl Eq for AStarNode {}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f.partial_cmp(&self.f).unwrap_or(Ordering::Equal)
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn world_to_tile(world: Vec2, half_w: f32, half_h: f32) -> TilePos {
    TilePos {
        x: ((world.x + half_w) / MAP_TILE_SIZE) as i32,
        y: ((world.y + half_h) / MAP_TILE_SIZE) as i32,
    }
}

pub(crate) fn tile_to_world(tile: TilePos, half_w: f32, half_h: f32) -> Vec2 {
    Vec2::new(
        tile.x as f32 * MAP_TILE_SIZE - half_w + MAP_TILE_SIZE * 0.5,
        tile.y as f32 * MAP_TILE_SIZE - half_h + MAP_TILE_SIZE * 0.5,
    )
}

fn is_walkable(terrain: &TerrainHeightMap, tx: i32, ty: i32) -> bool {
    let w = terrain.0.width as i32;
    let h = terrain.0.height as i32;
    if tx < 0 || tx >= w || ty < 0 || ty >= h {
        return false;
    }
    let zone = terrain.0.classify(tx as u32, ty as u32);
    if zone == TerrainZone::Water {
        return false;
    }
    // Reject water-adjacent tiles (visual border)
    for dx in -1i32..=1 {
        for dy in -1i32..=1 {
            if dx == 0 && dy == 0 { continue; }
            let nx = tx + dx;
            let ny = ty + dy;
            if nx < 0 || nx >= w || ny < 0 || ny >= h { continue; }
            if terrain.0.classify(nx as u32, ny as u32) == TerrainZone::Water {
                return false;
            }
        }
    }
    true
}

fn heuristic(a: TilePos, b: TilePos) -> f32 {
    // Octile distance (works for 8-directional movement)
    let dx = (a.x - b.x).abs() as f32;
    let dy = (a.y - b.y).abs() as f32;
    let straight = 1.0f32;
    let diagonal = std::f32::consts::SQRT_2;
    straight * (dx + dy) + (diagonal - 2.0 * straight) * dx.min(dy)
}

const NEIGHBORS_8: [(i32, i32); 8] = [
    (1, 0), (-1, 0), (0, 1), (0, -1),
    (1, 1), (-1, 1), (1, -1), (-1, -1),
];

// ── A* ────────────────────────────────────────────────────────────────────────

/// Returns a path as world-space Vec2 waypoints (start excluded, goal included).
/// Returns None if no path found or terrain unavailable.
pub(crate) fn astar_path(
    start_world: Vec2,
    goal_world: Vec2,
    terrain: &TerrainHeightMap,
    half_w: f32,
    half_h: f32,
) -> Option<Vec<Vec2>> {
    let start = world_to_tile(start_world, half_w, half_h);
    let goal  = world_to_tile(goal_world, half_w, half_h);

    if start == goal {
        return Some(vec![]);
    }

    // If goal is not walkable, find nearest walkable tile
    let goal = if is_walkable(terrain, goal.x, goal.y) {
        goal
    } else {
        find_nearest_walkable(terrain, goal, half_w, half_h)?
    };

    let mut open: BinaryHeap<AStarNode> = BinaryHeap::new();
    let mut came_from: HashMap<TilePos, TilePos> = HashMap::new();
    let mut g_score: HashMap<TilePos, f32> = HashMap::new();

    g_score.insert(start, 0.0);
    open.push(AStarNode { tile: start, f: heuristic(start, goal) });

    while let Some(AStarNode { tile: current, .. }) = open.pop() {
        if current == goal {
            // Reconstruct path
            let mut path = Vec::new();
            let mut cur = current;
            while let Some(&prev) = came_from.get(&cur) {
                path.push(tile_to_world(cur, half_w, half_h));
                cur = prev;
            }
            path.reverse();
            return Some(path);
        }

        let current_g = *g_score.get(&current).unwrap_or(&f32::INFINITY);

        for (dx, dy) in NEIGHBORS_8 {
            let neighbor = TilePos { x: current.x + dx, y: current.y + dy };

            if !is_walkable(terrain, neighbor.x, neighbor.y) {
                continue;
            }

            // Diagonal cost
            let step_cost = if dx != 0 && dy != 0 {
                // Block diagonal movement through two non-walkable corners
                if !is_walkable(terrain, current.x + dx, current.y)
                    || !is_walkable(terrain, current.x, current.y + dy)
                {
                    continue;
                }
                std::f32::consts::SQRT_2
            } else {
                1.0
            };

            let tentative_g = current_g + step_cost;
            let known_g = *g_score.get(&neighbor).unwrap_or(&f32::INFINITY);

            if tentative_g < known_g {
                came_from.insert(neighbor, current);
                g_score.insert(neighbor, tentative_g);
                let f = tentative_g + heuristic(neighbor, goal);
                open.push(AStarNode { tile: neighbor, f });
            }
        }
    }

    None // No path found
}

fn find_nearest_walkable(
    terrain: &TerrainHeightMap,
    origin: TilePos,
    _half_w: f32,
    _half_h: f32,
) -> Option<TilePos> {
    for radius in 1i32..=5 {
        for dx in -radius..=radius {
            for dy in -radius..=radius {
                if dx.abs() != radius && dy.abs() != radius { continue; }
                let candidate = TilePos { x: origin.x + dx, y: origin.y + dy };
                if is_walkable(terrain, candidate.x, candidate.y) {
                    return Some(candidate);
                }
            }
        }
    }
    None
}

// ── Path component ────────────────────────────────────────────────────────────

/// Attached to any entity that uses A* navigation.
#[derive(Component)]
pub(crate) struct AStarPath {
    /// Remaining waypoints (world space), front = next target
    pub(crate) waypoints: Vec<Vec2>,
    /// Countdown to next recompute
    pub(crate) recompute_timer: Timer,
}

impl Default for AStarPath {
    fn default() -> Self {
        Self {
            waypoints: Vec::new(),
            recompute_timer: Timer::from_seconds(0.25, TimerMode::Repeating),
        }
    }
}

pub(crate) fn seek_direction(
    path: &mut AStarPath,
    time: &Time,
    my_pos: Vec2,
    target_pos: Vec2,
    terrain: Option<&TerrainHeightMap>,
    half_w: f32,
    half_h: f32,
) -> Vec2 {
    path.recompute_timer.tick(time.delta());

    let needs_recompute = path.recompute_timer.just_finished() || path.waypoints.is_empty();
    if needs_recompute {
        path.waypoints = terrain
            .and_then(|t| astar_path(my_pos, target_pos, t, half_w, half_h))
            .unwrap_or_default();
    }

    while let Some(&next) = path.waypoints.first() {
        if my_pos.distance(next) <= WAYPOINT_REACHED_DISTANCE {
            path.waypoints.remove(0);
        } else {
            break;
        }
    }

    match path.waypoints.first() {
        Some(&next) => (next - my_pos).normalize_or_zero(),
        None => (target_pos - my_pos).normalize_or_zero(),
    }
}