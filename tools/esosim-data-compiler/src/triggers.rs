//! Trigger / condition lexicon: *when* does a tooltip say an effect happens? (roadmap 2.0 item 3,
//! 2.1b classes 3-4, 6.4; Appendix A.2.)
//!
//! `classify.rs` answers "which stat is this number?". This module answers the other half of an
//! `EffectDef`: "under what circumstances?". It reads a tooltip (or one clause of it) and returns
//! typed [`Cue`]s:
//!
//! | cue              | tooltip shape                                   | becomes                          |
//! |------------------|-------------------------------------------------|----------------------------------|
//! | `On(Event)`     | "when you drink a potion", "dealing critical damage" | a roadmap 6.4 `Trigger`     |
//! | `While(State)`  | "while in combat", "under 50% health", "while blocking" | a [`Condition`] where one exists |
//! | `Per(Count)`    | "per stack", "for each piece of heavy armor"    | `Stacking::PerStack` / a scale   |
//! | `Against(Foe)`  | "against monsters"                              | a target filter                  |
//! | `MaxStacks`, `Cooldown`, `Interval` | "up to 5 stacks", "once every #", "every 1.5 seconds" | `EffectDef` fields |
//! | `OffersSynergy` | "an ally may activate the # synergy"            | a synergy grant (not a trigger)  |
//!
//! [`Cue::to_condition`] is the bridge to `esosim-types`: it returns `Some` only for the cues the
//! current [`Condition`] enum can already express (in combat, health thresholds, bar, armour
//! pieces). Every other cue returns `None`, and the roadmap hand-off is the list of cues that need a
//! new `Condition` variant or a `Trigger` type (see `Event` and `State`).
//!
//! Like the channel lexicon, rules are a table of `rule("name", r"pattern", |caps| cues)` lines.
//! Patterns match lower-case text, use the `regex` crate's syntax (no look-around, no
//! back-references), must be written as one literal each (the tests and the review script read the
//! source) and must not contain `"`. Order matters: a rule *consumes* the text it matches, so
//! specific phrases come before the generic ones they contain (`take critical damage` before
//! `take damage`, `out of combat` before `in combat`, `an ally can activate the # synergy` before
//! `activate a synergy`).

use crate::classify::{BOUNDARY, MARKUP};
use esosim_types::{
    Affliction, ArmourWeight, Bar, CastKind, Condition, DamageDoneContext as DoneCtx, Form, PlayerClass,
    ResourceKind as Res, SelfState, TargetKind, TriggerEvent,
};
use eso_skill_data::enums::skill_line::SkillLine;
use crate::resolved::CueRecord;
use regex::{Captures, Regex};
use std::sync::LazyLock;

// ---------------------------------------------------------------------------
// Vocabulary
// ---------------------------------------------------------------------------

/// What kind of ability is being cast ("casting an ability", "casting a templar skill").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cast {
    /// "casting an ability", "using a spell"
    Any,
    /// "when you use an ultimate ability"
    Ultimate,
    /// "casting a non-ultimate ability"
    NonUltimate,
    /// "casting an ability with a cost"
    WithCost,
    /// "using an ability with a channel or cast time"
    CastOrChannelTime,
    /// "casting a templar skill", "using a siphoning ability": the matched text names the line
    SkillLine,
}

/// Something that *happens*: the thing a proc waits for (roadmap 6.4 `Trigger`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Event {
    /// "dealing damage", "when you deal direct damage", "damage with a light attack". The context
    /// is the same enum the damage-done channels use, so `LightAttack`, `HeavyAttack`, `Direct`,
    /// `DamageOverTime`, `AreaOfEffect` and `Bash` carry over; `All` is any damage.
    DealDamage(DoneCtx),
    /// "dealing critical damage", "when you critically strike"
    DealCritical,
    /// "when you take damage"
    TakeDamage,
    /// "when you take critical damage"
    TakeCritical,
    /// "activating a critical heal"
    CriticalHeal,
    /// "after completing a fully-charged heavy attack"
    FullyChargedHeavy,
    /// "when you block", "on block", "after blocking"
    Block,
    /// "whenever you bash a target"
    Bash,
    /// "when you roll dodge", "after dodge-rolling", "whenever you successfully dodge"
    RollDodge,
    Cast(Cast),
    /// "activating a synergy", "when you activate a synergy ability"
    ActivateSynergy,
    /// "drinking a potion", "when you drink a potion", "when using potions"
    DrinkPotion,
    /// "when you kill an enemy", "when an enemy you recently damaged dies"
    Kill,
    /// "on death", "when you die"
    Die,
    /// "when you interrupt an enemy"
    Interrupt,
    /// "when you enter combat"
    EnterCombat,
    /// "when you leave combat"
    LeaveCombat,
    /// "when you leave stealth", "when coming out of stealth"
    LeaveStealth,
    /// "when you heal yourself or an ally"
    Heal,
    /// the ability itself: "upon activation", "on use", "when cast", "when triggered"
    Activate,
    /// "when the effect ends"
    EffectEnds,
}

/// Something that is *true for a while*: the thing a conditional bonus waits for.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum State {
    InCombat,
    OutOfCombat,
    Blocking,
    /// sneaking, in stealth, invisible
    Stealthed,
    Sprinting,
    Mounted,
    /// "while channeling", "while you are using an ability with a channel or cast time"
    Channeling,
    /// "while active", "while toggled", "while the rune is active"
    Active,
    /// "while slotted" (the ability's own slotting, not a condition of the effect)
    Slotted,
    /// "while slotted on either bar"
    SlottedOnEitherBar,
    /// "while you have an armor active ability slotted": a skill-line slot count
    SkillLineSlotted,
    OnFrontBar,
    OnBackBar,
    Immobilized,
    Stunned,
    Snared,
    Feared,
    Disoriented,
    OffBalance,
    Silenced,
    WerewolfForm,
    /// transformed into something else (wraith, bear, ...)
    Transformed,
    /// Cyrodiil, Imperial City, a campaign, a battleground
    InPvpArea,
    /// one of your resources is under a percentage ("when you are under 75% health"); `None` = the
    /// number is a tooltip placeholder
    ResourceBelow(Res, Option<u8>),
    /// "while you have 50% or more stamina", "at full health" (`Some(100)`)
    ResourceAbove(Res, Option<u8>),
    /// "enemies below 25% health"
    TargetHealthBelow(Option<u8>),
    /// "enemies above 90% health"
    TargetHealthAbove(Option<u8>),
    /// "while you have a damage shield active", "while under the effect of ...": the matched text
    /// names the buff
    SelfBuff,
    /// "on an enemy with primal fury", "against targets with battle spirit"
    TargetDebuff,
    /// "while wearing 5 or more pieces of heavy armor"; the count is `None` when not stated
    WearingArmour(ArmourWeight, Option<u8>),
}

