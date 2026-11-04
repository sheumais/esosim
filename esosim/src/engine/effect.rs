use crate::data::enums::{cca::CoreCombatAbility, damage::ResistableDamageType};
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
pub enum Channel {
    Armour(ResistableDamageType),
    CriticalChance,
    CriticalDamage,
    /// Measured in 1% = 66 (see [CRIT_DAMAGE_SCALE][crate::stats::critical::CRIT_DAMAGE_SCALE])
    CriticalResistance,
    Power,
    Penetration(ResistableDamageType),
    Resource(ResourceKind, AggKind),
    Recovery(ResourceKind, AggKind),
    SynergyRestore(ResourceKind, AggKind),
    HealingDone,
    HealingTaken,
    MovementSpeed,
    CostReductionCCA(CoreCombatAbility),
    CostReduction(#[serde(with = "super::skill_serde::mechanic")] Mechanic),
    CostReductionSkillLine(#[serde(with = "super::skill_serde::skill_line")] SkillLine),
    MundusBoost,
    StatusEffectChance,
    DecisiveUltimateChance,
    KillExperience,
    OffhandWeaponPowerBonus,

    DamageDone(DamageDoneContext),
    DamageTaken(DamageTakenContext),
    DamageShieldStrength,
    BlockMitigation,
    CriticalDamageTaken,
    SpellCriticalChance,
    WeaponCriticalChance,
    WeaponPower,
    SpellPower,

    HealingDoneSingleTarget,
    HealingDoneOverTime,
    HealingDoneSkillLine(#[serde(with = "super::skill_serde::skill_line")] SkillLine),

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
}

impl Channel {
    pub fn agg_kind(&self) -> AggKind {
        match self {
            Channel::Resource(_, kind) => *kind,
            Channel::Recovery(_, kind) => *kind,
            Channel::SynergyRestore(_, kind) => *kind,
            _ => AggKind::Additive,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Effect {
    pub id: u32,
    pub name: &'static str,
    pub value: f64,
    pub value_per_stack: f64,
    pub channels: &'static [Channel],
}

pub static FORTIFIED: Effect = Effect {
    id: 142035,
    name: "Fortified",
    value: 0.0,
    value_per_stack: 34.62,
    channels: &[Channel::Armour(ResistableDamageType::All)],
};