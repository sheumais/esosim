//! ################################################################################################################
//! # READ THIS BEFORE YOU ADD, CHANGE OR "APPROXIMATE" ANY TRIGGER (agents: this means you)
//! ################################################################################################################
//!
//! **A trigger must be EXACTLY what the game's own text says. Never an approximation.**
//!
//! The owner of this project would rather have an ERROR, a failing `verify`, a `todo!()`, an effect left out of the
//! bundle or a loud "not modelled" than a trigger that is nearly right. A nearly-right trigger gives a wrong number
//! that looks fine, and someone has to hunt it down later. That costs far more than a missing feature.
//!
//! Therefore:
//!
//! 1. **Never widen a trigger to make it fit.** If the tooltip says "martial melee damage" and no [`TriggerEvent`]
//!    says exactly that, you may NOT use `DealDamage(All)` and write "an overestimate of uptime" in a note. That is the
//!    mistake this rule exists to stop. Add the missing precision to [`TriggerEvent`] (as [`TriggerEvent::DealMeleeDamage`]
//!    was added for exactly this), or do not write the effect.
//! 2. **Never narrow one either.** "Every time you hit a target with a weapon attack" is not "any damage" and is not
//!    "a light attack"; it is what it says. Read the game's text for the record (`tooltip_text` in the bundle), not a
//!    summary of it, and not an earlier agent's note about it. If the text and a note disagree, the text wins and the
//!    disagreement goes to the owner.
//! 3. **Every part of the text is part of the trigger.** The event ("when you deal ..."), the chance ("has a 33 % chance"),
//!    the internal cooldown ("can occur once every <<3>>"), who is hit, and what duration or stacks it gives. If one of
//!    them cannot be written, the effect is not finished: leave it out and say why.
//! 4. **If you are not sure, stop and fail.** Do not pick the closest variant "for now", do not add a TODO that still
//!    compiles and runs, do not add a fallback that makes an unknown case behave like [`DamageDoneContext::All`].
//!    Return an error, panic with a message that names the effect and the missing piece, or ask the owner.
//! 5. **A variant must be decidable.** Do not add an event the simulator cannot tell apart from its neighbours. The
//!    simulator must say "I cannot tell" (see `esosim-sim`'s `EventError`) and fail the run, never guess `true`.
//! 6. **Say what you could not check.** If you add or change a trigger, name the game text you matched it against in
//!    the commit or the final message. "I assumed" is not a source.
//!
//! These rules apply equally to `overrides.toml`, `cp_effects.toml`, the trigger lexicon (`triggers.rs`) and anything that
//! writes a [`Trigger`] or a [`TriggerEvent`].
//!
//! ################################################################################################################
//!
//! Things that *happen* in combat: the events a proc waits for (roadmap 6.4, Appendix A.2).
//!
//! [`Condition`](crate::Condition) says what is *true*; [`TriggerEvent`] says what *occurs*. The
//! simulator raises events; a [`Trigger`] says which event fires an effect, how often and with what
//! chance. Plain data (`Copy`, `Eq`, `Hash`, serde), like the rest of this crate.
//!
//! The game labels its own trigger kinds (`list26[].u2[0]`, `SkillData34::list26_trigger_label` in
//! `eso_skill_data`). [`TriggerEvent::from_list26_kind`] turns the ones that are combat events into
//! [`TriggerEvent`]s; the others (resource pool, ability range, fishing, wayshrine cost ...) are
//! passive modifiers, not events, and give `None`.

use crate::channel::DamageDoneContext;
use crate::condition::PlayerClass;
use crate::damage::ResistableDamageType;
use eso_skill_data::enums::skill_line::SkillLine;
use serde::{Deserialize, Serialize};
use std::fmt;

