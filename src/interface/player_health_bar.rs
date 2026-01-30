use bevy::prelude::*;
use crate::player::Player;

#[derive(Component)]
pub(crate) struct HealthBar;

pub(crate) fn spawn_player_health_bar(mut commands: Commands) {
    commands.spawn((
        Node {
            width: Val::Px(200.0),
            height: Val::Px(20.0),
            position_type: PositionType::Absolute,
            left: Val::Px(20.0),
            top: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgb(0.7098039215686275, 0.7098039215686275, 0.7019607843137254)),
    ))
        .with_children(|parent| {
            parent.spawn((
                Node {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.6784313725490196, 0.00784313725490196, 0.050980392156862744)),
                HealthBar,
            ));
        });
}

pub(crate) fn update_player_health_bar(
    mut player_query: Query<&Player>,
    mut bar_query: Query<&mut Node, With<HealthBar>>,
) {
    let Ok(player) = player_query.single_mut() else {
        return;
    };

    let Ok(mut bar) = bar_query.single_mut() else {
        return;
    };

    let health_percent = (player.health / 100.0).clamp(0.0, 1.0);
    bar.width = Val::Percent(health_percent * 100.0);
}