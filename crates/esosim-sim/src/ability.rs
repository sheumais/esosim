//! 5.2 — the ability model: what the simulator needs to know to *cast* something.
//!
//! # Where the data comes from
//!
//! One line of `esosim-data/data/bundle.jsonl` is a `ResolvedAbility`, but that type lives in the
//! data compiler (`tools/esosim-data-compiler/src/resolved.rs`), which the simulator must not depend
//! on. [`AbilityDef`] is therefore the simulator's own *read-only view* of such a line: the same
//! field names, only the fields the simulator reads, every one `#[serde(default)]`, unknown fields
//! ignored. When `resolved.rs` moves into a shared bundle crate (its own header says it can), replace
//! [`AbilityDef`] with it. `bundle.jsonl` is not compiled into `esosim-data` (8 MB), so
//! [`AbilityBook::from_jsonl`] takes the text from the caller.
//!
//! # Two levels
//!
//! The game splits one skill over several records. A *castable* skill (Toppling Charge, id 15540,
//! `ability_type` 72) has the cost, the cast time and the skill line; the record that actually deals
//! damage (15544, `ability_type` 1) has the `coef` rows, the damage type and the tick; the castable
//! one reaches it through a `Causes` / `ActionList` edge. So:
//!
//! * [`AbilityDef`]: one record, as the data has it. The damage formula reads these (`HitSpec`).
//! * [`Ability`]: one castable skill with its records already followed ([`AbilityBook::resolve`]).
//!   The time engine and rotations work with these.

use std::collections::HashMap;

use eso_skill_data::enums::{
    ability_tag::AbilityTag, ability_type::AbilityType, damage_type::DamageType, mechanic::Mechanic,
    skill_line::SkillLine,
};
use esosim::data::bundle::Bundle;
use esosim::engine::aggregator::ChannelSet;
use esosim_types::TriggerEvent;
use serde::Deserialize;

/// One weighted-sum row: `max(c1·t1 + c2·t2, c3·t3 + c4·t4)`. `types` are the game's coefficient
/// type ids (`eso_skill_data::enums::coefficient_type::CoefficientType::from_id`: 4 Maximum Magicka,
/// 7 Maximum Health, 8 Health Recovery, 13 Spell Resistance, 22 Physical Resistance, 25 Spell
/// Damage, 29 Maximum Stamina, 35 Weapon Damage; 0 = unused). `u20` is carried raw until its meaning
/// is known.
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct CoefRow {
    pub types: [u8; 4],
    pub coefs: [f32; 4],
    #[serde(default)]
    pub u20: u32,
}

/// "Deals up to `bonus_up_to_pct`% more damage to enemies below `threshold_below_health_pct`% Health"
/// (`ResolvedExecute`; tag `ExecuteScalingAbility`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
pub struct ExecuteScaling {
    pub threshold_below_health_pct: u8,
    pub bonus_up_to_pct: u16,
}

/// A raw `list26` entry (`ResolvedTrigger`). `u2[0]` is the game's trigger kind; `event` is what
/// `TriggerEvent::from_list26_kind` makes of it (absent for always-on and pool/range modifiers).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TriggerRow {
    pub u2: Vec<u32>,
    #[serde(default)]
    pub list3: Vec<u32>,
    #[serde(default)]
    pub event: Option<TriggerEvent>,
}

/// The edge kinds the simulator follows. The data has more (`Tooltip`, `CausedBy`, `Replaces`, the
/// unnamed `Field*` ones); they all read as `Other`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    /// The castable skill -> the record that does something when it is cast.
    Causes,
    ActionList,
    GrantsSynergy,
    /// Ability -> the record that holds its cooldown / an internal cooldown (roadmap 5.4).
    SetsCooldown,
    RemovesBuff,
    /// Morph -> base.
    BaseAbility,
    #[serde(other)]
    Other,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Edge {
    pub kind: EdgeKind,
    #[serde(default)]
    pub link_type: Option<u8>,
    pub to: u32,
}

/// One ability record, as `bundle.jsonl` has it (see the module docs).
#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct AbilityDef {
    pub id: u32,
    pub name: String,
    #[serde(default)]
    pub ability_type: u8,
    #[serde(default)]
    pub mechanic: u8,
    /// Skill line id; 0 for a record that is in no line (damage records usually are not: use the castable skill's).
    #[serde(default)]
    pub skill_line: u32,
    #[serde(default)]
    pub damage_type: u16,
    #[serde(default)]
    pub tags: Vec<u16>,
    /// `(index, value)` for every non-zero entry of the flag array (`eso_skill_data::enums::flags`).
    #[serde(default)]
    pub flags: Vec<(u8, u8)>,
    #[serde(default)]
    pub target_type: u16,
    #[serde(default)]
    pub duration_ms: u32,
    #[serde(default)]
    pub tick_ms: u32,
    #[serde(default)]
    pub cast_time_ms: u32,
    /// Base cost in the mechanic's resource, before any reduction.
    #[serde(default)]
    pub cost: u16,
    #[serde(default)]
    pub range: u32,
    #[serde(default)]
    pub radius: u32,
    #[serde(default)]
    pub target_cap: u16,
    #[serde(default)]
    pub coef: Vec<CoefRow>,
    #[serde(default)]
    pub execute: Option<ExecuteScaling>,
    #[serde(default)]
    pub triggers: Vec<TriggerRow>,
    #[serde(default)]
    pub edges: Vec<Edge>,
    #[serde(default)]
    pub morph_of: Option<u32>,
}