/// What a value scales with ("per stack").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Count {
    Stack,
    ArmourPiece(ArmourWeight),
    /// "per type of armor"
    ArmourType,
    Second,
    Tick,
    /// "for each enemy hit", "per target"
    Target,
    /// champion point stage
    Stage,
    Crux,
    PointsSpent,
    Level,
    MissingHealth,
    /// "for each support ability slotted": a skill-line slot count
    SlottedAbility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Foe {
    Monsters,
    Players,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cue {
    On(Event),
    While(State),
    Per(Count),
    Against(Foe),
    /// "with a maximum stack of 5"; `None` = a placeholder
    MaxStacks(Option<u8>),
    /// "once every #", "can only occur once per second": an internal cooldown
    Cooldown,
    /// "every 1.5 seconds", "every second": a repeating tick
    Interval,
    /// "an ally may activate the # synergy": the ability offers a synergy
    OffersSynergy,
}

impl Cue {
    /// The cue as a [`Condition`], when it can be said without looking at the matched text. `None`
    /// for events (see [`to_event`](Self::to_event)), for placeholder thresholds, and for the states
    /// that need the text: the crowd-control states (whose subject, you or the target, is in the
    /// words), skill-line slot counts and named buffs/debuffs. [`CueMatch::condition`] answers
    /// those where the text allows. `Slotted`, `SlottedOnEitherBar` are conditions of the source
    /// ability, not of the effect, so they are `None` here too.
    pub fn to_condition(&self) -> Option<Condition> {
        use Condition as C;
        let state = |s: SelfState| Some(C::SelfState(s));
        match self {
            Cue::While(State::InCombat) => Some(C::InCombat),
            Cue::While(State::OutOfCombat) => Some(C::out_of_combat()),
            Cue::While(State::ResourceBelow(Res::Health, Some(p))) => Some(C::SelfHealthBelowPct(*p)),
            Cue::While(State::ResourceBelow(r, Some(p))) => Some(C::SelfResourceBelowPct { resource: *r, pct: *p }),
            Cue::While(State::ResourceAbove(r, Some(p))) => Some(C::SelfResourceAbovePct { resource: *r, pct: *p }),
            Cue::While(State::TargetHealthBelow(Some(p))) => Some(C::TargetHealthBelowPct(*p)),
            Cue::While(State::TargetHealthAbove(Some(p))) => Some(C::TargetHealthAbovePct(*p)),
            Cue::While(State::OnFrontBar) => Some(C::OnBar(Bar::Front)),
            Cue::While(State::OnBackBar) => Some(C::OnBar(Bar::Backup)),
            Cue::While(State::WearingArmour(weight, Some(n))) => Some(C::ArmourPieces { weight: *weight, at_least: *n }),
            Cue::While(State::Blocking) => state(SelfState::Blocking),
            Cue::While(State::Stealthed) => state(SelfState::Stealthed),
            Cue::While(State::Sprinting) => state(SelfState::Sprinting),
            Cue::While(State::Mounted) => state(SelfState::Mounted),
            Cue::While(State::Channeling) => state(SelfState::Channeling),
            Cue::While(State::Active) => state(SelfState::SourceActive),
            Cue::While(State::WerewolfForm) => Some(C::InForm(Form::Werewolf)),
            Cue::While(State::Transformed) => Some(C::InForm(Form::Transformed)),
            Cue::While(State::InPvpArea) => Some(C::InPvpArea),
            Cue::Against(Foe::Monsters) => Some(C::TargetIs(TargetKind::Monster)),
            Cue::Against(Foe::Players) => Some(C::TargetIs(TargetKind::Player)),
            _ => None,
        }
    }

    /// The cue as a [`TriggerEvent`], for `On(..)` cues. A cast of "a <line> ability" has no line
    /// here (the text names it): it comes back as `Cast(Other)`; [`CueMatch::event`] resolves it.
    pub fn to_event(&self) -> Option<TriggerEvent> {
        let Cue::On(e) = self else { return None };
        Some(match e {
            Event::DealDamage(ctx) => TriggerEvent::DealDamage(*ctx),
            Event::DealCritical => TriggerEvent::DealCritical,
            Event::TakeDamage => TriggerEvent::TakeDamage,
            Event::TakeCritical => TriggerEvent::TakeCritical,
            Event::CriticalHeal => TriggerEvent::CriticalHeal,
            Event::FullyChargedHeavy => TriggerEvent::FullyChargedHeavy,
            Event::Block => TriggerEvent::Block,
            Event::Bash => TriggerEvent::Bash,
            Event::RollDodge => TriggerEvent::RollDodge,
            Event::Cast(c) => TriggerEvent::Cast(match c {
                Cast::Any => CastKind::Any,
                Cast::Ultimate => CastKind::Ultimate,
                Cast::NonUltimate => CastKind::NonUltimate,
                Cast::WithCost => CastKind::WithCost,
                Cast::CastOrChannelTime => CastKind::CastOrChannelTime,
                Cast::SkillLine => CastKind::Other,
            }),
            Event::ActivateSynergy => TriggerEvent::ActivateSynergy,
            Event::DrinkPotion => TriggerEvent::DrinkPotion,
            Event::Kill => TriggerEvent::Kill,
            Event::Die => TriggerEvent::Die,
            Event::Interrupt => TriggerEvent::Interrupt,
            Event::EnterCombat => TriggerEvent::EnterCombat,
            Event::LeaveCombat => TriggerEvent::LeaveCombat,
            Event::LeaveStealth => TriggerEvent::LeaveStealth,
            Event::Heal => TriggerEvent::Heal,
            Event::Activate => TriggerEvent::Activate,
            Event::EffectEnds => TriggerEvent::EffectEnds,
        })
    }

    /// True for cues that describe a combat-time trigger rather than a static circumstance.
    pub fn is_event(&self) -> bool {
        matches!(self, Cue::On(_))
    }
}

// ---------------------------------------------------------------------------
// Text -> clauses
// ---------------------------------------------------------------------------

/// Every placeholder (the broken `<<1)>>` the language files contain included) and the classifier's
/// `@` marker become `#`.
static PLACEHOLDER: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"<<[^>]*>>|@").unwrap());

/// Lower-case clauses of a tooltip: markup removed, placeholders written as `#`, split where
/// `classify::clause_for` splits (line breaks, `|`, `;`, `:`, bullets, full stops).
pub fn clauses(text: &str) -> Vec<String> {
    let clean = MARKUP.replace_all(text, "");
    let marked = PLACEHOLDER.replace_all(&clean, "#");
    BOUNDARY
        .split(&marked)
        .map(|seg| seg.to_lowercase().split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|seg| !seg.is_empty())
        .collect()
}

// ---------------------------------------------------------------------------
// Lexicon: phrase -> cues
// ---------------------------------------------------------------------------

struct Rule {
    name: &'static str,
    re: Regex,
    build: fn(&Captures) -> Vec<Cue>,
}

fn rule(name: &'static str, re: &str, build: fn(&Captures) -> Vec<Cue>) -> Rule {
    // a trailing `\b` cannot follow a `#` placeholder (neither side is a word character)
    let tail = if re.ends_with('#') { "" } else { r"\b" };
    let re = Regex::new(&format!(r"\b(?:{re}){tail}"))
        .unwrap_or_else(|e| panic!("trigger rule `{name}` does not compile: {e}"));
    Rule { name, re, build }
}

fn cap<'a>(c: &'a Captures, name: &str) -> Option<&'a str> {
    c.name(name).map(|m| m.as_str())
}

/// `75` -> `Some(75)`; a placeholder (`#`) or nothing -> `None`.
fn number(s: Option<&str>) -> Option<u8> {
    s.and_then(|s| s.parse::<u8>().ok())
}

/// "5" / "five" -> 5.
fn count_word(s: Option<&str>) -> Option<u8> {
    match s? {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        other => other.parse::<u8>().ok(),
    }
}

fn weight_of(s: &str) -> ArmourWeight {
    match s {
        "light" => ArmourWeight::Light,
        "medium" => ArmourWeight::Medium,
        _ => ArmourWeight::Heavy,
    }
}

fn res_of(s: &str) -> Res {
    match s {
        "health" => Res::Health,
        "magicka" => Res::Magicka,
        _ => Res::Stamina,
    }
}

