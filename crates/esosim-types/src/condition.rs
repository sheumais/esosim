//! When does an effect apply? (roadmap 1.5, Appendix A.2)
//!
//! `Condition` is plain data so it can live in effect files. Evaluation never touches the
//! engine: the caller supplies a [`ConditionContext`] (a player, an enemy, the simulator…)
//! and every question it cannot answer comes back as `None`. That gives three-valued logic:
//! `Some(true)`, `Some(false)`, or `None` = "cannot be decided here" (the UI shows it as
//! "conditional"; the calculator leaves the effect out).
//!
//! What a condition is about decides where it can be answered:
//!
//! * **build-time**: the gear, skills and bars alone decide it ([`Condition::is_build_time`]);
//!   the calculator evaluates these.
//! * **combat-time**: it needs a target or live state (health, buffs, posture, who you are
//!   fighting). The simulator answers these; the calculator shows the effect as "conditional".
//!
//! *Events* ("when you drink a potion") are not conditions; they are [`TriggerEvent`]s. A condition
//! can still ask whether one happened lately ([`Condition::RecentlyOccurred`]).
//!
//! Variants are only ever added, never changed, so effect files written for an older enum still
//! load.

use crate::channel::ResourceKind;
use crate::trigger::TriggerEvent;
use eso_skill_data::enums::skill_line::SkillLine;
use serde::{Deserialize, Serialize};
use std::fmt;

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

/// A weapon setup, as the skill lines of the same name name it ("while dual wielding").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponKind {
    OneHandAndShield,
    TwoHanded,
    DualWield,
    Bow,
    DestructionStaff,
    RestorationStaff,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PlayerClass {
    Dragonknight,
    Sorcerer,
    Nightblade,
    Warden,
    Necromancer,
    Templar,
    Arcanist,
}

/// What you are doing or have up, for the conditions that are just a yes/no about yourself.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SelfState {
    /// "while blocking", "while bracing"
    Blocking,
    /// "while sneaking", "while invisible", "while in stealth"
    Stealthed,
    Sprinting,
    Mounted,
    /// "while channeling this ability", "while using an ability with a channel or cast time"
    Channeling,
    /// "while active", "while toggled on": the ability this effect comes from is running
    SourceActive,
    /// "while you have a damage shield active"
    DamageShielded,
    /// "while you have a permanent summoned combat pet active"
    PetActive,
}

/// A crowd-control or debuff state that gates a bonus ("while stunned", "if the target is snared").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Affliction {
    Stunned,
    Immobilized,
    Snared,
    Feared,
    Disoriented,
    OffBalance,
    Silenced,
    Taunted,
}

/// A shape you take ("while in werewolf form").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Form {
    Werewolf,
    /// transformed into anything else
    Transformed,
}

/// What kind of enemy the effect is against ("against monsters", "against players").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TargetKind {
    /// dungeon, trial and arena monsters
    Monster,
    Boss,
    Player,
    Undead,
    Daedra,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub enum Condition {
    #[default]
    Always,
    All(Vec<Condition>),
    Any(Vec<Condition>),
    Not(Box<Condition>),


    SetPieces { set_id: u16, at_least: u8 },
    ArmourPieces { weight: ArmourWeight, at_least: u8 },
    SkillLineSlotted {
        #[serde(with = "crate::skill_serde::skill_line")]
        skill_line: SkillLine,
        at_least: u8,
    },
    AbilitySlotted { ability_id: u32 },
    OnBar(Bar),
    WeaponEquipped(WeaponKind),
    ClassIs(PlayerClass),
    /// "while you have a food buff active": the build names a food (Back-Alley Gourmand, Steadfast's
    /// Mettle, Full Belly Barricade).
    FoodActive,

    SelfBuffActive(u32),
    TargetDebuffActive(u32),
    TargetHealthBelowPct(u8),
    SelfHealthBelowPct(u8),
    InCombat,
    SelfBuffStacks { ability_id: u32, at_least: u8 },
    TargetDebuffStacks { ability_id: u32, at_least: u8 },
    SelfResourceBelowPct { resource: ResourceKind, pct: u8 },
    SelfResourceAbovePct { resource: ResourceKind, pct: u8 },
    TargetHealthAbovePct(u8),
    UltimateAtLeast(u16),
    SelfState(SelfState),
    SelfAfflicted(Affliction),
    TargetAfflicted(Affliction),
    InForm(Form),
    InPvpArea,
    TargetIs(TargetKind),
    SkillLineAbilityActive {
        #[serde(with = "crate::skill_serde::skill_line")]
        skill_line: SkillLine,
        at_least: u8,
    },
    RecentlyOccurred { event: TriggerEvent, within_ms: u32 },
}

