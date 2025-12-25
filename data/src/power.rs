use phf::{Map, phf_map};

use crate::{StatBuff as Buff, item_type::{ItemQuality, ItemType}, major_minor::{COURAGE_MAJOR_ID, COURAGE_MINOR_ID, SORCERY_MAJOR_ID, SORCERY_MINOR_ID}, skill::{BALANCED_WARRIOR_ID, EXPERT_MAGE_ID, HARNESSED_QUINTESSENCE_ID, SLAYER_ID}};

/// Assumes CP160
pub fn power_from_weapon_type(item_type: &ItemType, quality: &ItemQuality) -> Option<u16> {
    use ItemType as T;
    use ItemQuality as Q;
    match item_type {
        T::Axe | T::Dagger | T::Mace | T::Sword | T::FrostStaff | T::FireStaff | T::LightningStaff | T::HealingStaff | T::Bow => 
            Some(match quality {
                Q::Normal => 1037,
                Q::Fine => 1072,
                Q::Superior => 1108,
                Q::Epic => 1132,
                Q::Legendary => 1335,
            }),
        T::TwoHandedAxe | T::TwoHandedMace | T::TwoHandedSword => 
            Some(match quality {
                Q::Normal => 1220,
                Q::Fine => 1262,
                Q::Superior => 1304,
                Q::Epic => 1332,
                Q::Legendary => 1571,
            }),
        _ => None,
    }
}

pub const OFFHAND_MULTIPLIER: f32 = 0.1765;

pub static COURAGE_MINOR: Buff = Buff { id: COURAGE_MINOR_ID, value: 215, value_per_stack: 0 };
pub static COURAGE_MAJOR: Buff = Buff { id: COURAGE_MAJOR_ID, value: 430, value_per_stack: 0 };

pub static RUINATION: Buff = Buff { id: 45272, value: 258, value_per_stack: 0};
pub static SUNDERER: Buff = Buff { id: 215727, value: 100, value_per_stack: 0};
pub static BERSERKER_INFUSED: Buff = Buff { id: 21230, value: 452, value_per_stack: 0};
pub static POWERFUL_ASSAULT: Buff = Buff { id: 61771, value: 307, value_per_stack: 0};
pub static AURA_OF_PRIDE: Buff = Buff { id: 163401, value: 260, value_per_stack: 0};
pub static PEARLESCENT_WARD: Buff = Buff { id: 172621, value: 180, value_per_stack: 0}; // should scale based on group members alive; also should be given when wearing PW set because it doesn't show up on logs for the person wearing the set
// pub static CORAL_RIPTIDE
// pub static GLYPHIC: Buff = Buff {}; // scales in proportion to health, todo
pub static SEETHING_FURY: Buff = Buff { id: 122729, value: 0, value_per_stack: 100};
// pub static CLIFF_RACER: Buff = Buff { id: , value: 100, value_per_stack: 0};
pub static EXPERT_MAGE: Buff = Buff { id: EXPERT_MAGE_ID, value: 0, value_per_stack: 108};
pub static HARNESSED_QUINTESSENCE: Buff = Buff { id: HARNESSED_QUINTESSENCE_ID, value: 284, value_per_stack: 0};

pub static SORCERY_MINOR: Buff = Buff { id: SORCERY_MINOR_ID, value: 10, value_per_stack: 0};
pub static SORCERY_MAJOR: Buff = Buff { id: SORCERY_MAJOR_ID, value: 20, value_per_stack: 0};
pub static AGILITY: Buff = Buff { id: 45572, value: 0, value_per_stack: 2};
pub static SLAYER: Buff = Buff { id: SLAYER_ID, value: 0, value_per_stack: 3};
pub static BALANCED_WARRIOR: Buff = Buff { id: BALANCED_WARRIOR_ID, value: 6, value_per_stack: 0};
pub static SAVAGE_STRENGTH: Buff = Buff { id: 46139, value: 18, value_per_stack: 0};

pub static POWER_INCREASES_ADDITIVE: Map<u32, &'static Buff> = phf_map! {
    147417 => &COURAGE_MINOR,
    109966 => &COURAGE_MAJOR,
    45272 => &RUINATION,
    215727 => &SUNDERER,
    21230 => &BERSERKER_INFUSED,
    61771 => &POWERFUL_ASSAULT,
    45195 => &EXPERT_MAGE,
    163401 => &AURA_OF_PRIDE,
    172621 => &PEARLESCENT_WARD,
    122729 => &SEETHING_FURY,
    184860 => &HARNESSED_QUINTESSENCE,
};

pub static POWER_INCREASES_MULTIPLICATIVE: Map<u32, &'static Buff> = phf_map! {
    61685 => &SORCERY_MINOR,
    61687 => &SORCERY_MAJOR,
    45572 => &AGILITY,
    45596 => &SLAYER,
    44732 => &BALANCED_WARRIOR,
    46139 => &SAVAGE_STRENGTH,
};
