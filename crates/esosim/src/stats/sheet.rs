//! `ChannelSet` -> the numbers a player recognises (roadmap 1.8, Appendix A.5).
//!
//! Everything here is a pure function: no state, no `reset`. The formulas live in
//! `power.rs`, `resource.rs`, `critical.rs` and `armour.rs` and are re-exported below.

use crate::engine::aggregator::ChannelSet;
use crate::engine::effect::{ResistableDamageType as Dmg, ResourceKind};
use crate::entity::enemy::Enemy;
use serde::{Deserialize, Serialize};

pub use super::armour::{effective_resistance, penetration, resistance, resistance_to_mitigation};
pub use super::critical::{
    crit_chance, crit_damage_pct, crit_healing_pct, crit_rating, crit_rating_to_chance, effective_level,
};
pub use super::power::power;
pub use super::resource::{max_resource, recovery};

/// Attribute points spent on each pool.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attributes {
    pub health: u8,
    pub magicka: u8,
    pub stamina: u8,
}

impl Attributes {
    /// From a (magicka, health, stamina) tuple.
    pub fn from_player_tuple((magicka, health, stamina): (u8, u8, u8)) -> Self {
        Self { health, magicka, stamina }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StatSheet {
    pub max_health: u32,
    pub max_magicka: u32,
    pub max_stamina: u32,
    /// Weapon damage and spell damage: one number, since every bonus to one is a bonus to the other.
    pub power: u32,
    /// Weapon and spell critical rating, likewise one number.
    pub crit_rating: u32,
    pub crit_chance: f32,
    pub crit_damage_pct: f32,
    pub crit_healing_pct: f32,
    pub physical_resistance: u32,
    pub spell_resistance: u32,
    pub physical_penetration: u32,
    pub spell_penetration: u32,
    pub health_recovery: u32,
    pub magicka_recovery: u32,
    pub stamina_recovery: u32,
}

pub struct StatContext<'a> {
    /// Character level (50 for max level); drives the resource and power formulas.
    pub level: u8,
    pub attributes: Attributes,
    /// Reserved for target-dependent derivations (3.x); `derive_sheet` does not read it yet.
    pub target: Option<&'a Enemy>,
}

pub fn derive_sheet(c: &ChannelSet, ctx: &StatContext) -> StatSheet {
    let eff = effective_level(ctx.level);
    let pool = |kind, points| super::resource::max_resource_at_level(c, kind, points, ctx.level);
    StatSheet {
        max_health: pool(ResourceKind::Health, ctx.attributes.health),
        max_magicka: pool(ResourceKind::Magicka, ctx.attributes.magicka),
        max_stamina: pool(ResourceKind::Stamina, ctx.attributes.stamina),
        power: power(c, ctx.level),
        crit_rating: crit_rating(c),
        crit_chance: crit_chance(c, eff),
        crit_damage_pct: crit_damage_pct(c),
        crit_healing_pct: crit_healing_pct(c),
        physical_resistance: resistance(c, Dmg::Martial),
        spell_resistance: resistance(c, Dmg::Spell),
        physical_penetration: penetration(c, Dmg::Martial),
        spell_penetration: penetration(c, Dmg::Spell),
        health_recovery: recovery(c, ResourceKind::Health),
        magicka_recovery: recovery(c, ResourceKind::Magicka),
        stamina_recovery: recovery(c, ResourceKind::Stamina),
    }
}

#[cfg(test)]
mod tests {
    use esosim_types::AggKind::{Additive, Multiplicative};

use super::*;
    use crate::engine::effect::{AggKind, Channel};

    #[test]
    fn the_three_golden_numbers_come_out_of_derive_sheet() {
        let mut c = ChannelSet::new();
        // 33,456 max health (tank)
        c.add_additive(
            Channel::Resource(ResourceKind::Health, AggKind::Additive),
            4462 + 477 * 2 + 193 * 3 + 560 + 1206 * 2 + 1000,
        );
        c.add_multiplicative_bps(Channel::Resource(ResourceKind::Health, AggKind::Multiplicative), 1200);
        // 48.43 % crit chance = 10 % base + 8421 / 21912
        c.add_additive(Channel::CriticalChance, 2629 + 1922 + 219 * 6 + 320 + 657 + 1579);
        // 5179 weapon damage (dual wield example)
        c.add_additive(Channel::Power(esosim_types::DamageDoneContext::All, Additive), 1851 + 174 * 3 + 129 * 2 + 205);
        c.add_multiplicative_bps(Channel::Power(esosim_types::DamageDoneContext::All, Multiplicative), 3500);

        let ctx = StatContext { level: 50, attributes: Attributes { health: 32, magicka: 0, stamina: 0 }, target: None };
        let s = derive_sheet(&c, &ctx);
        assert_eq!(s.max_health, 33456);
        assert_eq!(s.max_magicka, 12000);
        assert_eq!(s.power, 5179);
        assert_eq!(s.crit_rating, 8421);
        assert!((s.crit_chance - 0.4843).abs() < 1e-4);
        assert_eq!(s.crit_damage_pct, 50.0);
    }

    #[test]
    fn every_sheet_field_has_a_channel_behind_it() {
        let ctx = StatContext { level: 50, attributes: Attributes::default(), target: None };
        let base = derive_sheet(&ChannelSet::new(), &ctx);
        let with = |channel: Channel| {
            let mut c = ChannelSet::new();
            c.add_additive(channel, 100);
            derive_sheet(&c, &ctx)
        };
        assert!(with(Channel::Resource(ResourceKind::Health, Additive)).max_health > base.max_health);
        assert!(with(Channel::Resource(ResourceKind::Magicka, Additive)).max_magicka > base.max_magicka);
        assert!(with(Channel::Resource(ResourceKind::Stamina, Additive)).max_stamina > base.max_stamina);
        assert!(with(Channel::Power(esosim_types::DamageDoneContext::All, Additive)).power > base.power);
        assert!(with(Channel::CriticalChance).crit_rating > base.crit_rating);
        assert!(with(Channel::CriticalChance).crit_chance > base.crit_chance);
        assert!(with(Channel::CriticalDamage).crit_damage_pct > base.crit_damage_pct);
        assert!(with(Channel::CriticalHealing).crit_healing_pct > base.crit_healing_pct);
        assert!(with(Channel::Armour(Dmg::Martial)).physical_resistance > base.physical_resistance);
        assert!(with(Channel::Armour(Dmg::Spell)).spell_resistance > base.spell_resistance);
        assert!(with(Channel::Penetration(Dmg::Martial)).physical_penetration > base.physical_penetration);
        assert!(with(Channel::Penetration(Dmg::Spell)).spell_penetration > base.spell_penetration);
        assert!(with(Channel::Recovery(ResourceKind::Health, Additive)).health_recovery > base.health_recovery);
        assert!(with(Channel::Recovery(ResourceKind::Magicka, Additive)).magicka_recovery > base.magicka_recovery);
        assert!(with(Channel::Recovery(ResourceKind::Stamina, Additive)).stamina_recovery > base.stamina_recovery);
    }

    #[test]
    fn attributes_follow_the_player_tuple_order() {
        let a = Attributes::from_player_tuple((10, 32, 22));
        assert_eq!((a.magicka, a.health, a.stamina), (10, 32, 22));
    }
}
