//! The driver: set a run up, process events in time order, produce a [`SimReport`].
//!
//! ```text
//!  SimSetup ──► PlayerModel ─┐
//!  (build, bundle,           ├─► Simulation::run(rotation) ──► SimReport
//!   abilities, enemy)  ──► EnemyState ┘     │
//!                                            ├─ Timeline ............ 5.3 (timeline.rs)
//!                                            ├─ TriggerBook ......... 5.4 (trigger.rs)
//!                                            ├─ CombatContext ....... 5.4 (context.rs)
//!                                            ├─ damage::resolve_hit . 5.1 (damage.rs)
//!                                            └─ ReportBuilder ....... 5.5 (report.rs)
//! ```
//!
//! # What a step does
//!
//! 1. `Timeline::pop_next` gives the next event and moves the clock.
//! 2. The channels are re-derived if anything that feeds them changed since the last event
//!    ([`PlayerModel::channels`]), with a [`CombatContext`] so combat-time conditions decide.
//! 3. The event is handled: a cast spends and schedules its components, a hit goes through
//!    `damage::resolve_hit` and into the enemy and the report, a tick does the same and reschedules,
//!    an effect applies or expires.
//! 4. Handling an event raises `SimEvent`s; `TriggerBook::dispatch` turns them into `Proc` events.
//!
//! # A gap in the engine
//!
//! [`PlayerModel`] needs what `Build::prepare_for` builds: the `Player`, its `EffectRegistry` and the
//! grant rules (`set_effect_rules`, `passive_effect_rules_with`). That is a private `Prepared` struct in
//! `esosim::entity::build_player` today, so [`PlayerModel::from_build`] cannot be written from the
//! outside. The engine change is small (make `Prepared` and `Build::prepare_for` `pub`, or add a
//! `Build::prepare` that returns the three parts) and is deliberately not made by this scaffold.

use std::collections::HashMap;

use esosim::data::bundle::Bundle;
use esosim::engine::aggregator::ChannelSet;
use esosim::engine::grant_rule::GrantRule;
use esosim::engine::registry::EffectRegistry;
use esosim::entity::build::Build;
use esosim::entity::player::Player;
use esosim::stats::sheet::StatSheet;
use esosim_types::{Bar, ConditionContext};

use crate::ability::{Ability, AbilityBook};
use crate::damage::CritMode;
use crate::enemy::{EnemyBehaviour, EnemyState};
use crate::report::SimReport;
use crate::rotation::Rotation;
use crate::timeline::{CastError, LiveEffects, Ms, ResourcePools, Timeline, TimingRules};
use crate::trigger::{EventHistory, ProcMode, TriggerBook};

/// Everything a run is built from.
pub struct SimSetup<'a> {
    pub build: &'a Build,
    pub bundle: &'a Bundle,
    pub abilities: &'a AbilityBook,
    pub enemy: EnemyBehaviour,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SimConfig {
    pub crit: CritMode,
    pub procs: ProcMode,
    pub timing: TimingRules,
    /// Stop when the clock reaches this, or when the rotation is done (not repeating).
    pub duration_ms: Ms,
    pub start_bar: Bar,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SimError {
    /// A step of the rotation could not be cast and the run cannot go on (see `CastError`).
    Cast { step: usize, error: CastError },
    /// An ability record the run needs is missing or malformed.
    Ability(String),
}

/// The player as the run sees it: what the engine derived from the build, and what is needed to
/// derive it again when buffs change. See the module docs for why this cannot be built yet.
pub struct PlayerModel {
    pub player: Player,
    pub registry: EffectRegistry,
    pub rules: Vec<GrantRule>,
}

impl PlayerModel {
    pub fn from_build(build: &Build, bundle: &Bundle) -> Self {
        unimplemented!()
    }

    /// The player's channels with the run's live effects switched on, conditions decided by `ctx`.
    pub fn channels(&self, live: &LiveEffects, ctx: &dyn ConditionContext) -> ChannelSet {
        unimplemented!()
    }

    pub fn sheet(&self, channels: &ChannelSet) -> StatSheet {
        unimplemented!()
    }
}

/// Everything that changes during a run, apart from the queue itself.
pub struct CombatState {
    pub now_ms: Ms,
    pub in_combat: bool,
    pub bar: Bar,
    pub pools: ResourcePools,
    pub player_effects: LiveEffects,
    pub enemy: EnemyState,
    /// When each ability may be cast again.
    pub cooldown_ready_at: HashMap<u32, Ms>,
    pub gcd_ready_at: Ms,
    /// The ability being cast or channeled, if any.
    pub casting: Option<u32>,
    pub history: EventHistory,
}

pub struct Simulation<'a> {
    setup: &'a SimSetup<'a>,
    config: SimConfig,
    model: PlayerModel,
    timeline: Timeline,
    state: CombatState,
    triggers: TriggerBook,
    /// The slotted abilities, resolved once.
    abilities: HashMap<u32, Ability>,
}

impl<'a> Simulation<'a> {
    pub fn new(setup: &'a SimSetup<'a>, config: SimConfig) -> Result<Self, SimError> {
        unimplemented!()
    }

    /// Run `rotation` to the end of the fight and report.
    pub fn run(self, rotation: &Rotation) -> Result<SimReport, SimError> {
        unimplemented!()
    }

    /// Handle the next event. `false` when the queue is empty.
    pub fn step(&mut self) -> bool {
        unimplemented!()
    }

    /// Start casting `ability_id`: checks slot, cost, cooldown and the global cooldown.
    fn try_cast(&mut self, ability_id: u32) -> Result<(), CastError> {
        unimplemented!()
    }
}

/// Set a run up and run it: the one call most users want.
pub fn simulate(setup: &SimSetup, rotation: &Rotation, config: SimConfig) -> Result<SimReport, SimError> {
    unimplemented!()
}
