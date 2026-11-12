//! Critical chance, critical damage and critical healing as pure functions of a `ChannelSet`.

use crate::engine::aggregator::ChannelSet;
use crate::engine::effect::Channel;
use esosim_types::CappedStat;

use super::armour::cap_raise;

pub const CRIT_DAMAGE_SCALE: u16 = 66;
const CRITICAL_DAMAGE_DEFAULT: f32 = 50.0;
const CRITICAL_HEALING_DEFAULT: f32 = 50.0;
const CRITICAL_CHANCE_DEFAULT: f32 = 0.1;
/// The level crit ratings are converted at: 50 + CP160 = effective level 66.
pub const EFFECTIVE_LEVEL: u8 = 66;

/// Effective level for rating conversions. Assumes a level 50 character has CP160.
pub fn effective_level(level: u8) -> u8 {
    if level >= 50 { EFFECTIVE_LEVEL } else { level }
}

/// Crit chance a rating is worth, without the 10 % base and without the cap.
pub fn crit_rating_to_chance(rating: u32, level: u8) -> f32 {
    let level_const = 2 * level as u32 * (100 + level as u32); // 21912 at level 66
    rating as f32 / level_const as f32
}

/// Total crit rating, never below 0. Weapon and spell critical are one stat.
pub fn crit_rating(c: &ChannelSet) -> u32 {
    c.get(Channel::CriticalChance).max(0) as u32
}

/// Crit chance (0.0..=1.0) at an effective `level`: 10 % base + rating, capped at 100 %.
pub fn crit_chance(c: &ChannelSet, level: u8) -> f32 {
    let percent = (crit_rating_to_chance(crit_rating(c), level) + CRITICAL_CHANCE_DEFAULT) as f64 * 100.0;
    (CappedStat::CriticalChance.apply(percent, cap_raise(c, CappedStat::CriticalChance)) / 100.0) as f32
}

/// Crit damage in percent (50 base + bonuses), capped. `Channel::CriticalDamage` is in
/// percentage points.
pub fn crit_damage_pct(c: &ChannelSet) -> f32 {
    CappedStat::CriticalDamage.apply(
        (CRITICAL_DAMAGE_DEFAULT + c.get(Channel::CriticalDamage) as f32) as f64,
        cap_raise(c, CappedStat::CriticalDamage),
    ) as f32
}

/// Crit healing in percent (50 base + bonuses), capped, in percentage points like crit damage.
pub fn crit_healing_pct(c: &ChannelSet) -> f32 {
    CappedStat::CriticalHealing.apply(
        (CRITICAL_HEALING_DEFAULT + c.get(Channel::CriticalHealing) as f32) as f64,
        cap_raise(c, CappedStat::CriticalHealing),
    ) as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_defaults() {
        let c = ChannelSet::new();
        assert_eq!(crit_damage_pct(&c), CRITICAL_DAMAGE_DEFAULT);
        assert_eq!(crit_healing_pct(&c), CRITICAL_HEALING_DEFAULT);
        assert_eq!(crit_chance(&c, EFFECTIVE_LEVEL), CRITICAL_CHANCE_DEFAULT);
    }

    #[test]
    fn test_calculate_crit_chance() {
        let mut c = ChannelSet::new();
        c.add_additive(
            Channel::CriticalChance,
            2629 // major savagery
                + 1922 // thief mundus
                + 219 * 6 // light armour
                + 320 // precision cp
                + 657 // slimecraw 1pc
                + 1579, // precise weapon
        );
        assert_eq!(crit_rating(&c), 8421);
        // read from /script d(GetCriticalStrikeChance(8421))
        assert!((crit_chance(&c, EFFECTIVE_LEVEL) - (CRITICAL_CHANCE_DEFAULT + 0.384309959)).abs() < 1e-6);
    }

    #[test]
    fn savagery_from_the_effect_data_is_six_and_twelve_percent() {
        use crate::engine::{active_effects::ActiveEffects, registry::EffectRegistry};
        let registry = EffectRegistry::embedded();
        let with = |id| {
            let mut active = ActiveEffects::new();
            active.set(id, 1);
            active.aggregate(&registry)
        };
        let (minor, major) = (with(61666), with(61667)); // Minor / Major Savagery
        assert_eq!((crit_rating(&minor), crit_rating(&major)), (1314, 2629));
        assert!((crit_chance(&minor, EFFECTIVE_LEVEL) - (CRITICAL_CHANCE_DEFAULT + 0.06)).abs() < 1e-3);
        assert!((crit_chance(&major, EFFECTIVE_LEVEL) - (CRITICAL_CHANCE_DEFAULT + 0.12)).abs() < 1e-3);
    }

    #[test]
    fn test_calculate_crit_damage() {
        let mut c = ChannelSet::new();
        c.add_additive(
            Channel::CriticalDamage,
            10 // animal companions (advanced species)
                + 2 // 1x medium armour
                + 12 // khajiit passive
                + 10, // velothi amulet/minor force
        );
        assert_eq!(crit_damage_pct(&c), 84.0);
        assert_eq!((crit_damage_pct(&c) * CRIT_DAMAGE_SCALE as f32) as u32, 84 * CRIT_DAMAGE_SCALE as u32);
    }

    #[test]
    fn an_effect_can_raise_the_crit_damage_cap() {
        let mut c = ChannelSet::new();
        c.add_additive(Channel::CriticalDamage, 500);
        c.add_additive(Channel::CapRaise(CappedStat::CriticalDamage), 10);
        assert_eq!(crit_damage_pct(&c), 135.0);
    }

    #[test]
    fn caps_hold() {
        let mut c = ChannelSet::new();
        c.add_additive(Channel::CriticalChance, 100_000);
        c.add_additive(Channel::CriticalDamage, 500);
        c.add_additive(Channel::CriticalHealing, 500);
        assert_eq!(crit_chance(&c, EFFECTIVE_LEVEL), 1.0);
        assert_eq!(crit_damage_pct(&c), 125.0);
        assert_eq!(crit_healing_pct(&c), 125.0);
    }
}
