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
    MoreHealth,
}

impl Upgrade {
    pub(crate) fn name(self) -> &'static str {
        match self {
            Upgrade::ExtraProjectile  => "Boule de feu supplementaire",
            Upgrade::FasterFireRate   => "Cadence de tir +20%",
            Upgrade::MoreDamage       => "Degats +20%",
            Upgrade::FasterProjectile => "Projectiles plus rapides",
            Upgrade::LargerRadius     => "Zone d'impact elargie",
            Upgrade::MoreHealth       => "Vitalite accrue",
        }
    }

    pub(crate) fn description(self) -> &'static str {
        match self {
            Upgrade::ExtraProjectile  => "Lance une boule de feu supplementaire a chaque tir",
            Upgrade::FasterFireRate   => "Reduit le delai entre chaque tir de 20% (cumulable)",
            Upgrade::MoreDamage       => "Augmente les degats de 20% par niveau",
            Upgrade::FasterProjectile => "Les projectiles voyagent 15% plus vite par niveau",
            Upgrade::LargerRadius     => "Les projectiles ont une hitbox plus grande (+10px)",
            Upgrade::MoreHealth       => "Augmente la vie maximale de 25 points",
        }
    }
}

#[derive(Resource, Default)]
pub(crate) struct PlayerUpgrades {
    pub(crate) extra_projectiles: u32,
    pub(crate) fire_rate_level:   u32,
    pub(crate) damage_level:      u32,
    pub(crate) speed_level:       u32,
    pub(crate) radius_level:      u32,
    pub(crate) health_level:      u32,
}

impl PlayerUpgrades {
    pub(crate) fn apply(&mut self, upgrade: Upgrade) {
        match upgrade {
            Upgrade::ExtraProjectile  => self.extra_projectiles += 1,
            Upgrade::FasterFireRate   => self.fire_rate_level   += 1,
            Upgrade::MoreDamage       => self.damage_level      += 1,
            Upgrade::FasterProjectile => self.speed_level       += 1,
            Upgrade::LargerRadius     => self.radius_level      += 1,
            Upgrade::MoreHealth       => self.health_level      += 1,
        }
    }

    /// Multiplicateur appliqué au délai entre les tirs (< 1.0 = plus rapide).
    /// -20% par niveau, calculé depuis le niveau 0.
    pub(crate) fn fire_rate_multiplier(&self) -> f32 {
        0.80_f32.powi(self.fire_rate_level as i32)
    }

    /// Multiplicateur de dégâts, +20% plat par niveau.
    pub(crate) fn damage_multiplier(&self) -> f32 {
        1.0 + 0.20 * self.damage_level as f32
    }

    /// Multiplicateur de vitesse des projectiles, +15% plat par niveau.
    pub(crate) fn speed_multiplier(&self) -> f32 {
        1.0 + 0.15 * self.speed_level as f32
    }

    /// Bonus de rayon en pixels, +10px plat par niveau.
    pub(crate) fn radius_bonus(&self) -> f32 {
        10.0 * self.radius_level as f32
    }

    /// Bonus de vie maximale, +25 par niveau.
    pub(crate) fn max_health_bonus(&self) -> f32 {
        25.0 * self.health_level as f32
    }

    /// Niveau actuel d'un upgrade donné (pour limiter le tirage si besoin).
    pub(crate) fn level_of(&self, upgrade: Upgrade) -> u32 {
        match upgrade {
            Upgrade::ExtraProjectile  => self.extra_projectiles,
            Upgrade::FasterFireRate   => self.fire_rate_level,
            Upgrade::MoreDamage       => self.damage_level,
            Upgrade::FasterProjectile => self.speed_level,
            Upgrade::LargerRadius     => self.radius_level,
            Upgrade::MoreHealth       => self.health_level,
        }
    }
}