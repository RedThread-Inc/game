use bevy::prelude::*;
use bevy::ui::BorderColor;
use crate::{GameState, PlayerName};
use crate::engine::GameFont;

const PARCHMENT: Color = Color::srgb(0.847, 0.769, 0.588);
const INK:       Color = Color::srgb(0.180, 0.118, 0.059);
const GOLD:      Color = Color::srgb(0.859, 0.686, 0.216);

const WOOD_NORMAL:  Color = Color::srgb(0.400, 0.243, 0.102);
const WOOD_HOVERED: Color = Color::srgb(0.576, 0.365, 0.161);
const WOOD_PRESSED: Color = Color::srgb(0.259, 0.153, 0.059);

#[derive(Component)]
struct NameEntryRoot;

#[derive(Component)]
struct PseudoText;

#[derive(Component)]
struct ConfirmButton;

pub struct NameEntryPlugin;

impl Plugin for NameEntryPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::NameEntry), spawn_name_entry)
            .add_systems(Update, handle_keyboard.run_if(in_state(GameState::NameEntry)))
            .add_systems(Update, handle_confirm.run_if(in_state(GameState::NameEntry)))
            .add_systems(OnExit(GameState::NameEntry), despawn_name_entry);
    }
}

fn spawn_name_entry(mut commands: Commands, font: Res<GameFont>) {
    let f = &font.0;

    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            row_gap: Val::Px(24.),
            ..default()
        },
        BackgroundColor(PARCHMENT),
        NameEntryRoot,
    )).with_children(|root| {

        root.spawn((
            Text::new("REDTHREAD"),
            TextFont { font: f.clone(), font_size: 72., ..default() },
            TextColor(INK),
        ));

        root.spawn((
            Node {
                width: Val::Px(360.),
                height: Val::Px(3.),
                margin: UiRect::vertical(Val::Px(8.)),
                ..default()
            },
            BackgroundColor(GOLD),
        ));

        root.spawn((
            Text::new("Quel est ton nom, Gardien ?"),
            TextFont { font: f.clone(), font_size: 24., ..default() },
            TextColor(INK),
        ));

        root.spawn((
            Node {
                width: Val::Px(360.),
                height: Val::Px(56.),
                border: UiRect::all(Val::Px(3.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.92, 0.86, 0.72)),
        )).with_children(|field| {
            field.spawn((
                Text::new("|"),
                TextFont { font: f.clone(), font_size: 26., ..default() },
                TextColor(INK),
                PseudoText,
            ));
        });

        root.spawn((
            Node {
                width: Val::Px(320.),
                height: Val::Px(68.),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                padding: UiRect::all(Val::Px(4.)),
                margin: UiRect::top(Val::Px(8.)),
                ..default()
            },
            BackgroundColor(GOLD),
        )).with_children(|frame| {
            frame.spawn((
                Button,
                Node {
                    width: Val::Percent(100.),
                    height: Val::Percent(100.),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(WOOD_NORMAL),
                ConfirmButton,
            )).with_children(|btn| {
                btn.spawn((
                    Text::new("Partir à l'aventure"),
                    TextFont { font: f.clone(), font_size: 22., ..default() },
                    TextColor(GOLD),
                ));
            });
        });
    });
}

fn handle_keyboard(
    keys: Res<ButtonInput<KeyCode>>,
    mut player_name: ResMut<PlayerName>,
    mut text_q: Query<&mut Text, With<PseudoText>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let shift = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    // Mapping AZERTY : touche physique → caractère
    let mappings: &[(KeyCode, char, char)] = &[
        (KeyCode::KeyA, 'q', 'Q'), (KeyCode::KeyB, 'b', 'B'),
        (KeyCode::KeyC, 'c', 'C'), (KeyCode::KeyD, 'd', 'D'),
        (KeyCode::KeyE, 'e', 'E'), (KeyCode::KeyF, 'f', 'F'),
        (KeyCode::KeyG, 'g', 'G'), (KeyCode::KeyH, 'h', 'H'),
        (KeyCode::KeyI, 'i', 'I'), (KeyCode::KeyJ, 'j', 'J'),
        (KeyCode::KeyK, 'k', 'K'), (KeyCode::KeyL, 'l', 'L'),
        (KeyCode::KeyN, 'n', 'N'), (KeyCode::KeyO, 'o', 'O'),
        (KeyCode::KeyP, 'p', 'P'), (KeyCode::KeyQ, 'a', 'A'),
        (KeyCode::KeyR, 'r', 'R'), (KeyCode::KeyS, 's', 'S'),
        (KeyCode::KeyT, 't', 'T'), (KeyCode::KeyU, 'u', 'U'),
        (KeyCode::KeyV, 'v', 'V'), (KeyCode::KeyW, 'z', 'Z'),
        (KeyCode::KeyX, 'x', 'X'), (KeyCode::KeyY, 'y', 'Y'),
        (KeyCode::KeyZ, 'w', 'W'),
        // M est sur Semicolon en AZERTY
        (KeyCode::Semicolon, 'm', 'M'),
        // Chiffres
        (KeyCode::Digit0, '0', '0'), (KeyCode::Digit1, '1', '1'),
        (KeyCode::Digit2, '2', '2'), (KeyCode::Digit3, '3', '3'),
        (KeyCode::Digit4, '4', '4'), (KeyCode::Digit5, '5', '5'),
        (KeyCode::Digit6, '6', '6'), (KeyCode::Digit7, '7', '7'),
        (KeyCode::Digit8, '8', '8'), (KeyCode::Digit9, '9', '9'),
        // Tiret/underscore
        (KeyCode::Minus, '-', '_'),
    ];

    for &(keycode, normal, shifted) in mappings {
        if keys.just_pressed(keycode) && player_name.0.len() < 20 {
            player_name.0.push(if shift { shifted } else { normal });
        }
    }

    if keys.just_pressed(KeyCode::Backspace) && !player_name.0.is_empty() {
        player_name.0.pop();
    }

    if keys.just_pressed(KeyCode::Enter) && !player_name.0.is_empty() {
        next_state.set(GameState::Lore);
    }

    if let Ok(mut text) = text_q.single_mut() {
        **text = if player_name.0.is_empty() {
            "|".to_string()
        } else {
            format!("{}|", player_name.0)
        };
    }
}

fn handle_confirm(
    mut interaction_q: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ConfirmButton>)>,
    player_name: Res<PlayerName>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_q {
        match interaction {
            Interaction::Hovered => *color = BackgroundColor(WOOD_HOVERED),
            Interaction::None    => *color = BackgroundColor(WOOD_NORMAL),
            Interaction::Pressed => {
                *color = BackgroundColor(WOOD_PRESSED);
                if !player_name.0.is_empty() {
                    next_state.set(GameState::Lore);
                }
            }
        }
    }
}

fn despawn_name_entry(mut commands: Commands, q: Query<Entity, With<NameEntryRoot>>) {
    for e in &q { commands.entity(e).despawn(); }
}