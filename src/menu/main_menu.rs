use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use crate::GameState;
use crate::settings::{t, GameSettings};
use bevy::app::AppExit;
use crate::round::RoundStartedEvent;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::MainMenu), (setup_menu, start_menu_music))
            .add_systems(Update, (
                handle_buttons,
                animate_torches,
            ).run_if(in_state(GameState::MainMenu)))
            .add_systems(OnExit(GameState::MainMenu), (cleanup_menu, stop_menu_music));
    }
}

fn start_menu_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("music/mainMenuMusic.ogg")),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: bevy::audio::Volume::Linear(0.1),
            ..default()
        },
        MainMenuMusic,
    ));
}

fn stop_menu_music(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuMusic>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

#[derive(Component)]
enum MenuButton {
    Play,
    Settings,
    Quit,
}

#[derive(Component)]
struct MainMenuMusic;

#[derive(Component)]
struct MenuRoot;

#[derive(Component)]
struct TorchFlame {
    phase: f32,
}

const PARCHMENT:    Color = Color::srgb(0.847, 0.769, 0.588);
const INK:          Color = Color::srgb(0.180, 0.118, 0.059);
const GOLD:         Color = Color::srgb(0.859, 0.686, 0.216);
const SUBTITLE:     Color = Color::srgb(0.450, 0.300, 0.120);

const WOOD_NORMAL:  Color = Color::srgb(0.400, 0.243, 0.102);
const WOOD_HOVERED: Color = Color::srgb(0.576, 0.365, 0.161);
const WOOD_PRESSED: Color = Color::srgb(0.259, 0.153, 0.059);

const FLAME_CORE:   Color = Color::srgb(1.00, 0.95, 0.40);
const FLAME_MID:    Color = Color::srgb(1.00, 0.55, 0.10);
const FLAME_OUTER:  Color = Color::srgb(0.80, 0.20, 0.05);

fn setup_menu(mut commands: Commands, settings: Res<GameSettings>) {
    let lang = &settings.language;
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(PARCHMENT),
            MenuRoot,
        ))
        .with_children(|root| {

            spawn_torch(root, 0.0);

            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    padding: UiRect::horizontal(Val::Px(60.0)),
                    ..default()
                },
                BackgroundColor(Color::NONE),
            ))
                .with_children(|col| {
                    col.spawn((
                        Text::new("- * -"),
                        TextFont { font_size: 28.0, ..default() },
                        TextColor(GOLD),
                    ));

                    col.spawn((
                        Text::new("REDTHREAD"),
                        TextFont { font_size: 72.0, ..default() },
                        TextColor(INK),
                    ));

                    col.spawn((
                        Node {
                            width: Val::Px(360.0),
                            height: Val::Px(3.0),
                            margin: UiRect::vertical(Val::Px(16.0)),
                            ..default()
                        },
                        BackgroundColor(GOLD),
                    ));

                    spawn_button(col, t(lang, "menu_play"),     MenuButton::Play);
                    spawn_button(col, t(lang, "menu_settings"), MenuButton::Settings);
                    spawn_button(col, t(lang, "menu_quit"),     MenuButton::Quit);

                    col.spawn((
                        Node {
                            width: Val::Px(360.0),
                            height: Val::Px(3.0),
                            margin: UiRect::vertical(Val::Px(16.0)),
                            ..default()
                        },
                        BackgroundColor(GOLD),
                    ));
                });

            spawn_torch(root, 1.8);
        });
}

