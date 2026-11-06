//! When does an effect apply? (roadmap 1.5, Appendix A.2)
//!
//! `Condition` is plain data so it can live in effect files. Evaluation never touches the
//! engine: the caller supplies a [`ConditionContext`] (a player, an enemy, the simulator…)
//! and every question it cannot answer comes back as `None`. That gives three-valued logic:
//! `Some(true)`, `Some(false)`, or `None` = "cannot be decided here" (the UI shows it as
//! "conditional"; the calculator leaves the effect out).

use eso_skill_data::enums::skill_line::SkillLine;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ArmourWeight {
    Light,
    Medium,
    Heavy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Bar {
    Front,
    Backup,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Condition {
    #[default]
    Always,
    All(Vec<Condition>),
    Any(Vec<Condition>),
    Not(Box<Condition>),

    // --- build-time: decidable from the build alone ---
    SetPieces { set_id: u16, at_least: u8 },
    ArmourPieces { weight: ArmourWeight, at_least: u8 },
    SkillLineSlotted {
        #[serde(with = "crate::skill_serde::skill_line")]
        skill_line: SkillLine,
        at_least: u8,
    },
    AbilitySlotted { ability_id: u32 },
    OnBar(Bar),

    // --- combat-time: needs live state (a target, the simulator) ---
    SelfBuffActive(u32),
    TargetDebuffActive(u32),
    TargetHealthBelowPct(u8),
    SelfHealthBelowPct(u8),
    InCombat,
}

/// What the evaluator knows. Every method defaults to "unknown", so a context only
/// implements what it can actually answer.
pub trait ConditionContext {
    fn set_pieces(&self, _set_id: u16) -> Option<u8> { None }
    fn armour_pieces(&self, _weight: ArmourWeight) -> Option<u8> { None }
    fn skill_line_slotted(&self, _line: SkillLine) -> Option<u8> { None }
    fn ability_slotted(&self, _ability_id: u32) -> Option<bool> { None }
    fn on_bar(&self, _bar: Bar) -> Option<bool> { None }
    fn self_buff_active(&self, _id: u32) -> Option<bool> { None }
    fn target_debuff_active(&self, _id: u32) -> Option<bool> { None }
    fn target_health_pct(&self) -> Option<u8> { None }
    fn self_health_pct(&self) -> Option<u8> { None }
    fn in_combat(&self) -> Option<bool> { None }
}

/// Knows nothing: only `Always` (and combinations that reduce to it) evaluate.
pub struct NoContext;
impl ConditionContext for NoContext {}

impl Condition {
    /// True when the build alone decides this condition (no target, no live state).
    pub fn is_build_time(&self) -> bool {
        match self {
            Condition::Always
            | Condition::SetPieces { .. }
            | Condition::ArmourPieces { .. }
            | Condition::SkillLineSlotted { .. }
            | Condition::AbilitySlotted { .. }
            | Condition::OnBar(_) => true,
            Condition::All(v) | Condition::Any(v) => v.iter().all(Condition::is_build_time),
            Condition::Not(c) => c.is_build_time(),
            Condition::SelfBuffActive(_)
            | Condition::TargetDebuffActive(_)
            | Condition::TargetHealthBelowPct(_)
            | Condition::SelfHealthBelowPct(_)
            | Condition::InCombat => false,
        }
    }

    /// `None` = cannot be decided with this context.
    pub fn evaluate(&self, ctx: &dyn ConditionContext) -> Option<bool> {
        match self {
            Condition::Always => Some(true),
            Condition::All(v) => {
                let mut unknown = false;
                for c in v {
                    match c.evaluate(ctx) {
                        Some(false) => return Some(false),
                        None => unknown = true,
                        Some(true) => {}
                    }
                }
                if unknown { None } else { Some(true) }
            }
            Condition::Any(v) => {
                let mut unknown = false;
                for c in v {
                    match c.evaluate(ctx) {
                        Some(true) => return Some(true),
                        None => unknown = true,
                        Some(false) => {}
                    }
                }
                if unknown { None } else { Some(false) }
            }
            Condition::Not(c) => c.evaluate(ctx).map(|b| !b),
            Condition::SetPieces { set_id, at_least } => {
                ctx.set_pieces(*set_id).map(|n| n >= *at_least)
            }
            Condition::ArmourPieces { weight, at_least } => {
                ctx.armour_pieces(*weight).map(|n| n >= *at_least)
            }
            Condition::SkillLineSlotted { skill_line, at_least } => {
                ctx.skill_line_slotted(*skill_line).map(|n| n >= *at_least)
            }
            Condition::AbilitySlotted { ability_id } => ctx.ability_slotted(*ability_id),
            Condition::OnBar(bar) => ctx.on_bar(*bar),
            Condition::SelfBuffActive(id) => ctx.self_buff_active(*id),
            Condition::TargetDebuffActive(id) => ctx.target_debuff_active(*id),
            Condition::TargetHealthBelowPct(p) => ctx.target_health_pct().map(|h| h < *p),
            Condition::SelfHealthBelowPct(p) => ctx.self_health_pct().map(|h| h < *p),
            Condition::InCombat => ctx.in_combat(),
        }
    }

    /// Ids of buffs/debuffs this condition refers to (for dangling-id checks).
    pub fn referenced_effect_ids(&self, out: &mut Vec<u32>) {
        match self {
            Condition::All(v) | Condition::Any(v) => v.iter().for_each(|c| c.referenced_effect_ids(out)),
            Condition::Not(c) => c.referenced_effect_ids(out),
            Condition::SelfBuffActive(id) | Condition::TargetDebuffActive(id) => out.push(*id),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Ctx;
    impl ConditionContext for Ctx {
        fn set_pieces(&self, set_id: u16) -> Option<u8> { (set_id == 1).then_some(5) }
        fn on_bar(&self, bar: Bar) -> Option<bool> { Some(bar == Bar::Front) }
    }

    #[test]
    fn always_needs_no_context() {
        assert_eq!(Condition::Always.evaluate(&NoContext), Some(true));
        assert!(Condition::Always.is_build_time());
    }

    #[test]
    fn build_time_conditions_evaluate() {
        assert_eq!(Condition::SetPieces { set_id: 1, at_least: 5 }.evaluate(&Ctx), Some(true));
        assert_eq!(Condition::SetPieces { set_id: 1, at_least: 6 }.evaluate(&Ctx), Some(false));
        assert_eq!(Condition::SetPieces { set_id: 2, at_least: 1 }.evaluate(&Ctx), None);
        assert_eq!(Condition::OnBar(Bar::Backup).evaluate(&Ctx), Some(false));
    }

    #[test]
    fn combat_time_is_undecided_without_state() {
        let c = Condition::TargetDebuffActive(1);
        assert!(!c.is_build_time());
        assert_eq!(c.evaluate(&Ctx), None);
    }

    #[test]
    fn three_valued_logic() {
        let unknown = Condition::InCombat;
        let no = Condition::OnBar(Bar::Backup);
        let yes = Condition::OnBar(Bar::Front);
        assert_eq!(Condition::All(vec![yes.clone(), unknown.clone()]).evaluate(&Ctx), None);
        assert_eq!(Condition::All(vec![no.clone(), unknown.clone()]).evaluate(&Ctx), Some(false));
        assert_eq!(Condition::Any(vec![no.clone(), unknown.clone()]).evaluate(&Ctx), None);
        assert_eq!(Condition::Any(vec![yes, unknown]).evaluate(&Ctx), Some(true));
        assert_eq!(Condition::Not(Box::new(no)).evaluate(&Ctx), Some(true));
    }

    #[test]
    fn serde_round_trip() {
        let c = Condition::All(vec![
            Condition::SetPieces { set_id: 7, at_least: 5 },
            Condition::Not(Box::new(Condition::OnBar(Bar::Backup))),
        ]);
        let json = serde_json::to_string(&c).unwrap();
        assert_eq!(serde_json::from_str::<Condition>(&json).unwrap(), c);
    }
}
