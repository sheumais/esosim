//! 5.1 — the damage formula for a single hit.
//!
//! The formula, from the roadmap:
//!
//! ```text
//! Damage = ((((Coefficients * Stats * Potency Modifiers)
//!             + Damage Done (flat) + Damage Done to Monsters (flat))
//!           * (Damage Done + Damage Done to Monsters))
//!         + Enemy Damage Taken (flat) * Enemy Damage Taken
//!         * Damage Reduction from Armour * Critical Damage Done [if applicable]
//! ```
//!
//! # How it is read here
//!
//! The text ends with a bare `+` before `Enemy Damage Taken (flat)`. Taken literally, that flat term
//! would be multiplied by the armour and the crit but the damage itself would not, which cannot be
//! meant. It is read as the second half mirroring the first (flat inside, multiplier outside), with
//! everything after it applying to the whole:
//!
//! ```text
//! coef_value = max(c1·t1 + c2·t2, c3·t3 + c4·t4)            [evaluate_coef]
//! base       = coef_value × potency
//! done       = (base + flat_done + flat_done_monsters) × (1 + pct_done + pct_done_monsters)
//! taken      = (done + flat_taken) × (1 + pct_taken)
//! hit        = taken × (1 − mitigation) × crit_mult          [crit_mult = 1 for a non-crit]
//! ```
//!
//! [`combine`] is that last block and nothing else, so it can be tested against hand-worked numbers
//! before any channel plumbing exists.
//!
//! # Where each term comes from
//!
//! | Term | Source |
//! |---|---|
//! | Coefficients, Stats | the record's `coef` row ([`CoefRow`]) and a [`CoefStats`] read off the attacker's [`StatSheet`] |
//! | Potency Modifiers | [`PotencyModifiers`]: execute scaling, and `1 + Σ Channel::DamagePotency` on the attacker |
//! | Damage Done (flat) / (%) | `Channel::DamageDone(ctx, Additive)` / `(ctx, Multiplicative)` over [`damage_done_contexts`] |
//! | Damage Done to Monsters | `Channel::DamageDone(DamageDoneContext::Monsters, …)`, only against a monster |
//! | Enemy Damage Taken (flat) / (%) | `Channel::DamageTaken(ctx, …)` on the *target's* channels over [`damage_taken_contexts`] |
//! | Damage Reduction from Armour | the target's resistance less the attacker's penetration, through `esosim::stats::armour` |
//! | Critical Damage Done | 50 % base + the attacker's `CriticalDamage` + the target's `CriticalDamageTaken`, *then* capped ([`crit_multiplier`]) |
//!
//! Units follow the engine: an `Additive` damage channel is flat points, a `Multiplicative` one is a
//! fraction (`0.05` = 5 %), which the aggregator keeps as basis points; everything here is `f64`
//! (roadmap decision D3), and the fractions are turned back into `f64` once, in [`sum_channel`].
//!
//! # Settled (by the author of the roadmap)
//!
//! * **Brittle is inside the crit damage cap.** The cap applies to the sum of the attacker's crit
//!   damage and the target's `CriticalDamageTaken`. The sheet's `crit_damage_pct` is already capped on
//!   its own, so [`crit_multiplier`] cannot start from it: it adds the raw channels and caps once.
//! * **Ability-only power goes into the stats.** Wrathful Strikes-style
//!   `Power(PowerContext::Damage(ctx), Additive)` is added to the power the coefficient reads, after
//!   the sheet's multiplicative bucket and not scaled by it ([`CoefStats::for_hit`]).
//! * **Oblivion damage bypasses resistance, penetration, block mitigation and shields** ([`resist_type`],
//!   [`HitSpec::bypasses_defences`]). The game has no "Generic" damage; the id only appears on castable
//!   skills, whose damage records carry the real type.
//! * **Anything that is not a player is a monster** ([`is_monster`]): `Boss`, `Undead`, `Daedra` and
//!   `Monster` all count, and so does a target whose kinds are unsaid (the trial dummy).
//!
//! # Open questions (each one is a test to write against a combat log)
//!
//! 1. The trailing `+` above (asked; the reading in "How it is read here" stands until answered).
//! 2. Nothing left on potency modifiers except Cycle Unending's two-Health gradient (see
//!    [`PotencyModifiers`]). Sources add (settled) and come from `Channel::DamagePotency`.
//! 3. Where the game rounds (the sets floor `86 × coef`; a hit is probably floored once at the end).
//!    To be found out against logs.
//! 4. `Drown` damage: not player-dealt, so [`resist_type`] gives it `None` until something needs it.