/// Words that follow a cast verb without naming a skill line ("when you use this ability").
const NOT_A_LINE: &[&str] = &[
    "a", "an", "any", "another", "each", "every", "its", "that", "the", "their", "this", "your", "next", "same",
    "one", "other", "damage", "healing", "class", "weapon", "magicka", "stamina", "channeled", "instant",
];

use Cue as Q;
use Event as E;
use State as S;

/// See the module comment for why the order is what it is. Rules are grouped: states first (the
/// "while ..." circumstances), then events, then scaling and timing.
static LEXICON: LazyLock<Vec<Rule>> = LazyLock::new(|| {
    vec![
        // ---- combat state ------------------------------------------------------------------
        rule("out_of_combat", r"(?:while |when |if |whilst )?(?:you are |you're |being |remaining |staying )?(?:out of combat|not in combat|outside of combat|outside combat)", |_| vec![Q::While(S::OutOfCombat)]),
        rule("in_combat", r"(?:while |when |if |whilst |during )?(?:you are |you're |being |remaining |staying )?in combat|during combat", |_| vec![Q::While(S::InCombat)]),
        // ---- events that must be read before the states/casts they contain ---------------------
        rule("enter_combat", r"(?:when|upon|after|whenever|each time) (?:you )?(?:enter(?:s|ing)?|start(?:s|ing)?|begin(?:s|ning)?) (?:in )?combat|on (?:entering|starting) combat|when combat (?:starts|begins)", |_| vec![Q::On(E::EnterCombat)]),
        rule("leave_combat", r"(?:when|upon|after|whenever) (?:you )?(?:leav(?:e|es|ing)|exit(?:s|ing)?) combat|when combat ends|on (?:leaving|exiting) combat", |_| vec![Q::On(E::LeaveCombat)]),
        rule("leave_stealth", r"(?:when|upon|after|whenever) (?:you )?(?:leav(?:e|es|ing)|exit(?:s|ing)?|com(?:e|es|ing) out of|break(?:s|ing)?)(?: out of)? (?:stealth|sneak|invisibility)(?: or invisibility)?", |_| vec![Q::On(E::LeaveStealth)]),
        // ---- states -------------------------------------------------------------------------
        rule("blocking", r"(?:while|as long as) (?:you are |you're |you remain |you |the wearer is )?(?:block(?:ing)?|brac(?:ing|ed))|(?:when|if) (?:you are|you're) (?:blocking|bracing)", |_| vec![Q::While(S::Blocking)]),
        rule("stealthed", r"(?:while|when|if|as long as) (?:you are |you're |you remain |you |the wearer is )?(?:sneaking|sneak|in stealth|stealthed|invisible|crouching)", |_| vec![Q::While(S::Stealthed)]),
        rule("sprinting", r"(?:while|when|if|as long as) (?:you are |you're |you |the wearer is )?(?:sprinting|running)", |_| vec![Q::While(S::Sprinting)]),
        rule("mounted", r"(?:while|when|if|as long as) (?:you are |you're |you |the wearer is )?(?:mounted|riding(?: a mount)?|on (?:a |your )mount)", |_| vec![Q::While(S::Mounted)]),
        rule("using_channel_ability", r"(?:you are |you're )using an? abilit(?:y|ies) with an? (?:cast|channel)(?: or (?:channel|cast))?(?: time)?", |_| vec![Q::While(S::Channeling)]),
        rule("channeling", r"(?:while|when|if|as long as) (?:you are |you're |you |the wearer is )?channel(?:l)?ing(?: this (?:ability|attack|spell))?", |_| vec![Q::While(S::Channeling)]),
        rule("active", r"(?:while|as long as|whilst) (?:(?:this|the|your|it is|it's) )?(?:(?:effect|ability|buff|rune|aura|toggle|skill|spell|shield)s? )?(?:is |remains |stays |are )?(?:toggled(?: on)?|active|activated|running)", |_| vec![Q::While(S::Active)]),
        rule("slotted_either_bar", r"(?:(?:while|when|if|as long as) (?:you have (?:this )?)?)?slotted on (?:either|any|both) (?:ability )?bars?", |_| vec![Q::While(S::SlottedOnEitherBar)]),
        rule("skill_line_slotted", r"(?:while|when|if|as long as) you have (?:an? |any |at least \d+ )?(?:[a-z']+ ){1,3}?(?:abilit(?:y|ies) )?slotted", |_| vec![Q::While(S::SkillLineSlotted)]),
        rule("slotted", r"(?:while|when|if|as long as) slotted", |_| vec![Q::While(S::Slotted)]),
        rule("on_bar", r"(?:while|when|if|as long as) (?:you are |you're )?(?:on|using|with) (?:the |your )?(?P<bar>front|main|primary|active|back|backup|secondary|inactive) (?:ability )?bar", |c| {
            match cap(c, "bar") {
                Some("back" | "backup" | "secondary" | "inactive") => vec![Q::While(S::OnBackBar)],
                _ => vec![Q::While(S::OnFrontBar)],
            }
        }),
        rule("crowd_controlled", r"(?:while|when|if|as long as|whilst) (?:you are |you're |you |the target is |they are |it is |enemies are |the enemy is )?(?:being )?(?P<cc>stunned|immobili[sz]ed|snared|rooted|feared|disoriented|silenced|off.balance|staggered)", |c| {
            vec![Q::While(match cap(c, "cc") {
                Some("stunned" | "staggered") => S::Stunned,
                Some("snared") => S::Snared,
                Some("feared") => S::Feared,
                Some("disoriented") => S::Disoriented,
                Some("silenced") => S::Silenced,
                Some(w) if w.starts_with("off") => S::OffBalance,
                _ => S::Immobilized,
            })]
        }),
        rule("werewolf_form", r"(?:while |when |if |as long as )?(?:you are |you're |you remain )?(?:in werewolf form|as a werewolf|transformed into a werewolf)", |_| vec![Q::While(S::WerewolfForm)]),
        rule("transformed", r"(?:while|when|if|as long as) (?:you are |you're |you remain )?(?:transformed|polymorphed)", |_| vec![Q::While(S::Transformed)]),
        rule("pvp_area", r"(?:while|when|if|as long as|whilst) (?:you are |you're |you remain )?(?:in|inside|within) (?:cyrodiil|imperial city|your campaign|a campaign|an? battleground|pvp|player vs player|(?:a )?pvp areas?)", |_| vec![Q::While(S::InPvpArea)]),
        rule("wearing_armour", r"(?:while|when|if|as long as) (?:you are |you're )?(?:wearing|wear|have) (?:at least |only )?(?:(?P<n>\d+|one|two|three|four|five|six|seven) )?(?:or more )?(?:pieces? of |items? of )?(?P<w>light|medium|heavy) armou?r", |c| {
            vec![Q::While(S::WearingArmour(weight_of(cap(c, "w").unwrap_or("heavy")), count_word(cap(c, "n"))))]
        }),
        // ---- health and resource thresholds --------------------------------------------------
        rule("target_health_below", r"(?:enem(?:y|ies)|targets?|monsters?|foes?|bosses|boss)(?: (?:is|are|has|have|with|at|that|who|which|falls|fall|drops|drop|goes|go|to|or|a|their|its|the))*? (?:below|under|less than|beneath) (?P<pct>\d{1,3}|#)%? (?:of )?(?:their |its |the )?(?:max(?:imum)? )?health", |c| vec![Q::While(S::TargetHealthBelow(number(cap(c, "pct"))))]),
        rule("target_health_above", r"(?:enem(?:y|ies)|targets?|monsters?|foes?|bosses|boss|they|them)(?: (?:is|are|has|have|with|at|that|who|which|stays|stay|remains|remain|a|their|its|the))*? (?:above|over|more than|greater than) (?P<pct>\d{1,3}|#)%? (?:of )?(?:their |its |the )?(?:max(?:imum)? )?health", |c| vec![Q::While(S::TargetHealthAbove(number(cap(c, "pct"))))]),
        rule("self_resource_below", r"(?:you are|you're|you have|your (?:health|magicka|stamina) (?:is|falls|drops)) (?:also )?(?:at or )?(?:below|under|less than|beneath) (?P<pct>\d{1,3}|#)%? (?:of )?(?:your )?(?:max(?:imum)? )?(?P<r>health|magicka|stamina)", |c| vec![Q::While(S::ResourceBelow(res_of(cap(c, "r").unwrap_or("health")), number(cap(c, "pct"))))]),
        rule("own_resource_below", r"your (?P<r>health|magicka|stamina) is (?:(?:below|under|less than) (?P<pct>\d{1,3}|#)|(?P<pct2>\d{1,3}|#)% or less)", |c| vec![Q::While(S::ResourceBelow(res_of(cap(c, "r").unwrap_or("health")), number(cap(c, "pct").or(cap(c, "pct2")))))]),
        rule("own_resource_above", r"your (?P<r>health|magicka|stamina) is (?:(?:above|over|at least|more than|greater than) (?P<pct>\d{1,3}|#)|(?P<pct2>\d{1,3}|#)% or more)", |c| vec![Q::While(S::ResourceAbove(res_of(cap(c, "r").unwrap_or("health")), number(cap(c, "pct").or(cap(c, "pct2")))))]),
        rule("self_resource_above", r"(?:you are|you're|you have|your (?:health|magicka|stamina) (?:is|stays|remains)) (?:also )?(?:(?:at or )?(?:above|over)|more than|greater than|at least) (?P<pct>\d{1,3}|#)%? (?:of )?(?:your )?(?:max(?:imum)? )?(?P<r>health|magicka|stamina)|you have (?P<pct2>\d{1,3}|#)%? or more (?:of your )?(?P<r2>health|magicka|stamina)", |c| {
            let res = cap(c, "r").or(cap(c, "r2")).unwrap_or("health");
            vec![Q::While(S::ResourceAbove(res_of(res), number(cap(c, "pct").or(cap(c, "pct2")))))]
        }),
        rule("full_resource", r"(?:at|with) full (?P<r>health|magicka|stamina)", |c| vec![Q::While(S::ResourceAbove(res_of(cap(c, "r").unwrap_or("health")), Some(100)))]),
        // ---- buffs on you / debuffs on the target ------------------------------------------
        rule("self_buff", r"(?:while|when|if|as long as) you (?:have|are under|are affected by) (?:an? |any |the |at least (?:one|\d+) |one or more |\d+ or more )?(?:[a-z'-]+ ){0,4}?(?:active|crux|stacks?|buff|debuff)(?: active)?(?: on yourself)?|while under the (?:effect|effects|influence) of (?:the |an? )?(?:[a-z'-]+ ?){1,3}|while affected by (?:the |an? )?(?:[a-z'-]+ ?){1,3}", |_| vec![Q::While(S::SelfBuff)]),
        rule("target_debuff", r"(?:on|against|vs|versus|hitting) (?:an? |the |any )?(?:enem(?:y|ies)|targets?|foes?|players?|monsters?) (?:that (?:have|has|are|is) |who (?:have|has|are|is) |with |affected by |afflicted with |under the effect of |suffering from |inflicted with )(?:an? |the |#\s)?(?:[a-z'-]+ ?){1,2}", |_| vec![Q::While(S::TargetDebuff)]),
        // ---- events: damage ----------------------------------------------------------------
        rule("take_critical", r"(?:tak(?:e|es|ing)|receiv(?:e|es|ing)|suffer(?:s|ing)?) (?:a )?critical (?:strike|hit|damage)s?|critical (?:strike|hit)s? (?:against|on) you|(?:are|get|gets|is) critically (?:struck|hit)", |_| vec![Q::On(E::TakeCritical)]),
        rule("take_damage", r"(?:(?:when|whenever|after|upon|each time|every time|if) (?:you |the wearer |an ally |allies )?(?:tak(?:e|es)|receiv(?:e|es)|suffer(?:s)?)|(?:tak|receiv|suffer)ing) (?:\w+ ){0,2}damage|(?:when|whenever|after|each time|every time) (?:you|the wearer) (?:are|get) (?:hit|damaged|attacked)", |_| vec![Q::On(E::TakeDamage)]),
        rule("deal_critical", r"(?:deal(?:s|ing)?|caus(?:e|es|ing)|inflict(?:s|ing)?|land(?:s|ing)?|scor(?:e|es|ing)) (?:a )?(?:\w+ )?critical (?:strike|hit|damage)s?|critically (?:strik(?:e|es|ing)|hit(?:s|ting)?)|(?:on|upon|after) (?:a |each |every )?critical (?:strike|hit)s?", |_| vec![Q::On(E::DealCritical)]),
        rule("critical_heal", r"(?:activat(?:e|es|ing)|land(?:s|ing)?|scor(?:e|es|ing)|deal(?:s|ing)?) (?:a )?critical (?:heal|healing)s?|critical heals? (?:create|creates|cause|causes|grant|grants|trigger|triggers)|(?:on|upon|after) (?:a )?critical heal", |_| vec![Q::On(E::CriticalHeal)]),
        rule("fully_charged_heavy", r"(?:complet(?:e|es|ing)|releas(?:e|es|ing)|finish(?:es|ing)?|land(?:s|ing)?) (?:a |an |your )?fully.?charged heavy attacks?|fully.?charged heavy attacks? (?:hits?|grants?|deals?)", |_| vec![Q::On(E::FullyChargedHeavy)]),
        rule("deal_light_heavy", r"(?:deal(?:s|ing)?|caus(?:e|es|ing)|inflict(?:s|ing)?) (?:\w+ ){0,2}damage with (?:a |an |your |any )?(?:light and heavy|heavy and light|light or heavy|heavy or light)(?: attacks?)?|(?:your |with )?light and heavy attacks? (?:now )?(?:generate|grant|apply|cause|restore|heal|deal)s?", |_| vec![Q::On(E::DealDamage(DoneCtx::LightAttack)), Q::On(E::DealDamage(DoneCtx::HeavyAttack))]),
        rule("deal_light", r"(?:deal(?:s|ing)?|caus(?:e|es|ing)|inflict(?:s|ing)?) (?:\w+ ){0,2}damage with (?:a |an |your |any )?light attacks?|light attacks? (?:hits?|deals?)", |_| vec![Q::On(E::DealDamage(DoneCtx::LightAttack))]),
        rule("deal_heavy", r"(?:deal(?:s|ing)?|caus(?:e|es|ing)|inflict(?:s|ing)?) (?:\w+ ){0,2}damage with (?:a |an |your |any )?heavy attacks?|heavy attacks? (?:hits?|deals?)", |_| vec![Q::On(E::DealDamage(DoneCtx::HeavyAttack))]),
        rule("bash", r"(?:when|whenever|after|upon|each time|every time) you (?:use |perform )?(?:a )?bash(?:es)?(?: attack)?|(?:use|using|perform|performing) (?:a )?bash(?: attack)?s?|(?:deal(?:s|ing)?|caus(?:e|es|ing)) (?:\w+ )?bash damage|bash(?:es)? hits?|damage with a bash attack", |_| vec![Q::On(E::Bash)]),
        rule("deal_dot", r"(?:deal(?:s|ing)?|caus(?:e|es|ing)|inflict(?:s|ing)?) (?:\w+ )?damage over time", |_| vec![Q::On(E::DealDamage(DoneCtx::DamageOverTime))]),
        rule("deal_direct", r"(?:deal(?:s|ing)?|caus(?:e|es|ing)|inflict(?:s|ing)?) (?:\w+ )?direct damage", |_| vec![Q::On(E::DealDamage(DoneCtx::Direct))]),
        rule("deal_damage", r"(?:(?:when|whenever|after|upon|each time|every time|if|while) (?:you |the wearer |your \w+ )?(?:deal|caus|inflict|do)(?:s|e|es)?|(?:deal|caus|inflict|do)ing) (?:an? |any )?(?:\w+ ){0,3}damage|(?:when|whenever|after) (?:you )?damag(?:e|ing) (?:an? |the |any )?(?:enem(?:y|ies)|target|them|foes?)|damaging (?:them|an enemy|enemies|a target)", |_| vec![Q::On(E::DealDamage(DoneCtx::All))]),
        // ---- events: defence, kills, combat flow -----------------------------------------------
        rule("block", r"(?:when|whenever|after|upon|each time|every time) you (?:successfully )?block(?:ed)?(?: an? (?:attack|hit)s?)?|(?:on|after|upon) (?:successfully )?block(?:ing)?|(?:successfully )?block(?:ing)? an? (?:attack|hit)s?|blocked an? attacks?", |_| vec![Q::On(E::Block)]),
        rule("roll_dodge", r"(?:when|whenever|after|upon|each time|every time) (?:you |the wearer )?(?:successfully )?(?:use |perform |using |performing )?(?:a |the )?(?:roll dodge|dodge.?roll(?:ing|ed|s)?|dodg(?:e|es|ing|ed)|roll(?:ing|s)? dodge)", |_| vec![Q::On(E::RollDodge)]),
        rule("kill", r"(?:when|whenever|after|upon|each time|every time|any time|if) (?:you |your allies |an ally |the wearer )?(?:kill|defeat|slay)(?:s|ing)? (?:an? |the |any )?(?:enem(?:y|ies)|targets?|players?|monsters?)|(?:kill|defeat|slay)ing (?:an? |the |any )?(?:enem(?:y|ies)|targets?|players?|monsters?)|on kill|(?:when|after|if) (?:an? |the )?(?:marked |affected )?(?:enem(?:y|ies)|target)(?: you (?:have )?(?:recently )?damaged)? dies", |_| vec![Q::On(E::Kill)]),
        rule("die", r"(?:when|if|upon|after) (?:you|the wearer) (?:die|are killed|are defeated)|(?:on|upon) death|if the wearer dies|when the wearer dies", |_| vec![Q::On(E::Die)]),
        rule("heal", r"(?:when|whenever|after|upon|each time|every time) you (?:successfully )?heal(?: yourself| an ally| allies| a target| another)?(?: or (?:yourself|an ally|allies))?|(?:when|whenever|after) (?:you are|an ally is|allies are) healed", |_| vec![Q::On(E::Heal)]),
        rule("interrupt", r"(?:when|whenever|after|upon|each time|every time) you (?:successfully )?interrupt(?:s|ing)?(?: an? | the )?(?:enem(?:y|ies)|target|spell|cast)?|on interrupt", |_| vec![Q::On(E::Interrupt)]),
        // ---- events: casting, synergies, potions -------------------------------------------------
        rule("channel_state_cast", r"(?:cast(?:s|ing)?|us(?:e|es|ing)|activat(?:e|es|ing)) (?:an? |any )?(?:\w+ )?abilit(?:y|ies) with an? (?:cast|channel)(?: or channel)?(?: time)?", |_| vec![Q::On(E::Cast(Cast::CastOrChannelTime))]),
        rule("cast_with_cost", r"(?:cast(?:s|ing)?|us(?:e|es|ing)|activat(?:e|es|ing)) (?:an? |any )?(?:\w+ )?abilit(?:y|ies) (?:with|that has|which has) an? (?:magicka |stamina |health )?cost", |_| vec![Q::On(E::Cast(Cast::WithCost))]),
        rule("cast_ultimate", r"(?:cast(?:s|ing)?|us(?:e|es|ing)|activat(?:e|es|ing)|fir(?:e|es|ing)) (?:an? |your |any |the )?ultimate(?: abilit(?:y|ies))?", |_| vec![Q::On(E::Cast(Cast::Ultimate))]),
        rule("cast_non_ultimate", r"(?:cast(?:s|ing)?|us(?:e|es|ing)|activat(?:e|es|ing)) (?:an? |any |your )?non-ultimate(?: abilit(?:y|ies))?|non-ultimate abilit(?:y|ies) (?:cast|used)", |_| vec![Q::On(E::Cast(Cast::NonUltimate))]),
        rule("synergy_offered", r"(?:may|can|could) (?:also )?(?:activate|use) (?:the |an? )?(?:# )?synerg(?:y|ies)(?: abilit(?:y|ies))?|(?:grant|give|creat)(?:s|es|e)? (?:the |an? )?(?:# )?synerg(?:y|ies)", |_| vec![Q::OffersSynergy]),
        rule("activate_synergy", r"(?:(?:when|whenever|after|upon|each time|every time|if) (?:you )?(?:activate|use)s?|activat(?:ing|ion of)|us(?:ing)) (?:an? |the |your |any )?(?:# )?synerg(?:y|ies)(?: abilit(?:y|ies))?", |_| vec![Q::On(E::ActivateSynergy)]),
        rule("cast_any", r"(?:(?:when|whenever|after|upon|each time|every time|if|while) (?:you |the wearer )?(?:cast|use|activate)s?|(?:cast|us|activat)ing) (?:an? |any |your |each |every )?(?:abilit(?:y|ies)|skills?|spells?)", |_| vec![Q::On(E::Cast(Cast::Any))]),
        rule("cast_skill_line", r"(?:(?:when|whenever|after|upon|each time|every time|if|while) (?:you |the wearer )?(?:cast|use|activate)s?|(?:cast|us|activat)ing) (?:an? |your |any )?(?P<line>(?:[a-z']+ ){1,3}?)(?:skills?|abilit(?:y|ies)|spells?)", |c| {
            let line = cap(c, "line").unwrap_or("").trim();
            if NOT_A_LINE.contains(&line) { Vec::new() } else { vec![Q::On(E::Cast(Cast::SkillLine))] }
        }),
        rule("drink_potion", r"(?:drink(?:s|ing)?|drank|us(?:e|es|ing|ed)|consum(?:e|es|ing)|quaff(?:s|ing)?) (?:an? |your |any |the )?(?:# )?(?:(?:health|magicka|stamina|restoration|tri-restoration) )?potions?|potions? (?:is |are )?(?:used|consumed|drunk)", |_| vec![Q::On(E::DrinkPotion)]),
        rule("activate_self", r"(?:upon|on|after|when) (?:its |this |your )?(?:activation|activating|being (?:used|activated|cast)|use|cast|casting)|(?:when|once) (?:it is |this is )?(?:activated|triggered)|when cast", |_| vec![Q::On(E::Activate)]),
        rule("effect_ends", r"(?:when|after|once|as) (?:the |this |your )?(?:effect|buff|duration|ability|rune)? ?(?:ends|expires|fades|wears off)", |_| vec![Q::On(E::EffectEnds)]),
        // ---- scaling: "per ..." ---------------------------------------------------------------------
        rule("max_stacks", r"(?:with )?(?:a )?max(?:imum)? stacks? of (?P<n>\d+|#)|up to (?P<n2>\d+|#) stacks|(?P<n3>\d+) stacks? max", |c| vec![Q::MaxStacks(number(cap(c, "n").or(cap(c, "n2")).or(cap(c, "n3"))))]),
        rule("per_armour_piece", r"(?:per|for each|each|for every) (?:piece|item) of (?P<w>light|medium|heavy) armou?r(?: (?:equipped|worn|you (?:are wearing|have equipped)))?|(?:per|for each|each) (?P<w2>light|medium|heavy) armou?r (?:piece|item)s?(?: equipped| worn)?", |c| vec![Q::Per(Count::ArmourPiece(weight_of(cap(c, "w").or(cap(c, "w2")).unwrap_or("heavy"))))]),
        rule("per_armour_type", r"(?:per|for each) type of armou?r|per armou?r type", |_| vec![Q::Per(Count::ArmourType)]),
        rule("per_stack", r"(?:per|for each|each|every|for every) (?:active |additional |current )?stacks?", |_| vec![Q::Per(Count::Stack)]),
        rule("per_crux", r"(?:per|for each|each) (?:active )?crux(?: spent| consumed)?|each active crux|crux spent", |_| vec![Q::Per(Count::Crux)]),
        rule("per_points", r"per (?:\d+|#) points? (?:spent|invested)|for every (?:\d+|#) points? (?:spent|invested)|per point (?:spent|invested)", |_| vec![Q::Per(Count::PointsSpent)]),
        rule("per_stage", r"per (?:stage|rank)", |_| vec![Q::Per(Count::Stage)]),
        rule("per_level", r"per (?:character |champion )?level|for each (?:character )?level", |_| vec![Q::Per(Count::Level)]),
        rule("per_missing_health", r"(?:based on |scaling off |scales off |scales with |depending on |per |for each |on )?(?:your |their |the target's |target's )?missing health", |_| vec![Q::Per(Count::MissingHealth)]),
        rule("per_slotted_ability", r"(?:per|for each|for every) (?:[a-z']+ ){1,3}?abilit(?:y|ies) slotted", |_| vec![Q::Per(Count::SlottedAbility)]),
        rule("per_target", r"(?:per|for each|each) (?:enemy|target|foe)(?: hit| struck| damaged| affected)?", |_| vec![Q::Per(Count::Target)]),
        rule("per_tick", r"(?:per|each|on each|with each|for each) (?:pulse|tick|swipe)", |_| vec![Q::Per(Count::Tick)]),
        rule("per_second", r"(?:per|each) second", |_| vec![Q::Per(Count::Second)]),
        // ---- timing ----------------------------------------------------------------------------
        rule("cooldown", r"once (?:every|per)(?: (?:#|\d+(?:\.\d+)?|half a|half|a|one|an?))?(?: (?:seconds?|secs?|s|minutes?|hours?|weeks?|days?|hit|cast))?|(?:internal )?cooldown of (?:#|\d+)", |_| vec![Q::Cooldown]),
        rule("interval", r"every (?:\d+(?:\.\d+)?|half a|half|one|other)(?: (?:seconds?|secs?|s|ticks?))?|every (?:second|tick)", |_| vec![Q::Interval]),
        // the duration is a placeholder: "every #", "every # for #"
        rule("interval_placeholder", r"every #", |_| vec![Q::Interval]),
        // ---- who -----------------------------------------------------------------------------------
        rule("against_monsters", r"(?:against|vs|versus|done to) (?:dungeon,? ?trial,? and arena |dungeon and trial )?(?:monsters?|bosses)", |_| vec![Q::Against(Foe::Monsters)]),
        rule("against_players", r"(?:against|vs|versus|done to|taken from) (?:enemy |other |hostile )?players?", |_| vec![Q::Against(Foe::Players)]),
    ]
});

