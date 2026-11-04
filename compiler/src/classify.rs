//! Regex classifier: which engine [`Channel`](esosim_types::Channel) does an
//! ability's value feed? (PLAN.md §3.8.)
//!
//! The ability itself says nothing about *what stat* its number is; the abilities that
//! reference it do. A passive's tooltip reads "Increases your Max Health by `<<1>>`", and
//! `tooltip_data[0].tooltip_ids` of that passive points at the record that carries the
//! coefficient equation. So for every ability with a value:
//!
//! 1. take each tooltip placeholder that resolves to it (`Graph::tooltip_refs`),
//! 2. read the pair's `<<n>>` and tooltip type,
//! 3. cut the clause around that placeholder out of the tooltip text,
//! 4. locate the stat phrase in the clause (three shapes, see [`locate_span`]),
//! 5. turn the phrase into channels with the lexicon ([`LEXICON`]).
//!
//! Every referencing tooltip votes; they must agree or the ability is reported as a
//! conflict. Anything that does not match a rule is kept with its clause so the rule table
//! can be extended from the `classify` report instead of guessed at.
//!
//! **Placeholder mapping (verified against the id dictionary's tooltip renderer):** every
//! `tooltip_data` entry holds parallel `tooltip_types` / `tooltip_ids`; each (type, id) *pair* is one
//! placeholder, numbered in order across all entries. The id of a pair is not always the ability that
//! carries the value: `graph::resolve_target` follows `causes_ids` etc. to the record that does, and
//! `Graph::tooltip_refs` is keyed by that record. `classify-mapping` compares this against
//! the older hypotheses on the real data.
//!
//! Rules are written as `noun("rule_name", r"pattern", |caps| channels)` lines so that
//! they can be read (and linted) as a table. Patterns match lower-case text, use the
//! `regex` crate's syntax (no look-around, no back-references) and must not contain `"`.

use esosim_types::cca::CoreCombatAbility as Cca;
use esosim_types::damage::ResistableDamageType as Dmg;
use esosim_types::{
    AggKind as Agg, Channel, DamageDoneContext as DoneCtx, DamageTakenContext as TakenCtx,
    ResourceKind as Res,
};
use eso_skill_data::enums::{mechanic::Mechanic as Mech, skill_line::SkillLine as Line};
use crate::corpus::Corpus;
use crate::graph::{Graph, TooltipRef};
use crate::resolved::{
    ClassStatus, Direction, ResolvedClassification, ResolvedEvidence,
};
use eso_skill_data::SkillData34;
use eso_skill_data::enums::tooltip_type::TooltipType;
use regex::{Captures, Regex};
use std::collections::BTreeMap;
use std::sync::LazyLock;

/// Human-readable channel set for reports, e.g. `Armour(All) + Power`.
pub fn label(channels: &[Channel]) -> String {
    channels.iter().map(|c| format!("{c:?}")).collect::<Vec<_>>().join(" + ")
}

/// The channel with its aggregation flavour set (only `Resource`/`Recovery` have one).
fn with_agg(ch: Channel, agg: Agg) -> Channel {
    match ch {
        Channel::Resource(r, _) => Channel::Resource(r, agg),
        Channel::Recovery(r, _) => Channel::Recovery(r, agg),
        Channel::SynergyRestore(r, _) => Channel::SynergyRestore(r, agg),
        other => other,
    }
}

// ---------------------------------------------------------------------------
// Which abilities have a value
// ---------------------------------------------------------------------------

/// A coefficient row with at least one non-zero term (same filter as `render_coef`).
pub fn has_coef(s: &SkillData34) -> bool {
    s.coef.iter().any(|c| {
        [(c.type1, c.coef1), (c.type2, c.coef2), (c.type3, c.coef3), (c.type4, c.coef4)]
            .iter()
            .any(|(t, k)| *t > 0 && *k != 0.0)
    })
}

/// "Has a value/coef equation": a coefficient equation or a flat/percent `value1`.
pub fn has_value(s: &SkillData34) -> bool {
    has_coef(s) || s.base_data.value1 != 0 || s.major_minor_buff().is_some()
}

// ---------------------------------------------------------------------------
// Tooltip entry filtering
// ---------------------------------------------------------------------------

/// Tooltip types that are never a stat bonus (durations, ticks, direct damage/heals, ...).
/// The type only says how the value is rendered, so `ResourceGain`, `Knockback` and `SelfHeal`
/// (used for plain stat bonuses like "Increase Max Health by <<1>>") are deliberately absent:
/// the text rules decide those.
fn is_non_stat(t: TooltipType) -> bool {
    matches!(
        t,
        TooltipType::Duration
            | TooltipType::DelayedStrike
            | TooltipType::DeprecatedZeroDuration
            | TooltipType::DeprecatedHalfSecondDuration
            | TooltipType::IncreaseDurationOf
            | TooltipType::IncreasedDurationVsMonsters
            | TooltipType::TickRate
            | TooltipType::MinimumCooldown
            | TooltipType::TargetCap
            | TooltipType::ThresholdBelowHealthPercent
            | TooltipType::BonusUpToPercent
            | TooltipType::MartialDamage
            | TooltipType::MagicalDamage
            | TooltipType::SingleTargetDoT
            | TooltipType::SingleTargetHeal
            | TooltipType::AreaHoT
            | TooltipType::DeprecatedMultiHit
            | TooltipType::NoblesConquest
    )
}

/// A placeholder whose type is unknown is not vetoed (the text rules decide).
fn is_stat_type(ty: Option<TooltipType>) -> bool {
    ty.map_or(true, |t| !is_non_stat(t))
}

fn is_percent(t: TooltipType) -> bool {
    matches!(
        t,
        TooltipType::Percentage | TooltipType::StatPercentage | TooltipType::PercentageConstant
    )
}

/// `Resource`/`Recovery` channels exist in an additive (flat) and a multiplicative
/// (percent) flavour. A scaling equation is always flat; a plain percent value is
/// multiplicative.
fn agg_kind(child: &SkillData34, ty: Option<TooltipType>) -> Agg {
    let percent = ty.is_some_and(is_percent);
    if percent && !has_coef(child) { Agg::Multiplicative } else { Agg::Additive }
}

// ---------------------------------------------------------------------------
// Text -> clause
// ---------------------------------------------------------------------------

static MARKUP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\|c[0-9a-fA-F]{6}|\|r|\^[A-Za-z]+").unwrap());
static PLACEHOLDER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"<<(?:[A-Za-z]+:)?(\d+)>>").unwrap());
/// Clause boundaries: line breaks, `|`, `;`, `:`, bullets, a literal `\n`, and sentence
/// full stops (a `.` inside a number is not one).
static BOUNDARY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[\n|;:\u{2022}\u{b7}]|\\n|\.(?:\s|$)").unwrap());

/// The clause of `text` that contains placeholder `<<n>>`, lower-cased, with that
/// placeholder written as `@` and every other one as `#`.
pub fn clause_for(text: &str, n: usize) -> Option<String> {
    let clean = MARKUP.replace_all(text, "");
    let marked = PLACEHOLDER.replace_all(&clean, |c: &Captures| {
        let is_target = c[1].parse::<usize>().ok() == Some(n);
        let s = if is_target { "@" } else { "#" };
        s.to_string()
    });
    BOUNDARY
        .split(&marked)
        .find(|seg| seg.contains('@'))
        .map(|seg| seg.to_lowercase().split_whitespace().collect::<Vec<_>>().join(" "))
}

