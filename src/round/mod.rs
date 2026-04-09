pub(crate) mod plugin;

use bevy::prelude::*;
use crate::enemy::Enemy;
use crate::InGameState;

#[derive(Resource)]
pub(crate) struct RoundState {
    pub(crate) current: u32,
    base_count: u32,
    scaling: u32,
    pub(crate) min_distance_from_player: f32,
    spawned: bool,
}

impl Default for RoundState {
    fn default() -> Self {
        Self {
            current: 1,
            base_count: 3,
            scaling: 2,
            min_distance_from_player: 200.0,
            spawned: false,
        }
    }
}

impl RoundState {
    pub(crate) fn enemy_count(&self) -> usize {
        (self.base_count + (self.current - 1) * self.scaling) as usize
    }

    pub(crate) fn mark_spawned(&mut self) {
        self.spawned = true;
    }

    pub(crate) fn advance(&mut self) {
        self.current += 1;
        self.spawned = false;
    }
}

pub(crate) fn check_round_end_system(
    enemies: Query<&Enemy>,
    round: Res<RoundState>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if round.spawned && enemies.is_empty() {
        next_state.set(InGameState::ChoosingUpgrade);
    }
}