/// One recognised phrase.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CueMatch {
    pub cue: Cue,
    /// The lexicon rule that matched.
    pub rule: &'static str,
    /// The words that matched (lower-case, placeholders as `#`). Where a cue names something the
    /// enum cannot (a skill line, a buff, a debuff), this is where to look it up.
    pub text: String,
}

const CLASSES: &[(&str, PlayerClass)] = &[
    ("dragonknight", PlayerClass::Dragonknight),
    ("sorcerer", PlayerClass::Sorcerer),
    ("nightblade", PlayerClass::Nightblade),
    ("warden", PlayerClass::Warden),
    ("necromancer", PlayerClass::Necromancer),
    ("templar", PlayerClass::Templar),
    ("arcanist", PlayerClass::Arcanist),
];

/// The class a phrase names ("a templar skill").
fn class_named(text: &str) -> Option<PlayerClass> {
    CLASSES.iter().find(|(n, _)| text.contains(n)).map(|(_, c)| *c)
}

/// The skill line a phrase names ("a mages guild ability"): the longest line name it contains.
/// Lines are matched by the name the game shows, lower-cased.
fn skill_line_named(text: &str) -> Option<SkillLine> {
    (0u32..4096)
        .filter_map(|id| SkillLine::from_id(&id))
        .map(|l| (l.as_str().to_lowercase(), l))
        .filter(|(name, _)| !name.is_empty() && text.contains(name.as_str()))
        .max_by_key(|(name, _)| name.len())
        .map(|(_, l)| l)
}