use eso_skill_data::enums::{coefficient_type::CoefficientType, damage_type::DamageType, skill_line::SkillLine};
use esosim::engine::aggregator::ChannelSet;
use esosim::stats::sheet::StatSheet;
use esosim_types::channel::StatusEffect;
use esosim_types::{AggKind, DamageDoneContext, DamageTakenContext, ResistableDamageType, TargetKind};

use crate::ability::{AbilityDef, CoefRow};

// ---------------------------------------------------------------------------------------------
// What a hit is
// ---------------------------------------------------------------------------------------------

/// How the damage is delivered. Picks the `Direct`, `DamageOverTime`, `LightAttack`, `HeavyAttack`,
/// `Bash` and `StatusEffect` damage-done contexts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HitKind {
    Direct,
    /// One tick of a damage-over-time effect.
    DamageOverTimeTick,
    LightAttack,
    HeavyAttack,
    Bash,
    /// The damage of a status effect (Burning, Poisoned ...); `None` for one the data does not name.
    StatusEffect(Option<StatusEffect>),
}

/// Picks the `SingleTarget` or `AreaOfEffect` contexts.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Spread {
    SingleTarget,
    AreaOfEffect,
}

/// Which of a hit's two outcomes is being worked out. They differ: `DamageDoneContext::NonCritical`
/// bonuses only exist in the first, and the crit multiplier only in the second.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CritOutcome {
    NonCritical,
    Critical,
}

/// How a run settles crits. Roadmap 5.5: expected value first, stochastic later.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CritMode {
    /// Never crit (a floor, and what a `can_crit() == false` hit always gets).
    Never,
    /// Always crit.
    Always,
    /// `p × crit + (1 − p) × non-crit`, `p` the crit chance. No randomness.
    Expected,
}

/// One damaging hit, described by what the formula reads from the ability. The record is the
/// *damage* record (`Component::Hit` / `Component::Periodic`'s `record_id`), not the castable skill.
#[derive(Debug, Clone)]
pub struct HitSpec<'a> {
    pub ability: &'a AbilityDef,
    /// Which of the record's `coef` rows this hit is (each row is one number in the tooltip).
    pub coef_row: usize,
    pub kind: HitKind,
    pub spread: Spread,
    /// The skill line of the castable skill the record belongs to (the record has none): what
    /// `DamageDoneContext::SkillLine` and `Penetration(SkillLine)` match.
    pub skill_line: Option<SkillLine>,
    /// Channeled abilities get `DamageDoneContext::Channeled`.
    pub channeled: bool,
}

impl<'a> HitSpec<'a> {
    /// The spec for `ability`'s first coefficient row, with `spread` worked out from its radius and
    /// target cap, and `channeled` from its flags.
    pub fn for_ability(ability: &'a AbilityDef, kind: HitKind, skill_line: Option<SkillLine>) -> Self {
        unimplemented!()
    }

    /// The resistance this hit is mitigated by, `None` if none ([`resist_type`]).
    pub fn resist_type(&self) -> Option<ResistableDamageType> {
        unimplemented!()
    }

    /// Whether the hit can crit at all. Some damage cannot; such a hit has no critical outcome.
    pub fn can_crit(&self) -> bool {
        unimplemented!()
    }

    /// Oblivion damage: ignores resistance and penetration (so [`mitigation`] is 0), and, against a
    /// player, block mitigation and shields. Only the first part matters while the enemy is the target;
    /// the rest is for 5.7.
    pub fn bypasses_defences(&self) -> bool {
        unimplemented!()
    }
}

