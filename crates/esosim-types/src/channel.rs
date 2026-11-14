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
    Channeled, // Denoted by flag 4 = 1 on an ability
    /// Abilities of one skill line ("your Two Handed abilities", "your Bow abilities"). Written as the
    /// line's name, like the skill-line channels below.
    SkillLine(#[serde(with = "crate::skill_serde::skill_line")] SkillLine),
    Type(ResistableDamageType),
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
    Power(DamageDoneContext, AggKind),
    Penetration(ResistableDamageType),
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
    StatusEffectChance(StatusEffectContext),
    DecisiveUltimateChance,
    KillExperience,
    OffhandWeaponPowerBonus,

    DamageDone(DamageDoneContext, AggKind),
    DamageTaken(DamageTakenContext, AggKind),
    DamageShieldStrength,
    BlockMitigation,
    CriticalDamageTaken,

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
    StatusEffectDuration,
    SwimmingSpeed,
    GoldGain,
    AlliancePointGain,
    InspirationGain,
    SneakEntryTime,
    DurationSkillLine(#[serde(with = "crate::skill_serde::skill_line")] SkillLine),
}
