//! Re-exports of the shared effect vocabulary. The owned effect model (`EffectDef` and
//! friends, roadmap 1.5) lives in `esosim-types`; effect data lives in `data/effects/*.jsonl`
//! and is loaded through `data::bundle::Bundle`. The old `&'static Effect` / `FORTIFIED`
//! statics are gone.

pub use esosim_types::{
    AggKind, ArmourWeight, Bar, Bucket, Channel, ChannelValue, Condition, ConditionContext,
    DamageDoneContext, DamageTakenContext, EffectDef, EffectSource, NoContext, ResourceKind,
    Stacking, cca::CoreCombatAbility, damage::ResistableDamageType,
};