impl AbilityDef {
    /// The tags the parser knows (`AbilityTag`); ids it does not know are skipped.
    pub fn ability_tags(&self) -> Vec<AbilityTag> {
        unimplemented!()
    }

    pub fn has_tag(&self, tag: AbilityTag) -> bool {
        unimplemented!()
    }

    pub fn ability_type_enum(&self) -> Option<AbilityType> {
        unimplemented!()
    }

    pub fn damage_type_enum(&self) -> Option<DamageType> {
        unimplemented!()
    }

    pub fn mechanic_enum(&self) -> Option<Mechanic> {
        unimplemented!()
    }

    /// The value of one flag (`eso_skill_data::enums::flags::FLAG_*`), 0 when unset.
    pub fn flag(&self, index: usize) -> u8 {
        unimplemented!()
    }

    /// Ids this record points at with an edge of `kind`.
    pub fn linked(&self, kind: EdgeKind) -> Vec<u32> {
        unimplemented!()
    }

    /// Deals damage: `AbilityType::Damage` with at least one coefficient row.
    pub fn is_damaging(&self) -> bool {
        unimplemented!()
    }

    /// Ticks: a `tick_ms` and a `duration_ms`, or the `DamageOverTimeAbility` tag.
    pub fn is_periodic(&self) -> bool {
        unimplemented!()
    }
}

/// Every ability record by id, loaded once.
#[derive(Debug, Default)]
pub struct AbilityBook {
    by_id: HashMap<u32, AbilityDef>,
}

impl AbilityBook {
    /// One `ResolvedAbility` per line, as `bundle.jsonl` has them. A bad line names its number.
    pub fn from_jsonl(text: &str) -> Result<Self, String> {
        unimplemented!()
    }

    pub fn get(&self, id: u32) -> Option<&AbilityDef> {
        unimplemented!()
    }

    /// Follow the castable skill `root_id` to its records and build the [`Ability`] the time engine
    /// casts. `bundle` supplies the skill line (`Bundle::skill_line_of`), which the damage records
    /// do not carry.
    pub fn resolve(&self, bundle: &Bundle, root_id: u32) -> Result<Ability, String> {
        unimplemented!()
    }
}

/// A castable skill with its records already followed: the unit a rotation casts (roadmap 5.2).
#[derive(Debug, Clone, PartialEq)]
pub struct Ability {
    pub id: u32,
    pub name: String,
    pub skill_line: Option<SkillLine>,
    pub tags: Vec<AbilityTag>,
    pub cast: CastSpec,
    pub cost: Cost,
    /// `None` = no cooldown beyond the global one. `ResolvedAbility` has no cooldown field: it has to
    /// come from the `SetsCooldown` edge (or the record's duration), which is still to be worked out.
    pub cooldown_ms: Option<u32>,
    pub components: Vec<Component>,
}

impl Ability {
    /// What the cast costs against `channels` (the player's aggregated channels): the base cost with
    /// `CostReduction`, `CostReductionSkillLine` and `CostReductionCCA` applied.
    pub fn effective_cost(&self, channels: &ChannelSet) -> u32 {
        unimplemented!()
    }

    pub fn is_ultimate(&self) -> bool {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CastSpec {
    /// 0 = instant.
    pub cast_time_ms: u32,
    /// Channeled abilities hold the player for this long and tick while they do.
    pub channel_ms: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cost {
    pub mechanic: Mechanic,
    /// Before reductions; per cast, or per tick for a `FLAG_COST_PER_TICK` ability.
    pub base: u32,
    pub per_tick: bool,
}

/// What a cast does, in the order it happens.
#[derive(Debug, Clone, PartialEq)]
pub enum Component {
    /// A direct hit.
    Hit(DamageComponent),
    /// Damage over time: `duration_ms` of ticks every `tick_ms`.
    Periodic(DamageComponent),
    /// Puts an effect (`EffectDef` id) on the caster or the target.
    ApplyEffect { effect_id: u32, to: EffectTarget, duration_ms: Option<u32> },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectTarget {
    Caster,
    Target,
}

/// One damaging record of an ability and how it lands.
#[derive(Debug, Clone, PartialEq)]
pub struct DamageComponent {
    /// The record the damage formula reads (`HitSpec::ability`).
    pub record_id: u32,
    /// Time from the end of the cast to the hit (projectiles, delayed explosions).
    pub delay_ms: u32,
    pub tick_ms: Option<u32>,
    pub duration_ms: Option<u32>,
    /// 0 = single target.
    pub radius: u32,
    /// Most targets one cast can hit; `None` = no cap known.
    pub target_cap: Option<u32>,
    pub execute: Option<ExecuteScaling>,
}
