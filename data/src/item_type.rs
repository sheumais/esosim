use std::collections::HashMap;
use lazy_static::lazy_static;

lazy_static! {
    // Item type from game using https://github.com/sheumais/ItemTypeDataExtractTool
    pub static ref ITEM_TYPES: HashMap<u32, ItemType> = parse_item_types_into_hashmap();
}

pub fn parse_item_types_into_hashmap() -> HashMap<u32, ItemType> {
    let mut item_type_table = HashMap::new();
    let data = include_str!("item_types.csv");

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

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ItemType {
    Axe,
    Dagger,
    Mace,
    Sword,
    TwoHandedAxe,
    TwoHandedMace,
    TwoHandedSword,
    FrostStaff,
    FireStaff,
    LightningStaff,
    HealingStaff,
    Shield,
    Bow,
    Light,
    Medium,
    Heavy,
    Mara,
    Unknown,
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

pub enum ItemQuality {
    Normal,
    Fine,
    Superior,
    Epic,
    Legendary,
}