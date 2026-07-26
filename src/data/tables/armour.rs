use crate::data::{enums::gear::{GearSlot, ItemQuality, ItemType}, tables::item_type::is_armour};

pub fn armour_from_armour_piece(item_type: &ItemType, item_slot: &GearSlot, quality: &ItemQuality) -> Option<u16> {
    use ItemType as T;
    use ItemQuality as Q;
    use GearSlot as G;
    const BASE_VALUE: f32 = 349f32;
    const WEIGHT_DIFF: f32 = 258f32;
    const PENALTY_BASE: u16 = 12;

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
            return Some(1500 + *quality as u16 * 60);
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