/// Highest `<<n>>` in a text (0 when none). Used by the report to test the
/// placeholder-numbering assumption.
pub fn max_placeholder(text: &str) -> usize {
    PLACEHOLDER
        .captures_iter(text)
        .filter_map(|c| c[1].parse::<usize>().ok())
        .max()
        .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Clause -> stat phrase
// ---------------------------------------------------------------------------

static VERB_CHANGE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b(?:increas(?:e|es|ed|ing)|rais(?:e|es|ed|ing)|boost(?:s|ed|ing)?|improv(?:e|es|ed|ing)|reduc(?:e|es|ed|ing)|decreas(?:e|es|ed|ing)|lower(?:s|ed|ing)?|diminish(?:es|ed|ing)?)\b",
    )
    .unwrap()
});
static VERB_ANY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b(?:increas(?:e|es|ed|ing)|rais(?:e|es|ed|ing)|boost(?:s|ed|ing)?|improv(?:e|es|ed|ing)|reduc(?:e|es|ed|ing)|decreas(?:e|es|ed|ing)|lower(?:s|ed|ing)?|diminish(?:es|ed|ing)?|grant(?:s|ed|ing)?|gain(?:s|ed|ing)?)\b",
    )
    .unwrap()
});
static BY_AT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(?:by|to)\s+(?:up to\s+)?@").unwrap());
static PARTICIPLE_BY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b(increased|raised|improved|boosted|reduced|decreased|lowered|diminished)\s+(?:by|to)\s+(?:up to\s+)?@",
    )
    .unwrap()
});
static TRAILING_COPULA: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\s+(?:is|are|will be|gets?|being)$").unwrap());
static AFTER_CUT: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"\b(?:for|while|when|if|to|per|every|until|after|which|that)\b|[,#]|\band (?:gain|grant|increas|reduc|restor|decreas)\w*",
    )
    .unwrap()
});

fn direction_of(verb: &str) -> Direction {
    if ["reduc", "decreas", "lower", "diminish"].iter().any(|p| verb.starts_with(p)) {
        Direction::Lowers
    } else {
        Direction::Raises
    }
}

fn last_match<'t>(re: &Regex, text: &'t str) -> Option<regex::Match<'t>> {
    re.find_iter(text).last()
}

/// Find the stat phrase for the `@` placeholder. Returns `(phrase, direction, bare_ok)`;
/// `bare_ok` says a bare noun ("damage") may be taken as a channel (only when a
/// "<verb> ... by @" / "... increased by @" frame names the stat, never for "dealing @ damage").
///
/// Shapes, tried in this order:
/// * C  `<stat> increased by @`      ("critical damage taken increased by @")
/// * A  `<verb> <stat> by @`         ("increases your max health by @")
/// * B  `<give-verb> ... @ <stat>`   ("grants you @ weapon and spell critical")
fn locate_span(clause: &str, at: usize) -> Option<(String, Direction, bool)> {
    if let Some(c) = PARTICIPLE_BY.captures(clause) {
        let m = c.get(0).unwrap();
        let before = &clause[..m.start()];
        let tail = before.rsplit(|ch| ch == ',' || ch == '#').next().unwrap_or(before).trim();
        let span = TRAILING_COPULA.replace(tail, "").trim().to_string();
        return Some((span, direction_of(&c[1]), true));
    }
    if let Some(m) = BY_AT.find(clause) {
        let before = &clause[..m.start()];
        if let Some(v) = last_match(&VERB_CHANGE, before) {
            let tail = &before[v.end()..];
            let span = tail.rsplit('#').next().unwrap_or(tail).trim().to_string();
            return Some((span, direction_of(v.as_str()), true));
        }
    }
    let before = &clause[..at];
    let v = last_match(&VERB_ANY, before)?;
    let after = clause[at + 1..].trim_start_matches('%').trim();
    let end = AFTER_CUT.find(after).map_or(after.len(), |m| m.start());
    let span = after[..end].trim();
    // "absorbs up to @ of your max health" names what the value scales with, not a bonus.
    if span == "of" || span.starts_with("of ") {
        return None;
    }
    Some((span.to_string(), direction_of(v.as_str()), false))
}

// ---------------------------------------------------------------------------
// Lexicon: stat phrase -> channels
// ---------------------------------------------------------------------------

struct Noun {
    rule: &'static str,
    re: Regex,
    build: fn(&Captures) -> Vec<Channel>,
}

fn noun(rule: &'static str, re: &str, build: fn(&Captures) -> Vec<Channel>) -> Noun {
    let re = Regex::new(&format!(r"\b(?:{re})\b"))
        .unwrap_or_else(|e| panic!("classify rule `{rule}` does not compile: {e}"));
    Noun { rule, re, build }
}

fn cap<'a>(c: &'a Captures, name: &str) -> Option<&'a str> {
    c.name(name).map(|m| m.as_str())
}

fn res_of(s: &str) -> Res {
    match s {
        "health" => Res::Health,
        "magicka" => Res::Magicka,
        _ => Res::Stamina,
    }
}

/// Resources named by the given capture groups, in order, without duplicates.
fn resources(c: &Captures, names: &[&str]) -> Vec<Res> {
    let mut out: Vec<Res> = Vec::new();
    for n in names {
        if let Some(r) = cap(c, n).map(res_of) {
            if !out.contains(&r) {
                out.push(r);
            }
        }
    }
    out
}

fn el_of(s: &str) -> Dmg {
    match s {
        "fire" | "flame" => Dmg::Flame,
        "frost" | "cold" => Dmg::Frost,
        "shock" | "lightning" => Dmg::Shock,
        "poison" => Dmg::Poison,
        "disease" => Dmg::Disease,
        "bleed" => Dmg::Bleed,
        "physical" => Dmg::Physical,
        "magic" => Dmg::Magic,
        "spell" => Dmg::Spell,
        "earth" => Dmg::Earth,
        _ => Dmg::All,
    }
}

fn mech_of(s: &str) -> Mech {
    match s {
        "magicka" => Mech::Magicka,
        "stamina" => Mech::Stamina,
        "health" => Mech::Health,
        _ => Mech::Ultimate,
    }
}

use Channel as C;