/// The attacker as the formula sees it: the channels **at the moment of the hit** (live buffs and
/// procs switched on, conditions decided by the simulator's `CombatContext`) and the sheet derived
/// from those same channels.
#[derive(Debug, Clone, Copy)]
pub struct AttackerView<'a> {
    pub channels: &'a ChannelSet,
    pub sheet: &'a StatSheet,
}

/// The target as the formula sees it.
#[derive(Debug, Clone, Copy)]
pub struct TargetView<'a> {
    /// `Enemy::channels`: its resistance and every debuff on it.
    pub channels: &'a ChannelSet,
    /// What the target is (`Monster`, `Boss`, ...). Empty = unsaid.
    pub kinds: &'a [TargetKind],
    /// Current Health, in percent (0.0 to 100.0), for execute scaling.
    pub health_pct: f32,
}

/// Anything that is not a player is a monster (`DamageDone(Monsters)`, Minor/Major Slayer). A target
/// with no kinds counts as one.
pub fn is_monster(target: TargetView) -> bool {
    unimplemented!()
}

// ---------------------------------------------------------------------------------------------
// Coefficients × stats
// ---------------------------------------------------------------------------------------------

/// The stats a coefficient row can read, as numbers.
///
/// Weapon and spell damage are one stat in the engine (`StatSheet::power`), so `spell_damage` and
/// `weapon_damage` are normally equal; they are separate fields only because the game's two
/// coefficient types are.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct CoefStats {
    pub max_magicka: f64,
    pub max_health: f64,
    pub health_recovery: f64,
    pub spell_resistance: f64,
    pub physical_resistance: f64,
    pub spell_damage: f64,
    pub max_stamina: f64,
    pub weapon_damage: f64,
}

impl CoefStats {
    /// The stats for `hit`: the sheet's numbers, with the ability-only power
    /// (`Channel::Power(PowerContext::Damage(ctx), Additive)` over [`damage_done_contexts`]) added to
    /// both damage stats as a flat amount, after the sheet's multiplicative bucket (settled: it is part
    /// of the stats, not scaled by that bucket).
    pub fn for_hit(attacker: AttackerView, hit: &HitSpec, outcome: CritOutcome) -> Self {
        unimplemented!()
    }

    pub fn get(&self, kind: CoefficientType) -> f64 {
        unimplemented!()
    }
}

/// `max(c1·t1 + c2·t2, c3·t3 + c4·t4)`. A type id the parser does not know, and id 0, reads as 0.
/// This is the *real* formula, not the collapsed "max power + max resource" shorthand (PLAN §6).
pub fn evaluate_coef(row: &CoefRow, stats: &CoefStats) -> f64 {
    unimplemented!()
}

/// The multiplier on the ability's own value (`coef_value`), before any flat bonus.
///
/// The game's wording is "increasing the **potency** of your damage done by X": Cycle Unending (263509,
/// Necromancer Class Mastery: 1 % per 1 % more Health than the target, capped at 25 %, or 10 % against
/// players) and Inexorable Descent (238232: per stack) are the two in the data that do it to damage.
/// Such a bonus is neither a `DamageDone` percentage (which sits in the `(1 + Σ)` bucket after the flat
/// terms) nor power; it scales the ability's own value, which is where the formula puts it.
///
/// Sources are additive and the sum is used as a multiplier: `1 + Σ` of `Channel::DamagePotency`
/// (a multiplicative-bucket channel, so the aggregator holds it as basis points). Neither effect is
/// written into the effect data yet.
///
/// # Not yet expressible in the engine
///
/// * Cycle Unending's size depends on *both* Healths (own minus target, in percentage points, capped by
///   target kind). `Stacking::PerHealthPct` reads one Health only, so it cannot say this; it needs a
///   second gradient kind (or a computed stack count from the simulator, which knows both).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct PotencyModifiers {
    /// Execute scaling at the target's health: `1.0` above the threshold, rising linearly to
    /// `1 + bonus_up_to_pct / 100` at 0 % Health.
    pub execute: f64,
    /// `1 + Σ` of `Channel::DamagePotency` on the attacker (a fraction: `0.25` is +25 %). `1.0` = none.
    pub damage_done_potency: f64,
}

