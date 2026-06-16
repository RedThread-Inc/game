use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use crate::GameState;
use crate::settings::{key_name, t, FpsLimit, GameSettings, KeyAction, Language};

pub struct SettingsMenuPlugin;

impl Plugin for SettingsMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<RebindState>()
            .add_systems(OnEnter(GameState::Settings), setup_settings_menu)
            .add_systems(Update, (
                handle_fps_clicks,
                handle_language_clicks,
                handle_keybind_clicks,
                listen_for_rebind,
                update_fps_visuals,
                update_language_visuals,
                update_keybind_visuals,
                handle_back_button,
            ).run_if(in_state(GameState::Settings)))
            .add_systems(OnExit(GameState::Settings), cleanup_settings_menu);
    }
}

#[derive(Resource, Default)]
pub struct RebindState {
    pub action: Option<KeyAction>,
}

#[derive(Component)]
struct SettingsRoot;

#[derive(Component)]
struct FpsOptionButton(FpsLimit);

#[derive(Component)]
struct LanguageOptionButton(Language);

#[derive(Component)]
struct KeybindButton(KeyAction);

#[derive(Component)]
struct KeybindLabel(KeyAction);

#[derive(Component)]
struct BackButton;

const PARCHMENT:    Color = Color::srgb(0.847, 0.769, 0.588);
const INK:          Color = Color::srgb(0.180, 0.118, 0.059);
const GOLD:         Color = Color::srgb(0.859, 0.686, 0.216);
const WOOD_NORMAL:  Color = Color::srgb(0.400, 0.243, 0.102);
const WOOD_HOVERED: Color = Color::srgb(0.576, 0.365, 0.161);
const WOOD_PRESSED: Color = Color::srgb(0.259, 0.153, 0.059);
const SELECTED:     Color = Color::srgb(0.65, 0.45, 0.05);
const AWAITING:     Color = Color::srgb(0.15, 0.35, 0.55);

fn setup_settings_menu(mut commands: Commands, settings: Res<GameSettings>) {
    let lang = &settings.language;

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        BackgroundColor(PARCHMENT),
        SettingsRoot,
    ))
    .with_children(|root| {
        root.spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                row_gap: Val::Px(14.0),
                padding: UiRect::all(Val::Px(40.0)),
                min_width: Val::Px(500.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|col| {
            col.spawn((
                Text::new("- * -"),
                TextFont { font_size: 22.0, ..default() },
                TextColor(GOLD),
            ));
            col.spawn((
                Text::new(t(lang, "settings_title")),
                TextFont { font_size: 52.0, ..default() },
                TextColor(INK),
            ));

            separator(col);
            section_label(col, t(lang, "settings_fps"));
            spawn_fps_row(col, &settings);

            separator(col);
            section_label(col, t(lang, "settings_controls"));
            spawn_keybind_row(col, t(lang, "key_left"),  KeyAction::Left,  key_name(settings.key_left));
            spawn_keybind_row(col, t(lang, "key_right"), KeyAction::Right, key_name(settings.key_right));
            spawn_keybind_row(col, t(lang, "key_up"),    KeyAction::Up,    key_name(settings.key_up));
            spawn_keybind_row(col, t(lang, "key_down"),  KeyAction::Down,  key_name(settings.key_down));
            spawn_keybind_row(col, t(lang, "key_pause"), KeyAction::Pause, key_name(settings.key_pause));

            separator(col);
            section_label(col, t(lang, "settings_language"));
            spawn_language_row(col, &settings);

            separator(col);
            spawn_back_button(col, t(lang, "settings_back"));
        });
    });
}

fn separator(col: &mut RelatedSpawnerCommands<ChildOf>) {
    col.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Px(2.0),
            margin: UiRect::vertical(Val::Px(2.0)),
            ..default()
        },
        BackgroundColor(GOLD),
    ));
}

fn section_label(col: &mut RelatedSpawnerCommands<ChildOf>, label: &str) {
    col.spawn((
        Text::new(label),
        TextFont { font_size: 16.0, ..default() },
        TextColor(GOLD),
    ));
}

