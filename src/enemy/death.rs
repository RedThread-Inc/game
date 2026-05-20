use crate::enemy::Enemy;
use crate::InGameEntity;
use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub(crate) struct HealthPotion {
    pub(crate) heal_amount: f32,
}

pub(crate) fn enemy_death_system(
    mut commands: Commands,
    query: Query<(Entity, &Enemy, &Transform)>,
    asset_server: Res<AssetServer>,
) {
    let mut rng = rand::rng();

    for (entity, enemy, transform) in query.iter() {
        if enemy.health <= 0.0 {
            commands.entity(entity).despawn();

            if rng.random_range(0.0..1.0) < 0.05 {
                commands.spawn((
                    Sprite {
                        image: asset_server.load("healthPotion.png"),
                        custom_size: Some(Vec2::splat(20.0)),
                        ..default()
                    },
                    Transform::from_translation(Vec3::new(
                        transform.translation.x,
                        transform.translation.y,
                        10.0,
                    )),
                    HealthPotion { heal_amount: 20.0 },
                    InGameEntity,
                ));
            }
        }
    }
}