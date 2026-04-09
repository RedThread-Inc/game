pub(crate) mod ui;
pub(crate) mod plugin;

use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum Upgrade {
    ExtraProjectile,
    FasterFireRate,
    MoreDamage,
    FasterProjectile,
    LargerRadius,
}

impl Upgrade {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Upgrade::ExtraProjectile  => "Boule de feu supplémentaire",
            Upgrade::FasterFireRate   => "Cadence de tir +25%",
            Upgrade::MoreDamage       => "Dégâts +50%",
            Upgrade::FasterProjectile => "Projectiles plus rapides",
            Upgrade::LargerRadius     => "Zone d'impact élargie",
        }
    }

    pub(crate) fn description(self) -> &'static str {
        match self {
            Upgrade::ExtraProjectile  => "Lance une boule de feu supplémentaire à chaque tir",
            Upgrade::FasterFireRate   => "Réduit le délai entre chaque tir de 25%",
            Upgrade::MoreDamage       => "Augmente les dégâts de chaque projectile de 50%",
            Upgrade::FasterProjectile => "Les projectiles voyagent 50% plus vite",
            Upgrade::LargerRadius     => "Les projectiles ont une hitbox plus grande",
        }
    }
}

#[derive(Resource)]
pub(crate) struct PlayerUpgrades {
    pub(crate) extra_projectiles: u32,
    pub(crate) fire_rate_multiplier: f32,
    pub(crate) damage_multiplier: f32,
    pub(crate) speed_multiplier: f32,
    pub(crate) radius_bonus: f32,
}

impl Default for PlayerUpgrades {
    fn default() -> Self {
        Self {
            extra_projectiles: 0,
            fire_rate_multiplier: 1.0,
            damage_multiplier: 1.0,
            speed_multiplier: 1.0,
            radius_bonus: 0.0,
        }
    }
}

impl PlayerUpgrades {
    pub(crate) fn apply(&mut self, upgrade: Upgrade) {
        match upgrade {
            Upgrade::ExtraProjectile  => self.extra_projectiles += 1,
            Upgrade::FasterFireRate   => self.fire_rate_multiplier *= 0.75,
            Upgrade::MoreDamage       => self.damage_multiplier *= 1.5,
            Upgrade::FasterProjectile => self.speed_multiplier *= 1.5,
            Upgrade::LargerRadius     => self.radius_bonus += 16.0,
        }
    }
}
