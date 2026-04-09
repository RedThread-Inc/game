use bevy::prelude::*;
use crate::upgrade::PlayerUpgrades;
use crate::upgrade::ui::{cleanup_upgrade_ui, handle_upgrade_buttons, spawn_upgrade_ui};
use crate::{GameState, InGameState};

pub(crate) struct UpgradePlugin;

impl Plugin for UpgradePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerUpgrades>()
            .add_systems(OnEnter(GameState::InGame), reset_upgrades)
            .add_systems(OnEnter(InGameState::ChoosingUpgrade), spawn_upgrade_ui)
            .add_systems(
                Update,
                handle_upgrade_buttons.run_if(in_state(InGameState::ChoosingUpgrade)),
            )
            .add_systems(OnExit(InGameState::ChoosingUpgrade), cleanup_upgrade_ui);
    }
}

fn reset_upgrades(mut upgrades: ResMut<PlayerUpgrades>) {
    *upgrades = PlayerUpgrades::default();
}
