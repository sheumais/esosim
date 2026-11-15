//! Critical chance, critical damage and critical healing as pure functions of a `ChannelSet`.

use crate::engine::aggregator::ChannelSet;
use crate::engine::effect::Channel;
use esosim_types::CappedStat;

use super::armour::cap_raise;

pub const CRIT_DAMAGE_SCALE: u16 = 66;
const CRITICAL_DAMAGE_DEFAULT: f32 = 50.0;
const CRITICAL_HEALING_DEFAULT: f32 = 50.0;
/// The base crit chance of a character below the effective level, where the game's base rating has not been
/// measured: 10 % of the level constant.
const CRITICAL_CHANCE_DEFAULT: f32 = 0.1;
/// The level crit ratings are converted at: 50 + CP160 = effective level 66.
pub const EFFECTIVE_LEVEL: u8 = 66;
/// The crit rating a character has with nothing equipped or active, at [`EFFECTIVE_LEVEL`]. Read from the game
/// (`CRITICAL_STRIKE.without_bonus` in both golden fixtures), and 10.2 short of the 10 % (2191.2) the engine used
/// to assume: 2181 / 21912 = 9.953 %. The game's totals only add up with this base (2181 + 6017 = 8198 on the
/// Arcanist, 2181 + 539 = 2720 on the Sorcerer).
pub const BASE_CRIT_RATING: f64 = 2181.0;

/// Effective level for rating conversions. Assumes a level 50 character has CP160.
pub fn effective_level(level: u8) -> u8 {
    if level >= 50 { EFFECTIVE_LEVEL } else { level }
}

/// What one point of crit rating is divided by at `level`: 2 x level x (100 + level), 21912 at level 66.
fn level_constant(level: u8) -> u32 {
    2 * level as u32 * (100 + level as u32)
}

/// Crit chance a rating is worth, without the base and without the cap.
pub fn crit_rating_to_chance(rating: u32, level: u8) -> f32 {
    rating as f32 / level_constant(level) as f32
}

/// The rating a character has before any source: [`BASE_CRIT_RATING`] at the effective level, 10 % of the level
/// constant at any other level (not measured there).
fn base_crit_rating(level: u8) -> f64 {
    if level == EFFECTIVE_LEVEL {
        BASE_CRIT_RATING
    } else {
        CRITICAL_CHANCE_DEFAULT as f64 * level_constant(level) as f64
    }
}

/// The crit rating the sources add (the base is not in it), never below 0. Weapon and spell critical are one stat.
///
/// The sources are added up with their fractions (6 % of 21,912 is 1314.72) and the total is cut down to a whole
/// number once, as the game's sheet does. Rounding each source first loses up to a point per source.
pub fn crit_rating(c: &ChannelSet) -> u32 {
    // the epsilon keeps a sum that is a whole number in the game, and a hair under it in floats, from losing a point
    (c.get_exact(Channel::CriticalChance) + 1e-6).floor().max(0.0) as u32
}

/// Crit chance (0.0..=1.0) at an effective `level`: (base rating + rating) / level constant, capped at 100 %.
pub fn crit_chance(c: &ChannelSet, level: u8) -> f32 {
    let total = base_crit_rating(level) + crit_rating(c) as f64;
    let percent = total / level_constant(level) as f64 * 100.0;
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
        // nothing equipped: the game's base rating alone, 2181 / 21912
        assert!((crit_chance(&c, EFFECTIVE_LEVEL) as f64 - 2181.0 / 21912.0).abs() < 1e-7);
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
        // read from /script d(GetCriticalStrikeChance(8421)): what the rating alone is worth
        assert!((crit_rating_to_chance(8421, EFFECTIVE_LEVEL) - 0.384309959).abs() < 1e-6);
        // the character's chance adds the base rating (2181) to the rating before dividing
        assert!((crit_chance(&c, EFFECTIVE_LEVEL) as f64 - (2181.0 + 8421.0) / 21912.0).abs() < 1e-6);
    }

    /// The Arcanist golden fixture: four 3 % sources, Major Savagery (12 %), Pressure Points (2 %) and the
    /// Precision star. Rounding each source gives 6015; the game adds them as they are (6017.12) and shows 6017.
    #[test]
    fn the_total_is_floored_once_not_each_source_rounded() {
        let mut c = ChannelSet::new();
        for _ in 0..4 {
            c.add_additive_f64(Channel::CriticalChance, 657.36); // 3 % of 21912
        }
        c.add_additive_f64(Channel::CriticalChance, 2629.44); // 12 %
        c.add_additive_f64(Channel::CriticalChance, 438.24); // 2 %
        c.add_additive_f64(Channel::CriticalChance, 320.0);
        assert_eq!(c.get(Channel::CriticalChance), 6015, "the sum of the whole parts");
        assert_eq!(crit_rating(&c), 6017);
        // 37.41328812 % in the game
        assert!((crit_chance(&c, EFFECTIVE_LEVEL) as f64 - 0.3741328812).abs() < 1e-6);

        // the Sorcerer golden fixture: Prodigy on one Light piece (1 %) + the Precision star = 539.12, so 2720 in all
        let mut c = ChannelSet::new();
        c.add_additive_f64(Channel::CriticalChance, 219.12);
        c.add_additive_f64(Channel::CriticalChance, 320.0);
        assert_eq!(crit_rating(&c), 539);
        assert!((crit_chance(&c, EFFECTIVE_LEVEL) as f64 - 0.1241328907).abs() < 1e-6);
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
        // 6 % and 12 % of the level constant on top of the game's base rating
        assert!((crit_chance(&minor, EFFECTIVE_LEVEL) as f64 - (2181.0 + 1314.0) / 21912.0).abs() < 1e-6);
        assert!((crit_chance(&major, EFFECTIVE_LEVEL) as f64 - (2181.0 + 2629.0) / 21912.0).abs() < 1e-6);
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
