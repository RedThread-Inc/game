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
            FpsLimit::Fps30 => Some(1.0 / 30.0),
            FpsLimit::Fps60 => Some(1.0 / 60.0),
            FpsLimit::Fps120 => Some(1.0 / 120.0),
            FpsLimit::Unlimited => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            FpsLimit::Fps30 => "30",
            FpsLimit::Fps60 => "60",
            FpsLimit::Fps120 => "120",
            FpsLimit::Unlimited => "Max",
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Language {
    French,
    English,
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
            language: Language::French,
            key_left: KeyCode::KeyA,
            key_right: KeyCode::KeyD,
            key_up: KeyCode::KeyW,
            key_down: KeyCode::KeyS,
            key_pause: KeyCode::Escape,
        }
    }
}

pub fn key_name(code: KeyCode) -> &'static str {
    match code {
        KeyCode::KeyA => "A", KeyCode::KeyB => "B", KeyCode::KeyC => "C",
        KeyCode::KeyD => "D", KeyCode::KeyE => "E", KeyCode::KeyF => "F",
        KeyCode::KeyG => "G", KeyCode::KeyH => "H", KeyCode::KeyI => "I",
        KeyCode::KeyJ => "J", KeyCode::KeyK => "K", KeyCode::KeyL => "L",
        KeyCode::KeyM => "M", KeyCode::KeyN => "N", KeyCode::KeyO => "O",
        KeyCode::KeyP => "P", KeyCode::KeyQ => "Q", KeyCode::KeyR => "R",
        KeyCode::KeyS => "S", KeyCode::KeyT => "T", KeyCode::KeyU => "U",
        KeyCode::KeyV => "V", KeyCode::KeyW => "W", KeyCode::KeyX => "X",
        KeyCode::KeyY => "Y", KeyCode::KeyZ => "Z",
        KeyCode::Digit0 => "0", KeyCode::Digit1 => "1", KeyCode::Digit2 => "2",
        KeyCode::Digit3 => "3", KeyCode::Digit4 => "4", KeyCode::Digit5 => "5",
        KeyCode::Digit6 => "6", KeyCode::Digit7 => "7", KeyCode::Digit8 => "8",
        KeyCode::Digit9 => "9",
        KeyCode::Space => "Espace",
        KeyCode::Escape => "Echap",
        KeyCode::Enter => "Entree",
        KeyCode::Tab => "Tab",
        KeyCode::ShiftLeft => "Maj G",
        KeyCode::ShiftRight => "Maj D",
        KeyCode::ControlLeft => "Ctrl G",
        KeyCode::ControlRight => "Ctrl D",
        KeyCode::AltLeft => "Alt G",
        KeyCode::AltRight => "Alt D",
        KeyCode::ArrowLeft => "<- Gauche",
        KeyCode::ArrowRight => "-> Droite",
        KeyCode::ArrowUp => "Haut",
        KeyCode::ArrowDown => "Bas",
        KeyCode::F1 => "F1", KeyCode::F2 => "F2", KeyCode::F3 => "F3",
        KeyCode::F4 => "F4", KeyCode::F5 => "F5", KeyCode::F6 => "F6",
        KeyCode::F7 => "F7", KeyCode::F8 => "F8", KeyCode::F9 => "F9",
        KeyCode::F10 => "F10", KeyCode::F11 => "F11", KeyCode::F12 => "F12",
        _ => "?",
    }
}

pub fn t(lang: &Language, key: &'static str) -> &'static str {
    match (lang, key) {
        // Main menu
        (Language::French,  "menu_play")     => "Partir a l'aventure",
        (Language::English, "menu_play")     => "Start the adventure",
        (Language::French,  "menu_settings") => "Parametres",
        (Language::English, "menu_settings") => "Settings",
        (Language::French,  "menu_quit")     => "Abandonner la quete",
        (Language::English, "menu_quit")     => "Quit",
        // Pause menu
        (Language::French,  "pause_title")     => "PAUSE",
        (Language::English, "pause_title")     => "PAUSE",
        (Language::French,  "pause_resume")    => "Reprendre",
        (Language::English, "pause_resume")    => "Resume",
        (Language::French,  "pause_restart")   => "Recommencer",
        (Language::English, "pause_restart")   => "Restart",
        (Language::French,  "pause_main_menu") => "Menu principal",
        (Language::English, "pause_main_menu") => "Main menu",
        (Language::French,  "pause_quit")      => "Abandonner la quete",
        (Language::English, "pause_quit")      => "Quit",
        // Death menu
        (Language::French,  "death_subtitle") => "Vous etes mort...",
        (Language::English, "death_subtitle") => "You have died...",
        (Language::French,  "death_retry")    => "Retenter l'aventure",
        (Language::English, "death_retry")    => "Try again",
        (Language::French,  "death_main_menu")=> "Menu principal",
        (Language::English, "death_main_menu")=> "Main menu",
        (Language::French,  "death_quit")     => "Abandonner la quete",
        (Language::English, "death_quit")     => "Quit",
        // Settings menu
        (Language::French,  "settings_title")    => "PARAMETRES",
        (Language::English, "settings_title")    => "SETTINGS",
        (Language::French,  "settings_fps")      => "LIMITE FPS",
        (Language::English, "settings_fps")      => "FPS LIMIT",
        (Language::French,  "settings_controls") => "CONTROLES",
        (Language::English, "settings_controls") => "CONTROLS",
        (Language::French,  "settings_language") => "LANGUE",
        (Language::English, "settings_language") => "LANGUAGE",
        (Language::French,  "settings_back")     => "Retour",
        (Language::English, "settings_back")     => "Back",
        (Language::French,  "key_left")  => "Aller a gauche",
        (Language::English, "key_left")  => "Move left",
        (Language::French,  "key_right") => "Aller a droite",
        (Language::English, "key_right") => "Move right",
        (Language::French,  "key_up")    => "Aller en haut",
        (Language::English, "key_up")    => "Move up",
        (Language::French,  "key_down")  => "Aller en bas",
        (Language::English, "key_down")  => "Move down",
        (Language::French,  "key_pause") => "Pause",
        (Language::English, "key_pause") => "Pause",
        _ => key,
    }
}
