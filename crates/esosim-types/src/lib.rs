//! Shared vocabulary for the ESO simulator: what a *stat channel* is.
//!
//! Everything here is plain data (`Copy`, `Eq`, `Hash`, serde). There is deliberately no
//! engine logic, no data tables and no file IO, so the data compiler can depend on this
//! crate alone and keep building while the engine is mid-refactor.

pub mod cca;
pub mod channel;
pub mod condition;
pub mod cp;
pub mod damage;
pub mod effect_def;
pub mod mundus;
pub mod set_behaviour;
pub mod skill_serde;
pub mod trigger;

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
pub use set_behaviour::{BehaviourClass, BlockBehaviour, SetBehaviour, SetKind};
pub use trigger::{CastKind, Trigger, TriggerEvent};

#[cfg(test)]
mod tests {
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
        round_trip(Channel::Resource(ResourceKind::Health, AggKind::Multiplicative));
    }

    #[test]
    fn every_known_mechanic_and_skill_line_round_trips_inside_a_channel() {
        for id in 0u8..=u8::MAX {
            if let Some(m) = Mechanic::from_id(&id) {
                round_trip(Channel::CostReduction(m));
            }
        }
        for id in 0u32..4096 {
            if let Some(l) = SkillLine::from_id(&id) {
                round_trip(Channel::CostReductionSkillLine(l));
                round_trip(Channel::HealingDoneSkillLine(l));
            }
        }
    }

    #[test]
    fn agg_kind_follows_the_channel_payload() {
        assert_eq!(Channel::CriticalChance.agg_kind(), AggKind::Additive);
        assert_eq!(
            Channel::Resource(ResourceKind::Stamina, AggKind::Multiplicative).agg_kind(),
            AggKind::Multiplicative
        );
    }

    #[test]
    fn agg_kind_follows_the_payload_of_every_channel_that_has_one() {
        use crate::{CoreCombatAbility, DamageDoneContext, DamageTakenContext};
        let kinds = [AggKind::Additive, AggKind::Multiplicative];
        for kind in kinds {
            assert_eq!(Channel::Power(DamageDoneContext::All, kind).agg_kind(), kind);
            assert_eq!(Channel::DamageDone(DamageDoneContext::Bash, kind).agg_kind(), kind);
            assert_eq!(Channel::DamageTaken(DamageTakenContext::Monsters, kind).agg_kind(), kind);
            assert_eq!(Channel::CostReductionCCA(CoreCombatAbility::Sneak, kind).agg_kind(), kind);
            assert_eq!(Channel::Recovery(ResourceKind::Health, kind).agg_kind(), kind);
        }
        // no payload: always additive
        assert_eq!(Channel::MovementSpeed.agg_kind(), AggKind::Additive);
        assert_eq!(Channel::HealingDone.agg_kind(), AggKind::Additive);
    }
}