/// What kind of ability is being cast ("casting an ability", "casting a templar skill").
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CastKind {
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
    /// "casting a mages guild ability", "using a siphoning ability"
    SkillLine(#[serde(with = "crate::skill_serde::skill_line")] SkillLine),
    /// "casting a templar skill"
    Class(PlayerClass),
    /// "casting a support ability" with a kind no variant names (game kind 39:
    /// "when you cast a specific kind of ability"); the tooltip text says which
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TriggerEvent {
    /// "dealing damage", "when you deal direct damage", "damage with a light attack". The context is
    /// the one the damage-done channels use, so `LightAttack`, `HeavyAttack`, `Direct`,
    /// `DamageOverTime`, `AreaOfEffect` and `Bash` carry over; `All` is any damage.
    DealDamage(DamageDoneContext),
    /// "when you deal Martial melee damage", "when you deal melee damage": damage dealt **in melee** whose damage type
    /// is the given type or group. `Martial` is Physical, Poison, Disease and Bleed; `Spell` the magical types; `All`
    /// any damage type. It is not a [`DamageDoneContext`]: nothing that scales damage cares whether a hit was melee,
    /// so it is an event only, and the channels keep their contexts as they were.
    ///
    /// Both parts must hold. Ranged damage of a martial type (a bow, a thrown or projectile attack) does not fire it, and
    /// melee damage of a magical type does not fire `DealMeleeDamage(Martial)`. A simulator that cannot tell whether a
    /// hit was melee must fail, not assume yes (rule 5 at the top of this file).
    DealMeleeDamage(ResistableDamageType),
    /// "dealing critical damage", "when you critically strike" (game kind 49)
    DealCritical,
    /// "when you take damage"
    TakeDamage,
    /// "when you take critical damage" (game kind 50)
    TakeCritical,
    /// "activating a critical heal"
    CriticalHeal,
    /// "after completing a fully-charged heavy attack"
    FullyChargedHeavy,
    /// "when you block", "on block", "after blocking" (game kind 18)
    Block,
    /// "when your attack is blocked" (game kind 14)
    AttackBlocked,
    /// "whenever you bash a target" (game kind 78)
    Bash,
    /// "when you roll dodge", "after dodge-rolling" (game kind 15)
    RollDodge,
    Cast(CastKind),
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
    /// "when you heal yourself or an ally" (game kind 9)
    Heal,
    EnterCombat,
    LeaveCombat,
    /// "when you leave stealth", "when coming out of stealth"
    LeaveStealth,
    /// the ability itself: "upon activation", "on use", "when cast", "when triggered" (game kind 1)
    Activate,
    /// "when the effect ends"
    EffectEnds,
    /// "every second in combat", "every 1.5 seconds while in combat" (game kind 86)
    EverySecondInCombat,
    /// "applying Malady gives you a stack of Contagion": an [`Application`](crate::Application) of this effect landed.
    /// Raised by whoever carries the application out, and only when it does land: an application held back by its own
    /// proc's cooldown raises nothing, so the effect that waits for it is held back too. Not a game kind: the game
    /// states these chains in the tooltip text only.
    EffectApplied(u32),
}

impl TriggerEvent {
    /// The event for the game's trigger kind (`list26[].u2[0]`). Kind 0 is the always-on passive
    /// marker and kinds 4, 7, 8, 76, 88, 92, 93, 96 and 98 modify a pool, range or cost rather than
    /// react to something, so all of those give `None`.
    pub fn from_list26_kind(kind: u32) -> Option<TriggerEvent> {
        Some(match kind {
            1 => TriggerEvent::Activate,
            9 => TriggerEvent::Heal,
            14 => TriggerEvent::AttackBlocked,
            15 => TriggerEvent::RollDodge,
            18 => TriggerEvent::Block,
            39 => TriggerEvent::Cast(CastKind::Other),
            49 => TriggerEvent::DealCritical,
            50 => TriggerEvent::TakeCritical,
            78 => TriggerEvent::Bash,
            86 => TriggerEvent::EverySecondInCombat,
            _ => return None,
        })
    }

    /// The game kinds that mean this event (the inverse of [`from_list26_kind`](Self::from_list26_kind)).
    pub fn list26_kinds(&self) -> &'static [u32] {
        match self {
            TriggerEvent::Activate => &[1],
            TriggerEvent::Heal => &[9],
            TriggerEvent::AttackBlocked => &[14],
            TriggerEvent::RollDodge => &[15],
            TriggerEvent::Block => &[18],
            TriggerEvent::Cast(CastKind::Other) => &[39],
            TriggerEvent::DealCritical => &[49],
            TriggerEvent::TakeCritical => &[50],
            TriggerEvent::Bash => &[78],
            TriggerEvent::EverySecondInCombat => &[86],
            _ => &[],
        }
    }
}

impl fmt::Display for TriggerEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TriggerEvent::DealDamage(DamageDoneContext::All) => f.write_str("you deal damage"),
            TriggerEvent::DealDamage(ctx) => write!(f, "you deal damage ({ctx:?})"),
            TriggerEvent::DealMeleeDamage(ResistableDamageType::All) => f.write_str("you deal melee damage"),
            TriggerEvent::DealMeleeDamage(ResistableDamageType::Martial) => f.write_str("you deal martial melee damage"),
            TriggerEvent::DealMeleeDamage(ResistableDamageType::Spell) => f.write_str("you deal spell melee damage"),
            TriggerEvent::DealMeleeDamage(kind) => write!(f, "you deal {} melee damage", format!("{kind:?}").to_lowercase()),
            TriggerEvent::DealCritical => f.write_str("you deal critical damage"),
            TriggerEvent::TakeDamage => f.write_str("you take damage"),
            TriggerEvent::TakeCritical => f.write_str("you take critical damage"),
            TriggerEvent::CriticalHeal => f.write_str("you land a critical heal"),
            TriggerEvent::FullyChargedHeavy => f.write_str("you complete a fully-charged heavy attack"),
            TriggerEvent::Block => f.write_str("you block"),
            TriggerEvent::AttackBlocked => f.write_str("your attack is blocked"),
            TriggerEvent::Bash => f.write_str("you bash"),
            TriggerEvent::RollDodge => f.write_str("you roll dodge"),
            TriggerEvent::Cast(CastKind::Any) => f.write_str("you cast an ability"),
            TriggerEvent::Cast(CastKind::Ultimate) => f.write_str("you cast an ultimate"),
            TriggerEvent::Cast(CastKind::NonUltimate) => f.write_str("you cast a non-ultimate ability"),
            TriggerEvent::Cast(CastKind::WithCost) => f.write_str("you cast an ability with a cost"),
            TriggerEvent::Cast(CastKind::CastOrChannelTime) => f.write_str("you cast an ability with a cast or channel time"),
            TriggerEvent::Cast(CastKind::SkillLine(line)) => write!(f, "you cast a {line} ability"),
            TriggerEvent::Cast(CastKind::Class(class)) => write!(f, "you cast a {class:?} skill"),
            TriggerEvent::Cast(CastKind::Other) => f.write_str("you cast a particular kind of ability"),
            TriggerEvent::ActivateSynergy => f.write_str("you activate a synergy"),
            TriggerEvent::DrinkPotion => f.write_str("you drink a potion"),
            TriggerEvent::Kill => f.write_str("you kill an enemy"),
            TriggerEvent::Die => f.write_str("you die"),
            TriggerEvent::Interrupt => f.write_str("you interrupt an enemy"),
            TriggerEvent::Heal => f.write_str("you heal"),
            TriggerEvent::EnterCombat => f.write_str("you enter combat"),
            TriggerEvent::LeaveCombat => f.write_str("you leave combat"),
            TriggerEvent::LeaveStealth => f.write_str("you leave stealth"),
            TriggerEvent::Activate => f.write_str("the ability is activated"),
            TriggerEvent::EffectEnds => f.write_str("the effect ends"),
            TriggerEvent::EverySecondInCombat => f.write_str("every second in combat"),
            TriggerEvent::EffectApplied(id) => write!(f, "you apply effect {id}"),
        }
    }
}

