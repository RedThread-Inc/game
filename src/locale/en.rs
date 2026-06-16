pub fn translate(key: &'static str) -> &'static str {
    match key {
        // Main menu
        "menu_play"     => "Start the adventure",
        "menu_settings" => "Settings",
        "menu_quit"     => "Quit",
        // Pause menu
        "pause_title"     => "PAUSE",
        "pause_resume"    => "Resume",
        "pause_restart"   => "Restart",
        "pause_main_menu" => "Main menu",
        "pause_quit"      => "Quit",
        // Death menu
        "death_subtitle"  => "You have died...",
        "death_retry"     => "Try again",
        "death_main_menu" => "Main menu",
        "death_quit"      => "Quit",
        // Settings menu
        "settings_title"    => "SETTINGS",
        "settings_fps"      => "FPS LIMIT",
        "settings_controls" => "CONTROLS",
        "settings_language" => "LANGUAGE",
        "settings_back"     => "Back",
        // Keys
        "key_left"  => "Move left",
        "key_right" => "Move right",
        "key_up"    => "Move up",
        "key_down"  => "Move down",
        "key_pause" => "Pause",
        _ => key,
    }
}