impl PotencyModifiers {
    /// No potency modifier.
    pub const NEUTRAL: PotencyModifiers = PotencyModifiers { execute: 1.0, damage_done_potency: 1.0 };

    pub fn multiplier(&self) -> f64 {
        unimplemented!()
    }
}

/// The hit's potency modifiers against `target`.
pub fn potency_modifiers(attacker: AttackerView, target: TargetView, hit: &HitSpec) -> PotencyModifiers {
    unimplemented!()
}

// ---------------------------------------------------------------------------------------------
// Damage done / damage taken
// ---------------------------------------------------------------------------------------------

/// The damage-done terms of the formula. Percentages are fractions (`0.05` = 5 %).
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DamageDoneTerms {
    /// `Damage Done (flat)`: `DamageDone(ctx, Additive)` summed over the applicable contexts.
    pub flat: f64,
    /// `Damage Done to Monsters (flat)`: `DamageDone(Monsters, Additive)`, 0 against a non-monster.
    pub flat_monsters: f64,
    /// `Damage Done`: `DamageDone(ctx, Multiplicative)` summed over the applicable contexts.
    pub pct: f64,
    /// `Damage Done to Monsters`: `DamageDone(Monsters, Multiplicative)`, 0 against a non-monster.
    pub pct_monsters: f64,
}

/// The damage-taken terms, read off the *target's* channels.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct DamageTakenTerms {
    /// `Enemy Damage Taken (flat)`.
    pub flat: f64,
    /// `Enemy Damage Taken`, a fraction (Minor Vulnerability is `0.05`).
    pub pct: f64,
}

/// Every damage-done context that applies to `hit` in `outcome`, for the attacker's channels: `All`,
/// the delivery (`Direct` / `DamageOverTime` / `LightAttack` / `HeavyAttack` / `Bash` /
/// `StatusEffects` / `StatusEffect(_)`), the spread, `Channeled`, `ClassAbilities` (tag `ClassSkill`),
/// `Abilities`, `SkillLine(_)`, `Mechanic(_)`, `NonCritical` for a non-critical outcome, and
/// `Type(_)` for the damage type and the group it belongs to (`Martial` or `Spell`).
///
/// `Monsters` is deliberately **not** in the list: the formula keeps it as terms of its own, read by
/// [`damage_done_terms`] only against a monster.
pub fn damage_done_contexts(hit: &HitSpec, outcome: CritOutcome) -> Vec<DamageDoneContext> {
    unimplemented!()
}

/// Every damage-taken context that applies to `hit` against `target`: `All`, `AreaOfEffect` or
/// `DamageOverTime`, `Players` / `Monsters` (what the *attacker* is: a player), and `Type(_)` for the
/// damage type and its group.
pub fn damage_taken_contexts(hit: &HitSpec, target: TargetView) -> Vec<DamageTakenContext> {
    unimplemented!()
}

/// The channel values summed over `contexts`: flat points for `AggKind::Additive`, a fraction for
/// `AggKind::Multiplicative` (the aggregator's basis points divided by 10,000).
///
/// Generic over the channel family so the done and taken sums share one body.
pub fn sum_channel<C: Copy>(
    channels: &ChannelSet,
    contexts: &[C],
    kind: AggKind,
    channel_of: fn(C, AggKind) -> esosim_types::Channel,
) -> f64 {
    unimplemented!()
}

pub fn damage_done_terms(
    attacker: AttackerView,
    target: TargetView,
    hit: &HitSpec,
    outcome: CritOutcome,
) -> DamageDoneTerms {
    unimplemented!()
}

pub fn damage_taken_terms(target: TargetView, hit: &HitSpec) -> DamageTakenTerms {
    unimplemented!()
}

// ---------------------------------------------------------------------------------------------
// Armour and crit
// ---------------------------------------------------------------------------------------------

