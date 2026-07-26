use std::collections::HashMap;

use crate::data::enums::gear::{GearTrait, ItemType};

pub fn parse_item_types_into_hashmap() -> HashMap<u32, ItemType> {
    let mut item_type_table = HashMap::new();
    let data = include_str!("../item_types.csv");

    for line in data.lines() {
        let parts: Vec<&str> = line.split(',').collect();
        
        if parts.len() > 1 {
            let item_type = calculate_item_type(parts[0]);
            for &id_str in &parts[1..] {
                if let Ok(id) = id_str.parse::<u32>() {
                    item_type_table.insert(id, item_type);
                }
            }
        }
    }

    item_type_table
}

pub fn calculate_item_type(str: &str) -> ItemType {
    match str {
        "AXE" => ItemType::Axe,
        "DAGGER" => ItemType::Dagger,
        "MACE" => ItemType::Mace,
        "SWORD" => ItemType::Sword,
        "TWO_HANDED_AXE" => ItemType::TwoHandedAxe,
        "TWO_HANDED_MACE" => ItemType::TwoHandedMace,
        "TWO_HANDED_SWORD" => ItemType::TwoHandedSword,
        "FROST_STAFF" => ItemType::FrostStaff,
        "FIRE_STAFF" => ItemType::FireStaff,
        "LIGHTNING_STAFF" => ItemType::LightningStaff,
        "HEALING_STAFF" => ItemType::HealingStaff,
        "SHIELD" => ItemType::Shield,
        "BOW" => ItemType::Bow,
        "LIGHT" => ItemType::Light,
        "MEDIUM" => ItemType::Medium,
        "HEAVY" => ItemType::Heavy,
        "MARA" => ItemType::Mara,
        _ => ItemType::Unknown,
    }
}

pub fn is_weapon(item: &ItemType) -> bool {
    matches!(
        item,
        ItemType::Axe
            | ItemType::Dagger
            | ItemType::Mace
            | ItemType::Sword
            | ItemType::TwoHandedAxe
            | ItemType::TwoHandedMace
            | ItemType::TwoHandedSword
            | ItemType::FrostStaff
            | ItemType::FireStaff
            | ItemType::LightningStaff
            | ItemType::HealingStaff
            | ItemType::Bow
    )
}

pub fn is_two_handed_weapon(item: &ItemType) -> bool {
    matches!(
        item,
        ItemType::TwoHandedAxe
            | ItemType::TwoHandedMace
            | ItemType::TwoHandedSword
            | ItemType::Bow
            | ItemType::FrostStaff
            | ItemType::FireStaff
            | ItemType::LightningStaff
            | ItemType::HealingStaff
    )
}

pub fn is_two_handed_weapon_option(item: Option<&ItemType>) -> bool {
    if let Some(item_type) = item {
        matches!(
        item_type,
        ItemType::TwoHandedAxe
            | ItemType::TwoHandedMace
            | ItemType::TwoHandedSword
            | ItemType::Bow
            | ItemType::FrostStaff
            | ItemType::FireStaff
            | ItemType::LightningStaff
            | ItemType::HealingStaff
        )
    } else {
        false
    }
}

pub fn is_armour(item: &ItemType) -> bool {
    matches!(item, ItemType::Light | ItemType::Medium | ItemType::Heavy | ItemType::Shield)
}

/// Doubles the effectiveness of the trait when applied to a two-handed weapon compared to one handed.
pub fn weapon_trait_doubles(trait_: &GearTrait) -> bool {
    matches!(
        trait_,
        GearTrait::WeaponPowered
            | GearTrait::WeaponCharged
            | GearTrait::WeaponPrecise
            | GearTrait::WeaponDefending
            | GearTrait::WeaponTraining
            | GearTrait::WeaponSharpened
            | GearTrait::WeaponDecisive
    )
}