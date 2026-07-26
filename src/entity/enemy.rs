use crate::engine::active_effects::ActiveEffects;
use crate::engine::registry::EffectRegistry;
use crate::stats::armour::Armour;

pub struct Enemy {
    pub id: u32,
    /// Same type a `Player` uses (§5) — this is what makes Major/Minor
    /// Breach, Vulnerability, and Brittle "just work" against an enemy
    /// with no new mechanism: they're `Effect`s applied here, feeding the
    /// same `Armour::calculate` a player's own defense goes through.
    pub active_effects: ActiveEffects,
    pub armour: Armour,
    pub max_health: f64,
    pub current_health: f64,
}

impl Enemy {
    pub fn health_percent(&self) -> f32 {
        if self.max_health <= 0.0 {
            return 0.0;
        }
        (self.current_health / self.max_health * 100.0) as f32
    }

    /// A "debuff" isn't a separately-tagged category anywhere in this
    /// model (§8.1: they're just `Effect`s on the enemy's own
    /// `ActiveEffects`), so counting them means counting active ids that
    /// resolve to *something* in the registry. This is looser than the
    /// real game's Reave (which counts only debuffs of specific types),
    /// so treat this as the wiring stub — narrowing it to "channels that
    /// weaken the target" is a one-line filter once `Effect` carries a
    /// `is_debuff`-style tag, which isn't in the doc's current `Effect`
    /// shape (§4.1) and would need to be added there, not invented here.
    pub fn debuff_count(&self, registry: &EffectRegistry) -> u32 {
        self.active_effects
            .ids()
            .filter(|&id| registry.contains(id))
            .count() as u32
    }
}