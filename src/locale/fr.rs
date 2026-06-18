pub fn translate(key: &'static str) -> &'static str {
    match key {
        // Menu principal
        "menu_play"     => "Partir a l'aventure",
        "menu_settings" => "Parametres",
        "menu_quit"     => "Abandonner la quete",
        // Menu pause
        "pause_title"     => "PAUSE",
        "pause_resume"    => "Reprendre",
        "pause_restart"   => "Recommencer",
        "pause_main_menu" => "Menu principal",
        "pause_quit"      => "Abandonner la quete",
        // Menu mort
        "death_subtitle" => "Vous etes mort...",
        "death_retry"    => "Retenter l'aventure",
        "death_main_menu"=> "Menu principal",
        "death_quit"     => "Abandonner la quete",
        // Menu parametres
        "settings_title"    => "PARAMETRES",
        "settings_fps"      => "LIMITE FPS",
        "settings_controls" => "CONTROLES",
        "settings_language" => "LANGUE",
        "settings_back"     => "Retour",
        // Touches
        "key_left"  => "Aller a gauche",
        "key_right" => "Aller a droite",
        "key_up"    => "Aller en haut",
        "key_down"  => "Aller en bas",
        "key_pause" => "Pause",
        _ => key,
    }
}
