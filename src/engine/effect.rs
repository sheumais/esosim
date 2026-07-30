use crate::data::enums::{cca::CoreCombatAbility, damage::ResistableDamageType};
use eso_skill_data::enums::{mechanic::Mechanic, skill_line::SkillLine};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ResourceKind {
    Health,
    Magicka,
    Stamina,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AggKind {
    Additive,
    Multiplicative,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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
    DamageTakenFromPlayers,
    MovementSpeed,
    CostReductionCCA(CoreCombatAbility),
    CostReduction(Mechanic),
    CostReductionSkillLine(SkillLine),
    MundusBoost,
    StatusEffectChance,
    DecisiveUltimateChance,
    KillExperience,
    OffhandWeaponPowerBonus,
}

impl Channel {
    pub fn agg_kind(&self) -> AggKind {
        match self {
            Channel::Resource(_, kind) => *kind,
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