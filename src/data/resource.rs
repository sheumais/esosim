use phf::{Map, phf_map};

use crate::data::{StatBuff as Buff, major_minor::*, skill::*};

// *todo, all the fucking foods. like a hundred foods. ugh.

/* ALL */
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
pub static TIRELESS_DISCIPLINE: Buff = Buff { id: 147888, value: 0f64, value_per_stack: 260f64};
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
    147888 => &TIRELESS_DISCIPLINE,
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
pub static HEROS_VIGOR: Buff = Buff { id: 149311, value: 0f64, value_per_stack: 280f64};
// Black Drake Villa buff ( Ice Avatar )
pub static JUGGERNAUT: Buff = Buff { id: 45546, value: 0f64, value_per_stack: 2f64}; // Multiplicative
pub static LAST_GASP: Buff = Buff { id: LAST_GASP_ID, value: 2412f64, value_per_stack: 0f64};
pub static MINOR_TOUGHNESS: Buff = Buff { id: TOUGHNESS_MINOR_ID, value: 10f64, value_per_stack: 0f64}; // Multiplicative
pub static BONE_GOLIATH_TRANSFORMATION: Buff = Buff { id: BONE_GOLIATH_TRANSFORMATION_ID, value: 30_000f64, value_per_stack: 0f64};
pub static RESIST_FROST: Buff = Buff { id: 45304, value: 1000f64, value_per_stack: 0f64};
pub static ARGONIAN_RESISTANCE: Buff = Buff { id: 45255, value: 1000f64, value_per_stack: 0f64};
pub static TOUGH: Buff = Buff { id: 50907, value: 2000f64, value_per_stack: 0f64};
pub static UNFLINCHING_RAGE: Buff = Buff { id: 84672, value: 1000f64, value_per_stack: 0f64};

// Ayleid well bonus

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
    149311 => &HEROS_VIGOR,
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
// Inner light has unique 2%
pub static MAGICKA_CONTROLLER: Buff = Buff { id: MAGICKA_CONTROLLER_ID, value: 0f64, value_per_stack: 2f64}; // Multiplicative
pub static MAGICKA_FLOOD: Buff = Buff {id: MAGICKA_FLOOD_ID, value: 6f64, value_per_stack: 0f64}; // Multiplicative
// Magicka Reserves IA Vision
// https://eso-sets.com/set/robes-of-destruction-mastery
// Horns
pub static SYRABANES_BOON: Buff = Buff { id: 117970, value: 2000f64, value_per_stack: 0f64};
pub static GIFT_OF_MAGNUS: Buff = Buff { id: 45260, value: 2000f64, value_per_stack: 0f64};
pub static ELDRITCH_INSIGHT: Buff = Buff { id: 149305, value: 0f64, value_per_stack: 260f64};
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
    149305 => &ELDRITCH_INSIGHT,
};

pub static RESOURCE_MAGICKA_MULTIPLICATIVE: Map<u32, &'static Buff> = phf_map! {
    40224 => &AGGRESSIVE_HORN,
    55386 => &UNDAUNTED_METTLE,
    45199 => &EXPERT_SUMMONER_MAGICKA,
    45603 => &MAGICKA_CONTROLLER,
    45150 => &MAGICKA_FLOOD,
    // ??? => &STURDY_HORN
};

pub static FOOD_BUFFS: phf::Map<u32, &'static FoodBuff> = phf_map! {
    86673  => &LAVA_FOOT_SOUP_AND_SOULTRICE,
    72824  => &ORZORGAS_SMOKED_BEAR_HAUNCH,
    61257  => &BISTAT_MAG,
    61255  => &BISTAT_STAM,
    127596 => &BEWITCHED_SUGAR_SKULLS,
    107789 => &ARTAEUM_TAKEAWAY_BROTH,
    100498 => &CLOCKWORK_CITRUS_FILET,
    84720  => &GHASTLY_EYE_BOWL,
    89971  => &JEWELS_OF_MISRULE,
    107748 => &ARTAEUM_PICKLED_FISH_BOWL,
    127572 => &PACK_LEADERS_BONE_BROTH,
    127531 => &CORRUPTING_BLOODY_MARA,
    84731  => &WITCHMOTHERS_POTENT_BREW,
    89957  => &DUBIOUS_CAMORAN_THRONE,
    89973  => &ORZORGAS_RED_FROTHGAR,
    61261  => &PARSE_STAMINA,
    61260  => &PARSE_MAGICKA,
    89955  => &CANDIED_JESTERS_COINS,
};

pub struct FoodBuff {
    pub id: u32,
    pub max_health: Option<u32>,
    pub max_magicka: Option<u32>,
    pub max_stamina: Option<u32>,
    pub health_recovery: Option<u32>,
    pub magicka_recovery: Option<u32>,
    pub stamina_recovery: Option<u32>,
}

impl FoodBuff {
    pub fn is_valid_source(id: &u32) -> bool {
        FOOD_BUFFS.get(id).is_some()
    }
}

pub static LAVA_FOOT_SOUP_AND_SOULTRICE: FoodBuff = FoodBuff { 
    id: 86673, max_health: None, max_magicka: None, max_stamina: Some(4936), health_recovery: None, magicka_recovery: None, stamina_recovery: Some(493)
};

pub static ORZORGAS_SMOKED_BEAR_HAUNCH: FoodBuff = FoodBuff { 
    id: 72824, max_health: Some(4312), max_magicka: None, max_stamina: None, health_recovery: Some(406), magicka_recovery: Some(369), stamina_recovery: Some(369)
};