fn spawn_torch(parent: &mut RelatedSpawnerCommands<ChildOf>, phase_offset: f32) {
    parent
        .spawn((
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::FlexEnd,
                width: Val::Px(80.0),
                height: Val::Px(280.0),
                ..default()
            },
            BackgroundColor(Color::NONE),
        ))
        .with_children(|torch| {
            torch.spawn((
                Node {
                    width: Val::Px(40.0),
                    height: Val::Px(60.0),
                    ..default()
                },
                BackgroundColor(FLAME_OUTER),
                TorchFlame { phase: phase_offset },
            ));

            torch.spawn((
                Node {
                    width: Val::Px(28.0),
                    height: Val::Px(50.0),
                    margin: UiRect {
                        top: Val::Px(-35.0),
                        ..default()
                    },
                    ..default()
                },
                BackgroundColor(FLAME_MID),
                TorchFlame { phase: phase_offset + 0.4 },
            ));

            torch.spawn((
                Node {
                    width: Val::Px(14.0),
                    height: Val::Px(36.0),
                    margin: UiRect {
                        top: Val::Px(-28.0),
                        ..default()
                    },
                    ..default()
                },
                BackgroundColor(FLAME_CORE),
                TorchFlame { phase: phase_offset + 0.9 },
            ));

            torch.spawn((
                Node {
                    width: Val::Px(14.0),
                    height: Val::Px(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.28, 0.18, 0.08)),
            ));

            torch.spawn((
                Node {
                    width: Val::Px(24.0),
                    height: Val::Px(12.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.20, 0.12, 0.05)),
            ));
        });
}

fn animate_torches(
    time: Res<Time>,
    mut query: Query<(&TorchFlame, &mut Node, &mut BackgroundColor)>,
) {
    for (flame, mut node, mut bg) in &mut query {
        let t = time.elapsed_secs() * 4.0 + flame.phase;

        let flicker = (t.sin() * 0.5 + (t * 1.7).cos() * 0.3 + 1.0) * 0.5;

        let base_width = match bg.0 {
            c if c == FLAME_OUTER => 40.0,
            c if c == FLAME_MID   => 28.0,
            _                     => 14.0,
        };
        node.width = Val::Px(base_width + flicker * 10.0 - 5.0);

        let brightness = 0.85 + flicker * 0.30;
        bg.0 = match bg.0 {
            c if c == FLAME_OUTER => Color::srgb(
                0.80 * brightness,
                0.20 * brightness,
                0.05 * brightness,
            ),
            c if c == FLAME_MID => Color::srgb(
                1.00 * brightness,
                0.55 * brightness,
                0.10 * brightness,
            ),
            _ => Color::srgb(
                1.00_f32.min(1.00 * brightness),
                1.00_f32.min(0.95 * brightness),
                0.40 * brightness,
            ),
        };
    }
}

fn spawn_button(parent: &mut RelatedSpawnerCommands<ChildOf>, label: &str, button: MenuButton) {
    parent
        .spawn((
            Node {
                width: Val::Px(320.0),
                height: Val::Px(68.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(4.0)),
                ..default()
            },
            BackgroundColor(GOLD),
        ))
        .with_children(|frame| {
            frame
                .spawn((
                    Button,
                    Node {
                        width: Val::Percent(100.0),
                        height: Val::Percent(100.0),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        padding: UiRect::horizontal(Val::Px(20.0)),
                        ..default()
                    },
                    BackgroundColor(WOOD_NORMAL),
                    button,
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(label),
                        TextFont { font_size: 22.0, ..default() },
                        TextColor(GOLD),
                    ));
                });
        });
}

fn handle_buttons(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &MenuButton), Changed<Interaction>>,
    mut next_state: ResMut<NextState<GameState>>,
    mut app_exit: MessageWriter<AppExit>,
    mut round_started: MessageWriter<RoundStartedEvent>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match interaction {
            Interaction::Hovered  => *color = BackgroundColor(WOOD_HOVERED),
            Interaction::None     => *color = BackgroundColor(WOOD_NORMAL),
            Interaction::Pressed  => {
                *color = BackgroundColor(WOOD_PRESSED);
                match button {
                    MenuButton::Play     => next_state.set(GameState::InGame),
                    MenuButton::Settings => next_state.set(GameState::Settings),
                    MenuButton::Quit     => { app_exit.write(AppExit::Success); }
                }
            }
        }
    }
}

fn cleanup_menu(mut commands: Commands, query: Query<Entity, With<MenuRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}