impl CueMatch {
    /// The cue's [`TriggerEvent`], with the text consulted for the cast of a named class or skill
    /// line. A cast whose line the text does not name stays `Cast(Other)`.
    pub fn event(&self) -> Option<TriggerEvent> {
        let e = self.cue.to_event()?;
        if self.cue != Cue::On(Event::Cast(Cast::SkillLine)) {
            return Some(e);
        }
        Some(TriggerEvent::Cast(
            class_named(&self.text)
                .map(CastKind::Class)
                .or_else(|| skill_line_named(&self.text).map(CastKind::SkillLine))
                .unwrap_or(CastKind::Other),
        ))
    }

    /// The cue's [`Condition`], with the text consulted where the cue alone is not enough: the
    /// crowd-control states are about the target when the words say so ("enemies are snared") and
    /// about you when they say "you"; with neither the subject is unknown and the answer is `None`.
    pub fn condition(&self) -> Option<Condition> {
        if let Some(c) = self.cue.to_condition() {
            return Some(c);
        }
        let Cue::While(state) = self.cue else { return None };
        let a = match state {
            State::Immobilized => Affliction::Immobilized,
            State::Stunned => Affliction::Stunned,
            State::Snared => Affliction::Snared,
            State::Feared => Affliction::Feared,
            State::Disoriented => Affliction::Disoriented,
            State::OffBalance => Affliction::OffBalance,
            State::Silenced => Affliction::Silenced,
            _ => return None,
        };
        let words: Vec<&str> = self.text.split_whitespace().collect();
        let about = |ws: &[&str]| words.iter().any(|w| ws.contains(w));
        if about(&["target", "they", "it", "enemies", "enemy"]) || self.text.contains("the target") {
            Some(Condition::TargetAfflicted(a))
        } else if about(&["you", "you're"]) {
            Some(Condition::SelfAfflicted(a))
        } else {
            None
        }
    }
}

