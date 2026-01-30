use bevy::prelude::*;
use crate::enemy::Enemy;
use crate::player::Player;

pub(crate) fn enemy_hits_player_system(
    time: Res<Time>,
    mut player_query: Query<(&Transform, &mut Player)>,
    enemy_query: Query<(&Transform, &Enemy)>,
) {
    let Ok((player_transform, mut player)) = player_query.single_mut() else {
        return;
    };

    player.damage_cooldown.tick(time.delta());

    if player.damage_cooldown.elapsed_secs() < player.damage_cooldown.duration().as_secs_f32() {
        return;
    }

    for (enemy_transform, enemy) in enemy_query.iter() {
        if (player_transform.translation.distance(enemy_transform.translation) < 28.0){
            player.health -= enemy.damage;
            player.damage_cooldown.reset();
            break;
        }
    }
}