/// Order matters: a rule consumes the text it matches, so specific phrases come before the
/// generic ones they contain (`damage taken from area` before `damage taken`,
/// `critical damage taken` before `critical damage`, `physical and spell resistance`
/// before `physical resistance`, ...).
static LEXICON: LazyLock<Vec<Noun>> = LazyLock::new(|| {
    vec![
        // ---- critical ---------------------------------------------------
        noun("crit_dmg_taken", r"critical (?:strike |hit )?damage taken", |_| vec![C::CriticalDamageTaken]),
        noun("crit_dmg", r"critical (?:strike |hit )?damage(?: done)?(?: and (?:critical )?healing(?: done)?)?", |_| vec![C::CriticalDamage]),
        // No engine channel for critical healing (it rides on CriticalDamage); consume it.
        noun("crit_heal", r"critical healing(?: done| chance)?", |_| vec![]),
        noun("crit_resistance", r"critical resistance", |_| vec![C::CriticalResistance]),
        noun("crit_chance_both", r"(?:weapon|spell) critical(?: strike)?(?: rating| chance)?(?: and (?:weapon|spell) critical(?: strike)?(?: rating| chance)?)|(?:weapon and spell|spell and weapon) critical(?: strike)?(?: rating| chance)?", |_| vec![C::CriticalChance]),
        noun("spell_crit", r"spell critical(?: strike)?(?: rating| chance)?", |_| vec![C::SpellCriticalChance]),
        noun("weapon_crit", r"weapon critical(?: strike)?(?: rating| chance)?", |_| vec![C::WeaponCriticalChance]),
        noun("crit_chance", r"critical(?: strike)? (?:chance|rating)(?: rating)?", |_| vec![C::CriticalChance]),
        // ---- damage taken ----------------------------------------------
        noun("taken_aoe", r"damage (?:you )?(?:take|taken) from (?:area|aoe)(?: of effect)?(?: attacks| damage| abilities| effects)?|area(?: of effect)? damage taken|damage from area(?: of effect)?(?: attacks| damage| abilities)?", |_| vec![C::DamageTaken(TakenCtx::AreaOfEffect)]),
        noun("taken_dot", r"damage (?:you )?(?:take|taken) from damage over time(?: abilities| effects| damage)?|damage over time damage taken", |_| vec![C::DamageTaken(TakenCtx::DamageOverTime)]),
        noun("taken_projectile", r"damage (?:you )?(?:take|taken) from projectiles?(?: attacks| abilities)?|projectile damage taken|damage from projectiles?", |_| vec![C::DamageTaken(TakenCtx::Projectile)]),
        noun("taken_monsters", r"damage (?:you )?(?:take|taken) from (?:dungeon,? ?trial,? and arena )?monsters|damage from (?:dungeon,? ?trial,? and arena )?monsters", |_| vec![C::DamageTaken(TakenCtx::Monsters)]),
        noun("taken_players", r"damage (?:you )?(?:take|taken) from players?|damage from players?", |_| vec![C::DamageTaken(TakenCtx::Players)]),
        noun("taken_siege", r"damage (?:you )?(?:take|taken) from siege(?: weapons)?|siege damage taken|damage from siege(?: weapons)?", |_| vec![C::DamageTaken(TakenCtx::Siege)]),
        noun("taken_fall", r"fall damage(?: taken| you take)?", |_| vec![C::DamageTaken(TakenCtx::Fall)]),
        noun("taken_element", r"(?P<el>fire|flame|frost|cold|shock|lightning|poison|disease|bleed|physical|magic|spell) damage(?: taken| you take)|damage (?:you )?(?:take|taken) from (?P<el2>fire|flame|frost|cold|shock|lightning|poison|disease|bleed|physical|magic|spell) (?:attacks|abilities|effects|damage)", |c| {
            let e = cap(c, "el").or(cap(c, "el2")).unwrap_or("");
            vec![C::DamageTaken(TakenCtx::Type(el_of(e)))]
        }),
        noun("taken_all", r"damage (?:you |they and their allies )?(?:take|taken)|incoming damage|damage reduction", |_| vec![C::DamageTaken(TakenCtx::All)]),
        // ---- damage done -----------------------------------------------
        noun("done_element", r"(?:spell )?damage with (?P<el>fire|flame|cold|frost|shock|lightning|poison|disease|bleed|physical|magic) effects?", |c| vec![C::DamageDone(DoneCtx::Type(el_of(cap(c, "el").unwrap_or(""))))]),
        noun("done_element_by", r"damage (?:done|dealt) (?:by|with) (?P<el>fire|flame|cold|frost|shock|lightning|poison|disease|bleed|physical|magic) (?:attacks|abilities|effects|damage)", |c| vec![C::DamageDone(DoneCtx::Type(el_of(cap(c, "el").unwrap_or(""))))]),
        noun("done_light_heavy", r"damage done with (?:light and heavy|heavy and light) attacks|(?:light and heavy|heavy and light) attack damage", |_| vec![C::DamageDone(DoneCtx::LightAttack), C::DamageDone(DoneCtx::HeavyAttack)]),
        noun("done_light", r"damage done with light attacks|light attack damage", |_| vec![C::DamageDone(DoneCtx::LightAttack)]),
        noun("done_heavy", r"damage done with heavy attacks|heavy attack damage", |_| vec![C::DamageDone(DoneCtx::HeavyAttack)]),
        noun("done_dot", r"damage done with damage over time(?: effects| abilities)?(?P<status>(?: and| or) status effects)?|damage over time damage done", |c| {
            let mut v = vec![C::DamageDone(DoneCtx::DamageOverTime)];
            if c.name("status").is_some() {
                v.push(C::DamageDone(DoneCtx::StatusEffects));
            }
            v
        }),
        noun("done_aoe", r"damage done with area(?: of effect)?(?P<ot> and over time)?(?: attacks| abilities| effects)?", |c| {
            let mut v = vec![C::DamageDone(DoneCtx::AreaOfEffect)];
            if c.name("ot").is_some() {
                v.push(C::DamageDone(DoneCtx::DamageOverTime));
            }
            v
        }),
        noun("done_direct", r"damage done with direct damage(?: and channeled)?(?: attacks| effects| abilities)?", |_| vec![C::DamageDone(DoneCtx::Direct)]),
        noun("done_single", r"damage done with single target(?: attacks| abilities| damage)?", |_| vec![C::DamageDone(DoneCtx::SingleTarget)]),
        noun("done_class", r"damage done with class abilities", |_| vec![C::DamageDone(DoneCtx::ClassAbilities)]),
        noun("done_monsters", r"damage (?:done )?(?:against|to) (?:dungeon,? ?trial,? and arena )?monsters", |_| vec![C::DamageDone(DoneCtx::Monsters)]),
        noun("done_noncrit", r"non-critical damage(?: done)?", |_| vec![C::DamageDone(DoneCtx::NonCritical)]),
        noun("done_and_healing", r"damage and healing done|damage done and healing done|healing and damage done|damage and healing", |_| vec![C::DamageDone(DoneCtx::All), C::HealingDone]),
        noun("done_all", r"damage (?:done|dealt)|outgoing damage|damage enemies do|damage they deal", |_| vec![C::DamageDone(DoneCtx::All)]),
        // ---- healing & shields ------------------------------------------
        noun("heal_taken", r"healing (?:received|taken)", |_| vec![C::HealingTaken]),
        noun("heal_done_restoration", r"healing with restoration staff(?: spells| abilities)?", |_| vec![C::HealingDoneSkillLine(Line::RestorationStaff)]),
        noun("heal_done_hot", r"healing done with healing over time(?: effects)?", |_| vec![C::HealingDoneOverTime]),
        noun("heal_done_single", r"healing done with single target(?: heals| abilities| healing)?", |_| vec![C::HealingDoneSingleTarget]),
        noun("heal_done", r"healing done|healing you (?:do|deal)", |_| vec![C::HealingDone]),
        noun("shield_strength", r"damage shields?(?: strength| effectiveness)?(?: applied to you)?|strength of (?:your )?damage shields?", |_| vec![C::DamageShieldStrength]),
        // ---- resources ---------------------------------------------------
        noun("heavy_restore", r"(?P<a>magicka|stamina)(?: (?:or|and) (?P<b>magicka|stamina))? (?:that )?your (?:fully-charged )?heavy attacks? (?:restore|recover|grants?)", |c| resources(c, &["a", "b"]).into_iter().map(C::HeavyAttackRestore).collect()),
        noun("recovery", r"(?P<a>magicka|stamina|health)(?:,? (?:and |or )?(?P<b>magicka|stamina|health))?(?:,? (?:and |or )?(?P<c>magicka|stamina|health))? (?:recovery|regeneration|regen)", |c| resources(c, &["a", "b", "c"]).into_iter().map(|r| C::Recovery(r, Agg::Additive)).collect()),
        noun("max_resource", r"(?:max|maximum|total) (?P<a>health|magicka|stamina)(?:,? (?:and |or )?(?:(?:max|maximum) )?(?P<b>health|magicka|stamina))?(?:,? (?:and |or )?(?:(?:max|maximum) )?(?P<c>health|magicka|stamina))?", |c| resources(c, &["a", "b", "c"]).into_iter().map(|r| C::Resource(r, Agg::Additive)).collect()),
        noun("resource_pool", r"(?P<a>health|magicka|stamina) pool", |c| resources(c, &["a"]).into_iter().map(|r| C::Resource(r, Agg::Additive)).collect()),
        // ---- resistance & penetration -------------------------------------
        noun("armour_both", r"physical and spell resist(?:ance)?|spell and physical resist(?:ance)?|physical resistance and spell resistance|spell resistance and physical resistance|armou?r and spell resist(?:ance)?|spell resist(?:ance)? and armou?r", |_| vec![C::Armour(Dmg::All)]),
        noun("armour_physical", r"physical resist(?:ance)?", |_| vec![C::Armour(Dmg::Martial)]),
        noun("armour_spell", r"spell resist(?:ance)?", |_| vec![C::Armour(Dmg::Spell)]),
        noun("armour_two_elements", r"(?P<a>fire|flame|frost|cold|shock|lightning|poison|disease|bleed|magic) and (?P<b>fire|flame|frost|cold|shock|lightning|poison|disease|bleed|magic) resist(?:ance)?", |c| vec![C::Armour(el_of(cap(c, "a").unwrap_or(""))), C::Armour(el_of(cap(c, "b").unwrap_or("")))]),
        noun("armour_element", r"(?P<el>fire|flame|frost|cold|shock|lightning|poison|disease|bleed|magic) resist(?:ance)?", |c| vec![C::Armour(el_of(cap(c, "el").unwrap_or("")))]),
        noun("armour_all", r"armou?r(?: rating)?", |_| vec![C::Armour(Dmg::All)]),
        noun("pen_all", r"offensive penetration|physical and spell penetration|spell and physical penetration", |_| vec![C::Penetration(Dmg::All)]),
        noun("pen_physical", r"physical penetration", |_| vec![C::Penetration(Dmg::Martial)]),
        noun("pen_spell", r"spell penetration", |_| vec![C::Penetration(Dmg::Spell)]),
        noun("pen_bare", r"penetration", |_| vec![C::Penetration(Dmg::All)]),
        // ---- costs -----------------------------------------------------------
        noun("cost_roll", r"(?:(?:magicka|stamina|health) )?cost of (?:your )?(?:roll dodge|dodge roll|rolling)|roll dodge(?: stamina)? cost|dodge roll cost", |_| vec![C::CostReductionCCA(Cca::DodgeRoll)]),
        noun("cost_sprint", r"(?:(?:magicka|stamina|health) )?cost of (?:your )?sprint(?:ing)?|sprint(?:ing)? cost", |_| vec![C::CostReductionCCA(Cca::Sprint)]),
        noun("cost_block", r"(?:(?:magicka|stamina|health) )?cost of (?:your )?block(?:ing)?|block(?:ing)? cost", |_| vec![C::CostReductionCCA(Cca::Block)]),
        noun("cost_bash", r"(?:(?:magicka|stamina|health) )?cost of (?:your )?bash|bash cost", |_| vec![C::CostReductionCCA(Cca::Bash)]),
        noun("cost_sneak", r"(?:(?:magicka|stamina|health) )?cost of (?:your )?sneak(?:ing)?|sneak(?:ing)? cost", |_| vec![C::CostReductionCCA(Cca::Sneak)]),
        noun("cost_break_free", r"(?:(?:magicka|stamina|health) )?cost of (?:your )?break free|break free cost", |_| vec![C::CostReductionCCA(Cca::BreakFree)]),
        noun("cost_mechanic", r"(?P<m>magicka|stamina|health|ultimate) costs?|cost of (?:your )?(?P<m2>magicka|stamina|health|ultimate) abilities", |c| vec![C::CostReduction(mech_of(cap(c, "m").or(cap(c, "m2")).unwrap_or("")))]),
        // ---- everything else -----------------------------------------------
        noun("block_mitigation", r"block mitigation|amount of damage you (?:can )?block", |_| vec![C::BlockMitigation]),
        noun("mount_speed", r"mount(?:ed)? speed", |_| vec![C::MountSpeed]),
        noun("movement_speed", r"movement speed(?: bonus| penalty)?|move speed|sprint speed", |_| vec![C::MovementSpeed]),
        noun("resurrection_speed", r"resurrect(?:ion)? speed", |_| vec![C::ResurrectionSpeed]),
        noun("ultimate_gain", r"ultimate (?:gain|generation)", |_| vec![C::UltimateGain]),
        noun("potion_effectiveness", r"potion effectiveness", |_| vec![C::PotionEffectiveness]),
        noun("potion_cooldown", r"potion cooldown", |_| vec![C::PotionCooldown]),
        noun("ability_cooldown", r"ability cooldowns?|cooldowns?", |_| vec![C::AbilityCooldown]),
        noun("attack_speed", r"attack speed", |_| vec![C::AttackSpeed]),
        noun("dodge_chance", r"dodge chance", |_| vec![C::DodgeChance]),
        noun("miss_chance", r"miss chance", |_| vec![C::MissChance]),
        noun("detection_radius", r"(?:radius|area) (?:where |in which )?you can be detected(?: while sneaking)?|detection radius(?: in stealth)?|stealth detection radius", |_| vec![C::DetectionRadius]),
        noun("status_effect_chance", r"chance to apply status effects?|status effect chance", |_| vec![C::StatusEffectChance]),
        noun("stealth_detection", r"stealth detection", |_| vec![C::StealthDetection]),
        // ---- power (last: "spell damage" is a suffix of several phrases above) --
        noun("power_both", r"weapon and spell (?:damage|power)|spell and weapon (?:damage|power)|weapon damage and spell damage|spell damage and weapon damage|weapon power and spell power|spell power and weapon power", |_| vec![C::Power]),
        noun("power_weapon", r"weapon (?:damage|power)", |_| vec![C::WeaponPower]),
        noun("power_spell", r"spell (?:damage|power)", |_| vec![C::SpellPower]),
    ]
});

