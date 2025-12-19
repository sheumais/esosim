use crate::item_type::{ItemQuality, ItemType};

pub const SET_LINE_POWER: u32 = 129; 

/// Assumes CP160
pub fn power_from_weapon_type(item_type: ItemType, quality: Option<ItemQuality>) -> Option<u16> {
    use ItemType as T;
    use ItemQuality as Q;
    match item_type {
        T::Axe | T::Dagger | T::Mace | T::Sword | T::FrostStaff | T::FireStaff | T::LightningStaff | T::HealingStaff | T::Bow => 
            Some(match quality.unwrap_or(Q::Legendary) {
                Q::Normal => 1037,
                Q::Fine => 1072,
                Q::Superior => 1108,
                Q::Epic => 1132,
                Q::Legendary => 1335,
            }),
        T::TwoHandedAxe | T::TwoHandedMace | T::TwoHandedSword => 
            Some(match quality.unwrap_or(Q::Legendary) {
                Q::Normal => 1220,
                Q::Fine => 1262,
                Q::Superior => 1304,
                Q::Epic => 1332,
                Q::Legendary => 1571,
            }),
        _ => None,
    }
}

pub const RUINATION: u32 = 258; // Dark Elf Passive
pub const DUAL_WIELD_EXPERT: f32 = 0.06;
pub const COURAGE_MINOR: u32 = 215;
pub const COURAGE_MAJOR: u32 = 430;

