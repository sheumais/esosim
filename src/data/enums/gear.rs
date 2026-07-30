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

#[derive(Debug, PartialEq, Clone, Copy)]
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

impl GearTrait {
    pub fn is_infused(&self) -> bool {
        match self {
            Self::ArmorInfused | Self::WeaponInfused | Self::JewelryInfused => true,
            _ => false,
        }
    }
}


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub enum ItemQuality {
    Normal,
    Fine,
    Superior,
    Epic,
    Legendary,
}

impl ItemQuality {
    pub fn find_higher_quality(&self, other: Self) -> ItemQuality {
        (*self).max(other)
    }

    pub fn find_lower_quality(&self, other: Self) -> ItemQuality {
        (*self).min(other)
    }
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

impl GearSlot {
    pub fn is_offhand(&self) -> bool {
        match self {
            GearSlot::OffHand | GearSlot::OffHandBackup => true,
            _ => false,
        }
    }
}

// Note that enchants are only bound to specific gear pieces by the rules of what can be applied in the game. This is why they are listed here as one enum.
// In fact, bugged items exist on live servers from trials that have armour enchantments on weapons, such as a +Max Stamina Lightning Staff
// If such an item exists but with traits instead of enchants then please let me know (however I doubt such an item exists).
#[derive(Debug, PartialEq, Clone, Copy)]
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
    FlameResistance,
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

pub enum EnchantLevel {
    One,
    Five,
    Ten,
    Fifteen,
    Twenty,
    TwentyFive,
    Thirty,
    ThirtyFive,
    Fourty,
    CPTen,
    CPThirty,
    CPFifty,
    CPSeventy,
    CPOneHundred,
    CPOneFifty,
    CPOneSixty,
}