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
use std::collections::HashMap;
use std::fmt;
use std::sync::LazyLock;

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

/// The type of one weapon, as the weapon passives name it ("each axe", "swords", "Inferno Staves"). A two-handed sword
/// is a sword like a one-handed one; a shield and a bow are not here.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WeaponType {
    Sword,
    Axe,
    Mace,
    Dagger,
    /// An Inferno Staff.
    FireStaff,
    /// An Ice Staff.
    FrostStaff,
    LightningStaff,
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

impl PlayerClass {
    /// The names of the class's three own skill lines, as `SkillLine::as_str` writes them. Weapon, armour, guild,
    /// world, Alliance War and racial lines are not the class's own.
    pub fn skill_line_names(self) -> [&'static str; 3] {
        match self {
            PlayerClass::Dragonknight => ["Ardent Flame", "Draconic Power", "Earthen Heart"],
            PlayerClass::Sorcerer => ["Dark Magic", "Daedric Summoning", "Storm Calling"],
            PlayerClass::Nightblade => ["Assassination", "Shadow", "Siphoning"],
            PlayerClass::Warden => ["Animal Companions", "Green Balance", "Winter's Embrace"],
            PlayerClass::Necromancer => ["Bone Tyrant", "Grave Lord", "Living Death"],
            PlayerClass::Templar => ["Aedric Spear", "Dawn's Wrath", "Restoring Light"],
            PlayerClass::Arcanist => ["Herald of the Tome", "Soldier of Apocrypha", "Curative Runeforms"],
        }
    }

    /// The class's own skill lines (`Condition::ClassAbilitiesSlotted` counts the abilities of these). Looked up by
    /// name among the lines `eso_skill_data` knows, once; a name it does not know is left out, and a test pins that
    /// every class has all three.
    pub fn skill_lines(self) -> &'static [SkillLine] {
        static LINES: LazyLock<HashMap<PlayerClass, Vec<SkillLine>>> = LazyLock::new(|| {
            use PlayerClass::*;
            [Dragonknight, Sorcerer, Nightblade, Warden, Necromancer, Templar, Arcanist]
                .into_iter()
                .map(|class| {
                    let lines = class
                        .skill_line_names()
                        .into_iter()
                        .filter_map(crate::skill_serde::skill_line_named)
                        .collect();
                    (class, lines)
                })
                .collect()
        });
        LINES.get(&self).map(Vec::as_slice).unwrap_or(&[])
    }
}

/// Where the character is, for the conditions that depend on it ("when you are near a Keep", "in a PvP area"). The
/// build chooses it (`Build::location`); a build that does not say leaves every location condition undecided, which
/// is what it was before there was a way to say.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Location {
    /// Not in a PvP area: dungeons, trials, overland.
    Pve,
    /// In a PvP area (Cyrodiil, Imperial City, battlegrounds) but not near a Keep.
    PvpArea,
    /// Near a Keep in a PvP area. This is also "in a PvP area".
    NearKeep,
}

impl Location {
    pub fn in_pvp_area(self) -> bool {
        !matches!(self, Location::Pve)
    }

