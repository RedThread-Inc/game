use bevy::prelude::*;

#[derive(Resource, Clone)]
pub struct GameSettings {
    pub fps_limit: FpsLimit,
    pub language: Language,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_pause: KeyCode,
}

#[derive(Clone, PartialEq, Debug)]
pub enum FpsLimit {
    Fps30,
    Fps60,
    Fps120,
    Unlimited,
}

impl FpsLimit {
    pub fn target_secs(&self) -> Option<f64> {
        match self {
            FpsLimit::Fps30     => Some(1.0 / 30.0),
            FpsLimit::Fps60     => Some(1.0 / 60.0),
            FpsLimit::Fps120    => Some(1.0 / 120.0),
            FpsLimit::Unlimited => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            FpsLimit::Fps30     => "30",
            FpsLimit::Fps60     => "60",
            FpsLimit::Fps120    => "120",
            FpsLimit::Unlimited => "Max",
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Language {
    French,
    English,
    Russian,
}

#[derive(Clone, PartialEq, Debug)]
pub enum KeyAction {
    Left,
    Right,
    Up,
    Down,
    Pause,
}

impl Default for GameSettings {
    fn default() -> Self {
        Self {
            fps_limit: FpsLimit::Fps60,
            language:  Language::French,
            key_left:  KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up:    KeyCode::KeyW,
            key_down:  KeyCode::KeyS,
            key_pause: KeyCode::Escape,
        }
    }
}
