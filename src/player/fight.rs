use crate::audio::maybe_play;
use crate::boss::Boss;
use crate::enemy::Enemy;
use crate::exceptions::RTGException;
use crate::player::Player;
use bevy::prelude::*;

const MELEE_RANGE: f32 = 25.0;
const BOSS_MELEE_RANGE: f32 = 120.0;

#[derive(Resource, Default)]
pub(crate) struct PlayerDamageSoundState {
    use_first: bool,
}

pub(crate) fn player_fight_system(
    mut commands: Commands,
    time: Res<Time>,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
    mut boss_query: Query<(&Transform, &mut Boss)>,
    asset_server: Res<AssetServer>,
    mut sound_state: ResMut<PlayerDamageSoundState>,
) -> Result<(), RTGException> {
    let Ok((player_transform, mut player)) = player_query.single_mut() else {
        return Err(RTGException::RTG_PLAYER_ANIMATION_CANT_LOAD);
    };

    player.damage_cooldown.tick(time.delta());
    player.attack_cooldown.tick(time.delta());

    // — Ennemis normaux —
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
            play_damage_sound(&mut commands, &asset_server, &mut sound_state);
        }

        if player.attack_cooldown.is_finished() {
            enemy.health -= player.damage;
            player.attack_cooldown.reset();
        }
    }

    // — Boss —
    for (boss_transform, mut boss) in boss_query.iter_mut() {
        println!("BOSS TROUVÉ dans la query, health={}", boss.health);

        let distance = player_transform
            .translation
            .truncate()
            .distance(boss_transform.translation.truncate());

        println!("distance={:.1}, attack_finished={}, damage_finished={}",
                 distance,
                 player.attack_cooldown.is_finished(),
                 player.damage_cooldown.is_finished()
        );
        let distance = player_transform
            .translation
            .truncate()
            .distance(boss_transform.translation.truncate());

        if distance > BOSS_MELEE_RANGE {
            continue;
        }

        if distance > BOSS_MELEE_RANGE {
            println!("→ trop loin, skip ({})", distance);
            continue;
        }

        println!("→ DANS LA RANGE, on attaque !");

        if player.attack_cooldown.is_finished() {
            boss.health -= player.damage;
            player.attack_cooldown.reset();
            println!("→ DÉGÂTS infligés ! health={}", boss.health);
        }

        if player.damage_cooldown.is_finished() {
            player.health -= 20.0;
            player.damage_cooldown.reset();
            play_damage_sound(&mut commands, &asset_server, &mut sound_state);
        }

        if player.attack_cooldown.is_finished() {
            boss.health -= player.damage;
            player.attack_cooldown.reset();
        }
    }

    Ok(())
}

fn play_damage_sound(
    commands: &mut Commands,
    asset_server: &AssetServer,
    sound_state: &mut PlayerDamageSoundState,
) {
    sound_state.use_first = !sound_state.use_first;
    let sound = if sound_state.use_first {
        "player_damage_1.ogg"
    } else {
        "player_damage_2.ogg"
    };
    maybe_play(commands, asset_server, sound, 0.80);
}