fn spawn_fps_row(col: &mut RelatedSpawnerCommands<ChildOf>, settings: &GameSettings) {
    let options = [FpsLimit::Fps30, FpsLimit::Fps60, FpsLimit::Fps120, FpsLimit::Unlimited];

    col.spawn(Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(8.0),
        ..default()
    })
    .with_children(|row| {
        for option in options {
            let label = option.label();
            let is_selected = option == settings.fps_limit;
            let bg = if is_selected { SELECTED } else { WOOD_NORMAL };

            row.spawn((
                Node {
                    width: Val::Px(76.0),
                    height: Val::Px(44.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(GOLD),
            ))
            .with_children(|frame| {
                frame.spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(bg),
                    FpsOptionButton(option),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(label),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(GOLD),
                    ));
                });
            });
        }
    });
}

fn spawn_keybind_row(
    col: &mut RelatedSpawnerCommands<ChildOf>,
    action_label: &str,
    action: KeyAction,
    key: &str,
) {
    col.spawn(Node {
        flex_direction: FlexDirection::Row,
        align_items: AlignItems::Center,
        justify_content: JustifyContent::SpaceBetween,
        width: Val::Px(420.0),
        ..default()
    })
    .with_children(|row| {
        row.spawn((
            Text::new(action_label),
            TextFont { font_size: 18.0, ..default() },
            TextColor(INK),
        ));

        row.spawn((
            Node {
                width: Val::Px(110.0),
                height: Val::Px(40.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(3.0)),
                ..default()
            },
            BackgroundColor(GOLD),
        ))
        .with_children(|frame| {
            frame.spawn((
                Button,
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(WOOD_NORMAL),
                KeybindButton(action.clone()),
            ))
            .with_children(|btn| {
                btn.spawn((
                    Text::new(key),
                    TextFont { font_size: 18.0, ..default() },
                    TextColor(GOLD),
                    KeybindLabel(action),
                ));
            });
        });
    });
}

fn spawn_language_row(col: &mut RelatedSpawnerCommands<ChildOf>, settings: &GameSettings) {
    let options = [(Language::French, "FR"), (Language::English, "EN")];

    col.spawn(Node {
        flex_direction: FlexDirection::Row,
        column_gap: Val::Px(8.0),
        ..default()
    })
    .with_children(|row| {
        for (lang, label) in options {
            let is_selected = lang == settings.language;
            let bg = if is_selected { SELECTED } else { WOOD_NORMAL };

            row.spawn((
                Node {
                    width: Val::Px(76.0),
                    height: Val::Px(44.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    padding: UiRect::all(Val::Px(3.0)),
                    ..default()
                },
                BackgroundColor(GOLD),
            ))
            .with_children(|frame| {
                frame.spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(bg),
                    LanguageOptionButton(lang),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(label),
                        TextFont { font_size: 18.0, ..default() },
                        TextColor(GOLD),
                    ));
                });
            });
        }
    });
}

