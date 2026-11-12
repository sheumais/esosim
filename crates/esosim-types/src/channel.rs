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
    /// Cost reduction of one resource: additive is flat cost points (a glyph's 203), multiplicative a
    /// *fraction* of the cost (0.10 = "reduces the cost of your Magicka abilities by 10%").
    CostReduction(#[serde(with = "crate::skill_serde::mechanic")] Mechanic, AggKind),
    /// As [`Channel::CostReduction`], for the abilities of one skill line (Hawk's Eye: Bow).
    CostReductionSkillLine(#[serde(with = "crate::skill_serde::skill_line")] SkillLine, AggKind),
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

    HealingDone(HealingDoneContext),

    HeavyAttackRestore(ResourceKind),
    UltimateGain,
    UltimatePerTick,
    AttackSpeed,
    AbilityCooldown,
    /// Milliseconds taken off the potion cooldown (Glyph of Potion Speed: 5 seconds is 5000).
    PotionCooldown,
    /// Milliseconds added to the duration of potion effects (Glyph of Potion Boost: "Increase the
    /// duration of Potion effects by 3.6 seconds" is 3600).
    PotionEffectiveness,

    DodgeChance,
    MissChance,
    StealthDetection,
    DetectionRadius,
    MountSpeed,
    ResurrectionSpeed,
    StatusEffectDuration,
}
