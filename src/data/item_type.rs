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

#[derive(Debug, PartialEq)]
pub enum ItemQuality {
    Normal,
    Fine,
    Superior,
    Epic,
    Legendary,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum GearSlot {
    Head,
    Shoulders,
    Chest,
    Hands,
    Waist,
    Legs,
    Feet,
    Necklace,
    Ring1,
    Ring2,
    MainHand,
    MainHandBackup,
    Poison,
    OffHand,
    OffHandBackup,
    BackupPoison,
}

#[derive(Debug, PartialEq)]
pub enum GearTrait {
    JewelryBloodthirsty,
    JewelryHarmony,
    JewelryProtective,
    JewelrySwift,
    JewelryTriune,
    JewelryInfused,
    JewelryArcane,
    JewelryRobust,
    JewelryHealthy,
    JewelryIntricate,
    JewelryOrnate,

    ArmorSturdy,
    ArmorImpenetrable,
    ArmorReinforced,
    ArmorWellFitted,
    ArmorDivines,
    ArmorNirnhoned,
    ArmorInfused,
    ArmorTraining,
    ArmorInvigorating,
    ArmorIntricate,
    ArmorOrnate,

    WeaponInfused,
    WeaponNirnhoned,
    WeaponCharged,
    WeaponDecisive,
    WeaponDefending,
    WeaponPowered,
    WeaponPrecise,
    WeaponSharpened,
    WeaponTraining,
    WeaponIntricate,
    WeaponOrnate,
}

// Note that enchants are only bound to specific gear pieces by the rules of what can be applied in the game. This is why they are listed here as one enum.
// In fact, bugged items exist on live servers from trials that have armour enchantments on weapons, such as a +Max Stamina Lightning Staff
// If such an item exists but with traits instead of enchants then please let me know.
#[derive(Debug, PartialEq)]
pub enum EnchantType {
    AbsorbHealth,
    AbsorbMagicka,
    AbsorbStamina,
    BefouledWeapon,
    Beserker,
    ChargedWeapon,
    DamageShield,
    DiseaseResistance,
    FieryWeapon,
    FireResistance,
    FrostResistance,
    FrozenWeapon,
    Health,
    HealthRegen,
    IncreaseBashDamage,
    IncreasePhysicalDamage,
    IncreasePotionEffectiveness,
    IncreaseSpellDamage,
    Magicka,
    MagickaRegen,
    OblivionDamage,
    PhysicalResistance,
    PoisonedWeapon,
    PoisonResistance,
    PrismaticDefense,
    PrismaticOnslaught,
    PrismaticRecovery,
    ReduceArmor,
    ReduceBlockAndBash,
    ReduceFeatCost,
    ReducePotionCooldown,
    ReducePower,
    ReduceSpellCost,
    ShockResistance,
    SpellResistance,
    Stamina,
    StaminaRegen,
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