/// What the evaluator knows. Every method defaults to "unknown", so a context only
/// implements what it can actually answer.
pub trait ConditionContext {
    // --- build-time ---
    fn set_pieces(&self, _set_id: u16) -> Option<u8> { None }
    fn armour_pieces(&self, _weight: ArmourWeight) -> Option<u8> { None }
    fn skill_line_slotted(&self, _line: SkillLine) -> Option<u8> { None }
    fn ability_slotted(&self, _ability_id: u32) -> Option<bool> { None }
    fn on_bar(&self, _bar: Bar) -> Option<bool> { None }
    fn weapon_equipped(&self, _kind: WeaponKind) -> Option<bool> { None }
    fn class(&self) -> Option<PlayerClass> { None }
    /// Whether the build has a food or drink buff on.
    fn food_active(&self) -> Option<bool> { None }

    // --- combat-time ---
    fn self_buff_active(&self, _id: u32) -> Option<bool> { None }
    fn target_debuff_active(&self, _id: u32) -> Option<bool> { None }
    /// Stacks of one of your buffs. The default derives 0 or 1 from [`self_buff_active`](Self::self_buff_active).
    fn self_buff_stacks(&self, id: u32) -> Option<u32> {
        self.self_buff_active(id).map(u32::from)
    }
    /// Stacks of a debuff on the target. The default derives 0 or 1 from
    /// [`target_debuff_active`](Self::target_debuff_active).
    fn target_debuff_stacks(&self, id: u32) -> Option<u32> {
        self.target_debuff_active(id).map(u32::from)
    }
    fn target_health_pct(&self) -> Option<u8> { None }
    fn self_health_pct(&self) -> Option<u8> { None }
    /// Percentage of a resource. The default answers health from [`self_health_pct`](Self::self_health_pct).
    fn self_resource_pct(&self, resource: ResourceKind) -> Option<u8> {
        match resource {
            ResourceKind::Health => self.self_health_pct(),
            _ => None,
        }
    }
    fn ultimate(&self) -> Option<u16> { None }
    fn in_combat(&self) -> Option<bool> { None }
    fn self_state(&self, _state: SelfState) -> Option<bool> { None }
    fn self_afflicted(&self, _a: Affliction) -> Option<bool> { None }
    fn target_afflicted(&self, _a: Affliction) -> Option<bool> { None }
    fn in_form(&self, _form: Form) -> Option<bool> { None }
    fn in_pvp_area(&self) -> Option<bool> { None }
    fn target_is(&self, _kind: TargetKind) -> Option<bool> { None }
    /// How many of your active abilities belong to the line.
    fn skill_line_active(&self, _line: SkillLine) -> Option<u8> { None }
    /// Milliseconds since `event` last happened; `Some(u32::MAX)` = it has not happened.
    fn ms_since(&self, _event: TriggerEvent) -> Option<u32> { None }
}

/// Knows nothing: only `Always` (and combinations that reduce to it) evaluate.
pub struct NoContext;
impl ConditionContext for NoContext {}

impl Condition {
    pub fn not(c: Condition) -> Condition {
        Condition::Not(Box::new(c))
    }

    /// "while out of combat".
    pub fn out_of_combat() -> Condition {
        Condition::not(Condition::InCombat)
    }

