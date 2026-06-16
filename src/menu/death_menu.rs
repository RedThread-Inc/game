use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use bevy::app::AppExit;

use crate::GameState;
use crate::engine::GameFont;
use crate::locale::t;
use crate::settings::GameSettings;

pub struct DeathMenuPlugin;

impl Plugin for DeathMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::GameOver), (setup_death_menu, start_menu_music))
            .add_systems(Update, handle_death_buttons.run_if(in_state(GameState::GameOver)))
            .add_systems(OnExit(GameState::GameOver), (cleanup_death_menu, stop_menu_music));
    }
}

#[derive(Component)]
enum DeathButton {
    Retry,
    MainMenu,
    Quit,
}

#[derive(Component)]
struct DeathMenuMusic;

#[derive(Component)]
struct DeathRoot;

const PARCHMENT:    Color = Color::srgba(0.847, 0.769, 0.588, 0.95);
const INK:          Color = Color::srgb(0.180, 0.118, 0.059);
const GOLD:         Color = Color::srgb(0.859, 0.686, 0.216);
const DARK_RED:     Color = Color::srgb(0.55, 0.07, 0.07);

const WOOD_NORMAL:  Color = Color::srgb(0.400, 0.243, 0.102);
const WOOD_HOVERED: Color = Color::srgb(0.576, 0.365, 0.161);
const WOOD_PRESSED: Color = Color::srgb(0.259, 0.153, 0.059);

fn setup_death_menu(mut commands: Commands, settings: Res<GameSettings>, font: Res<GameFont>) {
    let lang = &settings.language;
    let f = &font.0;
    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.7)),
            DeathRoot,
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    padding: UiRect::all(Val::Px(60.0)),
                    ..default()
                },
                BackgroundColor(PARCHMENT),
            ))
                .with_children(|col| {
                    col.spawn((
                        Text::new("- * -"),
                        TextFont { font: f.clone(), font_size: 24.0, ..default() },
                        TextColor(GOLD),
                    ));

                    col.spawn((
                        Text::new("GAME OVER"),
                        TextFont { font: f.clone(), font_size: 64.0, ..default() },
                        TextColor(DARK_RED),
                    ));

                    col.spawn((
                        Text::new(t(lang, "death_subtitle")),
                        TextFont { font: f.clone(), font_size: 20.0, ..default() },
                        TextColor(INK),
                    ));

                    separator(col);

                    spawn_death_button(col, t(lang, "death_retry"),     DeathButton::Retry,    f);
                    spawn_death_button(col, t(lang, "death_main_menu"), DeathButton::MainMenu, f);

                    separator(col);

                    spawn_death_button(col, t(lang, "death_quit"), DeathButton::Quit, f);
                });
        });
}

fn separator(col: &mut RelatedSpawnerCommands<ChildOf>) {
    col.spawn((
        Node {
            width: Val::Px(360.0),
            height: Val::Px(3.0),
            margin: UiRect::vertical(Val::Px(8.0)),
            ..default()
        },
        BackgroundColor(GOLD),
    ));
}

fn spawn_death_button(
    parent: &mut RelatedSpawnerCommands<ChildOf>,
    label: &str,
    button: DeathButton,
    font: &Handle<Font>,
) {
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
                        TextFont { font: font.clone(), font_size: 22.0, ..default() },
                        TextColor(GOLD),
                    ));
                });
        });
}

fn handle_death_buttons(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &DeathButton), Changed<Interaction>>,
    mut next_game: ResMut<NextState<GameState>>,
    mut app_exit: MessageWriter<AppExit>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match interaction {
            Interaction::Hovered  => *color = BackgroundColor(WOOD_HOVERED),
            Interaction::None     => *color = BackgroundColor(WOOD_NORMAL),
            Interaction::Pressed  => {
                *color = BackgroundColor(WOOD_PRESSED);
                match button {
                    DeathButton::Retry    => next_game.set(GameState::Restarting),
                    DeathButton::MainMenu => next_game.set(GameState::MainMenu),
                    DeathButton::Quit     => { app_exit.write(AppExit::Success); }
                }
            }
        }
    }
}

pub(crate) fn cleanup_death_menu(
    mut commands: Commands,
    query: Query<Entity, With<DeathRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn start_menu_music(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        AudioPlayer::new(asset_server.load("music/deathMenuMusic.ogg")),
        PlaybackSettings {
            mode: bevy::audio::PlaybackMode::Loop,
            volume: bevy::audio::Volume::Linear(0.3),
            ..default()
        },
        crate::menu::death_menu::DeathMenuMusic,
    ));
}

fn stop_menu_music(
    mut commands: Commands,
    query: Query<Entity, With<crate::menu::death_menu::DeathMenuMusic>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}