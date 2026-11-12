//! Bundle schema: the flat, denormalised record the compiler emits per ability.
//!
//! Design rules (see PLAN.md §3):
//!  * Raw numeric ids are always kept; human-readable `*_name` fields are
//!    convenience for diffs/review and are `None` when the id is unrecognised.
//!    Nothing unrecognised is ever dropped.
//!  * Both `Serialize` and `Deserialize`, so this file can move unchanged into a
//!    shared `esosim-bundle` crate that the engine also depends on.

use esosim_types::{Channel, Condition, TriggerEvent};
use serde::{Deserialize, Serialize};

fn is_zero<T: Default + PartialEq>(v: &T) -> bool {
    *v == T::default()
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EdgeKind {
    /// `causes_ids` (the `link_type` says how).
    Causes,
    /// `tooltip_data[].tooltip_ids`.
    Tooltip,
    /// Ability-type specific link out of `list26` (see `SkillData34::type_link`).
    ActionList,
    GrantsSynergy,
    SetsCooldown,
    RemovesBuff,
    /// `list25[0].base_ability_id` — morph -> base.
    BaseAbility,
    /// `base_data.caused_by`.
    CausedBy,
    /// `base_data.related_ability_id`.
    Related,
    /// `u8c` (`replaces_id`).
    Replaces,
    // Ability-id-shaped fields whose meaning is not yet pinned down. Named after
    // their parser field so a rename in eso_skill_data is a compile error here.
    FieldU33c,
    FieldU34c,
    FieldU374,
    FieldU39c,
}

/// Outgoing reference: this ability -> `to`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Edge {
    pub kind: EdgeKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_type: Option<u8>,
    pub to: u32,
}

/// Incoming reference: `from` -> this ability.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct BackRef {
    pub from: u32,
    pub kind: EdgeKind,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub link_type: Option<u8>,
}

/// One weighted-sum row. Real formula (PLAN.md §6): `max(c1*t1 + c2*t2, c3*t3 + c4*t4)`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedCoef {
    pub types: [u8; 4],
    pub coefs: [f32; 4],
    #[serde(default, skip_serializing_if = "is_zero")]
    pub u20: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedTooltip {
    pub types: Vec<u32>,
    pub ids: Vec<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedExecute {
    pub threshold_below_health_pct: u8,
    pub bonus_up_to_pct: u16,
}

/// Raw `list26` entry. `u2[0]` is the trigger kind (see `list26_trigger_label`);
/// the rest is kept raw until its meaning is understood.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedTrigger {
    pub u2: Vec<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub list3: Vec<u32>,
    /// What `u2[0]` means as a combat event (`TriggerEvent::from_list26_kind`). Absent for kind 0
    /// (always-on) and for the kinds that modify a pool, range or cost instead of reacting.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<TriggerEvent>,
}

/// One phrase the trigger lexicon (`triggers.rs`) found in a tooltip clause: *when* or *while* or
/// *against what* a value applies. Format 2 of `bundle.jsonl` carries these on every evidence clause.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CueRecord {
    /// The cue as the lexicon names it (`On(DrinkPotion)`, `While(InCombat)`, `Per(Stack)` ...).
    pub cue: String,
    /// The lexicon rule that matched.
    pub rule: String,
    /// The words that matched (lower-case, placeholders as `#`).
    pub text: String,
    /// For `On(..)` cues: the event the simulator would raise.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event: Option<TriggerEvent>,
    /// For `While(..)` and `Against(..)` cues `Condition` can express.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub condition: Option<Condition>,
}

fn is_always(c: &Condition) -> bool {
    *c == Condition::Always
}

/// What the tooltip's verb does to the stat it names ("reduces damage taken" -> `Lowers`).
/// This is the *text's* direction: a channel whose name already means a reduction
/// (`CostReduction`) still reads `Lowers` for "reduces the cost of ...".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Direction {
    Raises,
    Lowers,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClassStatus {
    /// One channel set, agreed by every referencing tooltip that matched a rule.
    Classified,
    /// Referencing tooltips matched different channels (see `evidence`).
    Conflict,
    /// No channel; `reason` says why.
    Unclassified,
}

/// One referencing tooltip that supports a classification.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedEvidence {
    /// The ability whose tooltip text references this one.
    pub parent: u32,
    /// The `<<n>>` placeholder (1-based) that maps to this ability.
    pub placeholder: u8,
    /// The tooltip clause around the placeholder (`@` = the placeholder, `#` = others).
    pub clause: String,
    /// Names of the lexicon rules that matched, joined with `+`.
    pub rule: String,
    pub channels: Vec<Channel>,
    pub direction: Direction,
    /// The tooltip's id did not carry the value itself; the dictionary's `resolve_id` heuristic
    /// followed links to this ability. Only used when no direct evidence exists.
    #[serde(default, skip_serializing_if = "is_false")]
    pub indirect: bool,
    /// Every when/while/per/against phrase in the clause, in text order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cues: Vec<CueRecord>,
    /// All the clause's expressible `While`/`Against` cues together (`Always` = none). This is the
    /// `EffectDef.condition` a reviewer would start from.
    #[serde(default, skip_serializing_if = "is_always")]
    pub condition: Condition,
    /// The clause's `On(..)` events (the proc triggers), in text order.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<TriggerEvent>,
}

fn is_false(b: &bool) -> bool {
    !*b
}

