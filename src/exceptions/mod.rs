use bevy::prelude::In;
use strum_macros::EnumIter;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, EnumIter, Clone)]
pub(crate) enum RTGException {
    // PLAYER SECTION
    RTG_PLAYER_FAILED_TO_SPAWN,
    RTG_PLAYER_FAILED_TO_ANIMATE,
    RTG_PLAYER_ANIMATION_CANT_LOAD,
    RTG_PLAYER_ANIMATION_TEXTURE_ATLAS_CANT_LOAD,
    RTG_PLAYER_MOVEMENT_DELTA_TIME_INVALID,

    // ENEMY SECTION
    RTG_ENEMY_ANIMATION_TEXTURE_ATLAS_CANT_LOAD,
    RTG_ENEMY_MOVEMENT_ZERO_DIRECTION_VECTOR,
    RTG_ENEMY_IMPOSSIBLE_TO_ANIMATE_ENEMY_NOT_FOUND,
    RTG_ENEMY_IMPOSSIBLE_TO_ANIMATE_MULTIPLE_ENEMIES_FOUND,
    RTG_ENEMY_FAILED_TO_ANIMATE,
    RTG_ENEMY_MOVE_TOWARDS_PLAYER_NOT_FOUND,
    RTG_ENEMY_MOVE_TOWARDS_PLAYER_MULTIPLE_PLAYER_FOUND,

    // MAP SECTION
    REDTHREAD_FAILED_TO_GENERATE_MAP_MISSING_GAME_WINDOW

}

impl RTGException {
    #[allow(unused)]
    pub(crate) fn to_string(&self) -> String {
        let target = concat!(module_path!(), ":", line!());
        match self {
            // PLAYER SECTION
            RTGException::RTG_PLAYER_FAILED_TO_SPAWN => {
                format!("[ERROR][{target}] Failed while trying to spawn player.")
            }
            RTGException::RTG_PLAYER_FAILED_TO_ANIMATE => {
                format!("[ERROR][{target}] Failed to animate the player.")
            }
            RTGException::RTG_PLAYER_ANIMATION_CANT_LOAD => {
                format!("[ERROR][{target}] Cannot load the player animation.")
            }
            RTGException::RTG_PLAYER_ANIMATION_TEXTURE_ATLAS_CANT_LOAD => {
                format!("[ERROR][{target}] Cannot load the player texture atlas.")
            }
            RTGException::RTG_PLAYER_MOVEMENT_DELTA_TIME_INVALID => {
                format!("[ERROR][{target}] Player movement delta time is invalid.")
            }

            // ENEMY SECTION
            // TODO: specify the enemy ID
            RTGException::RTG_ENEMY_ANIMATION_TEXTURE_ATLAS_CANT_LOAD => {
                format!("[ERROR][{target}] Cannot load an enemy texture atlas.")
            }
            RTGException::RTG_ENEMY_MOVEMENT_ZERO_DIRECTION_VECTOR => {
                format!("[WARN][{target}] Vector enemy direction is zero.")
            }
            RTGException::RTG_ENEMY_IMPOSSIBLE_TO_ANIMATE_ENEMY_NOT_FOUND => {
                format!("[ERROR][{target}] Impossible to animate, enemy not found.")
            }
            RTGException::RTG_ENEMY_IMPOSSIBLE_TO_ANIMATE_MULTIPLE_ENEMIES_FOUND => {
                format!("[ERROR][{target}] Multiple Enemy entities found, impossible to animate")
            }
            RTGException::RTG_ENEMY_FAILED_TO_ANIMATE => {
                format!("[ERROR][{target}] Multiple Enemy entities found, impossible to animate")
            }
            RTGException::RTG_ENEMY_MOVE_TOWARDS_PLAYER_NOT_FOUND => {
                format!("[ERROR][{target}] Enemy can't go towards player, player not found.")
            }
            RTGException::RTG_ENEMY_MOVE_TOWARDS_PLAYER_MULTIPLE_PLAYER_FOUND => {
                format!("[ERROR][{target}] Enemy can't go towards player, multiple players found.")
            }

            // MAP SECTION
            RTGException::REDTHREAD_FAILED_TO_GENERATE_MAP_MISSING_GAME_WINDOW => {
                format!("[FATAL][{target}] Impossible to generate the map, game window was not found.")
            }
        }
    }
}

pub(crate) fn log_rtg_exception(In(result): In<Result<(), RTGException>>) {
    if let Err(e) = result {
        println!("{}", e.to_string());
    }
}