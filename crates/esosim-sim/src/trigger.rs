//! 5.4 — triggers: what fires when an event is raised.
//!
//! The vocabulary already exists in `esosim-types`: [`TriggerEvent`] (what *happens*), [`Trigger`]
//! (which event fires an effect, with what chance, at most how often) and
//! `TriggerEvent::from_list26_kind` (the game's `list26` kinds as events). This module is the
//! simulator's side of it: who is listening, which events were raised, and the history that
//! `Condition::RecentlyOccurred` reads.
//!
//! # Two sources of listeners
//!
//! * **Set procs and class 4 effects.** An `EffectDef` with `trigger: Some(..)` is not on because of
//!   the build; no sheet counts it. `Player::conditional_effects` lists every one of them
//!   (`ConditionalEffect::trigger`), which is how [`TriggerBook::from_conditional_effects`] finds the
//!   ones this build has (set pieces worn, weapon equipped ...).
//! * **Ability `list26` rows.** An ability whose `triggers` row has an `event` reacts to it
//!   ([`TriggerBook::from_abilities`]), for the abilities that are slotted.
//!
//! # Internal cooldowns
//!
//! `Trigger::cooldown_ms` / `EffectDef::cooldown_ms` where the effect states one; otherwise the
//! duration of the record a `SetsCooldown` edge points at ([`icd_from_edges`]).

use std::collections::HashMap;

use esosim::entity::conditional::ConditionalEffect;
use esosim_types::{DamageDoneContext, Trigger, TriggerEvent};

use crate::ability::AbilityBook;
use crate::timeline::Ms;

/// How a chance is settled. Roadmap 5.5: expected value first, stochastic later.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProcMode {
    /// A proc with a 33 % chance applies at weight 0.33 (see [`Firing::weight`]): no randomness.
    Expected,
    /// A seeded draw per proc, so a run can be repeated.
    Stochastic { seed: u64 },
}

/// What is listening, and why.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ListenerSource {
    /// An `EffectDef` with a trigger (a set proc).
    Effect { effect_id: u32 },
    /// A `list26` row of an ability.
    AbilityRow { ability_id: u32, row: usize },
}

/// One thing that is waiting for an event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Listener {
    pub source: ListenerSource,
    /// The effect (`EffectDef` id) switched on when it fires.
    pub effect_id: u32,
    /// For how long it is on; `None` = the effect's own `duration_ms`.
    pub duration_ms: Option<u32>,
    pub trigger: Trigger,
}

/// What the simulator raises. Carries what the triggers can ask about.
#[derive(Debug, Clone, PartialEq)]
pub struct SimEvent {
    /// For damage this is `TriggerEvent::DealDamage(DamageDoneContext::All)`; what kind of damage it
    /// was is in `damage_contexts`.
    pub event: TriggerEvent,
    /// The ability that caused it, if one did.
    pub ability_id: Option<u32>,
    /// Damage or healing dealt, for the events that have an amount.
    pub amount: Option<f64>,
    /// For a `DealDamage` event: every context the hit counts as (`damage::damage_done_contexts`), so a
    /// listener for `DealDamage(Direct)` or `DealDamage(AreaOfEffect)` can tell whether this hit is one.
    /// Empty for every other event.
    pub damage_contexts: Vec<DamageDoneContext>,
    /// For a `DealDamage` event: whether the hit was dealt in melee. `None` = the simulator does not know, and any
    /// listener that needs the answer (`TriggerEvent::DealMeleeDamage`) makes [`event_matches`] fail with
    /// [`EventError::CannotTell`]. It is never read as `false` or as `true`.
    pub melee: Option<bool>,
}

/// An event that cannot be decided. Never turned into a yes or a no: the run stops (see the rules at the top of
/// `esosim_types::trigger`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EventError {
    /// `listening` depends on something the raised event does not say.
    CannotTell { listening: TriggerEvent, missing: &'static str },
}

impl std::fmt::Display for EventError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EventError::CannotTell { listening, missing } => {
                write!(f, "cannot tell whether the event satisfies \"{listening}\": the simulator does not know {missing}")
            }
        }
    }
}

impl std::error::Error for EventError {}

/// Whether `raised` is what `listening` waits for. Not plain equality: a listener for
/// `DealDamage(All)` hears every damage event, one for `DealDamage(Direct)` only a hit whose
/// `damage_contexts` hold `Direct`, `Cast(CastKind::Any)` hears every cast, and
/// `Cast(CastKind::SkillLine(l))` only a cast of that line (which needs the ability, hence
/// [`SimEvent::ability_id`] and the book).
///
/// `DealMeleeDamage(kind)` needs both a damage event whose `damage_contexts` hold `Type(kind)` (or any type for `All`)
/// **and** `raised.melee == Some(true)`. `Some(false)` is a plain no. `None` is `Err(EventError::CannotTell)`: this
/// function never guesses.
pub fn event_matches(listening: TriggerEvent, raised: &SimEvent, abilities: &AbilityBook) -> Result<bool, EventError> {
    unimplemented!()
}

/// A listener that fired.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Firing {
    pub listener: Listener,
    /// 1.0 for a proc that always fires or that a seeded draw hit; the chance (0.33) in
    /// [`ProcMode::Expected`], where the effect counts at that fraction of its value and uptime.
    pub weight: f64,
}

/// When each event last happened, for `ConditionContext::ms_since` and `Condition::RecentlyOccurred`.
#[derive(Debug, Clone, Default)]
pub struct EventHistory {
    last: HashMap<TriggerEvent, Ms>,
}

impl EventHistory {
    pub fn record(&mut self, event: TriggerEvent, at_ms: Ms) {
        unimplemented!()
    }

    /// Milliseconds since `event` last happened at `now_ms`; `u32::MAX` = it has not (the trait's
    /// convention, `ConditionContext::ms_since`).
    pub fn ms_since(&self, event: TriggerEvent, now_ms: Ms) -> u32 {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Default)]
pub struct TriggerBook {
    listeners: Vec<Listener>,
    /// When each listener may fire again.
    ready_at: HashMap<ListenerSource, Ms>,
}

impl TriggerBook {
    /// The listeners among a build's conditional effects: every one with a `trigger`.
    pub fn from_conditional_effects(effects: &[ConditionalEffect]) -> Self {
        unimplemented!()
    }

    /// The listeners of the abilities `slotted`: their `list26` rows that carry an event.
    pub fn from_abilities(book: &AbilityBook, slotted: &[u32]) -> Self {
        unimplemented!()
    }

    pub fn extend(&mut self, other: TriggerBook) {
        unimplemented!()
    }

    pub fn listeners(&self) -> &[Listener] {
        unimplemented!()
    }

    /// Everything `event` fires at `now_ms`: the listeners that hear it, are off cooldown, and (in
    /// [`ProcMode::Stochastic`]) won their draw. Puts each one that fires on its cooldown. The caller
    /// records the event in the [`EventHistory`] *after* dispatching, so a listener for
    /// `RecentlyOccurred(e)` does not see the event it is being fired by.
    pub fn dispatch(
        &mut self,
        event: &SimEvent,
        now_ms: Ms,
        mode: ProcMode,
        abilities: &AbilityBook,
    ) -> Vec<Firing> {
        unimplemented!()
    }
}

/// The internal cooldown an ability row gets from a `SetsCooldown` edge: the duration of the record
/// the edge points at.
pub fn icd_from_edges(book: &AbilityBook, ability_id: u32) -> Option<u32> {
    unimplemented!()
}