/// The records `bundle.jsonl` carries for one clause: its cues, the condition they make together
/// and the events among them.
pub fn records(clause: &str) -> (Vec<CueRecord>, Condition, Vec<TriggerEvent>) {
    let mut cues = Vec::new();
    let mut found = Vec::new();
    for c in clauses(clause) {
        found.extend(cues_in_clause(&c));
    }
    for m in &found {
        cues.push(CueRecord {
            cue: format!("{:?}", m.cue),
            rule: m.rule.to_string(),
            text: m.text.clone(),
            event: m.event(),
            condition: m.condition(),
        });
    }
    let events = found.iter().filter_map(CueMatch::event).collect();
    (cues, condition_of(&found), events)
}

/// The condition that all the `While`/`Against` cues of a clause make together: `Always` when
/// none can be expressed.
pub fn condition_of(matches: &[CueMatch]) -> Condition {
    Condition::all(matches.iter().filter_map(CueMatch::condition))
}

/// Cues of one already-normalised clause (see [`clauses`]), in the order they appear in it.
pub fn cues_in_clause(clause: &str) -> Vec<CueMatch> {
    let mut rest = clause.to_string();
    let mut out: Vec<(usize, CueMatch)> = Vec::new();
    for r in LEXICON.iter() {
        // A phrase can occur more than once ("light and heavy ... heavy attack"); each is blanked
        // so the generic rules further down never see it again.
        while let Some((built, range)) =
            r.re.captures(&rest).map(|c| ((r.build)(&c), c.get(0).unwrap().range()))
        {
            let text = rest[range.clone()].to_string();
            for cue in built {
                if !out.iter().any(|(_, m)| m.cue == cue && m.rule == r.name) {
                    out.push((range.start, CueMatch { cue, rule: r.name, text: text.clone() }));
                }
            }
            let blank = " ".repeat(range.len());
            rest.replace_range(range, &blank);
        }
    }
    out.sort_by_key(|(start, _)| *start); // stable: cues of one phrase keep their order
    out.into_iter().map(|(_, m)| m).collect()
}

