//! 5.3 — the time engine: an event queue, buff apply / refresh / expire, DoT ticks, resource
//! regeneration and spending, the global cooldown and animation cancelling.
//!
//! Event-driven, not fixed-tick (PLAN §9): nothing happens between events, so a ten-minute fight
//! costs as many steps as it has casts, ticks and expiries.
//!
//! Time is whole milliseconds ([`Ms`]), as in the data (`cast_time_ms`, `tick_ms`, `duration_ms`).
//! Two events at the same millisecond run in the order they were scheduled, so a run is
//! deterministic.

use std::collections::{BTreeMap, HashMap};

use eso_skill_data::enums::mechanic::Mechanic;
use esosim::engine::active_effects::ActiveEffects;
use esosim_types::Bar;

use crate::damage::HitKind;

/// Milliseconds since the start of the fight.
pub type Ms = u64;

/// Whose effect list an effect is on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Side {
    Player,
    Enemy,
}

/// What the queue holds.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// The player starts casting `ability_id` (the rotation's next step, once the global cooldown and
    /// the previous animation allow it).
    CastStart { ability_id: u32 },
    /// The cast time is over: cost is spent, components are applied.
    CastComplete { ability_id: u32 },
    /// A direct hit lands (`delay_ms` after the cast).
    Hit { ability_id: u32, record_id: u32, kind: HitKind },
    /// One tick of a damage-over-time instance.
    Tick { instance: InstanceId, ability_id: u32, record_id: u32 },
    /// An effect goes on `side` for `duration_ms` (`None` = until removed).
    EffectApply { effect_id: u32, side: Side, stacks: u32, duration_ms: Option<u32> },
    /// An effect ends. `generation` is the one the timer was set with: a refresh bumps it, so the
    /// expiry of the old timer finds a newer generation and does nothing.
    EffectExpire { effect_id: u32, side: Side, generation: u32 },
    /// Resource regeneration, once a second.
    ResourceTick,
    GlobalCooldownEnd,
    /// A proc fires (5.4).
    Proc { effect_id: u32 },
    /// The bar swap lands.
    BarSwap { to: Bar },
}

/// One running damage-over-time (or channel) instance; a recast replaces the instance of the same ability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct InstanceId(pub u64);

/// A queued event's place: when, and which of the events at that time (insertion order).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventId {
    pub at_ms: Ms,
    pub seq: u64,
}

#[derive(Debug, Default)]
pub struct Timeline {
    now_ms: Ms,
    next_seq: u64,
    queue: BTreeMap<EventId, Event>,
}

impl Timeline {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn now_ms(&self) -> Ms {
        unimplemented!()
    }

    /// Queue `event` at `at_ms` (not before now). Returns its place, to cancel it with.
    pub fn schedule(&mut self, at_ms: Ms, event: Event) -> EventId {
        unimplemented!()
    }

    pub fn schedule_in(&mut self, delay_ms: Ms, event: Event) -> EventId {
        unimplemented!()
    }

    pub fn cancel(&mut self, id: EventId) -> Option<Event> {
        unimplemented!()
    }

    /// Take the earliest event and move `now` to its time.
    pub fn pop_next(&mut self) -> Option<(Ms, Event)> {
        unimplemented!()
    }

    pub fn is_empty(&self) -> bool {
        unimplemented!()
    }
}

/// The timer of one effect on one side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EffectTimer {
    pub applied_at_ms: Ms,
    /// `None` = until removed.
    pub expires_at_ms: Option<Ms>,
    pub generation: u32,
}

/// The effects on one side, with their timers. The stacks are the engine's [`ActiveEffects`], so the
/// channels are aggregated by the same code as the build's sheet; this adds *when* they end.
#[derive(Debug, Clone, Default)]
pub struct LiveEffects {
    pub active: ActiveEffects,
    timers: HashMap<u32, EffectTimer>,
}

impl LiveEffects {
    /// Put `effect_id` on with `stacks` (a stack-limited effect is held to its maximum when it is
    /// aggregated, `EffectDef::effective_stacks`), for `duration_ms`. Applying an effect that is
    /// already on is a refresh: the timer restarts and its generation goes up. Returns the generation.
    pub fn apply(&mut self, effect_id: u32, stacks: u32, duration_ms: Option<u32>, now: Ms) -> u32 {
        unimplemented!()
    }

    /// End `effect_id` if `generation` is still its timer's. Returns whether it ended.
    pub fn expire(&mut self, effect_id: u32, generation: u32) -> bool {
        unimplemented!()
    }

    pub fn remove(&mut self, effect_id: u32) {
        unimplemented!()
    }

    pub fn timer(&self, effect_id: u32) -> Option<&EffectTimer> {
        unimplemented!()
    }
}

/// Health, Magicka, Stamina and Ultimate. Maxima come from the sheet at the time they are needed (a
/// buff can raise them mid-fight).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResourcePools {
    pub health: f64,
    pub magicka: f64,
    pub stamina: f64,
    pub ultimate: f64,
}

impl ResourcePools {
    /// Full Health, Magicka and Stamina, no Ultimate.
    pub fn full(max_health: f64, max_magicka: f64, max_stamina: f64) -> Self {
        unimplemented!()
    }

    pub fn can_afford(&self, mechanic: Mechanic, cost: u32) -> bool {
        unimplemented!()
    }

    /// Spend `cost` of the resource `mechanic` names. Err = what is missing.
    pub fn spend(&mut self, mechanic: Mechanic, cost: u32) -> Result<(), u32> {
        unimplemented!()
    }

    /// `dt_ms` of recovery at `recovery` points per second (`StatSheet::*_recovery`), held to the maxima.
    pub fn regenerate(&mut self, dt_ms: Ms, health_recovery: u32, magicka_recovery: u32, stamina_recovery: u32, max: &PoolMaxima) {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PoolMaxima {
    pub health: f64,
    pub magicka: f64,
    pub stamina: f64,
}

/// The rules of the clock: how soon the next ability can start. Measured values, not constants of
/// the data, so they are settings (roadmap 5.3: global cooldown and animation cancelling).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimingRules {
    /// Time after one ability starts before the next one can.
    pub global_cooldown_ms: u32,
    /// A light attack's animation.
    pub light_attack_ms: u32,
    /// How early into a light attack an ability can cancel it (weaving).
    pub light_attack_cancel_ms: u32,
    /// Time to swap bars, in which nothing else starts.
    pub bar_swap_ms: u32,
}

impl TimingRules {
    /// The values measured in the game; to be filled in from logs.
    pub fn standard() -> Self {
        unimplemented!()
    }
}

/// Why a cast could not start.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CastError {
    /// The ability is not on the active bar (and is not a weapon attack).
    NotSlotted { ability_id: u32 },
    /// Not enough of the resource; `missing` is how much short.
    NotEnoughResource { mechanic: Mechanic, missing: u32 },
    OnCooldown { ready_at_ms: Ms },
    GlobalCooldown { ready_at_ms: Ms },
    /// The ability id is in no loaded record.
    UnknownAbility { ability_id: u32 },
}
