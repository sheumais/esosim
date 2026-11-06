//! Resistance, penetration and mitigation as pure functions of a `ChannelSet`.

use crate::engine::aggregator::ChannelSet;
use crate::engine::effect::{Channel, ResistableDamageType as Dmg};

/// Resistance needed for the mitigation cap.
const MITIGATION_CAP_RESISTANCE: f32 = 33_000.0;
/// Mitigation at the cap.
const MITIGATION_CAP: f32 = 0.5;
/// Resistance per 100 % mitigation at the effective level 66 (66 * 1000).
const RESISTANCE_PER_UNIT: f32 = 66_000.0;

/// The broad group a damage type belongs to: `Spell` covers the magical types, `Martial` the
/// physical ones. `All`, `Spell` and `Martial` have no group of their own.
fn group(kind: Dmg) -> Option<Dmg> {
    match kind {
        Dmg::Flame | Dmg::Shock | Dmg::Frost | Dmg::Earth | Dmg::Magic => Some(Dmg::Spell),
        Dmg::Physical | Dmg::Disease | Dmg::Poison | Dmg::Bleed => Some(Dmg::Martial),
        Dmg::Martial | Dmg::Spell | Dmg::All => None,
    }
}

/// Sum of `All`, the group and the exact type, from one channel family.
fn layered(c: &ChannelSet, kind: Dmg, channel: fn(Dmg) -> Channel) -> u32 {
    let mut total = c.get(channel(Dmg::All));
    if kind != Dmg::All {
        total += c.get(channel(kind));
    }
    if let Some(g) = group(kind) {
        total += c.get(channel(g));
    }
    total.max(0) as u32
}

/// Resistance against `kind`: `All` + its group (`Spell`/`Martial`) + the exact type.
pub fn resistance(c: &ChannelSet, kind: Dmg) -> u32 {
    layered(c, kind, Channel::Armour)
}

/// Penetration against `kind`, layered the same way.
pub fn penetration(c: &ChannelSet, kind: Dmg) -> u32 {
    layered(c, kind, Channel::Penetration)
}

/// Resistance left after penetration. Can be negative (which increases damage taken).
pub fn effective_resistance(target_resistance: u32, attacker_penetration: u32) -> i64 {
    target_resistance as i64 - attacker_penetration as i64
}

/// Fraction of damage mitigated by `effective_resistance`: resistance / 66,000, capped at 50 %
/// (33,000 resistance). Negative resistance gives negative mitigation (extra damage), uncapped.
/// Formula and cap are the commonly quoted effective-level-66 values; not yet checked against
/// this repo's own data.
pub fn resistance_to_mitigation(effective_resistance: i64) -> f32 {
    let r = effective_resistance as f32;
    if r >= MITIGATION_CAP_RESISTANCE { MITIGATION_CAP } else { r / RESISTANCE_PER_UNIT }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_set_has_nothing() {
        let c = ChannelSet::new();
        assert_eq!(resistance(&c, Dmg::Spell), 0);
        assert_eq!(penetration(&c, Dmg::Physical), 0);
    }

    #[test]
    fn resistance_layers_all_group_and_type() {
        let mut c = ChannelSet::new();
        c.add_additive(Channel::Armour(Dmg::All), 1000);
        c.add_additive(Channel::Armour(Dmg::Spell), 200);
        c.add_additive(Channel::Armour(Dmg::Martial), 30);
        c.add_additive(Channel::Armour(Dmg::Frost), 4);
        assert_eq!(resistance(&c, Dmg::Frost), 1204);
        assert_eq!(resistance(&c, Dmg::Flame), 1200);
        assert_eq!(resistance(&c, Dmg::Poison), 1030);
        assert_eq!(resistance(&c, Dmg::Spell), 1200);
        assert_eq!(resistance(&c, Dmg::All), 1000);
    }

    #[test]
    fn penetration_layers_the_same_way() {
        let mut c = ChannelSet::new();
        c.add_additive(Channel::Penetration(Dmg::All), 2974);
        c.add_additive(Channel::Penetration(Dmg::Martial), 1000);
        assert_eq!(penetration(&c, Dmg::Bleed), 3974);
        assert_eq!(penetration(&c, Dmg::Shock), 2974);
    }

    #[test]
    fn negative_totals_are_floored_at_zero() {
        let mut c = ChannelSet::new();
        c.add_additive(Channel::Armour(Dmg::All), -5948);
        assert_eq!(resistance(&c, Dmg::All), 0);
    }

    #[test]
    fn mitigation_and_cap() {
        assert_eq!(effective_resistance(18200, 18200), 0);
        assert_eq!(effective_resistance(18200, 20000), -1800);
        assert_eq!(resistance_to_mitigation(0), 0.0);
        assert!((resistance_to_mitigation(16500) - 0.25).abs() < 1e-6);
        assert_eq!(resistance_to_mitigation(33_000), 0.5);
        assert_eq!(resistance_to_mitigation(60_000), 0.5);
        assert!(resistance_to_mitigation(-1800) < 0.0);
    }
}
