//! Shared vocabulary for the ESO simulator: what a *stat channel* is.
//!
//! Everything here is plain data (`Copy`, `Eq`, `Hash`, serde). There is deliberately no
//! engine logic, no data tables and no file IO, so the data compiler can depend on this
//! crate alone and keep building while the engine is mid-refactor.

pub mod cca;
pub mod channel;
pub mod condition;
pub mod damage;
pub mod effect_def;
pub mod set_behaviour;
pub mod skill_serde;

pub use cca::CoreCombatAbility;
pub use channel::{AggKind, Channel, DamageDoneContext, DamageTakenContext, ResourceKind};
pub use condition::{ArmourWeight, Bar, Condition, ConditionContext, NoContext};
pub use damage::ResistableDamageType;
pub use effect_def::{ChannelValue, EffectDef, EffectSource, Stacking};
pub use set_behaviour::{BehaviourClass, BlockBehaviour, SetBehaviour, SetKind};

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
        round_trip(Channel::DamageDone(DamageDoneContext::Type(ResistableDamageType::Flame)));
        round_trip(Channel::DamageTaken(DamageTakenContext::AreaOfEffect));
        round_trip(Channel::CostReductionCCA(CoreCombatAbility::Block));
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
}
