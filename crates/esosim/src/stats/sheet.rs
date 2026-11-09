//! `ChannelSet` -> the numbers a player recognises (roadmap 1.8, Appendix A.5).
//!
//! Everything here is a pure function: no state, no `reset`. The formulas live in
//! `power.rs`, `resource.rs`, `critical.rs` and `armour.rs` and are re-exported below.

use crate::engine::aggregator::ChannelSet;
use crate::engine::effect::{ResistableDamageType as Dmg, ResourceKind};
use crate::entity::enemy::Enemy;

pub use super::armour::{effective_resistance, penetration, resistance, resistance_to_mitigation};
pub use super::critical::{crit_chance, crit_damage_pct, crit_rating, crit_rating_to_chance, effective_level};
pub use super::power::{spell_damage, weapon_damage};
pub use super::resource::{max_resource, recovery};

/// Attribute points spent on each pool.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Attributes {
    pub health: u8,
    pub magicka: u8,
    pub stamina: u8,
}

impl Attributes {
    /// `Player::attributes` order: (magicka, health, stamina).
    pub fn from_player_tuple((magicka, health, stamina): (u8, u8, u8)) -> Self {
        Self { health, magicka, stamina }
    }
}

#[derive(Debug, Clone, Default)]
pub struct StatSheet {
    pub max_health: u32,
    pub max_magicka: u32,
    pub max_stamina: u32,
    pub weapon_damage: u32,
    pub spell_damage: u32,
    pub weapon_crit_rating: u32,
    pub spell_crit_rating: u32,
    pub weapon_crit_chance: f32,
    pub spell_crit_chance: f32,
    pub crit_damage_pct: f32,
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
        weapon_damage: weapon_damage(c, ctx.level),
        spell_damage: spell_damage(c, ctx.level),
        weapon_crit_rating: crit_rating(c, false),
        spell_crit_rating: crit_rating(c, true),
        weapon_crit_chance: crit_chance(c, false, eff),
        spell_crit_chance: crit_chance(c, true, eff),
        crit_damage_pct: crit_damage_pct(c),
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
        assert_eq!(s.weapon_damage, 5179);
        assert_eq!(s.weapon_crit_rating, 8421);
        assert!((s.weapon_crit_chance - 0.4843).abs() < 1e-4);
        assert_eq!(s.crit_damage_pct, 50.0);
    }

    #[test]
    fn attributes_follow_the_player_tuple_order() {
        let a = Attributes::from_player_tuple((10, 32, 22));
        assert_eq!((a.magicka, a.health, a.stamina), (10, 32, 22));
    }
}
