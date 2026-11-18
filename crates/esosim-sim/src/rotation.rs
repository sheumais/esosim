//! 5.5 — rotations: what the player does, in order.
//!
//! "A rotation is a `Vec<ability id>` with timings." Each step is a castable skill's id and,
//! optionally, the earliest time it may start; without one it starts as soon as the global cooldown
//! and the previous animation allow it.
//!
//! Light and heavy attacks and bar swaps are not skills with ids a player slots, so a rotation cannot
//! say them yet; they need ids of their own (or step kinds) once weaving is simulated.

use serde::{Deserialize, Serialize};

use crate::timeline::Ms;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct RotationStep {
    /// The castable skill (`Ability::id`), as slotted on the bar.
    pub ability_id: u32,
    /// Do not start before this time. `None` = as soon as possible.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub not_before_ms: Option<Ms>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize, Deserialize)]
pub struct Rotation {
    pub steps: Vec<RotationStep>,
    /// Run the steps again from the start until the fight's duration is up.
    #[serde(default)]
    pub repeat: bool,
}

impl Rotation {
    /// Every id, as soon as possible, once.
    pub fn from_ids(ids: &[u32]) -> Self {
        unimplemented!()
    }

    pub fn from_json(text: &str) -> Result<Self, String> {
        unimplemented!()
    }

    pub fn to_json(&self) -> Result<String, String> {
        unimplemented!()
    }

    /// What is wrong with the rotation against the abilities that are slotted (an id on neither bar,
    /// a `not_before_ms` that goes backwards, an empty rotation set to repeat).
    pub fn issues(&self, slotted: &[u32]) -> Vec<RotationIssue> {
        unimplemented!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RotationIssue {
    Empty,
    NotSlotted { step: usize, ability_id: u32 },
    TimeGoesBackwards { step: usize },
    RepeatsNothing,
}
