use phf::{Map, phf_map};

use crate::{StatBuff as Buff, major_minor::TOUGHNESS_MINOR_ID, skill::{BONE_GOLIATH_TRANSFORMATION_ID, DARK_VIGOR_ID, EXPERT_SUMMONER_ID, LAST_GASP_ID, MAGICKA_CONTROLLER_ID, MAGICKA_FLOOD_ID, UNDAUNTED_METTLE_ID}};

// *todo, all the fucking foods. like a hundred foods. ugh.

/* ALL */
// Undaunted Mettle
pub static UNDAUNTED_METTLE: Buff = Buff { id: UNDAUNTED_METTLE_ID, value: 0f64, value_per_stack: 2f64}; // Multiplicative
pub static LUNAR_BLESSINGS: Buff = Buff { id: 117848, value: 965f64, value_per_stack: 0f64};
pub static BLOOD_SCION: Buff = Buff { id: 32624, value: 10000f64, value_per_stack: 0f64};
pub static SWARMING_SCION: Buff = Buff { id: 38932, value: 10000f64, value_per_stack: 0f64};
pub static PERFECT_SCION: Buff = Buff { id: 38931, value: 10000f64, value_per_stack: 0f64};

/* MAG AND STAM */
pub static XORYNS_MASTERPIECE: Buff = Buff { id: 220010, value: 1667f64, value_per_stack: 0f64};
pub static IMPERIAL_METTLE: Buff = Buff { id: 45280, value: 2000f64,  value_per_stack: 0f64};
pub static DYNAMIC: Buff = Buff { id: 45267, value: 1910f64, value_per_stack: 0f64};
pub static RESOURCEFUL: Buff = Buff { id: 45247, value: 1000f64, value_per_stack: 0f64};
pub static AGGRESSIVE_HORN: Buff = Buff { id: 40224, value: 10f64, value_per_stack: 0f64}; // Multiplicative
// pub static STURDY_HORN: Buff = Buff { id: /* */, value: 10f64, value_per_stack: 0f64}; // Multiplicative

/* MAX STAMINA SOURCES */
// https://eso-sets.com/set/bone-pirates-tatters
// CP
// https://en.uesp.net/wiki/Online:Energetic_Expiration IA verse
// https://eso-sets.com/set/spriggans-vigor
pub static WEREWOLF_TRANSFORMATION: Buff = Buff { id: 33469, value: 30f64, value_per_stack: 0f64};
pub static BRAWNY: Buff = Buff { id: 45309, value: 1000f64, value_per_stack: 0f64};
pub static CONDITIONING: Buff = Buff { id: 117754, value: 2000f64, value_per_stack: 0f64};
pub static RESIST_AFFLICTION: Buff = Buff { id: 45319, value: 2000f64, value_per_stack: 0f64};
pub static EXPERT_SUMMONER_STAMINA: Buff = Buff { id: EXPERT_SUMMONER_ID, value: 5f64, value_per_stack: 0f64}; // Multiplicative

pub static RESOURCE_STAMINA_ADDITIVE: Map<u32, &'static Buff> = phf_map! {
    117848 => &LUNAR_BLESSINGS,
    32624 => &BLOOD_SCION,
    38932 => &SWARMING_SCION,
    38931 => &PERFECT_SCION,
    220010 => &XORYNS_MASTERPIECE,
    45280 => &IMPERIAL_METTLE,
    45267 => &DYNAMIC,
    45247 => &RESOURCEFUL,
    45309 => &BRAWNY,
    117754 => &CONDITIONING,
    45319 => &RESIST_AFFLICTION,
};

pub static RESOURCE_STAMINA_MULTIPLICATIVE: Map<u32, &'static Buff> = phf_map! {
    40224 => &AGGRESSIVE_HORN,
    55386 => &UNDAUNTED_METTLE,
    33469 => &WEREWOLF_TRANSFORMATION,
    45199 => &EXPERT_SUMMONER_STAMINA,
    // ??? => &STURDY_HORN
};

