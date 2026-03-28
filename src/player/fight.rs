use crate::enemy::Enemy;
use crate::exceptions::RTGException;
use crate::player::Player;
use bevy::prelude::*;

const MELEE_RANGE: f32 = 50.0;

pub(crate) fn player_fight_system(
    time: Res<Time>,
    mut player_query: Query<(&Transform, &mut Player)>,
    mut enemy_query: Query<(&Transform, &mut Enemy)>,
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
            println!("Player reçoit {} dégâts, HP restants: {}", enemy.damage, player.health);
        }

        if player.attack_cooldown.is_finished() {
            enemy.health -= player.damage;
            player.attack_cooldown.reset();
            println!("Ennemi reçoit {} dégâts, HP restants: {}", player.damage, enemy.health);
        }
    }

    Ok(())
}