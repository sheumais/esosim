use crate::data::{enums::gear::{GearSlot, ItemQuality, ItemType}, tables::item_type::is_armour};

pub const DEFAULT_ARMOUR_LEVEL: u8 = 66;

pub fn armour_from_armour_piece(item_type: &ItemType, item_slot: &GearSlot, quality: &ItemQuality) -> Option<u16> {
    armour_from_armour_piece_at_level(item_type, item_slot, quality, &DEFAULT_ARMOUR_LEVEL)
}

fn is_valid_level(level: &u8) -> bool {
    match level {
        1 | 4   => true,
        6..=50  => level % 2 == 0,
        51..=66 => true,
        _       => false,
    }
}

fn level_constants(level: &u8) -> Option<(f32, f32)> {
    if !is_valid_level(level) { return None; }
    let l = *level as i32;
    let (base, wd_x2) = match level {
        1      => (61, 2 * 42),
        4      => (77, 2 * 54),
        6..=50 => (4 * l + 65, 2 * (3 * l + 45)),
        66     => (349, 2 * 258),
        51..=65 if l % 2 == 1 => (6 * l - 45, 9 * l - 75),
        _      => (6 * l - 43, 9 * l - 72),
    };
    Some((base as f32, wd_x2 as f32 / 2.0))
}

fn shield_base(level: &u8) -> Option<u16> {
    if !is_valid_level(level) { return None; }
    let l = *level as i32;
    let b = match level {
        1      => 40,
        4      => 120,
        6..=50 => 20 * l + 60,
        66     => 30 * l - 480,
        51..=65 if l % 2 == 1 => 30 * l - 470,
        _      => 30 * l - 460,
    };
    Some(b as u16)
}

pub fn armour_from_armour_piece_at_level(
    item_type: &ItemType,
    item_slot: &GearSlot,
    quality: &ItemQuality,
    level: &u8,
) -> Option<u16> {
    use ItemType as T;
    use ItemQuality as Q;
    use GearSlot as G;
    const PENALTY_BASE: u32 = 12;

    if !is_armour(item_type) {return None}

    let slot_coef: u32 = match item_slot {
        G::Chest        => 8,
        G::Head
        | G::Shoulders
        | G::Legs
        | G::Feet       => 7,
        G::Hands        => 4,
        G::Waist        => 3,

        G::OffHand
        | G::OffHandBackup
        if matches!(item_type, T::Shield) => {
            return shield_base(level).map(|b| b + *quality as u16 * 60);
        }
        _ => return None,
    };

    let (base_value, weight_diff) = level_constants(level)?;

    let weight_idx: f32 = match item_type {
        T::Light  => 0.,
        T::Medium => 1.,
        T::Heavy  => 2.,
        _ => return None,
    };

    let legendary = ((base_value * slot_coef as f32) / 2.0
           + weight_idx * (weight_diff * slot_coef as f32 / 3.0)) as u16;

    let (weight_num, weight_den): (u32, u32) = match item_type {
        T::Heavy  => (1, 1),
        T::Medium => (3, 4),
        T::Light  => (1, 2),
        _ => return None,
    };

    let low = level <= &50;
    let quality_num: u32 = match quality {
        Q::Legendary => return Some(legendary),
        Q::Epic      => 3,
        Q::Superior  => if low { 6 }  else { 5 },
        Q::Fine      => if low { 9 }  else { 8 },
        Q::Normal    => if low { 12 } else { 11 },
    };

    let penalty = (PENALTY_BASE * slot_coef * weight_num * quality_num + (weight_den * 3 / 2)) / (weight_den * 3);

    Some(legendary - penalty as u16)
}