/* MAX HEALTH SOURCES */
// Stone Garden buffs ( Alchemized )
// Bal Sunnar buffs ( Ancestral Resolve )
// March of Sacrifices buff ( Boon of Hardiness )
// Naj-Caldeesh buff ( Borrowed Vitality )
// Ice Avatar, infinite Archive. ( Echo of Fortitude )
// Reflected Ruin Verse (infinite archive)
// Shapeshifters Chain Mythic
// Thrassian Stranglers Mythic
// Oathsworn Pit buff ( Totem of Blood )
// https://eso-sets.com/set/armor-master
pub static DARK_VIGOR: Buff = Buff { id: DARK_VIGOR_ID, value: 0f64, value_per_stack: 5f64}; // Multiplicative
pub static EBON_ARMOURY: Buff = Buff{ id: 47362, value: 1000f64, value_per_stack: 0f64}; // Additive
// Emperor
// Emperor Alliance Bonus I-VI
/// Increase stacks to 2 if player has a permanent pet, otherwise default stacks will make this buff give nothing for health specifically which is intended.
pub static EXPERT_SUMMONER_HEALTH: Buff = Buff { id: EXPERT_SUMMONER_ID, value: -5f64, value_per_stack: 5f64}; // Multiplicative
pub static MINOR_MANGLE: Buff = Buff { id: 61733, value: -10f64, value_per_stack: 0f64}; // Multiplicative
// https://eso-sets.com/set/green-pact
// Dread Cellar buff ( Grim Warden's Accession )
// Hero's Vigor CP
// Black Drake Villa buff ( Ice Avatar )
pub static JUGGERNAUT: Buff = Buff { id: 45546, value: 0f64, value_per_stack: 2f64}; // Multiplicative
pub static LAST_GASP: Buff = Buff { id: LAST_GASP_ID, value: 2412f64, value_per_stack: 0f64};
pub static MINOR_TOUGHNESS: Buff = Buff { id: TOUGHNESS_MINOR_ID, value: 10f64, value_per_stack: 0f64}; // Multiplicative
// Goliath Transformation
pub static BONE_GOLIATH_TRANSFORMATION: Buff = Buff { id: BONE_GOLIATH_TRANSFORMATION_ID, value: 30_000f64, value_per_stack: 0f64};
pub static RESIST_FROST: Buff = Buff { id: 45304, value: 1000f64, value_per_stack: 0f64};
pub static ARGONIAN_RESISTANCE: Buff = Buff { id: 45255, value: 1000f64, value_per_stack: 0f64};
pub static TOUGH: Buff = Buff { id: 50907, value: 2000f64, value_per_stack: 0f64};
pub static UNFLINCHING_RAGE: Buff = Buff { id: 84672, value: 1000f64, value_per_stack: 0f64};

pub static RESOURCE_HEALTH_ADDITIVE: Map<u32, &'static Buff> = phf_map! {
    117848 => &LUNAR_BLESSINGS,
    32624 => &BLOOD_SCION,
    38932 => &SWARMING_SCION,
    38931 => &PERFECT_SCION,
    47362 => &EBON_ARMOURY,
    116272 => &MINOR_TOUGHNESS,
    115001 => &BONE_GOLIATH_TRANSFORMATION,
    45304 => &RESIST_FROST,
    45255 => &ARGONIAN_RESISTANCE,
    50907 => &TOUGH,
    84672 => &UNFLINCHING_RAGE,
};

pub static RESOURCE_HEALTH_MULTIPLICATIVE: Map<u32, &'static Buff> = phf_map! {
    55386 => &UNDAUNTED_METTLE,
    45084 => &DARK_VIGOR,
    45199 => &EXPERT_SUMMONER_HEALTH,
    61733 => &MINOR_MANGLE,
    45546 => &JUGGERNAUT,
};

/* MAX MAGICKA SOURCES */
// CP
// https://eso-sets.com/set/grace-of-the-ancients
// https://eso-sets.com/set/bright-throats-boast
// Magical Expiration IA Verse
pub static MAGICKA_CONTROLLER: Buff = Buff { id: MAGICKA_CONTROLLER_ID, value: 0f64, value_per_stack: 2f64}; // Multiplicative
pub static MAGICKA_FLOOD: Buff = Buff {id: MAGICKA_FLOOD_ID, value: 6f64, value_per_stack: 0f64}; // Multiplicative
// Magicka Reserves IA Vision
// https://eso-sets.com/set/robes-of-destruction-mastery
// Horns
pub static SYRABANES_BOON: Buff = Buff { id: 117970, value: 2000f64, value_per_stack: 0f64};
pub static GIFT_OF_MAGNUS: Buff = Buff { id: 45260, value: 2000f64, value_per_stack: 0f64};
pub static EXPERT_SUMMONER_MAGICKA: Buff = Buff { id: EXPERT_SUMMONER_ID, value: 5f64, value_per_stack: 0f64}; // Multiplicative

pub static RESOURCE_MAGICKA_ADDITIVE: Map<u32, &'static Buff> = phf_map! {
    117848 => &LUNAR_BLESSINGS,
    32624 => &BLOOD_SCION,
    38932 => &SWARMING_SCION,
    38931 => &PERFECT_SCION,
    220010 => &XORYNS_MASTERPIECE,
    45280 => &IMPERIAL_METTLE,
    45267 => &DYNAMIC,
    45247 => &RESOURCEFUL,
    117970 => &SYRABANES_BOON,
    45260 => &GIFT_OF_MAGNUS,
};

pub static RESOURCE_MAGICKA_MULTIPLICATIVE: Map<u32, &'static Buff> = phf_map! {
    40224 => &AGGRESSIVE_HORN,
    55386 => &UNDAUNTED_METTLE,
    45199 => &EXPERT_SUMMONER_MAGICKA,
    45603 => &MAGICKA_CONTROLLER,
    45150 => &MAGICKA_FLOOD,
    // ??? => &STURDY_HORN
};