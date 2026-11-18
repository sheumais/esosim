use crate::bucket::CappedStat;
use crate::cca::CoreCombatAbility;
use crate::damage::ResistableDamageType;
use eso_skill_data::enums::{mechanic::Mechanic, skill_line::SkillLine};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ResourceKind {
    Health,
    Magicka,
    Stamina,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AggKind {
    Additive,
    Multiplicative,
}

/// Which kinds of outgoing damage a bonus applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageDoneContext {
    All,
    Direct,
    DamageOverTime,
    StatusEffects,
    AreaOfEffect,
    SingleTarget,
    LightAttack,
    HeavyAttack,
    ClassAbilities,
    Monsters,
    NonCritical,
    Bash,
    Channeled,
    StatusEffect(StatusEffect),
    Abilities,
    SkillLine(#[serde(with = "crate::skill_serde::skill_line")] SkillLine),
    Mechanic(#[serde(with = "crate::skill_serde::mechanic")] Mechanic),
    Type(ResistableDamageType),
}

/// Where a Weapon/Spell Damage bonus applies. Power is one stat in the game: `All` is the stat itself
/// (the character sheet's number, counted for every damage and healing event). `Damage` and `Healing` are
/// extra power that only counts while dealing that kind of damage or healing (Wrathful Strikes: damaging
/// abilities only), so they never show on the sheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PowerContext {
    All,
    Damage(DamageDoneContext),
    Healing(HealingDoneContext),
}

/// Which kinds of outgoing healing a bonus applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum HealingDoneContext {
    All,
    OverTime,
    AreaOfEffect,
    SingleTarget,
    ClassAbilities,
    Critical,
    NonCritical,
    Channeled,
    SkillLine(#[serde(with = "crate::skill_serde::skill_line")] SkillLine),
}

/// Which kinds of incoming damage a bonus applies to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DamageTakenContext {
    All,
    AreaOfEffect,
    DamageOverTime,
    Projectile,
    Monsters,
    Players,
    Siege,
    Fall,
    Lava,
    Type(ResistableDamageType),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusEffectContext {
    All,
    Martial,
    Magical,
    Type(StatusEffect),
    /// Status effects applied by the abilities of one skill line. Written as the line's name.
    SkillLine(#[serde(with = "crate::skill_serde::skill_line")] SkillLine),
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StatusEffect {
    Burning,
    Chilled,
    Concussion,
    Poisoned,
    Diseased,
    Hemorrhaging,
    Sundered,
    Overcharged,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Channel {
    Armour(ResistableDamageType),
    CriticalChance,
    CriticalDamage,
    /// Measured in 1% = 66
    CriticalResistance,
    Power(PowerContext, AggKind),
    Penetration(DamageDoneContext),
    Resource(ResourceKind, AggKind),
    Recovery(ResourceKind, AggKind),
    SynergyRestore(ResourceKind, AggKind),
    HealingTaken,
    CriticalHealing,
    MovementSpeed,
    CostReductionCCA(CoreCombatAbility, AggKind),
    CostReduction(#[serde(with = "crate::skill_serde::mechanic")] Mechanic, AggKind),
    CostReductionSkillLine(#[serde(with = "crate::skill_serde::skill_line")] SkillLine, AggKind),
    MundusBoost,
    CapRaise(CappedStat),
    /// In % points. Ability's base chance is dependent on damage category:
    /// Weapon Enchants                 20%
    /// Single Target Direct Damage     10%
    /// AoE Direct Damage               5%
    /// Single Target DoT (per tick)    3%
    /// AoE DoT (per tick, ground DoTs tick every 1s)   1%
    /// Light Attacks, Heavy Attacks, Status Effects    0%
    /// This stat increases it relatively, e.g. 100% chance makes weapon enchants 40%.
    StatusEffectChance(StatusEffectContext),
    DecisiveUltimateChance,
    KillExperience,
    OffhandWeaponPowerBonus,

    DamageDone(DamageDoneContext, AggKind),
    DamageTaken(DamageTakenContext, AggKind),
    DamageShieldStrength,
    BlockMitigation,
    CriticalDamageTaken,
    /// "Increasing the potency of your damage done by X" (Cycle Unending, Inexorable Descent): a fraction
    /// (0.25 = 25 %). Sources add up (`1 + Σ`) and the sum multiplies the ability's own value (coefficient ×
    /// stats) *before* any flat bonus: it is not a `DamageDone` percentage. Stored as basis points like every
    /// other fraction.
    DamagePotency,

    HealingDone(HealingDoneContext),

    HeavyAttackRestore(ResourceKind),
    UltimateGain,
    UltimatePerTick,
    AttackSpeed,
    AbilityCooldown,
    PotionCooldown,
    PotionEffectiveness,

    DodgeChance,
    MissChance,
    StealthDetection,
    DetectionRadius,
    MountSpeed,
    ResurrectionSpeed,
    StatusEffectDuration(AggKind),
    SwimmingSpeed,
    GoldGain,
    AlliancePointGain,
    InspirationGain,
    SneakEntryTime,
    DurationSkillLine(#[serde(with = "crate::skill_serde::skill_line")] SkillLine),

    SprintSpeed,
    BlockMitigationAgainst(DamageTakenContext),
    StatusEffectDamage(StatusEffectContext),
    CriticalChanceHealing,
}