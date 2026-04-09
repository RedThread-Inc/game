use bevy::ecs::relationship::RelatedSpawnerCommands;
use bevy::prelude::*;
use crate::round::RoundState;
use crate::upgrade::{PlayerUpgrades, Upgrade};
use crate::InGameState;
use rand::seq::SliceRandom;

const ALL_UPGRADES: [Upgrade; 5] = [
    Upgrade::ExtraProjectile,
    Upgrade::FasterFireRate,
    Upgrade::MoreDamage,
    Upgrade::FasterProjectile,
    Upgrade::LargerRadius,
];

const PARCHMENT:    Color = Color::srgba(0.847, 0.769, 0.588, 0.95);
const INK:          Color = Color::srgb(0.180, 0.118, 0.059);
const GOLD:         Color = Color::srgb(0.859, 0.686, 0.216);
const WOOD_NORMAL:  Color = Color::srgb(0.400, 0.243, 0.102);
const WOOD_HOVERED: Color = Color::srgb(0.576, 0.365, 0.161);
const WOOD_PRESSED: Color = Color::srgb(0.259, 0.153, 0.059);

#[derive(Component)]
pub(crate) struct UpgradeRoot;

#[derive(Component)]
pub(crate) struct UpgradeButton(Upgrade);

pub(crate) fn spawn_upgrade_ui(mut commands: Commands, round: Res<RoundState>) {
    let mut rng = rand::rng();
    let mut pool = ALL_UPGRADES.to_vec();
    pool.shuffle(&mut rng);
    let choices = &pool[..3];

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
            BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.75)),
            UpgradeRoot,
        ))
        .with_children(|root| {
            root.spawn((
                Node {
                    flex_direction: FlexDirection::Column,
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    row_gap: Val::Px(20.0),
                    padding: UiRect::all(Val::Px(50.0)),
                    ..default()
                },
                BackgroundColor(PARCHMENT),
            ))
            .with_children(|panel| {
                panel.spawn((
                    Text::new(format!("Vague {} terminee !", round.current)),
                    TextFont { font_size: 40.0, ..default() },
                    TextColor(INK),
                ));

                panel.spawn((
                    Text::new("Choisis une amelioration :"),
                    TextFont { font_size: 20.0, ..default() },
                    TextColor(INK),
                ));

                separator(panel);

                for &upgrade in choices {
                    spawn_upgrade_button(panel, upgrade);
                }
            });
        });
}

fn separator(parent: &mut RelatedSpawnerCommands<ChildOf>) {
    parent.spawn((
        Node {
            width: Val::Px(440.0),
            height: Val::Px(3.0),
            margin: UiRect::vertical(Val::Px(4.0)),
            ..default()
        },
        BackgroundColor(GOLD),
    ));
}

fn spawn_upgrade_button(parent: &mut RelatedSpawnerCommands<ChildOf>, upgrade: Upgrade) {
    parent
        .spawn((
            Node {
                width: Val::Px(440.0),
                height: Val::Px(84.0),
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
                        flex_direction: FlexDirection::Column,
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        row_gap: Val::Px(4.0),
                        padding: UiRect::horizontal(Val::Px(16.0)),
                        ..default()
                    },
                    BackgroundColor(WOOD_NORMAL),
                    UpgradeButton(upgrade),
                ))
                .with_children(|btn| {
                    btn.spawn((
                        Text::new(upgrade.name()),
                        TextFont { font_size: 20.0, ..default() },
                        TextColor(GOLD),
                    ));
                    btn.spawn((
                        Text::new(upgrade.description()),
                        TextFont { font_size: 13.0, ..default() },
                        TextColor(Color::srgb(0.9, 0.85, 0.7)),
                    ));
                });
        });
}

pub(crate) fn handle_upgrade_buttons(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &UpgradeButton),
        Changed<Interaction>,
    >,
    mut upgrades: ResMut<PlayerUpgrades>,
    mut round: ResMut<RoundState>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    for (interaction, mut color, button) in &mut interaction_query {
        match interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(WOOD_PRESSED);
                upgrades.apply(button.0);
                round.advance();
                next_state.set(InGameState::Playing);
            }
            Interaction::Hovered => *color = BackgroundColor(WOOD_HOVERED),
            Interaction::None    => *color = BackgroundColor(WOOD_NORMAL),
        }
    }
}

pub(crate) fn cleanup_upgrade_ui(
    mut commands: Commands,
    query: Query<Entity, With<UpgradeRoot>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}
