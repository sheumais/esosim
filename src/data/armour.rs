use phf::{Map, phf_map};

use crate::data::{StatBuff as Buff, critical_damage::TWIN_BLADE_AND_BLUNT_ID, item_type::{GearSlot, ItemQuality, ItemType, is_armour}, major_minor::*, skill::*};

const BASE_VALUE: f32 = 349f32;
const WEIGHT_DIFF: f32 = 258f32;
const PENALTY_BASE: u16 = 12;

pub fn armour_from_armour_piece(item_type: &ItemType, item_slot: &GearSlot, quality: &ItemQuality) -> Option<u16> {
    use ItemType as T;
    use ItemQuality as Q;
    use GearSlot as G;

    if !is_armour(item_type) {return None}

    let slot_coef = match item_slot {
        G::Chest        => 8,
        G::Head
        | G::Shoulders
        | G::Legs
        | G::Feet       => 7,
        G::Hands        => 4,
        G::Waist        => 3,

        // Shields
        G::OffHand
        | G::OffHandBackup
        if matches!(item_type, T::Shield) => {
            let base = match quality { // 1500 + quality * 60
                Q::Legendary => 1720,
                Q::Epic      => 1660,
                Q::Superior  => 1620,
                Q::Fine      => 1560,
                Q::Normal    => 1500,
            };
            return Some(base);
        }
        _ => return None,
    };

    let weight_idx: f32 = match item_type {
        T::Light  => 0.,
        T::Medium => 1.,
        T::Heavy  => 2.,
        _ => return None,
    };

    let legendary = ((BASE_VALUE  * slot_coef as f32) / 2.0
           + weight_idx * (WEIGHT_DIFF * slot_coef as f32  / 3.0)) as u16;

    let (weight_num, weight_den) = match item_type {
        T::Heavy  => (1, 1),
        T::Medium => (3, 4),
        T::Light  => (1, 2),
        _ => return None,
    };

    let quality_num  = match quality {
        Q::Legendary => return Some(legendary),
        Q::Epic      => 3,
        Q::Superior  => 5,
        Q::Fine      => 8,
        Q::Normal    => 11,
    };

    let penalty = (PENALTY_BASE * slot_coef * weight_num * quality_num + (weight_den * 3 / 2)) / (weight_den * 3);

    Some(legendary - penalty)
}

// Both
pub static MAJOR_RESOLVE: Buff = Buff { id: RESOLVE_MAJOR_ID, value: 5948f64, value_per_stack: 0f64};
pub static MINOR_RESOLVE: Buff = Buff { id: RESOLVE_MINOR_ID, value: 2974f64, value_per_stack: 0f64};
pub static FROZEN_ARMOUR: Buff = Buff { id: FROZEN_ARMOUR_ID, value: 0f64, value_per_stack: 1240f64}; // Winter's Embrace passive
pub static RESOLVE: Buff = Buff { id: 45533, value: 0f64, value_per_stack: 343.2}; // Heavy armour passive
pub static RUGGED: Buff = Buff { id: 45306, value: 2600f64, value_per_stack: 0f64}; // Nord passive
pub static HEART_OF_STONE: Buff = Buff { id: HEART_OF_STONE_ID, value: 2974f64, value_per_stack: 0f64}; // Draconic Power passive
/// Only shows up for the person logging.
pub static BULWARK: Buff = Buff { id: 64079, value: 1900f64, value_per_stack: 0f64}; // Blue CP.
/// Doesn't show on logs
pub static FORTIFIED: Buff = Buff { id: 142035, value: 0f64, value_per_stack: 34.62}; // Red CP.
pub static OZEZANS_PLATING: Buff = Buff { id: 188471, value: 4272f64, value_per_stack: 0.0};

// Decrease
pub static MAJOR_BREACH: Buff = Buff { id: BREACH_MAJOR_ID, value: -5948f64, value_per_stack: 0f64};
pub static MINOR_BREACH: Buff = Buff { id: BREACH_MINOR_ID, value: -2974f64, value_per_stack: 0f64};
pub static CRUSHER: Buff = Buff { id: 17906, value: -2108f64, value_per_stack: 0f64}; // Assumes infused. todo

pub static ARMOUR_ALL_BY_ID: Map<u32, &'static Buff> = phf_map! {
    61694 => &MAJOR_RESOLVE,
    61693 => &MINOR_RESOLVE,
    86190 => &FROZEN_ARMOUR,
    45533 => &RESOLVE,
    45306 => &RUGGED,
    44996 => &HEART_OF_STONE,
    61743 => &MAJOR_BREACH,
    61742 => &MINOR_BREACH,
    17906 => &CRUSHER,
    64079 => &BULWARK,
    188471 => &OZEZANS_PLATING,
};

