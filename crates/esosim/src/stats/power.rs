//! Weapon and spell damage (the "Power" stats) as pure functions of a `ChannelSet`.
//!
//! `level + 20` style formula from the old `Power` struct: `(level * 20 + additive)` then the
//! multiplicative bucket (basis points).

use esosim_types::AggKind::{Additive, Multiplicative};
use esosim_types::DamageDoneContext;

use crate::engine::aggregator::ChannelSet;
use crate::engine::effect::Channel;

pub const DEFAULT_LEVEL: u8 = 50;

/// `base * (1 + bps / 10_000)`, rounded half away from zero, never below 0.
pub(crate) fn apply_multiplicative_bps(base: i64, bps: i64) -> u32 {
    let v = base as f64 * (10_000 + bps) as f64 / 10_000.0;
    v.round().max(0.0) as u32
}

fn damage(c: &ChannelSet, single: Channel, level: u8) -> u32 {
    let base = level as i64 * 20 + c.get(Channel::Power(DamageDoneContext::All, Additive)) + c.get(single);
    let bps = c.get_multiplicative_bps(Channel::Power(DamageDoneContext::All, Multiplicative)) + c.get_multiplicative_bps(single);
    apply_multiplicative_bps(base, bps)
}

pub fn weapon_damage(c: &ChannelSet, level: u8) -> u32 {
    damage(c, Channel::WeaponPower, level)
}

pub fn spell_damage(c: &ChannelSet, level: u8) -> u32 {
    damage(c, Channel::SpellPower, level)
}

pub fn power(c: &ChannelSet, level: u8) -> u32 {
    damage(c, Channel::WeaponPower, level).max(damage(c, Channel::SpellPower, level))
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::data::tables::power::OFFHAND_MULTIPLIER;

    fn set(additive: i64, bps: i64) -> ChannelSet {
        let mut c = ChannelSet::new();
        c.add_additive(Channel::Power(DamageDoneContext::All, Additive), additive);
        c.add_multiplicative_bps(Channel::Power(DamageDoneContext::All, Multiplicative), bps);
        c
    }

    #[test]
    fn test_calculate_default_power() {
        assert_eq!(weapon_damage(&ChannelSet::new(), DEFAULT_LEVEL), DEFAULT_LEVEL as u32 * 20);
        assert_eq!(spell_damage(&ChannelSet::new(), DEFAULT_LEVEL), DEFAULT_LEVEL as u32 * 20);
    }

    fn dual_wield_additive() -> i64 {
        // offhand and passive plus mainhand
        (((OFFHAND_MULTIPLIER + 0.06) as f32 * 1335.0) as u32 + 1535) as i64
    }

    #[test]
    fn test_dual_wield() {
        assert_eq!(weapon_damage(&set(dual_wield_additive(), 0), DEFAULT_LEVEL), 2851);
    }

    #[test]
    fn test_dual_wield_fleshed_out_example() {
        let mut additive = dual_wield_additive();
        let mut bps = 300; // one fighters guild skill
        assert_eq!(weapon_damage(&set(additive, bps), DEFAULT_LEVEL), 2937);
        bps += 1200; // 6 medium pieces
        assert_eq!(weapon_damage(&set(additive, bps), DEFAULT_LEVEL), 3279);
        additive += 174 * 3; // jewellery with weapon damage enchants
        additive += 129 * 2; // 2x set lines
        assert_eq!(weapon_damage(&set(additive, bps), DEFAULT_LEVEL), 4176);
        bps += 2000; // major brutality
        assert_eq!(weapon_damage(&set(additive, bps), DEFAULT_LEVEL), 4902);
        additive += 205; // wrathful strikes CP <!> Doesn't show up on character sheet
        assert_eq!(weapon_damage(&set(additive, bps), DEFAULT_LEVEL), 5179);
    }

    #[test]
    fn single_type_power_only_counts_for_its_own_damage() {
        let mut c = ChannelSet::new();
        c.add_additive(Channel::WeaponPower, 100);
        c.add_additive(Channel::SpellPower, 40);
        assert_eq!(weapon_damage(&c, 50), 1100);
        assert_eq!(spell_damage(&c, 50), 1040);
    }
}
