use bevy::prelude::*;
use crate::{GameState, PlayerName};

const LORE_TEXT: &str = "\
Il y a trois siècles, les Gardiens ont scellé le Néant \
derrière la Porte d'Aelith. Mais le sceau se fissure, \
et les créatures de l'ombre s'infiltrent à nouveau dans \
les terres du vivant et menace le royaume d'Eryndor.

Tu es l'un des rares Gardiens encore en vie. \
Ta mission : protéger les civils, \
lutter contre les forces corrompues, \
et mettre fin au maléfice.

Le chemin sera long et tes prédécesseurs n'en sont jamais revenus.";

#[derive(Component)]
struct LoreUi;

#[derive(Component)]
struct ContinueButton;

pub struct LoreScreenPlugin;

impl Plugin for LoreScreenPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Lore), spawn_lore)
            .add_systems(Update, on_continue.run_if(in_state(GameState::Lore)))
            .add_systems(OnExit(GameState::Lore), despawn_lore);
    }
}

fn spawn_lore(mut commands: Commands, player_name: Res<PlayerName>) {
    commands.spawn((
        Node {
            width: Val::Percent(100.),
            height: Val::Percent(100.),
            flex_direction: FlexDirection::Column,
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(60.)),
            row_gap: Val::Px(32.),
            ..default()
        },
        BackgroundColor(Color::srgb(0.04, 0.04, 0.08)),
        LoreUi,
    )).with_children(|p| {
        p.spawn((
            Text::new(format!("Bienvenue, {}…", player_name.0)),
            TextFont { font_size: 30., ..default() },
            TextColor(Color::srgb(0.95, 0.80, 0.30)),
        ));

        p.spawn((
            Text::new(LORE_TEXT),
            TextFont { font_size: 17., ..default() },
            TextColor(Color::srgb(0.85, 0.85, 0.85)),
            Node { max_width: Val::Px(700.), ..default() },
        ));

        p.spawn((
            Button,
            Node {
                width: Val::Px(240.),
                height: Val::Px(52.),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.18, 0.45, 0.20)),
            ContinueButton,
        )).with_children(|btn| {
            btn.spawn((
                Text::new("Continuer"),
                TextFont { font_size: 22., ..default() },
                TextColor(Color::WHITE),
            ));
        });
    });
}

fn on_continue(
    interaction_q: Query<&Interaction, (Changed<Interaction>, With<ContinueButton>)>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interaction_q {
        if *interaction == Interaction::Pressed {
            next_state.set(GameState::InGame);
        }
    }
}

fn despawn_lore(mut commands: Commands, q: Query<Entity, With<LoreUi>>) {
    for e in &q { commands.entity(e).despawn(); }
}