// Spell Resistance Only
pub static SISLEAS_DEFENSE: Buff = Buff { id: 139698, value: 15000f64, value_per_stack: 0f64}; // Kyne's Aegis prisoner buff
pub static SPELL_ATTUNEMENT: Buff = Buff { id: 45262, value: 0f64, value_per_stack: 2310f64}; // Breton Passive. Doubles if afflicted by burning, chilled or concussed.
pub static SPELL_RESIST_POTION: Buff = Buff { id: 64562, value: 5280f64, value_per_stack: 0f64};
pub static SPELL_WARDING: Buff = Buff { id: 45559, value: 0f64, value_per_stack: 726f64}; // Light Armour Passive

pub static SPELL_RESISTANCE_BY_ID: Map<u32, &'static Buff> = phf_map! {
    139698 => &SISLEAS_DEFENSE,
    45262 => &SPELL_ATTUNEMENT,
    64562 => &SPELL_RESIST_POTION,
    45559 => &SPELL_WARDING,
};

// Physical Resistance Only
pub static PHYSICAL_RESIST_POTION: Buff = Buff { id: 64564, value: 5280f64, value_per_stack: 0f64};

pub static PHYSICAL_RESISTANCE_BY_ID: Map<u32, &'static Buff> = phf_map! {
    64564 => &PHYSICAL_RESIST_POTION,
};

// Specific Type of Damage
pub static RESIST_FROST: Buff = Buff { id: 45304, value: 4620f64, value_per_stack: 0f64}; // frost, nord
pub static ARGONIAN_RESISTANCE: Buff = Buff { id: 45255, value: 2310f64, value_per_stack: 0f64}; // poison + disease, argonian
pub static RESIST_AFFLICTION: Buff = Buff { id: 45319, value: 2310f64, value_per_stack: 0f64}; // poison + disease, wood elf

pub static FROST_RESISTANCE_BY_ID: Map<u32, &'static Buff> = phf_map! {
    45304 => &RESIST_FROST,
};

pub static POISON_DISEASE_RESISTANCE_BY_ID: Map<u32, &'static Buff> = phf_map! {
    45255 => &ARGONIAN_RESISTANCE,
    45319 => &RESIST_AFFLICTION,
};


// TODO:
// https://eso-sets.com/set/ancient-dragonguard
// https://eso-sets.com/set/armor-master
// 2H ult
// https://eso-sets.com/set/bloodspawn
// Defensive Scroll Bonuses
// https://eso-sets.com/set/dunerippers-scales
// https://eso-sets.com/set/embershield
// https://eso-sets.com/set/grave-guardian
// https://eso-sets.com/set/jolting-arms
// https://eso-sets.com/set/lord-warden
// https://eso-sets.com/set/mark-of-the-pariah
// https://eso-sets.com/set/meritorious-service
// https://eso-sets.com/set/orgnums-scales
// https://eso-sets.com/set/renalds-resolve
// https://eso-sets.com/set/senche-rahts-grit
// https://eso-sets.com/set/roar-of-alkosh
// Balanced Warrior passive (aedric spear)




// Balorgh
// Pen CP
pub static CONCENTRATION: Buff = Buff { id: 45562, value: 0f64, value_per_stack: 939f64};
pub static DISMEMBER: Buff = Buff { id: DISMEMBER_ID, value: 3271f64, value_per_stack: 0f64};
pub static FORCE_OF_NATURE: Buff = Buff { id: 174250, value: 0f64, value_per_stack: 660f64};
pub static HEAVY_WEAPONS: Buff = Buff { id: 45430, value: 2974f64, value_per_stack: 0f64};
// Hew and Sunder set
pub static HUNTERS_EYE: Buff = Buff { id: 45296, value: 950f64, value_per_stack: 0f64};
// Oakfather's Retribution
// 2H Ultimate
// Piercing Perfection vision
// Shell Splitter
pub static SPLINTERED_SECRETS: Buff = Buff { id: SPLINTERED_SECRETS_ID, value: 0f64, value_per_stack: 1240f64};
// Stuhn's Favour
// Titanborn
// Twice-fanged Serpent
pub static TWIN_BLADE_AND_BLUNT_MACE: Buff = Buff { id: TWIN_BLADE_AND_BLUNT_ID, value: 0f64, value_per_stack: 1487f64};
pub static PIERCING: Buff = Buff { id: 141895, value: 0.0, value_per_stack: 350f64};

pub static PENETRATION_ADDITIVE: Map<u32, &'static Buff> = phf_map! {
    45562 => &CONCENTRATION,
    116194 => &DISMEMBER,
    174250 => &FORCE_OF_NATURE,
    45430 => &HEAVY_WEAPONS,
    45296 => &HUNTERS_EYE,
    184887 => &SPLINTERED_SECRETS,
    45482 => &TWIN_BLADE_AND_BLUNT_MACE,
    141895 => &PIERCING,
};