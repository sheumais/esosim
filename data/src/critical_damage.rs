use phf::{Map, phf_map};

use crate::{StatBuff as Buff, major_minor::{BRITTLE_MAJOR_ID, BRITTLE_MINOR_ID, ENERVATION_MINOR_ID, FORCE_MAJOR_ID, FORCE_MINOR_ID}};

pub static CRITICAL_DAMAGE_DONE_BY_ID: Map<u32, &'static Buff> = phf_map! {
    13984 => &THE_SHADOW,
    44046 => &PIERCING_SPEAR,
    45060 => &HEMORRHAGE_PASSIVE,
    45301 => &FELINE_AMBUSH,
    45430 => &HEAVY_WEAPONS,
    45482 => &TWIN_BLADE_AND_BLUNT_AXE,
    45564 => &DEXTERITY,
    61746 => &FORCE_MINOR,
    61747 => &FORCE_MAJOR,
    79113 => &ENERVATION_MINOR,
    86069 => &ADVANCED_SPECIES,
    154737 => &SUL_XANS_TORMENT,
    155150 => &HARPOONERS_WADING_KILT,
    194875 => &FATED_FORTUNE,
    220015 => &LUCENT_ECHOES,
    220315 => &MORA_SCRIBES_THESIS, // todo
    // order's wrath // todo
    // true-sworn fury // maybe?
    // senches bite // probably not
};

pub static CRITICAL_DAMAGE_TAKEN_BY_ID: Map<u32, &'static Buff> = phf_map! {
    142610 => &FLAME_WEAKNESS,
    142652 => &FROST_WEAKNESS,
    142653 => &SHOCK_WEAKNESS,
    145975 => &BRITTLE_MINOR,
    145977 => &BRITTLE_MAJOR,
};

pub static FORCE_MINOR: Buff = Buff { id: FORCE_MINOR_ID, value: 10f64, value_per_stack: 0f64};
pub static FORCE_MAJOR: Buff = Buff { id: FORCE_MAJOR_ID, value: 20f64, value_per_stack: 0f64};
pub static ENERVATION_MINOR: Buff = Buff { id: ENERVATION_MINOR_ID, value: -5f64, value_per_stack: 0f64};
pub static BRITTLE_MINOR: Buff = Buff { id: BRITTLE_MINOR_ID, value: 10f64, value_per_stack: 0f64};
pub static BRITTLE_MAJOR: Buff = Buff { id: BRITTLE_MAJOR_ID, value: 20f64, value_per_stack: 0f64};

pub const THE_SHADOW_ID: u32 = 13984;
/// Each stack is a piece of CP160 Gold Divines gear, which rounds to an additional 1% crit damage per piece. This relationship does not hold for lower quality and level items. See https://en.uesp.net/wiki/Online:The_Shadow_(Mundus_Stone)
pub static THE_SHADOW: Buff = Buff { id: THE_SHADOW_ID, value: 11f64, value_per_stack: 1f64};

pub const FLAME_WEAKNESS_ID: u32 = 142610;
pub static FLAME_WEAKNESS: Buff = Buff { id: FLAME_WEAKNESS_ID, value: 5f64, value_per_stack: 0f64};

pub const FROST_WEAKNESS_ID: u32 = 142652;
pub static FROST_WEAKNESS: Buff = Buff { id: FROST_WEAKNESS_ID, value: 5f64, value_per_stack: 0f64};

pub const SHOCK_WEAKNESS_ID: u32 = 142653;
pub static SHOCK_WEAKNESS: Buff = Buff { id: SHOCK_WEAKNESS_ID, value: 5f64, value_per_stack: 0f64};

pub const HARPOONERS_WADING_KILT_ID: u32 = 155150;
pub static HARPOONERS_WADING_KILT: Buff = Buff { id: HARPOONERS_WADING_KILT_ID, value: 0f64, value_per_stack: 2f64};

pub const SUL_XAN_SOULBOUND: u32 = 154737;
pub static SUL_XANS_TORMENT: Buff = Buff { id: SUL_XAN_SOULBOUND, value: 12f64, value_per_stack: 0f64};

pub const DEXTERITY_ID: u32 = 45564;
/// Only shows up for the person logging
pub static DEXTERITY: Buff = Buff { id: DEXTERITY_ID, value: 0f64, value_per_stack: 2f64};

pub const FELINE_AMBUSH_ID: u32 = 45301;
pub static FELINE_AMBUSH: Buff = Buff { id: FELINE_AMBUSH_ID, value: 12f64, value_per_stack: 0f64};

pub const HEMORRHAGE_PASSIVE_ID: u32 = 45060;
pub static HEMORRHAGE_PASSIVE: Buff = Buff { id: HEMORRHAGE_PASSIVE_ID, value: 10f64, value_per_stack: 0f64};

pub const PIERCING_SPEAR_ID: u32 = 44046;
pub static PIERCING_SPEAR: Buff = Buff { id: PIERCING_SPEAR_ID, value: 10f64, value_per_stack: 0f64};

pub const TWIN_BLADE_AND_BLUNT_ID: u32 = 45482;
pub static TWIN_BLADE_AND_BLUNT_AXE: Buff = Buff { id: TWIN_BLADE_AND_BLUNT_ID, value: 0f64, value_per_stack: 6f64};

pub const HEAVY_WEAPONS_ID: u32 = 45430;
pub static HEAVY_WEAPONS: Buff = Buff { id: HEAVY_WEAPONS_ID, value: 0f64, value_per_stack: 12f64};

pub const ADVANCED_SPECIES_ID: u32 = 86069;
pub static ADVANCED_SPECIES: Buff = Buff { id: ADVANCED_SPECIES_ID, value: 0f64, value_per_stack: 5f64};

pub const FATED_FORTUNE_ID: u32 = 194875;
pub static FATED_FORTUNE: Buff = Buff { id: FATED_FORTUNE_ID, value: 12f64, value_per_stack: 0f64};

pub const LUCENT_ECHOES_ID: u32 = 220015;
pub static LUCENT_ECHOES: Buff = Buff { id: LUCENT_ECHOES_ID, value: 11f64, value_per_stack: 0f64};

pub const MORA_SCRIBES_THESIS_ID: u32 = 220315;
pub static MORA_SCRIBES_THESIS: Buff = Buff { id: MORA_SCRIBES_THESIS_ID, value: 0f64, value_per_stack: 1f64};

// pub const MALACATHS_BAND_OF_BRUTALITY_ID: u32 = ;
// pub static MALACATHS_BAND_OF_BRUTALITY: Buff = Buff {
//     id: ,
//     value: -0.5,
//     value_per_stack: 0,
// };