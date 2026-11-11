//! Weapon and spell damage as one stat, `power`, as a pure function of a `ChannelSet`.
//!
//! Every bonus to weapon damage is also a bonus to spell damage (the game hybridises them), so the
//! engine has one `Power` channel family and one number: `(level * 20 + additive)` then the
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

/// Weapon and spell damage, which are the same number.
pub fn power(c: &ChannelSet, level: u8) -> u32 {
    let base = level as i64 * 20 + c.get(Channel::Power(DamageDoneContext::All, Additive));
    let bps = c.get_multiplicative_bps(Channel::Power(DamageDoneContext::All, Multiplicative));
    apply_multiplicative_bps(base, bps)
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
        assert_eq!(power(&ChannelSet::new(), DEFAULT_LEVEL), DEFAULT_LEVEL as u32 * 20);
    }

    fn dual_wield_additive() -> i64 {
        // offhand and passive plus mainhand
        (((OFFHAND_MULTIPLIER + 0.06) as f32 * 1335.0) as u32 + 1535) as i64
    }

    #[test]
    fn test_dual_wield() {
        assert_eq!(power(&set(dual_wield_additive(), 0), DEFAULT_LEVEL), 2851);
    }

    #[test]
    fn test_dual_wield_fleshed_out_example() {
        let mut additive = dual_wield_additive();
        let mut bps = 300; // one fighters guild skill
        assert_eq!(power(&set(additive, bps), DEFAULT_LEVEL), 2937);
        bps += 1200; // 6 medium pieces
        assert_eq!(power(&set(additive, bps), DEFAULT_LEVEL), 3279);
        additive += 174 * 3; // jewellery with weapon damage enchants
        additive += 129 * 2; // 2x set lines
        assert_eq!(power(&set(additive, bps), DEFAULT_LEVEL), 4176);
        bps += 2000; // major brutality
        assert_eq!(power(&set(additive, bps), DEFAULT_LEVEL), 4902);
        additive += 205; // wrathful strikes CP <!> Doesn't show up on character sheet
        assert_eq!(power(&set(additive, bps), DEFAULT_LEVEL), 5179);
    }
}