/// Bare nouns, accepted only when the whole phrase is that noun and only inside a
/// "<verb> <noun> by @" frame (so "dealing @ damage" never becomes a channel).
static BARE: LazyLock<Vec<Noun>> = LazyLock::new(|| {
    vec![
        noun("bare_damage", r"^(?:(?:your|their|the|all|enemy|enemies|target's)\s+)*damage$", |_| vec![C::DamageDone(DoneCtx::All)]),
        noun("bare_healing", r"^(?:(?:your|their|the|all)\s+)*healing$", |_| vec![C::HealingDone]),
    ]
});

const STOPWORDS: &[&str] = &[
    "a", "all", "an", "and", "base", "bonus", "by", "current", "effective", "enemies", "enemy",
    "enemy's", "extra", "from", "in", "of", "or", "own", "target", "target's", "the", "their",
    "total", "up", "you", "your", "additional", "ability", "abilities", "group", "group's",
    "allies", "ally", "allies'",
];

struct SpanMatch {
    channels: Vec<Channel>,
    rules: Vec<&'static str>,
    /// Words left over that no rule explained.
    partial: bool,
}

fn match_span(span: &str, bare_ok: bool) -> Option<SpanMatch> {
    let mut rest = span.to_string();
    let mut channels: Vec<Channel> = Vec::new();
    let mut rules: Vec<&'static str> = Vec::new();

    for n in LEXICON.iter() {
        // A noun can occur more than once ("light attack damage ... heavy attack damage"
        // are different rules, but "max health" twice would be one rule twice).
        while let Some((built, range)) =
            n.re.captures(&rest).map(|c| ((n.build)(&c), c.get(0).unwrap().range()))
        {
            for ch in built {
                if !channels.contains(&ch) {
                    channels.push(ch);
                }
            }
            if !rules.contains(&n.rule) {
                rules.push(n.rule);
            }
            let blank = " ".repeat(range.len());
            rest.replace_range(range, &blank);
        }
    }

    if rules.is_empty() && bare_ok {
        for n in BARE.iter() {
            if let Some(c) = n.re.captures(span) {
                return Some(SpanMatch { channels: (n.build)(&c), rules: vec![n.rule], partial: false });
            }
        }
    }
    if rules.is_empty() {
        return None;
    }
    let partial = rest
        .split(|ch: char| !(ch.is_alphanumeric() || ch == '\'' || ch == '-'))
        .any(|w| !w.is_empty() && !STOPWORDS.contains(&w));
    Some(SpanMatch { channels, rules, partial })
}