/// Cues of a whole tooltip: every clause in turn, in text order, without duplicates.
pub fn cues_in(text: &str) -> Vec<CueMatch> {
    let mut out: Vec<CueMatch> = Vec::new();
    for clause in clauses(text) {
        for m in cues_in_clause(&clause) {
            if !out.iter().any(|o| o.cue == m.cue) {
                out.push(m);
            }
        }
    }
    out
}

/// The cues as one comment-friendly line (`On(DrinkPotion), While(InCombat)`), empty when none.
pub fn summary(cues: &[CueMatch]) -> String {
    cues.iter().map(|m| format!("{:?}", m.cue)).collect::<Vec<_>>().join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// `(tooltip text, the cues it must give)`. The cue is written as `Debug` prints it.
    const CASES: &[(&str, &[&str])] = &[
        // --- the phrases the roadmap names -------------------------------------------------
        ("grants you # ultimate every 1.5 seconds while in combat", &["Interval", "While(InCombat)"]),
        ("activating a synergy restores 2% of your max health, stamina, and magicka", &["On(ActivateSynergy)"]),
        ("when you activate a synergy ability while in combat, you generate 2 ultimate", &["On(ActivateSynergy)", "While(InCombat)"]),
        ("casting an ability grants you minor courage", &["On(Cast(Any))"]),
        ("when you cast an ability with a cost, causing a burst of magic around you", &["On(Cast(WithCost))"]),
        ("when you drink a potion, you restore # health, magicka, and stamina", &["On(DrinkPotion)"]),
        ("after drinking a potion you gain # ultimate", &["On(DrinkPotion)"]),
        ("when using potions, resulting effects last 10% longer", &["On(DrinkPotion)"]),
        ("when you deal critical damage, gain a stack", &["On(DealCritical)"]),
        ("dealing critical damage heals you for #", &["On(DealCritical)"]),
        ("when you take critical damage, reduce it", &["On(TakeCritical)"]),
        // --- combat events -----------------------------------------------------------------
        ("when you take damage, you gain # ultimate", &["On(TakeDamage)"]),
        ("whenever you kill an enemy, restore stamina", &["On(Kill)"]),
        ("when an enemy you recently damaged dies, you restore # stamina", &["On(Kill)"]),
        ("when you block, resist additional # damage", &["On(Block)"]),
        ("after successfully blocking an attack the cost of your next roll dodge is reduced", &["On(Block)"]),
        ("grants you major expedition for # after you use roll dodge", &["On(RollDodge)"]),
        ("after dodge-rolling you take less damage from area effects", &["On(RollDodge)"]),
        ("whenever you successfully dodge, increase your critical damage", &["On(RollDodge)"]),
        ("when you use an ultimate ability while in combat, you and the closest 5 group members gain major slayer", &["On(Cast(Ultimate))", "While(InCombat)"]),
        ("while slotted on either bar, casting a non-ultimate ability has a 33% chance", &["While(SlottedOnEitherBar)", "On(Cast(NonUltimate))"]),
        ("casting a templar skill while in combat generates # ultimate", &["On(Cast(SkillLine))", "While(InCombat)"]),
        ("when you are using an ability with a channel or cast time, you take # less damage", &["While(Channeling)"]),
        ("when you deal damage with a light or heavy attack, gain a stack", &["On(DealDamage(LightAttack))", "On(DealDamage(HeavyAttack))"]),
        ("after completing a fully-charged heavy attack, gain # for #", &["On(FullyChargedHeavy)"]),
        ("whenever you bash a target, they are stunned for #", &["On(Bash)"]),
        ("when you deal direct damage with a blink, restore magicka", &["On(DealDamage(Direct))"]),
        ("when you deal damage, you heal for # health", &["On(DealDamage(All))"]),
        ("on death if you are in combat", &["On(Die)", "While(InCombat)"]),
        ("when you leave stealth, gain major expedition", &["On(LeaveStealth)"]),
        ("upon activation you gain #", &["On(Activate)"]),
        ("when the effect ends, heal nearby allies", &["On(EffectEnds)"]),
        // --- states ------------------------------------------------------------------------
        ("while out of combat, recover # magicka", &["While(OutOfCombat)"]),
        ("while blocking, reduce damage taken", &["While(Blocking)"]),
        ("this bonus doubles to # weapon damage when you are under 75% health", &["While(ResourceBelow(Health, Some(75)))"]),
        ("deals up to #% more damage to enemies with less than #% health", &["While(TargetHealthBelow(None))"]),
        ("deals #% more damage to enemies below 25% health", &["While(TargetHealthBelow(Some(25)))"]),
        ("adds # critical chance while you have 50% or more stamina", &["While(ResourceAbove(Stamina, Some(50)))"]),
        ("at full health the glyphic grants #, reducing damage taken by #", &["While(ResourceAbove(Health, Some(100)))"]),
        ("increases your physical resistance by # while wearing 5 or more pieces of heavy armor", &["While(WearingArmour(Heavy, Some(5)))"]),
        ("while in werewolf form, can devour humanoid corpses", &["While(WerewolfForm)"]),
        ("you cannot move while immobilized", &["While(Immobilized)"]),
        ("you are unable to attack or move while stunned", &["While(Stunned)"]),
        ("increases your damage done by # while you have a damage shield active", &["While(SelfBuff)"]),
        ("increase max health by # while you have an armor active ability slotted", &["While(SkillLineSlotted)"]),
        ("while toggled: defend #% damage", &["While(Active)"]),
        ("# increased weapon damage when dual wielding while in cyrodiil and imperial city", &["While(InPvpArea)"]),
        ("deals more damage on an enemy with primal fury", &["While(TargetDebuff)"]),
        ("while on the front bar, gain #", &["While(OnFrontBar)"]),
        ("while sneaking, your detection radius is reduced", &["While(Stealthed)"]),
        // --- scaling, timing, targets ---------------------------------------------------------
        ("reduces the magicka cost of your abilities by # for each piece of light armor equipped", &["Per(ArmourPiece(Light))"]),
        ("increases your max health by # per stack with a maximum stack of 5", &["Per(Stack)", "MaxStacks(Some(5))"]),
        ("each active crux reduces the cost of this ability by #", &["Per(Crux)"]),
        ("reduce the cost of block by 20 stamina per 10 points spent", &["Per(PointsSpent)"]),
        ("up to once every #", &["Cooldown"]),
        ("deals # damage every 2 seconds", &["Interval"]),
        ("increases damage done to dungeon, trial, and arena monsters by #", &["Against(Monsters)"]),
        ("deals # more damage against players", &["Against(Players)"]),
        ("while your health is above 50%", &["While(ResourceAbove(Health, Some(50)))"]),
        ("while your health is 50% or less", &["While(ResourceBelow(Health, Some(50)))"]),
        ("when you heal yourself or an ally with a restoration staff", &["On(Heal)"]),
        ("for each support ability slotted, gain #", &["Per(SlottedAbility)"]),
        ("deals # to enemies every #, dealing # more", &["Interval"]),
        ("an ally may activate the # synergy, damaging and immobilizing enemies in the area", &["OffersSynergy"]),
        ("allies in the area can activate the # synergy, healing for # health", &["OffersSynergy"]),
    ];

    fn found(text: &str) -> Vec<String> {
        cues_in(text).iter().map(|m| format!("{:?}", m.cue)).collect()
    }

    #[test]
    fn every_case_gives_exactly_the_listed_cues() {
        for (text, want) in CASES {
            let mut got = found(text);
            let mut want: Vec<String> = want.iter().map(|s| s.to_string()).collect();
            got.sort();
            want.sort();
            assert_eq!(got, want, "{text}");
        }
    }

    #[test]
    fn text_that_names_no_circumstance_gives_no_cues() {
        for text in [
            "Increases your Max Health by <<1>>",
            "Deals <<1>> flame damage to enemies in the area",
            "Use Roll Dodge to remove immobilization",
            "This stun cannot be blocked",
            "reduces the cost of roll dodge by #",
            "increases your critical damage done by #",
            "Summon an immobile gloom wraith for 60 seconds",
        ] {
            assert!(found(text).is_empty(), "{text}: {:?}", found(text));
        }
    }

    #[test]
    fn out_of_combat_is_not_also_in_combat() {
        assert_eq!(found("only usable while out of combat"), ["While(OutOfCombat)"]);
    }

    #[test]
    fn take_critical_is_not_also_deal_critical_or_take_damage() {
        assert_eq!(found("when you take critical damage"), ["On(TakeCritical)"]);
    }

    #[test]
    fn a_synergy_offered_is_not_a_synergy_activated() {
        assert_eq!(found("you or an ally can activate the # synergy"), ["OffersSynergy"]);
        assert_eq!(found("when you activate the # synergy"), ["On(ActivateSynergy)"]);
    }

    #[test]
    fn cast_skill_line_ignores_filler_words() {
        assert!(found("when you use this ability").is_empty());
        assert_eq!(found("when you cast a support ability"), ["On(Cast(SkillLine))"]);
        let m = cues_in("casting an animal companions ability while in combat");
        assert!(m.iter().any(|m| m.rule == "cast_skill_line" && m.text.contains("animal companions")), "{m:?}");
    }

    #[test]
    fn placeholders_and_markup_do_not_break_a_phrase() {
        assert_eq!(found("|cffffffWhen|r you drink a <<1>> potion"), ["On(DrinkPotion)"]);
        assert_eq!(found("when you deal critical damage"), found("When you deal |c00ff00Critical Damage|r"));
    }

    #[test]
    fn clauses_split_where_the_channel_classifier_splits() {
        assert_eq!(clauses("While toggled:\n· Defend <<1>>% damage"), ["while toggled", "defend #% damage"]);
    }

    #[test]
    fn only_some_cues_are_conditions_today() {
        let c = |t: &str| cues_in(t).first().and_then(|m| m.cue.to_condition());
        assert_eq!(c("while in combat"), Some(Condition::InCombat));
        assert_eq!(c("while out of combat"), Some(Condition::Not(Box::new(Condition::InCombat))));
        assert_eq!(c("when you are under 75% health"), Some(Condition::SelfHealthBelowPct(75)));
        assert_eq!(c("enemies below 25% health"), Some(Condition::TargetHealthBelowPct(25)));
        assert_eq!(
            c("while wearing 5 or more pieces of heavy armor"),
            Some(Condition::ArmourPieces { weight: ArmourWeight::Heavy, at_least: 5 })
        );
        assert_eq!(
            c("when you are under 50% magicka"),
            Some(Condition::SelfResourceBelowPct { resource: Res::Magicka, pct: 50 })
        );
        assert_eq!(c("while blocking"), Some(Condition::SelfState(SelfState::Blocking)));
        assert_eq!(c("while in werewolf form"), Some(Condition::InForm(Form::Werewolf)));
        assert_eq!(c("against monsters"), Some(Condition::TargetIs(TargetKind::Monster)));
        // a placeholder threshold and an event: nothing to evaluate
        assert_eq!(c("enemies below #% health"), None);
        assert_eq!(c("when you drink a potion"), None);
        assert!(Cue::On(Event::DrinkPotion).is_event());
    }

    #[test]
    fn every_rule_name_is_unique_and_no_pattern_matches_nothing() {
        let mut names: Vec<&str> = LEXICON.iter().map(|r| r.name).collect();
        let total = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), total, "duplicate rule names");
        // an empty match would loop forever in `cues_in_clause`
        for r in LEXICON.iter() {
            assert!(!r.re.is_match(""), "rule `{}` matches the empty string", r.name);
        }
    }

    #[test]
    fn events_carry_over_and_a_named_line_or_class_is_resolved_from_the_text() {
        let ev = |t: &str| cues_in(t).iter().find_map(CueMatch::event);
        assert_eq!(ev("when you drink a potion"), Some(TriggerEvent::DrinkPotion));
        assert_eq!(ev("casting an ability grants minor courage"), Some(TriggerEvent::Cast(CastKind::Any)));
        assert_eq!(ev("while in combat"), None);
        assert_eq!(
            ev("when you cast a templar skill"),
            Some(TriggerEvent::Cast(CastKind::Class(PlayerClass::Templar)))
        );
        assert_eq!(
            ev("whenever you cast a mages guild ability"),
            Some(TriggerEvent::Cast(CastKind::SkillLine(SkillLine::MagesGuild)))
        );
    }

    #[test]
    fn crowd_control_conditions_take_their_subject_from_the_words() {
        let cond = |t: &str| condition_of(&cues_in_clause(t));
        assert_eq!(cond("while you are stunned"), Condition::SelfAfflicted(Affliction::Stunned));
        assert_eq!(cond("while the target is snared"), Condition::TargetAfflicted(Affliction::Snared));
        assert_eq!(cond("when enemies are immobilized"), Condition::TargetAfflicted(Affliction::Immobilized));
        assert_eq!(
            cond("while in combat against monsters"),
            Condition::All(vec![Condition::InCombat, Condition::TargetIs(TargetKind::Monster)])
        );
        assert_eq!(cond("when you drink a potion"), Condition::Always);
    }
}
