//! Shared vocabulary for the ESO simulator: what a *stat channel* is.
//!
//! Everything here is plain data (`Copy`, `Eq`, `Hash`, serde). There is deliberately no
//! engine logic, no data tables and no file IO, so the data compiler can depend on this
//! crate alone and keep building while the engine is mid-refactor.

pub mod bucket;
pub mod cca;
pub mod channel;
pub mod condition;
pub mod cp;
pub mod damage;
pub mod effect_def;
pub mod mundus;
pub mod race;
pub mod set_behaviour;
pub mod skill_serde;
pub mod trigger;

pub use bucket::{Bucket, Cap, CappedStat};
pub use cca::CoreCombatAbility;
pub use channel::{AggKind, Channel, DamageDoneContext, DamageTakenContext, ResourceKind};
pub use condition::{
    Affliction, ArmourWeight, Bar, Condition, ConditionContext, Form, NoContext, PlayerClass, SelfState, TargetKind,
    WeaponKind,
};
pub use cp::{CP_SLOTS_PER_TREE, CpIssue, CpLoadout, CpPerk, CpStar};
pub use damage::ResistableDamageType;
pub use effect_def::{ChannelValue, EffectDef, EffectSource, Stacking};
pub use mundus::{BASE_MUNDUS_STONES, MUNDUS_STONES_WITH_EXTRA_SLOT, MundusIssue, MundusLoadout};
pub use race::Race;
pub use set_behaviour::{BehaviourClass, BlockBehaviour, SetBehaviour, SetKind};
pub use trigger::{CastKind, Trigger, TriggerEvent};

#[cfg(test)]
mod tests {
    use crate::channel::HealingDoneContext;

use super::*;
    use eso_skill_data::enums::{mechanic::Mechanic, skill_line::SkillLine};

    fn round_trip(c: Channel) {
        let json = serde_json::to_string(&c).expect("serialise");
        let back: Channel = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(c, back, "{json}");
    }

    #[test]
    fn plain_channels_round_trip() {
        round_trip(Channel::Armour(ResistableDamageType::Spell));
        round_trip(Channel::CapRaise(CappedStat::CriticalDamage));
        round_trip(Channel::Resource(ResourceKind::Health, AggKind::Multiplicative));
    }

    #[test]
    fn every_known_mechanic_and_skill_line_round_trips_inside_a_channel() {
        for id in 0u8..=u8::MAX {
            if let Some(m) = Mechanic::from_id(&id) {
                for kind in [AggKind::Additive, AggKind::Multiplicative] {
                    round_trip(Channel::CostReduction(m, kind));
                }
            }
        }
        for id in 0u32..4096 {
            if let Some(l) = SkillLine::from_id(&id) {
                for kind in [AggKind::Additive, AggKind::Multiplicative] {
                    round_trip(Channel::CostReductionSkillLine(l, kind));
                }
                round_trip(Channel::Power(DamageDoneContext::SkillLine(l), AggKind::Additive));
                round_trip(Channel::DamageDone(DamageDoneContext::SkillLine(l), AggKind::Multiplicative));
                round_trip(Channel::HealingDone(HealingDoneContext::SkillLine(l)));
            }
        }
    }

    #[test]
    fn bucket_follows_the_channel_payload() {
        assert_eq!(Channel::CriticalChance.bucket(), Bucket::Additive);
        assert_eq!(
            Channel::Resource(ResourceKind::Stamina, AggKind::Multiplicative).bucket(),
            Bucket::Multiplicative
        );
    }

    #[test]
    fn bucket_follows_the_payload_of_every_channel_that_has_one() {
        use crate::{CoreCombatAbility, DamageDoneContext, DamageTakenContext};
        for (kind, bucket) in [(AggKind::Additive, Bucket::Additive), (AggKind::Multiplicative, Bucket::Multiplicative)] {
            assert_eq!(Channel::Power(DamageDoneContext::All, kind).bucket(), bucket);
            assert_eq!(Channel::DamageDone(DamageDoneContext::Bash, kind).bucket(), bucket);
            assert_eq!(Channel::DamageTaken(DamageTakenContext::Monsters, kind).bucket(), bucket);
            assert_eq!(Channel::CostReductionCCA(CoreCombatAbility::Sneak, kind).bucket(), bucket);
            assert_eq!(Channel::Recovery(ResourceKind::Health, kind).bucket(), bucket);
            assert_eq!(Channel::SynergyRestore(ResourceKind::Magicka, kind).bucket(), bucket);
            // a percent cost reduction is a fraction of the cost, a flat one is cost points
            assert_eq!(Channel::CostReduction(Mechanic::Magicka, kind).bucket(), bucket);
            assert_eq!(Channel::CostReductionSkillLine(SkillLine::Support, kind).bucket(), bucket);
        }
        // the one payload-less channel that is a fraction: Divines pieces sum, every mundus stone scales by it
        assert_eq!(Channel::MundusBoost.bucket(), Bucket::Multiplicative);
        // no payload: always additive
        assert_eq!(Channel::MovementSpeed.bucket(), Bucket::Additive);
        assert_eq!(Channel::HealingDone(HealingDoneContext::All).bucket(), Bucket::Additive);
        // flat Ultimate points per tick (Heroism), not a fraction
        assert_eq!(Channel::UltimatePerTick.bucket(), Bucket::Additive);
    }

    #[test]
    fn caps_hold_a_stat_to_its_limit() {
        assert_eq!(CappedStat::CriticalChance.apply(170.0, 0), 100.0);
        assert_eq!(CappedStat::CriticalDamage.apply(500.0, 0), 125.0);
        assert_eq!(CappedStat::CriticalDamage.apply(-20.0, 0), 0.0);
        assert_eq!(CappedStat::Resistance.apply(60_000.0, 0), 33_000.0);
        assert_eq!(CappedStat::Resistance.apply(-1800.0, 0), 0.0); // no over-penetration: below 0 is 0
    }

    #[test]
    fn an_effect_can_raise_a_cap() {
        assert_eq!(CappedStat::CriticalDamage.apply(500.0, 10), 135.0);
        assert_eq!(CappedStat::Resistance.apply(60_000.0, 2_000), 35_000.0);
        assert_eq!(CappedStat::Resistance.apply(34_000.0, 2_000), 34_000.0);
        assert_eq!(Channel::CapRaise(CappedStat::Resistance).bucket(), Bucket::Additive);
    }
}
