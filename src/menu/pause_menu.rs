use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use crate::InGameState;
use crate::GameState;
use crate::settings::{t, GameSettings};
use bevy::app::AppExit;

pub struct PauseMenuPlugin;

impl Plugin for PauseMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(InGameState::Paused), setup_pause_menu)
            .add_systems(Update, (
                handle_pause_buttons,
                toggle_pause,
            ).run_if(in_state(InGameState::Paused)))
            .add_systems(Update, toggle_pause.run_if(in_state(InGameState::Playing)))
            .add_systems(OnExit(InGameState::Paused), cleanup_pause_menu);
    }
}

#[derive(Component)]
enum PauseButton {
    Resume,
    Restart,
    MainMenu,
    Quit,
}

#[derive(Component)]
struct PauseRoot;

const PARCHMENT:    Color = Color::srgba(0.847, 0.769, 0.588, 0.95);
const INK:          Color = Color::srgb(0.180, 0.118, 0.059);
const GOLD:         Color = Color::srgb(0.859, 0.686, 0.216);

const WOOD_NORMAL:  Color = Color::srgb(0.400, 0.243, 0.102);
const WOOD_HOVERED: Color = Color::srgb(0.576, 0.365, 0.161);
const WOOD_PRESSED: Color = Color::srgb(0.259, 0.153, 0.059);

fn toggle_pause(
    keys: Res<ButtonInput<KeyCode>>,
    settings: Res<GameSettings>,
    state: Res<State<InGameState>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    if keys.just_pressed(settings.key_pause) {
        match state.get() {
            InGameState::Playing => next_state.set(InGameState::Paused),
            InGameState::Paused => next_state.set(InGameState::Playing),
            _ => {}
        }
    }
}

fn setup_pause_menu(mut commands: Commands, settings: Res<GameSettings>) {
    let lang = &settings.language;
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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.5)),
            PauseRoot,
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
                        TextFont { font_size: 24.0, ..default() },
                        TextColor(GOLD),
                    ));

                    col.spawn((
                        Text::new(t(lang, "pause_title")),
                        TextFont { font_size: 64.0, ..default() },
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

                    spawn_pause_button(col, t(lang, "pause_resume"),    PauseButton::Resume);
                    spawn_pause_button(col, t(lang, "pause_restart"),   PauseButton::Restart);
                    spawn_pause_button(col, t(lang, "pause_main_menu"), PauseButton::MainMenu);

                    col.spawn((
                        Node {
                            width: Val::Px(360.0),
                            height: Val::Px(3.0),
                            margin: UiRect::vertical(Val::Px(16.0)),
                            ..default()
                        },
                        BackgroundColor(GOLD),
                    ));

                    spawn_pause_button(col, t(lang, "pause_quit"), PauseButton::Quit);
                });
        });
}

fn spawn_pause_button(parent: &mut RelatedSpawnerCommands<ChildOf>, label: &str, button: PauseButton) {
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

fn handle_pause_buttons(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, &PauseButton), Changed<Interaction>>,
    mut next_in_game: ResMut<NextState<InGameState>>,
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
                    PauseButton::Resume   => next_in_game.set(InGameState::Playing),
                    PauseButton::Restart => {
                        next_in_game.set(InGameState::Playing);
                        next_game.set(GameState::Restarting);
                    }
                    PauseButton::MainMenu => next_game.set(GameState::MainMenu),
                    PauseButton::Quit     => { app_exit.write(AppExit::Success); }
                }
            }
        }
    }
}

fn cleanup_pause_menu(mut commands: Commands, query: Query<Entity, With<PauseRoot>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}