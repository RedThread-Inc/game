use bevy::prelude::*;
use crate::round::{check_round_end_system, RoundState};
use crate::{GameState, InGameState};

pub(crate) struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoundState>()
            .add_systems(OnEnter(GameState::InGame), reset_round)
            .add_systems(
                Update,
                check_round_end_system
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(InGameState::Playing)),
            );
    }
}

fn reset_round(mut round: ResMut<RoundState>) {
    *round = RoundState::default();
}
