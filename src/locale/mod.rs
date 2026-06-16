mod fr;
mod en;
mod ru;

use crate::settings::Language;
use bevy::prelude::KeyCode;

pub fn t(lang: &Language, key: &'static str) -> &'static str {
    match lang {
        Language::French  => fr::translate(key),
        Language::English => en::translate(key),
        Language::Russian => ru::translate(key),
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
        KeyCode::Space      => "Space",
        KeyCode::Escape     => "Echap",
        KeyCode::Enter      => "Enter",
        KeyCode::Tab        => "Tab",
        KeyCode::ShiftLeft  => "Shift G",
        KeyCode::ShiftRight => "Shift D",
        KeyCode::ControlLeft  => "Ctrl G",
        KeyCode::ControlRight => "Ctrl D",
        KeyCode::AltLeft  => "Alt G",
        KeyCode::AltRight => "Alt D",
        KeyCode::ArrowLeft  => "<- Gauche",
        KeyCode::ArrowRight => "-> Droite",
        KeyCode::ArrowUp    => "Haut",
        KeyCode::ArrowDown  => "Bas",
        KeyCode::F1  => "F1",  KeyCode::F2  => "F2",  KeyCode::F3  => "F3",
        KeyCode::F4  => "F4",  KeyCode::F5  => "F5",  KeyCode::F6  => "F6",
        KeyCode::F7  => "F7",  KeyCode::F8  => "F8",  KeyCode::F9  => "F9",
        KeyCode::F10 => "F10", KeyCode::F11 => "F11", KeyCode::F12 => "F12",
        _ => "?",
    }
}