/// One classified clause.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    pub channels: Vec<Channel>,
    pub direction: Direction,
    pub rule: String,
    pub partial: bool,
}

/// Classify one clause (as produced by [`clause_for`]). `None` when no channel was found.
pub fn classify_clause(clause: &str) -> Option<Candidate> {
    let at = clause.find('@')?;
    let (span, direction, bare_ok) = locate_span(clause, at)?;
    let m = match_span(&span, bare_ok)?;
    if m.channels.is_empty() {
        return None;
    }
    Some(Candidate { channels: m.channels, direction, rule: m.rules.join("+"), partial: m.partial })
}

// ---------------------------------------------------------------------------
// Ability-level classification
// ---------------------------------------------------------------------------

/// Reasons ranked by how far the ability got; the highest one seen is reported.
const REASONS: [&str; 6] = [
    "no_references",
    "no_tooltip_text",
    "no_placeholder",
    "not_a_stat_entry",
    "no_rule_matched",
    // A rule matched but words of the stat phrase were left over ("cost of dual wield
    // abilities"): the channel would be a guess, so it is reported, never emitted.
    "partial_match",
];

fn rank(reason: &str) -> usize {
    REASONS.iter().position(|r| *r == reason).unwrap_or(0)
}

fn unclassified(reason: &str, clauses: Vec<String>) -> ResolvedClassification {
    ResolvedClassification {
        status: ClassStatus::Unclassified,
        channels: Vec::new(),
        direction: None,
        evidence: Vec::new(),
        reason: Some(reason.to_string()),
        unmatched_clauses: clauses,
    }
}

/// How `<<n>>` is tied to the (type, id) pairs of `tooltip_data` (see [`TooltipRef`]).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Mapping {
    /// `<<n>>` = `tooltip_data[n-1]` (the original, wrong assumption).
    EntryIndex,
    /// `<<n>>` = pair `n - 1` of all entries' (type, id) pairs, flattened.
    #[default]
    PairFlattened,
    /// No mapping: try every placeholder of the text; only clean matches are kept.
    AnyPlaceholder,
}

pub const ALL_MAPPINGS: [Mapping; 3] = [Mapping::EntryIndex, Mapping::PairFlattened, Mapping::AnyPlaceholder];

fn candidate_placeholders(r: &TooltipRef, text: &str, m: Mapping) -> Vec<usize> {
    match m {
        Mapping::EntryIndex => vec![r.entry + 1],
        Mapping::PairFlattened => vec![r.flat + 1],
        Mapping::AnyPlaceholder => (1..=max_placeholder(text)).collect(),
    }
}

/// `None` when the ability has no value/coef (nothing to classify).
pub fn classify_one(child: &SkillData34, corpus: &Corpus, graph: &Graph) -> Option<ResolvedClassification> {
    classify_one_with(child, corpus, graph, Mapping::default())
}

pub fn classify_one_with(
    child: &SkillData34,
    corpus: &Corpus,
    graph: &Graph,
    mapping: Mapping,
) -> Option<ResolvedClassification> {
    if !has_value(child) {
        return None;
    }
    let id = child.ability_id1;
    let mut evidence: Vec<(ResolvedEvidence, Vec<Channel>)> = Vec::new();
    let mut unmatched: Vec<String> = Vec::new();
    let mut reason = REASONS[0];
    let mut note = |r: &'static str| {
        if rank(r) > rank(reason) {
            reason = r;
        }
    };

    for r in graph.tooltip_refs.get(&id).into_iter().flatten() {
        if !is_stat_type(r.ty) {
            note("not_a_stat_entry");
            continue;
        }
        let texts = corpus.texts_for(r.parent);
        if texts.is_empty() {
            note("no_tooltip_text");
            continue;
        }
        // A parent can carry several alternative texts (e.g. a short passive text and a long
        // activated text) that number their placeholders differently. The pair list belongs to
        // the text whose highest <<n>> equals the number of pairs.
        let pairs: usize =
            corpus.skills.get(&r.parent).map_or(0, |p| p.tooltip_data.iter().map(|t| t.tooltip_ids.len()).sum::<usize>());
        let fitting: Vec<&str> = if mapping == Mapping::PairFlattened {
            texts.iter().copied().filter(|t| max_placeholder(t) == pairs).collect()
        } else {
            texts
        };
        if fitting.is_empty() {
            note("no_placeholder");
            continue;
        }
        for text in fitting {
            for n in candidate_placeholders(r, text, mapping) {
                let Some(clause) = clause_for(text, n) else {
                    note("no_placeholder");
                    continue;
                };
                match classify_clause(&clause) {
                    Some(c) if c.partial => {
                        note("partial_match");
                        if unmatched.len() < 3 && !unmatched.contains(&clause) {
                            unmatched.push(clause);
                        }
                    }
                    Some(c) => {
                        let agg = agg_kind(child, r.ty);
                        let channels: Vec<Channel> = c.channels.iter().map(|ch| with_agg(*ch, agg)).collect();
                        evidence.push((
                            ResolvedEvidence {
                                parent: r.parent,
                                placeholder: u8::try_from(n).unwrap_or(u8::MAX),
                                clause,
                                rule: c.rule,
                                channels: channels.clone(),
                                direction: c.direction,
                                indirect: r.hops > 0,
                            },
                            channels,
                        ));
                    }
                    None => {
                        note("no_rule_matched");
                        if unmatched.len() < 3 && !unmatched.contains(&clause) {
                            unmatched.push(clause);
                        }
                    }
                }
            }
        }
    }

    // Direct references outrank resolved-by-hops ones: a hop is a guess, a direct id is not.
    if evidence.iter().any(|(e, _)| !e.indirect) {
        evidence.retain(|(e, _)| !e.indirect);
    }

    if evidence.is_empty() {
        return Some(unclassified(reason, unmatched));
    }

    // Channel has no Ord, so group by linear scan (a handful of entries at most).
    let mut groups: Vec<(Vec<Channel>, Direction)> = Vec::new();
    for (e, ch) in &evidence {
        if !groups.iter().any(|(c, d)| c == ch && *d == e.direction) {
            groups.push((ch.clone(), e.direction));
        }
    }
    let all: Vec<ResolvedEvidence> = evidence.into_iter().map(|(e, _)| e).collect();
    if groups.len() == 1 {
        let (channels, direction) = groups.into_iter().next().expect("one group");
        return Some(ResolvedClassification {
            status: ClassStatus::Classified,
            channels,
            direction: Some(direction),
            evidence: all,
            reason: None,
            unmatched_clauses: Vec::new(),
        });
    }
    Some(ResolvedClassification {
        status: ClassStatus::Conflict,
        channels: Vec::new(),
        direction: None,
        evidence: all,
        reason: None,
        unmatched_clauses: Vec::new(),
    })
}