    /// Everything must hold. Nested `All`s are flattened, `Always` is dropped, one condition is
    /// itself and none is `Always`.
    pub fn all(parts: impl IntoIterator<Item = Condition>) -> Condition {
        let mut out: Vec<Condition> = Vec::new();
        for p in parts {
            match p {
                Condition::Always => {}
                Condition::All(inner) => match Condition::all(inner) {
                    Condition::Always => {}
                    Condition::All(flat) => out.extend(flat),
                    other => out.push(other),
                },
                other => out.push(other),
            }
        }
        match out.len() {
            0 => Condition::Always,
            1 => out.remove(0),
            _ => Condition::All(out),
        }
    }

    /// True when the build alone decides this condition (no target, no live state).
    pub fn is_build_time(&self) -> bool {
        match self {
            Condition::Always
            | Condition::SetPieces { .. }
            | Condition::ArmourPieces { .. }
            | Condition::SkillLineSlotted { .. }
            | Condition::AbilitySlotted { .. }
            | Condition::OnBar(_)
            | Condition::WeaponEquipped(_)
            | Condition::ClassIs(_)
            | Condition::FoodActive => true,
            Condition::All(v) | Condition::Any(v) => v.iter().all(Condition::is_build_time),
            Condition::Not(c) => c.is_build_time(),
            Condition::SelfBuffActive(_)
            | Condition::TargetDebuffActive(_)
            | Condition::TargetHealthBelowPct(_)
            | Condition::SelfHealthBelowPct(_)
            | Condition::InCombat
            | Condition::SelfBuffStacks { .. }
            | Condition::TargetDebuffStacks { .. }
            | Condition::SelfResourceBelowPct { .. }
            | Condition::SelfResourceAbovePct { .. }
            | Condition::TargetHealthAbovePct(_)
            | Condition::UltimateAtLeast(_)
            | Condition::SelfState(_)
            | Condition::SelfAfflicted(_)
            | Condition::TargetAfflicted(_)
            | Condition::InForm(_)
            | Condition::InPvpArea
            | Condition::TargetIs(_)
            | Condition::SkillLineAbilityActive { .. }
            | Condition::RecentlyOccurred { .. } => false,
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
            Condition::WeaponEquipped(kind) => ctx.weapon_equipped(*kind),
            Condition::ClassIs(class) => ctx.class().map(|c| c == *class),
            Condition::FoodActive => ctx.food_active(),
            Condition::SelfBuffActive(id) => ctx.self_buff_active(*id),
            Condition::TargetDebuffActive(id) => ctx.target_debuff_active(*id),
            Condition::TargetHealthBelowPct(p) => ctx.target_health_pct().map(|h| h < *p),
            Condition::SelfHealthBelowPct(p) => ctx.self_health_pct().map(|h| h < *p),
            Condition::InCombat => ctx.in_combat(),
            Condition::SelfBuffStacks { ability_id, at_least } => {
                ctx.self_buff_stacks(*ability_id).map(|n| n >= u32::from(*at_least))
            }
            Condition::TargetDebuffStacks { ability_id, at_least } => {
                ctx.target_debuff_stacks(*ability_id).map(|n| n >= u32::from(*at_least))
            }
            Condition::SelfResourceBelowPct { resource, pct } => {
                ctx.self_resource_pct(*resource).map(|v| v < *pct)
            }
            Condition::SelfResourceAbovePct { resource, pct } => {
                ctx.self_resource_pct(*resource).map(|v| v >= *pct)
            }
            Condition::TargetHealthAbovePct(p) => ctx.target_health_pct().map(|h| h >= *p),
            Condition::UltimateAtLeast(n) => ctx.ultimate().map(|u| u >= *n),
            Condition::SelfState(state) => ctx.self_state(*state),
            Condition::SelfAfflicted(a) => ctx.self_afflicted(*a),
            Condition::TargetAfflicted(a) => ctx.target_afflicted(*a),
            Condition::InForm(form) => ctx.in_form(*form),
            Condition::InPvpArea => ctx.in_pvp_area(),
            Condition::TargetIs(kind) => ctx.target_is(*kind),
            Condition::SkillLineAbilityActive { skill_line, at_least } => {
                ctx.skill_line_active(*skill_line).map(|n| n >= *at_least)
            }
            Condition::RecentlyOccurred { event, within_ms } => {
                ctx.ms_since(*event).map(|ms| ms <= *within_ms)
            }
        }
    }

