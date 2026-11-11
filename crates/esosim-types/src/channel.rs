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
    Type(ResistableDamageType),
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
    HealingDone,
    HealingTaken,
    CriticalHealing,
    MovementSpeed,
    CostReductionCCA(CoreCombatAbility, AggKind),
    CostReduction(#[serde(with = "crate::skill_serde::mechanic")] Mechanic),
    CostReductionSkillLine(#[serde(with = "crate::skill_serde::skill_line")] SkillLine),
    MundusBoost,
    /// Raises the cap of a derived stat (armour cap, critical damage cap), in that stat's unit: see
    /// [`CappedStat`].
    CapRaise(CappedStat),
    StatusEffectChance(StatusEffectContext),
    DecisiveUltimateChance,
    KillExperience,
    OffhandWeaponPowerBonus,

    DamageDone(DamageDoneContext, AggKind),
    DamageTaken(DamageTakenContext, AggKind),
    DamageShieldStrength,
    BlockMitigation,
    CriticalDamageTaken,
    SpellCriticalChance,
    WeaponCriticalChance,
    WeaponPower,
    SpellPower,

    HealingDoneSingleTarget,
    HealingDoneOverTime,
    HealingDoneSkillLine(#[serde(with = "crate::skill_serde::skill_line")] SkillLine),

    HeavyAttackRestore(ResourceKind),
    UltimateGain,
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
}