/// Result of the regex classifier (`classify.rs`) for an ability that carries a value or a
/// coefficient equation.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedClassification {
    pub status: ClassStatus,
    /// The engine channels, serialised exactly as in `sets.jsonl` (e.g. `{"Armour":"All"}`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub channels: Vec<Channel>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub direction: Option<Direction>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub evidence: Vec<ResolvedEvidence>,
    /// `Unclassified` only: `no_references`, `no_tooltip_text`, `no_placeholder`,
    /// `not_a_stat_entry`, `no_rule_matched` or `partial_match`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// `Unclassified` only: up to three clauses no rule matched (feeds `classify` report).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub unmatched_clauses: Vec<String>,
}

/// Why an ability is in the bundle: the root category whose closure over the ability graph
/// contains it (roadmap 2.0 item 1). `Skills`, `Sets` and `Cp` come from the game tables, the rest
/// from `data/roots.toml`. An ability can belong to several.
///
/// The declaration order is the sort order of `categories` in `bundle.jsonl`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RootCategory {
    Skills,
    Sets,
    Cp,
    Mundus,
    Consumables,
    Racial,
    Buffs,
    Class,
    /// Armour-weight passives (ability tag 339 `ArmourPassive`; roadmap 2.4).
    ArmourPassives,
    /// Weapon-line passives (ability tag 337 `WeaponPassive`; roadmap 2.4).
    WeaponPassives,
}

impl RootCategory {
    pub const ALL: [RootCategory; 10] = [
        RootCategory::Skills,
        RootCategory::Sets,
        RootCategory::Cp,
        RootCategory::Mundus,
        RootCategory::Consumables,
        RootCategory::Racial,
        RootCategory::Buffs,
        RootCategory::Class,
        RootCategory::ArmourPassives,
        RootCategory::WeaponPassives,
    ];

    /// The name used in `roots.toml` and in `categories`.
    pub fn name(self) -> &'static str {
        match self {
            RootCategory::Skills => "skills",
            RootCategory::Sets => "sets",
            RootCategory::Cp => "cp",
            RootCategory::Mundus => "mundus",
            RootCategory::Consumables => "consumables",
            RootCategory::Racial => "racial",
            RootCategory::Buffs => "buffs",
            RootCategory::Class => "class",
            RootCategory::ArmourPassives => "armour_passives",
            RootCategory::WeaponPassives => "weapon_passives",
        }
    }

    pub fn parse(name: &str) -> Option<RootCategory> {
        Self::ALL.into_iter().find(|c| c.name() == name)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ResolvedAbility {
    pub id: u32,
    pub name: String,

    #[serde(default, skip_serializing_if = "is_zero")] pub ability_type: u8,
    #[serde(default, skip_serializing_if = "is_zero")] pub mechanic: u8,
    #[serde(default, skip_serializing_if = "is_zero")] pub skill_line: u32,
    #[serde(default, skip_serializing_if = "is_zero")] pub player_skill_index: u32,
    #[serde(default, skip_serializing_if = "is_zero")] pub scribing_index: u32,
    /// Raw `major_minor_id` code (NOT an ability id — see PLAN.md §10).
    #[serde(default, skip_serializing_if = "is_zero")] pub major_minor_id: u8,
    #[serde(default, skip_serializing_if = "is_zero")] pub damage_type: u16,

    /// Raw tag ids. Unrecognised ones are kept (reported in the compile stats).
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub tags: Vec<u16>,
    /// `(index, value)` for every non-zero entry of the 177-byte flag array.
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub flags: Vec<(u8, u8)>,

    #[serde(default, skip_serializing_if = "is_zero")] pub target_type: u16,
    #[serde(default, skip_serializing_if = "is_zero")] pub duration_ms: u32,
    #[serde(default, skip_serializing_if = "is_zero")] pub tick_ms: u32,
    #[serde(default, skip_serializing_if = "is_zero")] pub cast_time_ms: u32,
    #[serde(default, skip_serializing_if = "is_zero")] pub cost: u16,
    #[serde(default, skip_serializing_if = "is_zero")] pub range: u32,
    #[serde(default, skip_serializing_if = "is_zero")] pub radius: u32,
    #[serde(default, skip_serializing_if = "is_zero")] pub value0: i32,
    #[serde(default, skip_serializing_if = "is_zero")] pub value1: i32,
    #[serde(default, skip_serializing_if = "is_zero")] pub value2: i32,
    #[serde(default, skip_serializing_if = "is_zero")] pub target_cap: u16,

    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub coef: Vec<ResolvedCoef>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub execute: Option<ResolvedExecute>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub triggers: Vec<ResolvedTrigger>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub tooltips: Vec<ResolvedTooltip>,
    #[serde(default, skip_serializing_if = "Option::is_none")] pub tooltip_text: Option<String>,

    /// Which engine channel(s) this ability's value feeds, if it has a value (see `classify.rs`).
    #[serde(default, skip_serializing_if = "Option::is_none")] pub classification: Option<ResolvedClassification>,

    /// The root categories whose closure contains this ability, sorted. Empty in a bundle written
    /// before root categories existed (`verify` then only warns about `roots.toml`).
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub categories: Vec<RootCategory>,

    #[serde(default, skip_serializing_if = "Option::is_none")] pub morph_of: Option<u32>,
    /// Outgoing references, sorted, only to ids that exist in the corpus.
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub edges: Vec<Edge>,
    /// Incoming references (who points at me), sorted. Denormalised on purpose:
    /// answers "which abilities grant Minor Slayer?" without a second pass.
    #[serde(default, skip_serializing_if = "Vec::is_empty")] pub referenced_by: Vec<BackRef>,
}