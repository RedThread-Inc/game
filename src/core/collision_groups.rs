use bevy_rapier2d::prelude::*;

pub const GROUP_PLAYER:            Group = Group::GROUP_1;
pub const GROUP_ENEMY:             Group = Group::GROUP_2;
pub const GROUP_BOSS:              Group = Group::GROUP_3;
pub const GROUP_PLAYER_PROJECTILE: Group = Group::GROUP_4;
pub const GROUP_ENEMY_PROJECTILE:  Group = Group::GROUP_5;
pub const GROUP_WORLD:             Group = Group::GROUP_6;

pub fn player_membership() -> CollisionGroups {
    CollisionGroups::new(
        GROUP_PLAYER,
        GROUP_WORLD | GROUP_ENEMY | GROUP_BOSS | GROUP_ENEMY_PROJECTILE,
    )
}

pub fn enemy_membership() -> CollisionGroups {
    CollisionGroups::new(
        GROUP_ENEMY,
        GROUP_WORLD | GROUP_PLAYER | GROUP_PLAYER_PROJECTILE,
    )
}

pub fn boss_membership() -> CollisionGroups {
    CollisionGroups::new(
        GROUP_BOSS,
        GROUP_WORLD | GROUP_PLAYER | GROUP_PLAYER_PROJECTILE,
    )
}

pub fn player_projectile_membership() -> CollisionGroups {
    CollisionGroups::new(
        GROUP_PLAYER_PROJECTILE,
        GROUP_ENEMY | GROUP_BOSS,
    )
}

pub fn enemy_projectile_membership() -> CollisionGroups {
    CollisionGroups::new(
        GROUP_ENEMY_PROJECTILE,
        GROUP_PLAYER,
    )
}

pub fn world_membership() -> CollisionGroups {
    CollisionGroups::new(
        GROUP_WORLD,
        GROUP_PLAYER | GROUP_ENEMY | GROUP_BOSS,
    )
}