/// Which resistance a game damage type is mitigated by. `None` = not mitigated at all: `Oblivion`
/// (settled: bypasses every defence), and `Drown` and `Generic`, which a player's damage record does not
/// carry (see the module docs).
pub fn resist_type(damage_type: DamageType) -> Option<ResistableDamageType> {
    unimplemented!()
}

/// The attacker's penetration against this hit: `Channel::Penetration(ctx)` summed over
/// [`damage_done_contexts`] (so `All`, the damage type and its group, and ability-specific ones such as
/// `SkillLine(Destruction Staff)`). `esosim::stats::armour::penetration` only layers `All` and the type,
/// which is not enough for the skill-line penetration passives.
pub fn penetration_for_hit(attacker: AttackerView, hit: &HitSpec, outcome: CritOutcome) -> u32 {
    unimplemented!()
}

/// The fraction of the hit the target's armour stops: its resistance to the hit's type, less
/// `penetration` (never below 0), held to the resistance cap and divided by 66,000
/// (`esosim::stats::armour::{effective_resistance, resistance_to_mitigation}`). 0 for an unmitigated
/// hit, and for one that [`HitSpec::bypasses_defences`]. Returns the effective resistance too, for the breakdown.
pub fn mitigation(target: TargetView, hit: &HitSpec, penetration: u32) -> (u32, f64) {
    unimplemented!()
}

/// `1 + crit damage` as a multiplier: `CappedStat::CriticalDamage` applied **once** to 50 % base + the
/// attacker's `Channel::CriticalDamage` + the target's `Channel::CriticalDamageTaken` (Brittle is inside
/// the cap), with the cap lifted by the attacker's `Channel::CapRaise` (`esosim::stats::armour::cap_raise`).
/// Reads the channels, not `StatSheet::crit_damage_pct`, which is already capped without Brittle.
/// 1.0 if the hit cannot crit.
pub fn crit_multiplier(attacker: AttackerView, target: TargetView, hit: &HitSpec) -> f64 {
    unimplemented!()
}

// ---------------------------------------------------------------------------------------------
// The formula
// ---------------------------------------------------------------------------------------------

/// Every factor of one hit, kept so a number that disagrees with a combat log can be traced to the
/// term that is wrong (roadmap 5.1: "validate against combat logs").
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DamageBreakdown {
    pub outcome: CritOutcome,
    /// `max(c1·t1 + c2·t2, c3·t3 + c4·t4)`.
    pub coef_value: f64,
    pub potency: f64,
    pub done: DamageDoneTerms,
    pub taken: DamageTakenTerms,
    /// The target's resistance after penetration.
    pub effective_resistance: u32,
    /// The fraction of the hit armour stops (0.0 to the cap).
    pub mitigation: f64,
    /// `1.0` for a non-critical outcome.
    pub crit_mult: f64,
    /// The result of [`combine`].
    pub total: f64,
}

/// The formula, with every input already worked out (see the module docs for how it is read).
pub fn combine(
    coef_value: f64,
    potency: f64,
    done: &DamageDoneTerms,
    taken: &DamageTakenTerms,
    mitigation: f64,
    crit_mult: f64,
) -> f64 {
    unimplemented!()
}

/// One hit in one outcome: gathers every term from the two views and combines them.
pub fn hit_damage(
    attacker: AttackerView,
    target: TargetView,
    hit: &HitSpec,
    outcome: CritOutcome,
) -> DamageBreakdown {
    unimplemented!()
}

/// What a hit is worth once crits are settled by a [`CritMode`].
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct HitResult {
    pub non_crit: DamageBreakdown,
    /// `None` when the hit cannot crit.
    pub crit: Option<DamageBreakdown>,
    /// The attacker's crit chance (0.0 to 1.0), 0 if the hit cannot crit.
    pub crit_chance: f64,
    /// The damage the mode settles on: `Never` the non-crit total, `Always` the crit total (the
    /// non-crit one if it cannot crit), `Expected` the weighted mean.
    pub damage: f64,
}

pub fn resolve_hit(attacker: AttackerView, target: TargetView, hit: &HitSpec, mode: CritMode) -> HitResult {
    unimplemented!()
}
