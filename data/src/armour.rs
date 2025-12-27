use phf::{Map, phf_map};

use crate::{StatBuff as Buff, item_type::{GearSlot, ItemQuality, ItemType, is_armour}, major_minor::{BREACH_MAJOR_ID, BREACH_MINOR_ID, RESOLVE_MAJOR_ID, RESOLVE_MINOR_ID}, skill::{FROZEN_ARMOUR_ID, SCALED_ARMOUR_ID}};

pub fn armour_from_armour_piece(item_type: &ItemType, item_slot: &GearSlot, quality: &ItemQuality) -> Option<u16> {
    use ItemType as T;
    use ItemQuality as Q;
    use GearSlot as G;

    if !is_armour(item_type) {return None}

    Some(match item_type {
        T::Light => match item_slot {
            G::Chest => match quality {
                Q::Normal => 1220,
                Q::Fine => 1268,
                Q::Superior => 1316,
                Q::Epic => 1348,
                Q::Legendary => 1396,
            },
            G::Head | G::Shoulders | G::Legs | G::Feet => match quality {
                Q::Normal => 1067,
                Q::Fine => 1109,
                Q::Superior => 1151,
                Q::Epic => 1179,
                Q::Legendary => 1221,
            },
            G::Hands => match quality {
                Q::Normal => 610,
                Q::Fine => 634,
                Q::Superior => 658,
                Q::Epic => 674,
                Q::Legendary => 698,
            },
            G::Waist => match quality {
                Q::Normal => 457,
                Q::Fine => 475,
                Q::Superior => 493,
                Q::Epic => 505,
                Q::Legendary => 523,
            },
            _ => return None,
        },
        T::Medium => match item_slot {
            G::Chest => match quality {
                Q::Normal => 1820,
                Q::Fine => 1892,
                Q::Superior => 1964,
                Q::Epic => 2012,
                Q::Legendary => 2084,
            },
            G::Head | G::Shoulders | G::Legs | G::Feet => match quality {
                Q::Normal => 1592,
                Q::Fine => 1655,
                Q::Superior => 1718,
                Q::Epic => 1760,
                Q::Legendary => 1823,
            },
            G::Hands => match quality {
                Q::Normal => 910,
                Q::Fine => 946,
                Q::Superior => 982,
                Q::Epic => 1006,
                Q::Legendary => 1042,
            },
            G::Waist => match quality {
                Q::Normal => 682,
                Q::Fine => 709,
                Q::Superior => 736,
                Q::Epic => 754,
                Q::Legendary => 781,
            },
            _ => return None,
        },
        T::Heavy => match item_slot {
            G::Chest => match quality {
                Q::Normal => 2420,
                Q::Fine => 2516,
                Q::Superior => 2612,
                Q::Epic => 2676,
                Q::Legendary => 2772,
            },
            G::Head | G::Shoulders | G::Legs | G::Feet => match quality {
                Q::Normal => 2117,
                Q::Fine => 2201,
                Q::Superior => 2285,
                Q::Epic => 2341,
                Q::Legendary => 2425,
            },
            G::Hands => match quality {
                Q::Normal => 1210,
                Q::Fine => 1258,
                Q::Superior => 1306,
                Q::Epic => 1338,
                Q::Legendary => 1386,
            },
            G::Waist => match quality {
                Q::Normal => 907,
                Q::Fine => 943,
                Q::Superior => 979,
                Q::Epic => 1003,
                Q::Legendary => 1039,
            },
            _ => return None,
        },
        T::Shield => match item_slot {
            G::OffHand | G::OffHandBackup => match quality {
                Q::Normal => 1500,
                Q::Fine => 1560,
                Q::Superior => 1620,
                Q::Epic => 1660,
                Q::Legendary => 1720,
            },
            _ => return None,
        },
        _ => return None,
    })
}

// Both
pub static MAJOR_RESOLVE: Buff = Buff { id: RESOLVE_MAJOR_ID, value: 5948.0, value_per_stack: 0.0};
pub static MINOR_RESOLVE: Buff = Buff { id: RESOLVE_MINOR_ID, value: 2974.0, value_per_stack: 0.0};
pub static FROZEN_ARMOUR: Buff = Buff { id: FROZEN_ARMOUR_ID, value: 0.0, value_per_stack: 1240.0}; // Winter's Embrace passive
pub static RESOLVE: Buff = Buff { id: 45533, value: 0.0, value_per_stack: 343.2}; // Heavy armour passive
pub static RUGGED: Buff = Buff { id: 45306, value: 2600.0, value_per_stack: 0.0}; // Nord passive
pub static SCALED_ARMOUR: Buff = Buff { id: SCALED_ARMOUR_ID, value: 2974.0, value_per_stack: 0.0}; // Draconic Power passive

// Decrease
pub static MAJOR_BREACH: Buff = Buff { id: BREACH_MAJOR_ID, value: -5948.0, value_per_stack: 0.0};
pub static MINOR_BREACH: Buff = Buff { id: BREACH_MINOR_ID, value: -2974.0, value_per_stack: 0.0};
pub static CRUSHER: Buff = Buff { id: 17906, value: -2108.0, value_per_stack: 0.0}; // Assumes infused. todo

pub static ARMOUR_ALL_BY_ID: Map<u32, &'static Buff> = phf_map! {
    61694 => &MAJOR_RESOLVE,
    61693 => &MINOR_RESOLVE,
    86190 => &FROZEN_ARMOUR,
    45533 => &RESOLVE,
    45306 => &RUGGED,
    44953 => &SCALED_ARMOUR,
    61743 => &MAJOR_BREACH,
    61742 => &MINOR_BREACH,
    17906 => &CRUSHER,
};

// Spell Resistance Only
pub static SISLEAS_DEFENSE: Buff = Buff { id: 139698, value: 15000.0, value_per_stack: 0.0}; // Kyne's Aegis prisoner buff
pub static SPELL_ATTUNEMENT: Buff = Buff { id: 45262, value: 0.0, value_per_stack: 2310.0}; // Breton Passive. Doubles if afflicted by burning, chilled or concussed.
pub static SPELL_RESIST_POTION: Buff = Buff { id: 64562, value: 5280.0, value_per_stack: 0.0};
pub static SPELL_WARDING: Buff = Buff { id: 45559, value: 0.0, value_per_stack: 726.0}; // Light Armour Passive

pub static SPELL_RESISTANCE_BY_ID: Map<u32, &'static Buff> = phf_map! {
    139698 => &SISLEAS_DEFENSE,
    45262 => &SPELL_ATTUNEMENT,
    64562 => &SPELL_RESIST_POTION,
    45559 => &SPELL_WARDING,
};

// Physical Resistance Only
pub static PHYSICAL_RESIST_POTION: Buff = Buff { id: 64564, value: 5280.0, value_per_stack: 0.0};

pub static PHYSICAL_RESISTANCE_BY_ID: Map<u32, &'static Buff> = phf_map! {
    64564 => &PHYSICAL_RESIST_POTION,
};

// Specific Type of Damage
pub static RESIST_FROST: Buff = Buff { id: 45304, value: 4620.0, value_per_stack: 0.0}; // frost, nord
pub static ARGONIAN_RESISTANCE: Buff = Buff { id: 45255, value: 2310.0, value_per_stack: 0.0}; // poison + disease, argonian
pub static RESIST_AFFLICTION: Buff = Buff { id: 45319, value: 2310.0, value_per_stack: 0.0}; // poison + disease, wood elf

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