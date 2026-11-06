//! Critical chance and critical damage as pure functions of a `ChannelSet`.

use crate::engine::aggregator::ChannelSet;
use crate::engine::effect::Channel;

pub const CRIT_DAMAGE_SCALE: u16 = 66;
const CRITICAL_DAMAGE_DEFAULT: f32 = 50.0;
/// Crit damage cap, percent. (The old `critical.rs` declared it but never applied it.)
const CRITICAL_DAMAGE_MAXIMUM_DEFAULT: f32 = 125.0;
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

/// Total crit rating: the "both" channel plus the weapon or spell one, never below 0.
pub fn crit_rating(c: &ChannelSet, spell: bool) -> u32 {
    let single = if spell { Channel::SpellCriticalChance } else { Channel::WeaponCriticalChance };
    (c.get(Channel::CriticalChance) + c.get(single)).max(0) as u32
}

/// Crit chance (0.0..=1.0) at an effective `level`: 10 % base + rating, capped at 100 %.
pub fn crit_chance(c: &ChannelSet, spell: bool, level: u8) -> f32 {
    (crit_rating_to_chance(crit_rating(c, spell), level) + CRITICAL_CHANCE_DEFAULT).min(1.0)
}

/// Crit damage in percent (50 base + bonuses), capped. `Channel::CriticalDamage` is in
/// percentage points.
pub fn crit_damage_pct(c: &ChannelSet) -> f32 {
    (CRITICAL_DAMAGE_DEFAULT + c.get(Channel::CriticalDamage) as f32).clamp(0.0, CRITICAL_DAMAGE_MAXIMUM_DEFAULT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_defaults() {
        let c = ChannelSet::new();
        assert_eq!(crit_damage_pct(&c), CRITICAL_DAMAGE_DEFAULT);
        assert_eq!(crit_chance(&c, false, EFFECTIVE_LEVEL), CRITICAL_CHANCE_DEFAULT);
        assert_eq!(crit_chance(&c, true, EFFECTIVE_LEVEL), CRITICAL_CHANCE_DEFAULT);
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
        assert_eq!(crit_rating(&c, false), 8421);
        // read from /script d(GetCriticalStrikeChance(8421))
        assert!((crit_chance(&c, false, EFFECTIVE_LEVEL) - (CRITICAL_CHANCE_DEFAULT + 0.384309959)).abs() < 1e-6);
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
    fn weapon_and_spell_chance_use_their_own_channel() {
        let mut c = ChannelSet::new();
        c.add_additive(Channel::CriticalChance, 1000);
        c.add_additive(Channel::WeaponCriticalChance, 2629);
        c.add_additive(Channel::SpellCriticalChance, 1314);
        assert_eq!(crit_rating(&c, false), 3629);
        assert_eq!(crit_rating(&c, true), 2314);
    }

    #[test]
    fn caps_hold() {
        let mut c = ChannelSet::new();
        c.add_additive(Channel::CriticalChance, 100_000);
        c.add_additive(Channel::CriticalDamage, 500);
        assert_eq!(crit_chance(&c, false, EFFECTIVE_LEVEL), 1.0);
        assert_eq!(crit_damage_pct(&c), 125.0);
    }
}