    /// Ids of buffs/debuffs this condition refers to (for dangling-id checks).
    pub fn referenced_effect_ids(&self, out: &mut Vec<u32>) {
        match self {
            Condition::All(v) | Condition::Any(v) => v.iter().for_each(|c| c.referenced_effect_ids(out)),
            Condition::Not(c) => c.referenced_effect_ids(out),
            Condition::SelfBuffActive(id) | Condition::TargetDebuffActive(id) => out.push(*id),
            Condition::SelfBuffStacks { ability_id, .. } | Condition::TargetDebuffStacks { ability_id, .. } => {
                out.push(*ability_id)
            }
            _ => {}
        }
    }
}

/// The condition as UI text ("Conditional effects listed separately with their condition text",
/// roadmap 3.7).
impl fmt::Display for Condition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fn list(f: &mut fmt::Formatter<'_>, v: &[Condition], sep: &str) -> fmt::Result {
            for (i, c) in v.iter().enumerate() {
                if i > 0 {
                    f.write_str(sep)?;
                }
                match c {
                    Condition::All(_) | Condition::Any(_) => write!(f, "({c})")?,
                    _ => write!(f, "{c}")?,
                }
            }
            Ok(())
        }
        match self {
            Condition::Always => f.write_str("always"),
            Condition::All(v) if v.is_empty() => f.write_str("always"),
            Condition::Any(v) if v.is_empty() => f.write_str("never"),
            Condition::All(v) => list(f, v, " and "),
            Condition::Any(v) => list(f, v, " or "),
            Condition::Not(c) => match &**c {
                Condition::InCombat => f.write_str("out of combat"),
                other => write!(f, "not ({other})"),
            },
            Condition::SetPieces { set_id, at_least } => write!(f, "{at_least} or more pieces of set {set_id}"),
            Condition::ArmourPieces { weight, at_least } => {
                write!(f, "{at_least} or more pieces of {} armour", format!("{weight:?}").to_lowercase())
            }
            Condition::SkillLineSlotted { skill_line, at_least } => {
                write!(f, "{at_least} or more {skill_line} abilities slotted")
            }
            Condition::AbilitySlotted { ability_id } => write!(f, "ability {ability_id} slotted"),
            Condition::OnBar(Bar::Front) => f.write_str("on the front bar"),
            Condition::OnBar(Bar::Backup) => f.write_str("on the backup bar"),
            Condition::WeaponEquipped(kind) => write!(f, "wielding {kind:?}"),
            Condition::ClassIs(class) => write!(f, "as a {class:?}"),
            Condition::FoodActive => f.write_str("you have a food buff active"),
            Condition::SelfBuffActive(id) => write!(f, "buff {id} on you"),
            Condition::TargetDebuffActive(id) => write!(f, "debuff {id} on the target"),
            Condition::TargetHealthBelowPct(p) => write!(f, "target below {p}% health"),
            Condition::SelfHealthBelowPct(p) => write!(f, "you are below {p}% health"),
            Condition::InCombat => f.write_str("in combat"),
            Condition::SelfBuffStacks { ability_id, at_least } => {
                write!(f, "{at_least} or more stacks of buff {ability_id} on you")
            }
            Condition::TargetDebuffStacks { ability_id, at_least } => {
                write!(f, "{at_least} or more stacks of debuff {ability_id} on the target")
            }
            Condition::SelfResourceBelowPct { resource, pct } => {
                write!(f, "you are below {pct}% {}", format!("{resource:?}").to_lowercase())
            }
            Condition::SelfResourceAbovePct { resource, pct: 100 } => {
                write!(f, "you are at full {}", format!("{resource:?}").to_lowercase())
            }
            Condition::SelfResourceAbovePct { resource, pct } => {
                write!(f, "you have {pct}% or more {}", format!("{resource:?}").to_lowercase())
            }
            Condition::TargetHealthAbovePct(p) => write!(f, "target at {p}% health or more"),
            Condition::UltimateAtLeast(n) => write!(f, "you have {n} or more ultimate"),
            Condition::SelfState(state) => write!(f, "{}", format!("{state:?}").to_lowercase()),
            Condition::SelfAfflicted(a) => write!(f, "you are {}", format!("{a:?}").to_lowercase()),
            Condition::TargetAfflicted(a) => write!(f, "the target is {}", format!("{a:?}").to_lowercase()),
            Condition::InForm(form) => write!(f, "in {} form", format!("{form:?}").to_lowercase()),
            Condition::InPvpArea => f.write_str("in a PvP area"),
            Condition::TargetIs(kind) => write!(f, "against {}", format!("{kind:?}").to_lowercase()),
            Condition::SkillLineAbilityActive { skill_line, at_least } => {
                write!(f, "{at_least} or more {skill_line} abilities active")
            }
            Condition::RecentlyOccurred { event, within_ms } => {
                write!(f, "within {} ms of when {event}", within_ms)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::channel::DamageDoneContext;

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

    /// Every variant, for the tests that must not forget one.
    fn every_variant() -> Vec<Condition> {
        vec![
            Condition::Always,
            Condition::All(vec![]),
            Condition::Any(vec![]),
            Condition::not(Condition::InCombat),
            Condition::SetPieces { set_id: 1, at_least: 5 },
            Condition::ArmourPieces { weight: ArmourWeight::Heavy, at_least: 5 },
            Condition::SkillLineSlotted { skill_line: SkillLine::Support, at_least: 1 },
            Condition::AbilitySlotted { ability_id: 9 },
            Condition::OnBar(Bar::Front),
            Condition::WeaponEquipped(WeaponKind::DualWield),
            Condition::ClassIs(PlayerClass::Necromancer),
            Condition::FoodActive,
            Condition::SelfBuffActive(1),
            Condition::TargetDebuffActive(2),
            Condition::TargetHealthBelowPct(25),
            Condition::SelfHealthBelowPct(50),
            Condition::InCombat,
            Condition::SelfBuffStacks { ability_id: 3, at_least: 5 },
            Condition::TargetDebuffStacks { ability_id: 4, at_least: 2 },
            Condition::SelfResourceBelowPct { resource: ResourceKind::Stamina, pct: 50 },
            Condition::SelfResourceAbovePct { resource: ResourceKind::Magicka, pct: 50 },
            Condition::TargetHealthAbovePct(90),
            Condition::UltimateAtLeast(50),
            Condition::SelfState(SelfState::Blocking),
            Condition::SelfAfflicted(Affliction::Stunned),
            Condition::TargetAfflicted(Affliction::Snared),
            Condition::InForm(Form::Werewolf),
            Condition::InPvpArea,
            Condition::TargetIs(TargetKind::Monster),
            Condition::SkillLineAbilityActive { skill_line: SkillLine::BoneTyrant, at_least: 1 },
            Condition::RecentlyOccurred { event: TriggerEvent::Kill, within_ms: 2000 },
        ]
    }

    #[test]
    fn every_variant_serialises_and_reads_as_text() {
        for c in every_variant() {
            let json = serde_json::to_string(&c).unwrap();
            assert_eq!(serde_json::from_str::<Condition>(&json).unwrap(), c, "{json}");
            assert!(!c.to_string().is_empty(), "{c:?}");
        }
    }

    #[test]
    fn nothing_new_is_decidable_without_a_context_except_the_empty_combinations() {
        for c in every_variant() {
            let want = match c {
                Condition::Always | Condition::All(_) => Some(true),
                Condition::Any(_) => Some(false),
                _ => None,
            };
            assert_eq!(c.evaluate(&NoContext), want, "{c:?}");
        }
    }

    #[test]
    fn build_time_versus_combat_time() {
        let build: Vec<bool> = every_variant().iter().map(Condition::is_build_time).collect();
        // Always, All([]), Any([]), Not(InCombat), then the build-time block, then combat-time
        assert_eq!(&build[..4], &[true, true, true, false]);
        assert!(build[4..12].iter().all(|b| *b), "set, armour, line slotted, ability slotted, bar, weapon, class, food");
        assert!(build[12..].iter().all(|b| !*b), "everything after the food is combat-time");
        assert!(Condition::All(vec![Condition::WeaponEquipped(WeaponKind::Bow), Condition::OnBar(Bar::Front)]).is_build_time());
        assert!(!Condition::All(vec![Condition::WeaponEquipped(WeaponKind::Bow), Condition::InCombat]).is_build_time());
    }

    /// A fight in progress: 40 % health, 60 % stamina, a snared monster at 20 % health.
    struct Fight;
    impl ConditionContext for Fight {
        fn weapon_equipped(&self, k: WeaponKind) -> Option<bool> { Some(k == WeaponKind::DualWield) }
        fn class(&self) -> Option<PlayerClass> { Some(PlayerClass::Necromancer) }
        fn food_active(&self) -> Option<bool> { Some(true) }
        fn self_buff_active(&self, id: u32) -> Option<bool> { Some(id == 3) }
        fn self_buff_stacks(&self, id: u32) -> Option<u32> { Some(if id == 3 { 5 } else { 0 }) }
        fn target_health_pct(&self) -> Option<u8> { Some(20) }
        fn self_health_pct(&self) -> Option<u8> { Some(40) }
        fn self_resource_pct(&self, r: ResourceKind) -> Option<u8> {
            Some(match r {
                ResourceKind::Health => 40,
                ResourceKind::Stamina => 60,
                ResourceKind::Magicka => 100,
            })
        }
        fn ultimate(&self) -> Option<u16> { Some(120) }
        fn in_combat(&self) -> Option<bool> { Some(true) }
        fn self_state(&self, s: SelfState) -> Option<bool> { Some(s == SelfState::Blocking) }
        fn self_afflicted(&self, a: Affliction) -> Option<bool> { Some(a == Affliction::Stunned) }
        fn target_afflicted(&self, a: Affliction) -> Option<bool> { Some(a == Affliction::Snared) }
        fn in_form(&self, f: Form) -> Option<bool> { Some(f == Form::Werewolf) }
        fn in_pvp_area(&self) -> Option<bool> { Some(false) }
        fn target_is(&self, k: TargetKind) -> Option<bool> { Some(k == TargetKind::Monster) }
        fn skill_line_active(&self, l: SkillLine) -> Option<u8> { Some(if l == SkillLine::BoneTyrant { 2 } else { 0 }) }
        fn ms_since(&self, e: TriggerEvent) -> Option<u32> {
            Some(if e == TriggerEvent::Kill { 1500 } else { u32::MAX })
        }
    }

    #[test]
    fn combat_time_conditions_evaluate_against_a_fight() {
        let yes = |c: Condition| assert_eq!(c.evaluate(&Fight), Some(true), "{c}");
        let no = |c: Condition| assert_eq!(c.evaluate(&Fight), Some(false), "{c}");
        yes(Condition::WeaponEquipped(WeaponKind::DualWield));
        no(Condition::WeaponEquipped(WeaponKind::Bow));
        yes(Condition::ClassIs(PlayerClass::Necromancer));
        no(Condition::ClassIs(PlayerClass::Templar));
        yes(Condition::FoodActive);
        yes(Condition::SelfBuffStacks { ability_id: 3, at_least: 5 });
        no(Condition::SelfBuffStacks { ability_id: 3, at_least: 6 });
        no(Condition::SelfBuffStacks { ability_id: 4, at_least: 1 });
        yes(Condition::SelfHealthBelowPct(50));
        yes(Condition::SelfResourceBelowPct { resource: ResourceKind::Health, pct: 50 });
        no(Condition::SelfResourceBelowPct { resource: ResourceKind::Stamina, pct: 50 });
        yes(Condition::SelfResourceAbovePct { resource: ResourceKind::Stamina, pct: 60 });
        yes(Condition::SelfResourceAbovePct { resource: ResourceKind::Magicka, pct: 100 });
        no(Condition::SelfResourceAbovePct { resource: ResourceKind::Health, pct: 100 });
        yes(Condition::TargetHealthBelowPct(25));
        no(Condition::TargetHealthAbovePct(90));
        yes(Condition::UltimateAtLeast(120));
        no(Condition::UltimateAtLeast(121));
        yes(Condition::SelfState(SelfState::Blocking));
        no(Condition::SelfState(SelfState::Mounted));
        yes(Condition::SelfAfflicted(Affliction::Stunned));
        yes(Condition::TargetAfflicted(Affliction::Snared));
        no(Condition::TargetAfflicted(Affliction::Stunned));
        yes(Condition::InForm(Form::Werewolf));
        no(Condition::InPvpArea);
        yes(Condition::TargetIs(TargetKind::Monster));
        no(Condition::TargetIs(TargetKind::Player));
        yes(Condition::SkillLineAbilityActive { skill_line: SkillLine::BoneTyrant, at_least: 2 });
        no(Condition::SkillLineAbilityActive { skill_line: SkillLine::BoneTyrant, at_least: 3 });
        yes(Condition::RecentlyOccurred { event: TriggerEvent::Kill, within_ms: 2000 });
        no(Condition::RecentlyOccurred { event: TriggerEvent::Kill, within_ms: 1000 });
        // "has not happened" is a definite no, not an unknown
        no(Condition::RecentlyOccurred { event: TriggerEvent::Block, within_ms: 60_000 });
        no(Condition::out_of_combat());
    }

    #[test]
    fn the_default_stack_and_resource_answers_follow_the_simple_questions() {
        struct Simple;
        impl ConditionContext for Simple {
            fn self_buff_active(&self, id: u32) -> Option<bool> { Some(id == 1) }
            fn self_health_pct(&self) -> Option<u8> { Some(30) }
        }
        assert_eq!(Condition::SelfBuffStacks { ability_id: 1, at_least: 1 }.evaluate(&Simple), Some(true));
        assert_eq!(Condition::SelfBuffStacks { ability_id: 1, at_least: 2 }.evaluate(&Simple), Some(false));
        assert_eq!(Condition::SelfBuffStacks { ability_id: 2, at_least: 1 }.evaluate(&Simple), Some(false));
        assert_eq!(Condition::SelfResourceBelowPct { resource: ResourceKind::Health, pct: 50 }.evaluate(&Simple), Some(true));
        // only health has a default answer
        assert_eq!(Condition::SelfResourceBelowPct { resource: ResourceKind::Stamina, pct: 50 }.evaluate(&Simple), None);
    }

    #[test]
    fn all_flattens_and_drops_always() {
        assert_eq!(Condition::all([]), Condition::Always);
        assert_eq!(Condition::all([Condition::Always, Condition::InCombat]), Condition::InCombat);
        assert_eq!(
            Condition::all([Condition::InCombat, Condition::All(vec![Condition::InPvpArea, Condition::Always])]),
            Condition::All(vec![Condition::InCombat, Condition::InPvpArea])
        );
    }

    #[test]
    fn buff_stacks_are_dangling_id_candidates_too() {
        let c = Condition::All(vec![
            Condition::SelfBuffStacks { ability_id: 3, at_least: 5 },
            Condition::not(Condition::TargetDebuffStacks { ability_id: 4, at_least: 1 }),
            Condition::SelfBuffActive(5),
            Condition::AbilitySlotted { ability_id: 6 },
        ]);
        let mut ids = Vec::new();
        c.referenced_effect_ids(&mut ids);
        assert_eq!(ids, [3, 4, 5]);
    }

    #[test]
    fn conditions_read_as_text() {
        assert_eq!(Condition::InCombat.to_string(), "in combat");
        assert_eq!(Condition::out_of_combat().to_string(), "out of combat");
        assert_eq!(Condition::SelfResourceAbovePct { resource: ResourceKind::Health, pct: 100 }.to_string(), "you are at full health");
        assert_eq!(
            Condition::All(vec![
                Condition::ArmourPieces { weight: ArmourWeight::Heavy, at_least: 5 },
                Condition::Any(vec![Condition::InCombat, Condition::InPvpArea]),
            ])
            .to_string(),
            "5 or more pieces of heavy armour and (in combat or in a PvP area)"
        );
        assert_eq!(
            Condition::RecentlyOccurred { event: TriggerEvent::DealDamage(DamageDoneContext::All), within_ms: 2000 }.to_string(),
            "within 2000 ms of when you deal damage"
        );
    }
}
