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
    fn a_plain_trigger_has_no_chance_or_cooldown() {
        let t = Trigger::on(TriggerEvent::Block);
        assert_eq!((t.chance_pct, t.cooldown_ms), (None, None));
    }
}