// ---------------------------------------------------------------------------
// Report (`esosim-data-compiler classify`)
// ---------------------------------------------------------------------------

/// Classify every record in the corpus and print coverage, the channel histogram, the
/// most common unmatched clauses (the to-do list for new rules) and a few conflicts.
pub fn report(corpus: &Corpus, graph: &Graph, limit: usize) {
    let mut with_value = 0usize;
    let mut classified = 0usize;
    let mut conflicts: Vec<(u32, ResolvedClassification)> = Vec::new();
    let mut reasons: BTreeMap<String, usize> = BTreeMap::new();
    let mut by_channel: BTreeMap<String, usize> = BTreeMap::new();
    let mut clauses: BTreeMap<String, usize> = BTreeMap::new();

    // Placeholder-numbering check: does the number of (type, id) pairs equal the sum over
    // the ability's texts of the highest <<n>>?
    let (mut parents, mut numbering_ok) = (0usize, 0usize);
    for (id, s) in &corpus.skills {
        let pairs: usize = s.tooltip_data.iter().map(|t| t.tooltip_ids.len()).sum();
        if pairs == 0 {
            continue;
        }
        let texts = corpus.texts_for(*id);
        if texts.is_empty() {
            continue;
        }
        parents += 1;
        if texts.iter().map(|t| max_placeholder(t)).sum::<usize>() == pairs {
            numbering_ok += 1;
        }
    }

    for (id, s) in &corpus.skills {
        let Some(c) = classify_one(s, corpus, graph) else { continue };
        with_value += 1;
        match c.status {
            ClassStatus::Classified => {
                classified += 1;
                *by_channel.entry(label(&c.channels)).or_insert(0) += 1;
            }
            ClassStatus::Conflict => conflicts.push((*id, c)),
            ClassStatus::Unclassified => {
                *reasons.entry(c.reason.clone().unwrap_or_default()).or_insert(0) += 1;
                for cl in c.unmatched_clauses {
                    *clauses.entry(cl).or_insert(0) += 1;
                }
            }
        }
    }

    println!("abilities with a value/coef: {with_value}");
    println!("  classified:   {classified}");
    println!("  conflicts:    {}", conflicts.len());
    println!("  unclassified: {}", reasons.values().sum::<usize>());
    for (r, n) in &reasons {
        println!("      {n:>6}  {r}");
    }
    println!(
        "placeholder numbering: {numbering_ok}/{parents} parents have (sum of max <<n>> over texts) == (number of tooltip (type,id) pairs)"
    );

    let mut top: Vec<_> = by_channel.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    println!("\nchannels:");
    for (ch, n) in top.iter().take(limit) {
        println!("  {n:>6}  {ch}");
    }

    let mut top: Vec<_> = clauses.into_iter().collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    println!("\nunmatched clauses (add a rule in classify.rs):");
    for (cl, n) in top.iter().take(limit) {
        println!("  {n:>6}  {cl}");
    }

    if !conflicts.is_empty() {
        println!("\nconflicts (first {}):", limit.min(conflicts.len()));
        for (id, c) in conflicts.iter().take(limit) {
            let names: Vec<String> =
                c.evidence.iter().map(|e| format!("{} <- {}", label(&e.channels), e.parent)).collect();
            println!("  {id}: {}", names.join(" | "));
        }
    }
}


// ---------------------------------------------------------------------------
// Mapping report (`esosim-data-compiler classify-mapping`)
// ---------------------------------------------------------------------------