/// How an event fires an effect (the proc side of an `EffectDef`, roadmap 6.4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Trigger {
    pub event: TriggerEvent,
    /// "has a 33% chance"; `None` = always.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub chance_pct: Option<u8>,
    /// "once every 5 seconds": the internal cooldown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooldown_ms: Option<u32>,
}

impl Trigger {
    pub fn on(event: TriggerEvent) -> Trigger {
        Trigger { event, chance_pct: None, cooldown_ms: None }
    }
}

impl fmt::Display for Trigger {
    /// "you deal critical damage, 33% chance, at most once every 5 seconds"
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.event)?;
        if let Some(p) = self.chance_pct {
            write!(f, ", {p}% chance")?;
        }
        if let Some(ms) = self.cooldown_ms {
            if ms % 1000 == 0 {
                write!(f, ", at most once every {} second{}", ms / 1000, if ms == 1000 { "" } else { "s" })?;
            } else {
                write!(f, ", at most once every {:.1} seconds", ms as f64 / 1000.0)?;
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn game_kinds_map_to_events_and_back() {
        for kind in 0u32..128 {
            if let Some(e) = TriggerEvent::from_list26_kind(kind) {
                assert_eq!(e.list26_kinds(), &[kind], "{e:?}");
            }
        }
        assert_eq!(TriggerEvent::from_list26_kind(49), Some(TriggerEvent::DealCritical));
        assert_eq!(TriggerEvent::from_list26_kind(86), Some(TriggerEvent::EverySecondInCombat));
        assert_eq!(TriggerEvent::from_list26_kind(39), Some(TriggerEvent::Cast(CastKind::Other)));
    }

    #[test]
    fn melee_damage_is_its_own_event_and_is_not_any_damage() {
        let martial = TriggerEvent::DealMeleeDamage(ResistableDamageType::Martial);
        assert_ne!(martial, TriggerEvent::DealDamage(DamageDoneContext::All));
        assert_ne!(martial, TriggerEvent::DealMeleeDamage(ResistableDamageType::Spell));
        assert_eq!(martial.to_string(), "you deal martial melee damage");
        assert_eq!(TriggerEvent::DealMeleeDamage(ResistableDamageType::All).to_string(), "you deal melee damage");
        assert_eq!(TriggerEvent::DealMeleeDamage(ResistableDamageType::Bleed).to_string(), "you deal bleed melee damage");
        // not a game kind: nothing in list26 says "melee"
        assert_eq!(martial.list26_kinds(), &[] as &[u32]);
        // the form `overrides.toml` writes: `trigger = { event = { DealMeleeDamage = "Martial" } }`
        let json = serde_json::to_string(&martial).unwrap();
        assert_eq!(json, r#"{"DealMeleeDamage":"Martial"}"#);
        assert_eq!(serde_json::from_str::<TriggerEvent>(&json).unwrap(), martial);
    }

    #[test]
    fn passive_modifier_kinds_are_not_events() {
        // always-on marker, resource pool, food duration, ultimate pool, range, wayshrine, fishing
        for kind in [0, 4, 7, 8, 76, 88, 92, 93, 96, 98, 1000] {
            assert_eq!(TriggerEvent::from_list26_kind(kind), None, "{kind}");
        }
    }

    #[test]
    fn events_read_as_english() {
        assert_eq!(TriggerEvent::DrinkPotion.to_string(), "you drink a potion");
        assert_eq!(TriggerEvent::DealDamage(DamageDoneContext::All).to_string(), "you deal damage");
        assert_eq!(
            TriggerEvent::Cast(CastKind::SkillLine(SkillLine::MagesGuild)).to_string(),
            "you cast a Mages Guild ability"
        );
    }

    #[test]
    fn applying_an_effect_is_an_event_with_the_applied_id() {
        let e: TriggerEvent = serde_json::from_str(r#"{"EffectApplied":202586}"#).unwrap();
        assert_eq!(e, TriggerEvent::EffectApplied(202586));
        assert_eq!(e.to_string(), "you apply effect 202586");
        assert!(e.list26_kinds().is_empty(), "not one of the game's kinds");
    }

    #[test]
    fn a_plain_trigger_has_no_chance_or_cooldown() {
        let t = Trigger::on(TriggerEvent::Block);
        assert_eq!((t.chance_pct, t.cooldown_ms), (None, None));
    }
}