pub static BISTAT_MAG: FoodBuff = FoodBuff { 
    id: 61257, max_health: Some(5395), max_magicka: Some(4936), max_stamina: None, health_recovery: None, magicka_recovery: None, stamina_recovery: None
};

pub static BISTAT_STAM: FoodBuff = FoodBuff { 
    id: 61255, max_health: Some(5395), max_magicka: None, max_stamina: Some(4936), health_recovery: None, magicka_recovery: None, stamina_recovery: None
};

pub static BEWITCHED_SUGAR_SKULLS: FoodBuff = FoodBuff {
    id: 127596, max_health: Some(4620), max_magicka: Some(4250), max_stamina: Some(4250), health_recovery: Some(462), magicka_recovery: None, stamina_recovery: None,
};

pub static ARTAEUM_TAKEAWAY_BROTH: FoodBuff = FoodBuff {
    id: 107789, max_health: Some(3326), max_magicka: None, max_stamina: Some(3080), health_recovery: Some(406), magicka_recovery: None, stamina_recovery: Some(338),
};

pub static CLOCKWORK_CITRUS_FILET: FoodBuff = FoodBuff {
    id: 100498, max_health: Some(3326), max_magicka: Some(3080), max_stamina: None, health_recovery: Some(406), magicka_recovery: Some(338), stamina_recovery: None,
};

pub static GHASTLY_EYE_BOWL: FoodBuff = FoodBuff {
    id: 84720, max_health: None, max_magicka: Some(4592), max_stamina: None, health_recovery: None, magicka_recovery: Some(459), stamina_recovery: None,
};

pub static JEWELS_OF_MISRULE: FoodBuff = FoodBuff {
    id: 89971, max_health: Some(3927), max_magicka: None, max_stamina: None, health_recovery: None, magicka_recovery: Some(357), stamina_recovery: Some(357),
};

pub static ARTAEUM_PICKLED_FISH_BOWL: FoodBuff = FoodBuff {
    id: 107748, max_health: Some(5652), max_magicka: Some(5117), max_stamina: None, health_recovery: None, magicka_recovery: None, stamina_recovery: None,
};

pub static PACK_LEADERS_BONE_BROTH: FoodBuff = FoodBuff {
    id: 127572, max_health: Some(5051), max_magicka: None, max_stamina: Some(4620), health_recovery: None, magicka_recovery: None, stamina_recovery: None,
};

pub static CORRUPTING_BLOODY_MARA: FoodBuff = FoodBuff {
    id: 127531, max_health: Some(5051), max_magicka: Some(4620), max_stamina: None, health_recovery: Some(505), magicka_recovery: None, stamina_recovery: None,
};

pub static WITCHMOTHERS_POTENT_BREW: FoodBuff = FoodBuff {
    id: 84731, max_health: Some(3094), max_magicka: Some(2856), max_stamina: None, health_recovery: None, magicka_recovery: Some(315), stamina_recovery: None,
};

pub static DUBIOUS_CAMORAN_THRONE: FoodBuff = FoodBuff {
    id: 89957, max_health: Some(3094), max_magicka: None, max_stamina: Some(2856), health_recovery: None, magicka_recovery: None, stamina_recovery: Some(315),
};

pub static ORZORGAS_RED_FROTHGAR: FoodBuff = FoodBuff {
    id: 89973, max_health: Some(5395), max_magicka: None, max_stamina: None, health_recovery: None, magicka_recovery: Some(493), stamina_recovery: None,
};

pub static PARSE_STAMINA: FoodBuff = FoodBuff {
    id: 61261, max_health: None, max_magicka: None, max_stamina: Some(6048), health_recovery: None, magicka_recovery: None, stamina_recovery: None,
};

pub static PARSE_MAGICKA: FoodBuff = FoodBuff {
    id: 61260, max_health: None, max_magicka: Some(6048), max_stamina: None, health_recovery: None, magicka_recovery: None, stamina_recovery: None,
};

pub static CANDIED_JESTERS_COINS: FoodBuff = FoodBuff {
    id: 89955, max_health: None, max_magicka: None, max_stamina: Some(4592), health_recovery: None, magicka_recovery: Some(459), stamina_recovery: None,
};


// Health
// 28x + 122y = z (0 <= x <= 50, 0 <= y <= 64)
// 28(x+4y) = z
// z/28 = x+4y

// Mag & Stam
// 26x + 111y = z (0 <= x <= 50, 0 <= y <= 64)
// 2*13x + 3*37y = z

// /// Returns (CP, ATTRIBUTE)
// pub fn lookup_resource(value: usize) -> Option<(u8, u8)> {
//     if value > resource_lookup::MAX_Z {
//         return None;
//     }
//     resource_lookup::RESOURCE_LOOKUP[value].get(0).copied()
// }

// /// Value is divided by 28 from real value
// /// Returns CP
// pub fn lookup_health(value: usize, attributes: u8) -> Option<u8> {
//     if value > health_lookup::MAX_V {
//         return None;
//     }

//     for &(row_y, x) in health_lookup::HEALTH_LOOKUP[value] {
//         if row_y == attributes {
//             return Some(x);
//         }
//     }
//     None
// }

// /// Returns CP
// pub fn lookup_health_from_raw(value: usize, attributes: u8) -> Option<u8> {
//     lookup_health(value / 28, attributes)
// }