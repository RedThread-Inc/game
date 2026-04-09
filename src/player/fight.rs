use crate::audio::maybe_play;
use crate::enemy::Enemy;
use crate::exceptions::RTGException;
use crate::player::Player;
use bevy::prelude::*;

const MELEE_RANGE: f32 = 50.0;

#[derive(Resource, Default)]
pub(crate) struct PlayerDamageSoundState {
    use_first: bool,
}

pub(crate) fn player_fight_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    asset_server: Res<AssetServer>,
    mut sound_state: ResMut<PlayerDamageSoundState>,
) -> Result<(), RTGException> {
    let Ok((player_transform, mut player)) = player_query.single_mut() else {
        return Err(RTGException::RTG_PLAYER_ANIMATION_CANT_LOAD);
    };

    player.damage_cooldown.tick(time.delta());
    player.attack_cooldown.tick(time.delta());

    for (enemy_transform, mut enemy) in enemy_query.iter_mut() {
        let distance = player_transform
            .translation
            .truncate()
            .distance(enemy_transform.translation.truncate());

        if distance > MELEE_RANGE {
            continue;
        }

        if player.damage_cooldown.is_finished() {
            player.health -= enemy.damage;
            player.damage_cooldown.reset();

            sound_state.use_first = !sound_state.use_first;
            let sound = if sound_state.use_first {
                "player_damage_1.ogg"
            } else {
                "player_damage_2.ogg"
            };
            maybe_play(&mut commands, &asset_server, sound, 0.80);
        }

        if player.attack_cooldown.is_finished() {
            enemy.health -= player.damage;
            player.attack_cooldown.reset();
        }
    }

    Ok(())
}
