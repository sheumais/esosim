//! 5.6 — enemy behaviour: what the target does over time.
//!
//! The engine's `Enemy` is a snapshot: health, resistance and the debuffs it was given. A fight needs
//! it to *change*: health that falls (and crosses execute thresholds), debuffs that go on and off,
//! windows in which it cannot be hurt, and a distance that moves. [`EnemyBehaviour`] is that script and
//! [`EnemyState`] is the snapshot plus the script's state at the current time.
//!
//! Scope, per the roadmap: health over time, debuff uptimes, minimal movement and immunities. No AI.

use esosim::engine::registry::EffectRegistry;
use esosim::entity::enemy::{Enemy, EnemyPreset};
use esosim_types::TargetKind;

use crate::timeline::Ms;

/// How the enemy's Health moves.
#[derive(Debug, Clone, PartialEq)]
pub enum HealthModel {
    /// Health never moves: a trial dummy. Execute abilities see whatever `Enemy::current_health` is.
    Static,
    /// Every point of damage taken comes off `Enemy::current_health` (and 0 is dead).
    DepletesWithDamage,
    /// Health follows a script regardless of damage: reach `pct` at `at_ms`, linearly in between.
    /// This is how an execute phase is tested at a known time without simulating the whole boss.
    Scripted(Vec<HealthKeyframe>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HealthKeyframe {
    pub at_ms: Ms,
    /// Percent of max Health, 0.0 to 100.0.
    pub pct: f32,
}

/// When a debuff is on the enemy without the player's rotation putting it there.
#[derive(Debug, Clone, PartialEq)]
pub enum DebuffUptime {
    /// For the whole fight (the trial dummy's debuffs).
    Always,
    /// Only in these `[from, to)` windows.
    Windows(Vec<(Ms, Ms)>),
    /// Never by itself: the rotation applies it (`Component::ApplyEffect`), and its uptime is what
    /// the report measures.
    ByRotation,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DebuffSchedule {
    /// `EffectDef` id of the debuff.
    pub effect_id: u32,
    pub uptime: DebuffUptime,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImmunityKind {
    /// Takes no damage.
    Damage,
    /// Cannot be stunned, snared ... (it does not change damage; `Affliction` conditions read it).
    CrowdControl,
}

/// A window in which the enemy is immune.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImmunityWindow {
    pub from_ms: Ms,
    pub to_ms: Ms,
    pub kind: ImmunityKind,
}

/// How far the enemy is from the player, for `ConditionContext::target_distance` (and range-gated bonuses).
#[derive(Debug, Clone, PartialEq)]
pub enum MovementModel {
    Stationary { distance_m: f32 },
    /// `(at_ms, distance in metres)`, linear in between.
    Scripted(Vec<(Ms, f32)>),
}

/// Everything about the enemy that is not in the build.
#[derive(Debug, Clone, PartialEq)]
pub struct EnemyBehaviour {
    /// Health, resistance and starting debuffs (`Build::enemy` has one).
    pub preset: EnemyPreset,
    /// What the enemy is: `TargetIs` conditions and `DamageDone(Monsters)` read it.
    pub kinds: Vec<TargetKind>,
    pub health: HealthModel,
    pub debuffs: Vec<DebuffSchedule>,
    pub immunities: Vec<ImmunityWindow>,
    pub movement: MovementModel,
}

impl EnemyBehaviour {
    /// A trial dummy: static health, its debuffs always on, no immunities, standing still in melee range.
    pub fn trial_dummy() -> Self {
        unimplemented!()
    }
}

/// The enemy at the current time of a run.
pub struct EnemyState {
    pub enemy: Enemy,
    pub behaviour: EnemyBehaviour,
}

impl EnemyState {
    pub fn new(behaviour: EnemyBehaviour, registry: &EffectRegistry) -> Self {
        unimplemented!()
    }

    /// Take `amount` damage at `now_ms`. Returns what was actually taken (0 inside an immunity
    /// window). Applies the [`HealthModel`].
    pub fn take_damage(&mut self, amount: f64, now_ms: Ms) -> f64 {
        unimplemented!()
    }

    /// Current Health in percent at `now_ms`, per the [`HealthModel`].
    pub fn health_pct(&self, now_ms: Ms) -> f32 {
        unimplemented!()
    }

    pub fn is_immune(&self, kind: ImmunityKind, now_ms: Ms) -> bool {
        unimplemented!()
    }

    pub fn distance_m(&self, now_ms: Ms) -> f32 {
        unimplemented!()
    }

    /// Switch the scheduled debuffs on or off for `now_ms` and rebuild `enemy.channels`
    /// (`Enemy::recompute`). Debuffs the rotation applied are left alone.
    pub fn refresh_debuffs(&mut self, registry: &EffectRegistry, now_ms: Ms) {
        unimplemented!()
    }
}