fn spawn_back_button(col: &mut RelatedSpawnerCommands<ChildOf>, label: &str) {
    col.spawn((
        Node {
            width: Val::Px(200.0),
            height: Val::Px(52.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(GOLD),
    ))
    .with_children(|frame| {
        frame.spawn((
            Button,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(WOOD_NORMAL),
            BackButton,
        ))
        .with_children(|btn| {
            btn.spawn((
                Text::new(label),
                TextFont { font_size: 20.0, ..default() },
                TextColor(GOLD),
            ));
        });
    });
}

fn handle_fps_clicks(
    interaction_query: Query<(&Interaction, &FpsOptionButton), Changed<Interaction>>,
    mut settings: ResMut<GameSettings>,
) {
    for (interaction, btn) in &interaction_query {
        if *interaction == Interaction::Pressed {
            settings.fps_limit = btn.0.clone();
        }
    }
}

fn handle_language_clicks(
    interaction_query: Query<(&Interaction, &LanguageOptionButton), Changed<Interaction>>,
    mut settings: ResMut<GameSettings>,
) {
    for (interaction, btn) in &interaction_query {
        if *interaction == Interaction::Pressed {
            settings.language = btn.0.clone();
        }
    }
}

fn handle_keybind_clicks(
    interaction_query: Query<(&Interaction, &KeybindButton), Changed<Interaction>>,
    mut rebind_state: ResMut<RebindState>,
) {
    for (interaction, btn) in &interaction_query {
        if *interaction == Interaction::Pressed {
            let already_waiting = rebind_state.action.as_ref() == Some(&btn.0);
            rebind_state.action = if already_waiting { None } else { Some(btn.0.clone()) };
        }
    }
}

fn listen_for_rebind(
    keys: Res<ButtonInput<KeyCode>>,
    mut rebind_state: ResMut<RebindState>,
    mut settings: ResMut<GameSettings>,
) {
    let Some(ref action) = rebind_state.action.clone() else { return };

    let Some(new_key) = keys.get_just_pressed().copied().next() else { return };

    match action {
        KeyAction::Left  => settings.key_left  = new_key,
        KeyAction::Right => settings.key_right = new_key,
        KeyAction::Up    => settings.key_up    = new_key,
        KeyAction::Down  => settings.key_down  = new_key,
        KeyAction::Pause => settings.key_pause = new_key,
    }

    rebind_state.action = None;
}

fn update_fps_visuals(
    settings: Res<GameSettings>,
    mut buttons: Query<(&mut BackgroundColor, &FpsOptionButton, &Interaction)>,
) {
    for (mut color, btn, interaction) in &mut buttons {
        *color = BackgroundColor(match interaction {
            Interaction::Hovered => WOOD_HOVERED,
            Interaction::Pressed => WOOD_PRESSED,
            Interaction::None    => if btn.0 == settings.fps_limit { SELECTED } else { WOOD_NORMAL },
        });
    }
}

fn update_language_visuals(
    settings: Res<GameSettings>,
    mut buttons: Query<(&mut BackgroundColor, &LanguageOptionButton, &Interaction)>,
) {
    for (mut color, btn, interaction) in &mut buttons {
        *color = BackgroundColor(match interaction {
            Interaction::Hovered => WOOD_HOVERED,
            Interaction::Pressed => WOOD_PRESSED,
            Interaction::None    => if btn.0 == settings.language { SELECTED } else { WOOD_NORMAL },
        });
    }
}

fn update_keybind_visuals(
    settings: Res<GameSettings>,
    rebind_state: Res<RebindState>,
    mut labels: Query<(&mut Text, &KeybindLabel)>,
    mut buttons: Query<(&mut BackgroundColor, &KeybindButton, &Interaction)>,
) {
    for (mut text, label) in &mut labels {
        let is_awaiting = rebind_state.action.as_ref() == Some(&label.0);
        *text = if is_awaiting {
            Text::new("[...]")
        } else {
            let key = match label.0 {
                KeyAction::Left  => settings.key_left,
                KeyAction::Right => settings.key_right,
                KeyAction::Up    => settings.key_up,
                KeyAction::Down  => settings.key_down,
                KeyAction::Pause => settings.key_pause,
            };
            Text::new(key_name(key))
        };
    }

    for (mut color, btn, interaction) in &mut buttons {
        let is_awaiting = rebind_state.action.as_ref() == Some(&btn.0);
        *color = BackgroundColor(match interaction {
            Interaction::Hovered => WOOD_HOVERED,
            Interaction::Pressed => WOOD_PRESSED,
            Interaction::None    => if is_awaiting { AWAITING } else { WOOD_NORMAL },
        });
    }
}

fn handle_back_button(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<BackButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match interaction {
            Interaction::Hovered => *color = BackgroundColor(WOOD_HOVERED),
            Interaction::None    => *color = BackgroundColor(WOOD_NORMAL),
            Interaction::Pressed => {
                *color = BackgroundColor(WOOD_PRESSED);
                next_state.set(GameState::MainMenu);
            }
        }
    }
}

fn cleanup_settings_menu(
    mut commands: Commands,
    query: Query<Entity, With<SettingsRoot>>,
    mut rebind_state: ResMut<RebindState>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    rebind_state.action = None;
}
