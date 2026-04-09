use bevy::prelude::*;
use rand::Rng;

/// Cooldown entre deux sons du même type pour éviter le spam.
#[derive(Resource)]
pub(crate) struct SoundCooldowns {
    pub(crate) enemy_hit: Timer,
}

impl Default for SoundCooldowns {
    fn default() -> Self {
        let mut t = Timer::from_seconds(0.12, TimerMode::Once);
        t.tick(std::time::Duration::from_secs_f32(0.12)); // prêt dès le départ
        Self { enemy_hit: t }
    }
}

/// Plays a sound with a probability `chance` (0.0–1.0).
/// Returns `true` if the sound was triggered.
pub(crate) fn maybe_play(
    commands: &mut Commands,
    asset_server: &AssetServer,
    path: &'static str,
    chance: f32,
) -> bool {
    if rand::rng().random::<f32>() < chance {
        commands.spawn((
            AudioPlayer::new(asset_server.load(path)),
            PlaybackSettings {
                volume: bevy::audio::Volume::Linear(0.05),
                ..default()
            },
        ));
        return true;
    }
    false
}

pub(crate) fn tick_cooldowns(time: Res<Time>, mut cooldowns: ResMut<SoundCooldowns>) {
    cooldowns.enemy_hit.tick(time.delta());
}