/// Measure every [`Mapping`] against the real data and show why mismatches happen.
pub fn mapping_report(corpus: &Corpus, graph: &Graph, limit: usize) {
    let targets: Vec<&SkillData34> = corpus
        .skills
        .values()
        .filter(|s| has_value(s))
        .filter(|s| graph.tooltip_refs.contains_key(&s.ability_id1))
        .collect();
    println!("abilities with a value AND a resolved tooltip placeholder: {}", targets.len());
    println!("\n{:<16} {:>10} {:>10} {:>12} {:>14}", "mapping", "classified", "conflict", "unclassified", "clean/(c+conf)");
    for m in ALL_MAPPINGS {
        let (mut ok, mut conflict, mut un) = (0usize, 0usize, 0usize);
        for s in &targets {
            match classify_one_with(s, corpus, graph, m).map(|c| c.status) {
                Some(ClassStatus::Classified) => ok += 1,
                Some(ClassStatus::Conflict) => conflict += 1,
                Some(ClassStatus::Unclassified) => un += 1,
                None => {}
            }
        }
        let ratio = if ok + conflict == 0 { 0.0 } else { ok as f64 / (ok + conflict) as f64 };
        println!("{:<16} {ok:>10} {conflict:>10} {un:>12} {ratio:>14.3}", format!("{m:?}"));
    }

    // pairs - max placeholder, per text layout
    let mut hist_best: BTreeMap<i64, usize> = BTreeMap::new();
    let mut hist_sum: BTreeMap<i64, usize> = BTreeMap::new();
    let (mut two_texts, mut ragged) = (0usize, 0usize);
    let mut samples: Vec<u32> = Vec::new();
    for (id, s) in &corpus.skills {
        let pairs: usize = s.tooltip_data.iter().map(|t| t.tooltip_ids.len()).sum();
        let texts = corpus.texts_for(*id);
        if pairs == 0 || texts.is_empty() {
            continue;
        }
        if texts.len() > 1 {
            two_texts += 1;
        }
        if s.tooltip_data.iter().any(|t| t.tooltip_ids.len() != t.tooltip_types.len()) {
            ragged += 1;
        }
        let maxes: Vec<usize> = texts.iter().map(|t| max_placeholder(t)).collect();
        let best = maxes.iter().copied().max().unwrap_or(0);
        let sum: usize = maxes.iter().sum();
        *hist_best.entry(pairs as i64 - best as i64).or_insert(0) += 1;
        *hist_sum.entry(pairs as i64 - sum as i64).or_insert(0) += 1;
        if pairs as i64 - sum as i64 != 0 && samples.len() < limit {
            samples.push(*id);
        }
    }
    println!("\nparents with more than one tooltip text: {two_texts}; entries where ids.len() != types.len(): {ragged}");
    println!("\npairs - (largest <<n>> of any one text)");
    for (d, n) in &hist_best {
        println!("  {d:>4}  {n:>6}");
    }
    println!("\npairs - (sum of largest <<n>> over all texts)   (0 = numbering explained)");
    for (d, n) in &hist_sum {
        println!("  {d:>4}  {n:>6}");
    }

    println!("\nsample parents where the sum differs:");
    for id in samples {
        let s = &corpus.skills[&id];
        println!("--- {id}  {}", corpus.name_of(id));
        for (i, t) in corpus.texts_for(id).iter().enumerate() {
            println!("    text[{i}]: {}", t.replace('\n', " / "));
        }
        let mut flat = 0usize;
        for (k, td) in s.tooltip_data.iter().enumerate() {
            for (j, tid) in td.tooltip_ids.iter().enumerate() {
                let ty = td
                    .tooltip_types
                    .get(j)
                    .map_or_else(|| "-".to_string(), |t| TooltipType::from_id(t).map_or_else(|| format!("?{t}"), |x| format!("{x:?}")));
                println!("    <<{}>> entry {k}: {ty} id={tid}({})", flat + 1, corpus.name_of(*tid));
                flat += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eso_skill_data::{SkillCoef, TooltipData};
use esosim_types::{AggKind::{Additive, Multiplicative}, Channel::Resource, ResourceKind::Health};

    fn chans(clause: &str) -> Vec<String> {
        classify_clause(clause).map(|c| c.channels.iter().map(|ch| format!("{ch:?}")).collect()).unwrap_or_default()
    }

    fn dir(clause: &str) -> Direction {
        classify_clause(clause).expect("classified").direction
    }

    #[test]
    fn clause_picks_the_sentence_with_the_target_placeholder() {
        let text = "Deal <<1>> damage to enemies. Increases your Max Health by <<2>> for <<3>>|Also grants Major Sorcery";
        assert_eq!(clause_for(text, 2).as_deref(), Some("increases your max health by @ for #"));
        assert_eq!(clause_for(text, 1).as_deref(), Some("deal @ damage to enemies"));
        assert_eq!(clause_for(text, 9), None);
    }

    #[test]
    fn markup_and_decimal_points_do_not_split_clauses() {
        let text = "|cffffffIncreases|r your Weapon Damage by <<1>>, up to 2.5% more";
        assert_eq!(clause_for(text, 1).as_deref(), Some("increases your weapon damage by @, up to 2.5% more"));
    }

    #[test]
    fn verb_by_shape() {
        assert_eq!(chans("increases your max health by @"), ["Resource(Health, Additive)"]);
        assert_eq!(dir("increases your max health by @"), Direction::Raises);
        assert_eq!(chans("reduces damage taken from area attacks by @"), ["DamageTaken(AreaOfEffect)"]);
        assert_eq!(dir("reduces damage taken from area attacks by @"), Direction::Lowers);
        assert_eq!(chans("increases your weapon and spell damage by @"), ["Power"]);
        assert_eq!(chans("increases your weapon damage by @"), ["WeaponPower"]);
        assert_eq!(chans("increases your spell damage by @"), ["SpellPower"]);
    }

    #[test]
    fn the_closest_verb_wins() {
        assert_eq!(
            chans("reducing damage taken by # and increasing your max magicka by @"),
            ["Resource(Magicka, Additive)"]
        );
        assert_eq!(
            chans("increasing damage done by # and increasing damage done with heavy attacks against monsters by @"),
            ["DamageDone(HeavyAttack)"]
        );
        // "against monsters" is an extra condition no rule models, so it is flagged partial
        // (and therefore not accepted at ability level)
        let clause = "increasing damage done by # and increasing damage done with heavy attacks against monsters by @";
        assert!(classify_clause(clause).unwrap().partial);
    }

    #[test]
    fn lists_expand_to_several_channels() {
        assert_eq!(
            chans("increases your max magicka and max stamina by @"),
            [
                "Resource(Magicka, Additive)",
                "Resource(Stamina, Additive)"
            ]
        );
        assert_eq!(
            chans("increases your magicka and stamina recovery by @"),
            [
                "Recovery(Magicka, Additive)",
                "Recovery(Stamina, Additive)"
            ]
        );
        assert_eq!(
            chans("increases healing received and health recovery by @"),
            ["HealingTaken", "Recovery(Health, Additive)"]
        );
        assert_eq!(
            chans("increases your light and heavy attack damage by @"),
            ["DamageDone(LightAttack)", "DamageDone(HeavyAttack)"]
        );
    }

    #[test]
    fn specific_phrases_beat_the_generic_ones_they_contain() {
        assert_eq!(chans("increases critical damage taken by @"), ["CriticalDamageTaken"]);
        assert_eq!(chans("increases your critical damage done by @"), ["CriticalDamage"]);
        assert_eq!(chans("increases your spell critical rating by @"), ["SpellCriticalChance"]);
        assert_eq!(chans("increases your weapon and spell critical rating by @"), ["CriticalChance"]);
        assert_eq!(chans("increases your physical and spell resistance by @"), ["Armour(All)"]);
        assert_eq!(chans("increases your physical resistance by @"), ["Armour(Martial)"]);
        assert_eq!(chans("increases your fire resistance by @"), ["Armour(Flame)"]);
        assert_eq!(chans("reduces fall damage taken by @"), ["DamageTaken(Fall)"]);
        assert_eq!(chans("reduces your damage taken from players by @"), ["DamageTaken(Players)"]);
        assert_eq!(chans("reduces damage taken by @"), ["DamageTaken(All)"]);
    }

    #[test]
    fn participle_shape() {
        assert_eq!(chans("each stack increases critical damage taken by @"), ["CriticalDamageTaken"]);
        assert_eq!(chans("critical damage taken is increased by @"), ["CriticalDamageTaken"]);
        assert_eq!(dir("enemy movement speed is reduced by @"), Direction::Lowers);
    }

    #[test]
    fn give_shape_needs_a_verb_and_a_stat() {
        assert_eq!(chans("gain @ weapon and spell critical rating"), ["CriticalChance"]);
        assert_eq!(chans("grants you @ max health for #"), ["Resource(Health, Additive)"]);
        // "dealing @ damage" is a damage value, not a stat bonus
        assert!(chans("dealing @ damage to enemies").is_empty());
        assert!(chans("call forth talons, dealing @ to enemies near you").is_empty());
    }

    #[test]
    fn bare_nouns_only_inside_a_by_frame() {
        assert_eq!(chans("increases damage by @"), ["DamageDone(All)"]);
        assert!(chans("deals @ damage").is_empty());
    }

    #[test]
    fn costs_and_misc() {
        assert_eq!(chans("reduces the cost of sprint by @"), ["CostReductionCCA(Sprint)"]);
        assert_eq!(chans("reduces the cost of roll dodge by @"), ["CostReductionCCA(DodgeRoll)"]);
        assert_eq!(chans("reduces your magicka cost by @"), ["CostReduction(Magicka)"]);
        assert_eq!(chans("increases your movement speed by @"), ["MovementSpeed"]);
        assert_eq!(chans("increases potion effectiveness by @"), ["PotionEffectiveness"]);
        assert_eq!(
            chans("increases the magicka or stamina your heavy attacks restore by @"),
            ["HeavyAttackRestore(Magicka)", "HeavyAttackRestore(Stamina)"]
        );
        assert_eq!(chans("increases damage shield strength by @"), ["DamageShieldStrength"]);
    }

    #[test]
    fn a_value_is_not_the_stat_next_to_a_different_placeholder() {
        // @ is the movement speed; the recovery belongs to the next placeholder
        assert_eq!(chans("you gain @ movement speed and # magicka and stamina recovery for 20 seconds"), ["MovementSpeed"]);
        assert_eq!(
            chans("you gain # movement speed and @ magicka and stamina recovery for 20 seconds"),
            ["Recovery(Magicka, Additive)", "Recovery(Stamina, Additive)"]
        );
    }

    #[test]
    fn scaling_sources_and_buff_ids_are_not_stats() {
        // "absorbs up to @ of your max health": the shield's size, scaled by max health
        assert!(chans("gaining a damage shield that absorbs up to @ of your max health").is_empty());
        // "grants @ which increases ...": @ is a buff, the stat belongs to another placeholder
        assert!(chans("grants @ which increases your health recovery by #").is_empty());
    }

    #[test]
    fn longer_lists_and_new_phrases() {
        assert_eq!(
            chans("increases your health, magicka, and stamina recovery by @"),
            [
                "Recovery(Health, Additive)",
                "Recovery(Magicka, Additive)",
                "Recovery(Stamina, Additive)"
            ]
        );
        assert_eq!(
            chans("increases your disease and poison resistance by @"),
            ["Armour(Disease)", "Armour(Poison)"]
        );
        assert_eq!(chans("damage dealt by fire attacks increased by @"), ["DamageDone(Type(Flame))"]);
        assert_eq!(chans("reduces the stamina cost of roll dodge by @"), ["CostReductionCCA(DodgeRoll)"]);
        assert_eq!(chans("reduces the cost of sneak by @"), ["CostReductionCCA(Sneak)"]);
        assert_eq!(chans("increases the amount of damage you block by @"), ["BlockMitigation"]);
        assert_eq!(chans("increases your damage done to dungeon, trial, and arena monsters by @"), ["DamageDone(Monsters)"]);
        assert_eq!(chans("increases your healing with restoration staff spells by @"), ["HealingDoneSkillLine(RestorationStaff)"]);
    }

    #[test]
    fn partial_phrases_are_rejected_at_ability_level() {
        // a rule matches ("stamina cost") but "dual wield" is left over: a skill-line cost, not a mechanic cost
        assert!(classify_clause("reduces the stamina cost of dual wield abilities by @").is_some_and(|c| c.partial));
        let (c, g) = world("Reduces the Stamina cost of Dual Wield abilities by <<1>>", vec![], with_coef(skill(2000)));
        let r = classify_one(&c.skills[&2000], &c, &g).unwrap();
        assert_eq!(r.status, ClassStatus::Unclassified);
        assert_eq!(r.reason.as_deref(), Some("partial_match"));
    }

    #[test]
    fn unknown_phrases_are_not_guessed() {
        assert!(chans("increases your research speed by @").is_empty());
        assert!(chans("increases your wibble by @").is_empty());
    }

    // ---- ability level -----------------------------------------------------

    fn skill(id: u32) -> SkillData34 {
        let mut s = SkillData34::default();
        s.ability_id1 = id;
        s
    }

    fn id_of(t: TooltipType) -> u32 {
        (0u32..256).find(|i| TooltipType::from_id(i) == Some(t)).expect("tooltip type has an id")
    }

    fn with_coef(mut s: SkillData34) -> SkillData34 {
        s.coef = vec![SkillCoef { type1: 4, coef1: 0.1, ..Default::default() }];
        s
    }

    /// Parent 1000 whose single tooltip entry points at child 2000. Ids must be at or above
    /// `MIN_TOOLTIP_ABILITY_ID`, or the graph treats a tooltip id as a constant, not an ability.
    fn world(text: &str, types: Vec<u32>, child: SkillData34) -> (Corpus, Graph) {
        let mut parent = skill(1000);
        parent.tooltip_data = vec![TooltipData { tooltip_ids: vec![child.ability_id1], tooltip_types: types, ..Default::default() }];
        let mut c = Corpus::from_skills(vec![parent, child]);
        c.tooltips.insert(1000, text.to_string());
        let g = Graph::build(&c);
        (c, g)
    }

    #[test]
    fn an_ability_is_classified_from_the_tooltip_that_references_it() {
        let (c, g) = world("Increases your Max Health by <<1>>", vec![], with_coef(skill(2000)));
        let r = classify_one(&c.skills[&2000], &c, &g).unwrap();
        assert_eq!(r.status, ClassStatus::Classified);
        assert_eq!(r.channels, [Resource(Health, Additive)]);
        assert_eq!(r.direction, Some(Direction::Raises));
        assert_eq!(r.evidence.len(), 1);
        assert_eq!(r.evidence[0].parent, 1000);
        assert_eq!(r.evidence[0].placeholder, 1);
    }

    #[test]
    fn percent_value_without_equation_is_multiplicative() {
        let mut child = skill(2000);
        child.base_data.value1 = 5;
        let pct = id_of(TooltipType::Percentage);
        let (c, g) = world("Increases your Max Health by <<1>>", vec![pct], child);
        let r = classify_one(&c.skills[&2000], &c, &g).unwrap();
        assert_eq!(r.channels, [Resource(Health, Multiplicative)]);
    }

    #[test]
    fn duration_entries_are_not_stat_bonuses() {
        let dur = id_of(TooltipType::Duration);
        let (c, g) = world("Increases your Max Health by <<1>>", vec![dur], with_coef(skill(2000)));
        let r = classify_one(&c.skills[&2000], &c, &g).unwrap();
        assert_eq!(r.status, ClassStatus::Unclassified);
        assert_eq!(r.reason.as_deref(), Some("not_a_stat_entry"));
    }

    #[test]
    fn abilities_without_a_value_are_skipped_and_unreferenced_ones_say_why() {
        let (c, g) = world("Increases your Max Health by <<1>>", vec![], skill(2000));
        assert!(classify_one(&c.skills[&2000], &c, &g).is_none());

        let lonely = with_coef(skill(3000));
        let c = Corpus::from_skills(vec![lonely]);
        let g = Graph::build(&c);
        let r = classify_one(&c.skills[&3000], &c, &g).unwrap();
        assert_eq!(r.reason.as_deref(), Some("no_references"));
    }

    #[test]
    fn unmatched_clauses_are_kept_for_the_report() {
        let (c, g) = world("Increases your wibble by <<1>>", vec![], with_coef(skill(2000)));
        let r = classify_one(&c.skills[&2000], &c, &g).unwrap();
        assert_eq!(r.reason.as_deref(), Some("no_rule_matched"));
        assert_eq!(r.unmatched_clauses, ["increases your wibble by @"]);
    }

    #[test]
    fn disagreeing_parents_are_a_conflict() {
        let child = with_coef(skill(2000));
        let mut a = skill(1000);
        a.tooltip_data = vec![TooltipData { tooltip_ids: vec![2000], ..Default::default() }];
        let mut b = skill(1001);
        b.tooltip_data = vec![TooltipData { tooltip_ids: vec![2000], ..Default::default() }];
        let mut c = Corpus::from_skills(vec![a, b, child]);
        c.tooltips.insert(1000, "Increases your Max Health by <<1>>".into());
        c.tooltips.insert(1001, "Increases your Max Magicka by <<1>>".into());
        let g = Graph::build(&c);
        let r = classify_one(&c.skills[&2000], &c, &g).unwrap();
        assert_eq!(r.status, ClassStatus::Conflict);
        assert_eq!(r.evidence.len(), 2);
    }

    #[test]
    fn agreeing_parents_are_one_classification_with_both_as_evidence() {
        let child = with_coef(skill(2000));
        let mut a = skill(1000);
        a.tooltip_data = vec![TooltipData { tooltip_ids: vec![2000], ..Default::default() }];
        let mut b = skill(1001);
        b.tooltip_data = vec![TooltipData { tooltip_ids: vec![2000], ..Default::default() }];
        let mut c = Corpus::from_skills(vec![a, b, child]);
        c.tooltips.insert(1000, "Increases your Max Health by <<1>>".into());
        c.tooltips.insert(1001, "Increase your Max Health by <<1>> while slotted".into());
        let g = Graph::build(&c);
        let r = classify_one(&c.skills[&2000], &c, &g).unwrap();
        assert_eq!(r.status, ClassStatus::Classified);
        assert_eq!(r.evidence.len(), 2);
    }

    #[test]
    fn every_rule_name_is_unique_and_every_pattern_compiles() {
        let mut names: Vec<&str> = LEXICON.iter().chain(BARE.iter()).map(|n| n.rule).collect();
        let total = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), total, "duplicate rule names");
    }
}