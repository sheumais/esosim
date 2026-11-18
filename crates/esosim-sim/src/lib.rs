//! Combat simulator (roadmap Stage 5).
//!
//! A separate crate that depends on the engine (`esosim`), never the reverse. `TriggerEvent`,
//! `CastKind`, `Trigger` and the `list26` kind mapping already live in `esosim-types`; the simulator
//! raises events, and [`context::CombatContext`] answers the combat-time conditions.
//!
//! | Step | Module |
//! |---|---|
//! | 5.1 damage formula, single hit | [`damage`] |
//! | 5.2 ability model | [`ability`] |
//! | 5.3 time engine | [`timeline`] |
//! | 5.4 triggers | [`trigger`], [`context`] |
//! | 5.5 rotations and reports | [`rotation`], [`report`] |
//! | 5.6 enemy behaviour | [`enemy`] |
//! | driver (ties them together) | [`sim`] |
//!
//! 5.7 (healing, shields, resources for tank and healer builds) is "only if wanted" and has no module.
//!
//! **This is a scaffold.** Every function body is `unimplemented!()`; the types, signatures and docs
//! are the design to review. Delete the `allow` below once the bodies exist (it hides unused-parameter and never-read-field warnings).

#![allow(unused_variables, dead_code)]

pub mod ability;
pub mod context;
pub mod damage;
pub mod enemy;
pub mod report;
pub mod rotation;
pub mod sim;
pub mod timeline;
pub mod trigger;