    pub fn near_keep(self) -> bool {
        matches!(self, Location::NearKeep)
    }
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
    /// The status effects (Spell Attunement: "doubled if you are afflicted with Burning, Chilled or Concussed").
    Burning,
    Chilled,
    Concussed,
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
    /// "when you are near a Keep" (Combat Medic): decided by `Build::location`.
    NearKeep,
    /// "per type of armour worn": how many of the three weights (Heavy, Medium, Light) have at least one piece on
    /// the active bar's gear, 0 to 3 (Undaunted Mettle). `ArmourPieces` counts pieces of one weight; this counts
    /// weights.
    ArmourWeightsWorn { at_least: u8 },
    /// "for each Sorcerer / Nightblade ability slotted": the abilities on the active bar from all three skill lines
    /// of the character's own class (`PlayerClass::skill_lines`), in one number (Expert Mage, Pressure Points).
    /// Weapon, guild, world and Alliance War lines do not count; what is on the bar counts, an ultimate included.
    ClassAbilitiesSlotted { at_least: u8 },
    /// The ability is slotted on either bar, not only the active one: for the skills that give their buff from
    /// whichever bar they are on (Dark Cloak, Bound Aegis). Which skills do is game data and is listed by hand.
    AbilitySlottedAnyBar { ability_id: u32 },
    /// How many weapons of one type are in the hands on the active bar: a one-handed sword in each hand is two, a
    /// two-handed sword is one (Twin Blade and Blunt: "each axe"; Heavy Weapons: "swords"; Ancient Knowledge: the staff).
    WeaponTypeEquipped { weapon: WeaponType, at_least: u8 },
    /// The target is this many metres away or closer (Vinedusk Training: "enemies 15 meters or closer"); "further than"
    /// is `Not` of it. Combat-time: the calculator has no distance and lists the effect as conditional.
    TargetWithinMeters(u8),
    /// You are flanking the target (Master Assassin). Combat-time.
    TargetFlanked,
    /// You have a negative effect on you (Curative Curse). Combat-time.
    SelfDebuffed,
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
    /// Distinct armour weights worn (0 to 3).
    fn armour_weights_worn(&self) -> Option<u8> { None }
    /// Abilities on the active bar that belong to the character's own class lines.
    fn class_abilities_slotted(&self) -> Option<u8> { None }
    /// Whether the ability is slotted on either bar. There is no default from `ability_slotted`: an ability that is
    /// not on the active bar may well be on the other.
    fn ability_slotted_any_bar(&self, _ability_id: u32) -> Option<bool> { None }
    /// Whether the character is near a Keep (`Build::location`).
    fn near_keep(&self) -> Option<bool> { None }
    /// How many weapons of the type are in the hands on the active bar.
    fn weapons_of_type(&self, _weapon: WeaponType) -> Option<u8> { None }

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
    /// Distance to the target in metres.
    fn target_distance(&self) -> Option<f32> { None }
    /// Whether you are flanking the target.
    fn target_flanked(&self) -> Option<bool> { None }
    /// Whether you have a negative effect on you.
    fn self_debuffed(&self) -> Option<bool> { None }
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
            | Condition::FoodActive
            | Condition::InPvpArea
            | Condition::NearKeep
            | Condition::ArmourWeightsWorn { .. }
            | Condition::ClassAbilitiesSlotted { .. }
            | Condition::AbilitySlottedAnyBar { .. }
            | Condition::WeaponTypeEquipped { .. } => true,
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
            | Condition::TargetIs(_)
            | Condition::SkillLineAbilityActive { .. }
            | Condition::RecentlyOccurred { .. }
            | Condition::TargetWithinMeters(_)
            | Condition::TargetFlanked
            | Condition::SelfDebuffed => false,
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
            Condition::NearKeep => ctx.near_keep(),
            Condition::ArmourWeightsWorn { at_least } => ctx.armour_weights_worn().map(|n| n >= *at_least),
            Condition::ClassAbilitiesSlotted { at_least } => ctx.class_abilities_slotted().map(|n| n >= *at_least),
            Condition::AbilitySlottedAnyBar { ability_id } => ctx.ability_slotted_any_bar(*ability_id),
            Condition::WeaponTypeEquipped { weapon, at_least } => ctx.weapons_of_type(*weapon).map(|n| n >= *at_least),
            Condition::TargetWithinMeters(m) => ctx.target_distance().map(|d| d <= f32::from(*m)),
            Condition::TargetFlanked => ctx.target_flanked(),
            Condition::SelfDebuffed => ctx.self_debuffed(),
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
            Condition::NearKeep => f.write_str("near a Keep"),
            Condition::ArmourWeightsWorn { at_least } => write!(f, "{at_least} or more types of armour worn"),
            Condition::ClassAbilitiesSlotted { at_least } => {
                write!(f, "{at_least} or more abilities of your class slotted")
            }
            Condition::AbilitySlottedAnyBar { ability_id } => write!(f, "ability {ability_id} slotted on either bar"),
            Condition::WeaponTypeEquipped { weapon, at_least } => {
                write!(f, "{at_least} or more {} wielded", format!("{weapon:?}").to_lowercase())
            }
            Condition::TargetWithinMeters(m) => write!(f, "the target is {m} m away or closer"),
            Condition::TargetFlanked => f.write_str("flanking the target"),
            Condition::SelfDebuffed => f.write_str("you have a negative effect on you"),
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
            Condition::NearKeep,
            Condition::ArmourWeightsWorn { at_least: 2 },
            Condition::ClassAbilitiesSlotted { at_least: 3 },
            Condition::AbilitySlottedAnyBar { ability_id: 9 },
            Condition::WeaponTypeEquipped { weapon: WeaponType::Axe, at_least: 2 },
            Condition::TargetWithinMeters(15),
            Condition::TargetFlanked,
            Condition::SelfDebuffed,
        ]
    }

    /// The conditions the build alone decides, by name, so adding a variant forces a decision here.
    fn is_build_time_by_hand(c: &Condition) -> bool {
        matches!(
            c,
            Condition::Always
                | Condition::All(_)
                | Condition::Any(_)
                | Condition::SetPieces { .. }
                | Condition::ArmourPieces { .. }
                | Condition::SkillLineSlotted { .. }
                | Condition::AbilitySlotted { .. }
                | Condition::OnBar(_)
                | Condition::WeaponEquipped(_)
                | Condition::ClassIs(_)
                | Condition::FoodActive
                | Condition::InPvpArea
                | Condition::NearKeep
                | Condition::ArmourWeightsWorn { .. }
                | Condition::ClassAbilitiesSlotted { .. }
                | Condition::AbilitySlottedAnyBar { .. }
                | Condition::WeaponTypeEquipped { .. }
        )
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
        for c in every_variant() {
            // `Not(InCombat)` is the one wrapper in the list that is combat-time
            let want = is_build_time_by_hand(&c);
            assert_eq!(c.is_build_time(), want, "{c:?}");
        }
        // the location is chosen with the build, so it is a build-time question now
        assert!(Condition::InPvpArea.is_build_time() && Condition::NearKeep.is_build_time());
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
        fn near_keep(&self) -> Option<bool> { Some(false) }
        fn armour_weights_worn(&self) -> Option<u8> { Some(2) }
        fn class_abilities_slotted(&self) -> Option<u8> { Some(4) }
        fn ability_slotted_any_bar(&self, id: u32) -> Option<bool> { Some(id == 9) }
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
        no(Condition::NearKeep);
        yes(Condition::ArmourWeightsWorn { at_least: 2 });
        no(Condition::ArmourWeightsWorn { at_least: 3 });
        yes(Condition::ClassAbilitiesSlotted { at_least: 4 });
        no(Condition::ClassAbilitiesSlotted { at_least: 5 });
        yes(Condition::AbilitySlottedAnyBar { ability_id: 9 });
        no(Condition::AbilitySlottedAnyBar { ability_id: 10 });
        yes(Condition::SkillLineAbilityActive { skill_line: SkillLine::BoneTyrant, at_least: 2 });
        no(Condition::SkillLineAbilityActive { skill_line: SkillLine::BoneTyrant, at_least: 3 });
        yes(Condition::RecentlyOccurred { event: TriggerEvent::Kill, within_ms: 2000 });
        no(Condition::RecentlyOccurred { event: TriggerEvent::Kill, within_ms: 1000 });
        // "has not happened" is a definite no, not an unknown
        no(Condition::RecentlyOccurred { event: TriggerEvent::Block, within_ms: 60_000 });
        no(Condition::out_of_combat());
    }

    #[test]
    fn a_location_answers_both_location_questions() {
        assert_eq!((Location::Pve.in_pvp_area(), Location::Pve.near_keep()), (false, false));
        assert_eq!((Location::PvpArea.in_pvp_area(), Location::PvpArea.near_keep()), (true, false));
        // near a Keep is also in a PvP area
        assert_eq!((Location::NearKeep.in_pvp_area(), Location::NearKeep.near_keep()), (true, true));
        let json = serde_json::to_string(&Location::NearKeep).unwrap();
        assert_eq!(serde_json::from_str::<Location>(&json).unwrap(), Location::NearKeep);
    }

    #[test]
    fn every_class_has_its_three_own_skill_lines() {
        for class in [
            PlayerClass::Dragonknight,
            PlayerClass::Sorcerer,
            PlayerClass::Nightblade,
            PlayerClass::Warden,
            PlayerClass::Necromancer,
            PlayerClass::Templar,
            PlayerClass::Arcanist,
        ] {
            let lines = class.skill_lines();
            assert_eq!(lines.len(), 3, "{class:?}: a line name the parser does not know ({:?})", class.skill_line_names());
            for (line, name) in lines.iter().zip(class.skill_line_names()) {
                assert_eq!(line.as_str(), name);
            }
        }
        // no line belongs to two classes
        let mut all: Vec<SkillLine> = Vec::new();
        for class in [PlayerClass::Dragonknight, PlayerClass::Sorcerer, PlayerClass::Nightblade, PlayerClass::Warden, PlayerClass::Necromancer, PlayerClass::Templar, PlayerClass::Arcanist] {
            all.extend_from_slice(class.skill_lines());
        }
        let n = all.len();
        all.sort_by_key(|l| l.as_str());
        all.dedup();
        assert_eq!(all.len(), n);
    }

    #[test]
    fn the_new_conditions_round_trip_and_read_as_text() {
        assert_eq!(Condition::NearKeep.to_string(), "near a Keep");
        assert_eq!(Condition::ArmourWeightsWorn { at_least: 3 }.to_string(), "3 or more types of armour worn");
        assert_eq!(Condition::ClassAbilitiesSlotted { at_least: 1 }.to_string(), "1 or more abilities of your class slotted");
        assert_eq!(Condition::AbilitySlottedAnyBar { ability_id: 7 }.to_string(), "ability 7 slotted on either bar");
        let json = serde_json::to_string(&Condition::ClassAbilitiesSlotted { at_least: 1 }).unwrap();
        assert_eq!(json, r#"{"ClassAbilitiesSlotted":{"at_least":1}}"#);
        assert_eq!(serde_json::to_string(&Condition::NearKeep).unwrap(), r#""NearKeep""#);
    }

    #[test]
    fn weapon_type_distance_flank_and_debuff_conditions() {
        struct Hands;
        impl ConditionContext for Hands {
            fn weapons_of_type(&self, weapon: WeaponType) -> Option<u8> {
                Some(match weapon {
                    WeaponType::Axe => 2,
                    WeaponType::Dagger => 1,
                    _ => 0,
                })
            }
            fn target_distance(&self) -> Option<f32> { Some(15.0) }
            fn target_flanked(&self) -> Option<bool> { Some(true) }
            fn self_debuffed(&self) -> Option<bool> { Some(false) }
        }
        let has = |weapon, at_least| Condition::WeaponTypeEquipped { weapon, at_least };
        assert_eq!(has(WeaponType::Axe, 2).evaluate(&Hands), Some(true));
        assert_eq!(has(WeaponType::Axe, 3).evaluate(&Hands), Some(false));
        assert_eq!(has(WeaponType::Sword, 1).evaluate(&Hands), Some(false));
        assert!(has(WeaponType::Axe, 1).is_build_time());
        // "15 meters or closer" holds at 15; "further than 15" is its `Not`
        assert_eq!(Condition::TargetWithinMeters(15).evaluate(&Hands), Some(true));
        assert_eq!(Condition::TargetWithinMeters(14).evaluate(&Hands), Some(false));
        assert_eq!(Condition::not(Condition::TargetWithinMeters(14)).evaluate(&Hands), Some(true));
        assert_eq!(Condition::TargetFlanked.evaluate(&Hands), Some(true));
        assert_eq!(Condition::SelfDebuffed.evaluate(&Hands), Some(false));
        // none of the three combat-time ones is decided by the build alone, and none is known without a context
        for c in [Condition::TargetWithinMeters(15), Condition::TargetFlanked, Condition::SelfDebuffed] {
            assert!(!c.is_build_time(), "{c:?}");
            assert_eq!(c.evaluate(&NoContext), None, "{c:?}");
        }
        assert_eq!(has(WeaponType::Axe, 2).to_string(), "2 or more axe wielded");
        assert_eq!(Condition::TargetWithinMeters(15).to_string(), "the target is 15 m away or closer");
        assert_eq!(serde_json::to_string(&has(WeaponType::Dagger, 1)).unwrap(), r#"{"WeaponTypeEquipped":{"weapon":"Dagger","at_least":1}}"#);
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
