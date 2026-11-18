//! 5.5 — reports: what a run produced. Total and DPS, damage per ability, and uptimes.

use std::collections::BTreeMap;

use crate::damage::HitResult;
use crate::timeline::{Ms, Side};

/// What one ability did over the run.
#[derive(Debug, Clone, PartialEq)]
pub struct AbilityReport {
    /// The castable skill, or the record's own id for damage that no cast caused (a proc, a status effect).
    pub ability_id: u32,
    pub name: String,
    pub casts: u32,
    pub hits: u32,
    /// Crits: a whole number in a stochastic run, the expected count in an expected-value one.
    pub crits: f64,
    pub damage: f64,
    /// Of the run's total damage, 0.0 to 1.0.
    pub share: f64,
}

/// How much of the run an effect was on.
#[derive(Debug, Clone, PartialEq)]
pub struct EffectUptime {
    pub effect_id: u32,
    pub side: Side,
    pub up_ms: Ms,
    /// Of the run's duration, 0.0 to 1.0.
    pub fraction: f64,
    pub applications: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SimReport {
    pub duration_ms: Ms,
    pub total_damage: f64,
    /// `total_damage / duration` in damage per second.
    pub dps: f64,
    pub per_ability: BTreeMap<u32, AbilityReport>,
    pub uptimes: BTreeMap<(Side, u32), EffectUptime>,
    /// How many times each proc effect fired.
    pub procs: BTreeMap<u32, u32>,
}

/// Collects a run's results as the time engine goes.
#[derive(Debug, Default)]
pub struct ReportBuilder {
    // private: per-ability totals, open uptime intervals, proc counts
}

impl ReportBuilder {
    pub fn new() -> Self {
        unimplemented!()
    }

    pub fn record_cast(&mut self, ability_id: u32, name: &str) {
        unimplemented!()
    }

    /// A hit that landed (after the enemy's immunities): `taken` is what the enemy actually lost.
    pub fn record_hit(&mut self, ability_id: u32, name: &str, result: &HitResult, taken: f64) {
        unimplemented!()
    }

    pub fn effect_on(&mut self, side: Side, effect_id: u32, at_ms: Ms) {
        unimplemented!()
    }

    pub fn effect_off(&mut self, side: Side, effect_id: u32, at_ms: Ms) {
        unimplemented!()
    }

    pub fn record_proc(&mut self, effect_id: u32, weight: f64) {
        unimplemented!()
    }

    /// Close every open interval at `end_ms` and compute the totals, shares and fractions.
    pub fn finish(self, end_ms: Ms) -> SimReport {
        unimplemented!()
    }
}
