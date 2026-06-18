use bevy::prelude::*;
use crate::round::{check_round_end_system, RoundStartedEvent, RoundState};
use crate::{GameState, InGameState};
use crate::boss::spawn::spawn_boss;
use crate::enemy::spawn::spawn_enemies;

pub(crate) struct RoundPlugin;

impl Plugin for RoundPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RoundState>()
            .add_message::<RoundStartedEvent>()
            .add_systems(OnEnter(GameState::InGame), (reset_round, start_first_round).chain())
            .add_systems(
                Update,
                check_round_end_system
                    .after(spawn_enemies)
                    .after(spawn_boss)
                    .run_if(in_state(GameState::InGame))
                    .run_if(in_state(InGameState::Playing)),
            );
    }
}

fn start_first_round(mut writer: MessageWriter<RoundStartedEvent>) {
    writer.write(RoundStartedEvent);
}

fn reset_round(mut round: ResMut<RoundState>) {
    *round = RoundState::default();
}
