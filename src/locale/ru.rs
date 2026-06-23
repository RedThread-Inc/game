pub fn translate(key: &'static str) -> &'static str {
    match key {
        // Главное меню
        "menu_play"     => "Начать приключение",
        "menu_settings" => "Настройки",
        "menu_quit"     => "Выйти",
        // Меню паузы
        "pause_title"     => "ПАУЗА",
        "pause_resume"    => "Продолжить",
        "pause_restart"   => "Начать заново",
        "pause_main_menu" => "Главное меню",
        "pause_quit"      => "Выйти",
        // Меню смерти
        "death_subtitle"  => "Вы погибли...",
        "death_retry"     => "Попробовать снова",
        "death_main_menu" => "Главное меню",
        "death_quit"      => "Выйти",
        // Меню настроек
        "settings_title"    => "НАСТРОЙКИ",
        "settings_fps"      => "ЛИМИТ FPS",
        "settings_controls" => "УПРАВЛЕНИЕ",
        "settings_language" => "ЯЗЫК",
        "settings_back"     => "Назад",
        // Клавиши
        "key_left"  => "Движение влево",
        "key_right" => "Движение вправо",
        "key_up"    => "Движение вверх",
        "key_down"  => "Движение вниз",
        "key_pause" => "Пауза",
        _ => key,
    }
}
