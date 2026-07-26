use crate::data::enums::gear::{ItemQuality, ItemType};

pub const OFFHAND_MULTIPLIER: f32 = 0.1765;

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