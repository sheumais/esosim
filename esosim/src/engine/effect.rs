pub use esosim_types::{
    AggKind, Channel, DamageDoneContext, DamageTakenContext, ResourceKind,
    cca::CoreCombatAbility, damage::ResistableDamageType,
};

// `Channel` and friends now live in `esosim-types` (roadmap 1.1); they are re-exported here so
// existing `crate::engine::effect::*` paths keep working until 1.5 replaces `Effect`.

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