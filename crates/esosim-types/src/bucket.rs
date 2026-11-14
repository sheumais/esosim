//! How a channel's values are combined (the *bucket*) and which stats have a hard cap
//! (roadmap 1.5). Both are tables, so a new channel or cap is one line here.

use serde::{Deserialize, Serialize};

use crate::channel::{AggKind, Channel};

/// The rule the aggregator uses to combine the values of one channel.
///
/// Units follow the bucket: an `Additive` value is a flat amount (or percentage points for channels
/// such as `CriticalDamage`), a `Multiplicative` value is a *fraction* (0.03 = 3 %) that the
/// aggregator turns into basis points.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Bucket {
    Additive,
    Multiplicative,
}

impl From<AggKind> for Bucket {
    fn from(kind: AggKind) -> Self {
        match kind {
            AggKind::Additive => Bucket::Additive,
            AggKind::Multiplicative => Bucket::Multiplicative,
        }
    }
}

impl Channel {
    /// The bucket a value of this channel goes into. Every channel is listed, with no wildcard, so
    /// adding a channel does not compile until someone decides its bucket.
    ///
    /// A channel that carries an [`AggKind`] says so itself; the rest are fixed here.
    pub fn bucket(&self) -> Bucket {
        use Channel::*;
        match self {
            Resource(_, kind)
            | Recovery(_, kind)
            | SynergyRestore(_, kind)
            | Power(_, kind)
            | DamageDone(_, kind)
            | DamageTaken(_, kind)
            | CostReduction(_, kind)
            | CostReductionSkillLine(_, kind)
            | CostReductionCCA(_, kind) => (*kind).into(),
            // a fraction (0.091 = 9.1 %), summed over Divines pieces and scaling every mundus stone
            MundusBoost => Bucket::Multiplicative,
            CapRaise(_)
            | Armour(_)
            | CriticalChance
            | CriticalDamage
            | CriticalResistance
            | Penetration(_)
            | HealingDone(_)
            | HealingTaken
            | CriticalHealing
            | MovementSpeed
            | StatusEffectChance(_)
            | DecisiveUltimateChance
            | KillExperience
            | OffhandWeaponPowerBonus
            | DamageShieldStrength
            | BlockMitigation
            | CriticalDamageTaken
            | HeavyAttackRestore(_)
            | UltimateGain
            | UltimatePerTick
            | AttackSpeed
            | AbilityCooldown
            | PotionCooldown
            | PotionEffectiveness
            | DodgeChance
            | MissChance
            | StealthDetection
            | DetectionRadius
            | MountSpeed
            | ResurrectionSpeed
            | StatusEffectDuration
            | SwimmingSpeed
            | GoldGain
            | AlliancePointGain
            | InspirationGain
            | SneakEntryTime
            | DurationSkillLine(_) => Bucket::Additive,
        }
    }
}

/// An inclusive range a derived stat is held to.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cap {
    pub min: f64,
    pub max: f64,
}

impl Cap {
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn apply(self, value: f64) -> f64 {
        value.clamp(self.min, self.max)
    }
}

/// A derived stat the game holds to a limit. The limit is on the stat as the player sees it (base
/// included), not on one channel, because several channels feed each of these.
///
/// The limit is not fixed: effects raise it through [`Channel::CapRaise`], which is summed in the
/// stat's own unit (below) and added to the base cap. Whoever derives the stat reads that channel and
/// passes it to [`CappedStat::apply`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CappedStat {
    /// Critical chance in percent (100 = 100 %).
    CriticalChance,
    /// Critical damage in percent, the 50 % base included.
    CriticalDamage,
    /// Critical healing in percent, the 50 % base included.
    CriticalHealing,
    /// Resistance, in resistance points: the most that counts towards mitigation.
    Resistance,
}

impl CappedStat {
    /// The cap before any effect raises it, in the stat's own unit. Only the upper limit is ever
    /// raised; none of these stats goes below 0 (there is no over-penetration).
    pub const fn base_cap(self) -> Cap {
        match self {
            CappedStat::CriticalChance => Cap::new(0.0, 100.0),
            CappedStat::CriticalDamage => Cap::new(0.0, 125.0),
            CappedStat::CriticalHealing => Cap::new(0.0, 125.0),
            CappedStat::Resistance => Cap::new(0.0, 33_000.0),
        }
    }

    /// The cap with `raise` (the summed `Channel::CapRaise(self)`) added to its upper limit.
    pub fn cap(self, raise: i64) -> Cap {
        let base = self.base_cap();
        Cap::new(base.min, base.max + raise as f64)
    }

    /// `value` held to the cap, which `raise` has lifted.
    pub fn apply(self, value: f64, raise: i64) -> f64 {
        self.cap(raise).apply(value)
    }
}
