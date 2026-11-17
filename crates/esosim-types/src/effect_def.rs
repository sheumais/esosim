//! The owned, data-driven description of "something that changes your stats"
//! (roadmap 1.5, Appendix A.1). Replaces the old `&'static Effect`.

use crate::{
    Channel,
    trigger::Trigger,
    condition::{Condition, ConditionContext, HealthOf, HealthPart},
};
use serde::{Deserialize, Serialize};

/// Where an effect comes from. Drives UI grouping ("why do I have +2629 crit?") and the
/// per-source breakdown in the calculator.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectSource {
    Set { set_id: u16, pieces_required: u8 },
    ChampionPoint { cp_id: u32 },
    Mundus { ability_id: u32 },
    /// A racial passive (top rank). `name` is the race that has it, `ability_id` the passive: every effect written for
    /// the one passive names the same `ability_id`, so a passive with two numbers (two effects) is one entry in the
    /// build. Like every passive it counts only when `Build::passives` lists `ability_id`.
    Race { name: String, ability_id: u32 },
    /// A class passive (top rank): `name` is the class, `ability_id` and the unlocking as for `Race`.
    Class { name: String, ability_id: u32 },
    /// Any other passive of a skill line: armour weight, weapon, world and guild lines (Mages Guild, Fighters Guild,
    /// Undaunted ...). Nothing is on because of gear or class alone: `Build::to_player` switches it on only when
    /// `Build::passives` lists `ability_id`, because unlocking it is a choice of the build. Every effect written for
    /// the one passive names the same `ability_id`, so a passive with two numbers (two effects) is one entry in the
    /// build; its `condition` (armour pieces, weapon) then decides whether it counts.
    Passive { ability_id: u32 },
    ActiveSkill { ability_id: u32 },
    /// Major/Minor, group buffs, synergies.
    Buff { ability_id: u32 },
    /// Applied TO the enemy.
    Debuff { ability_id: u32 },
    Food,
    Potion,
    /// The pieces themselves: armour value and weapon power. Traits and enchants are their own sources.
    Gear,
    GearTrait,
    Enchant,
    /// Block, sprint, roll dodge…
    CoreCombat,
}

/// How repeated applications combine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum Stacking {
    /// The same id from several sources counts once (Major/Minor X).
    #[default]
    Unique,
    /// Value is `value + value_per_stack * n`, with `n` capped at `max`.
    PerStack { max: u32 },
    /// Every source counts: value is `value * n` (e.g. one entry per gear piece).
    Additive,
    /// The value grows with a Health percentage: `value` is the amount for each `step_pct` points of the Health
    /// (the stacks are how many whole steps there are, up to `100 / step_pct`). "Up to 700 Health Recovery based on
    /// your missing Health" is 7 a step at `step_pct: 1`; "1 % for every 10 % current Health they have" is 0.01 a
    /// step at `step_pct: 10` on `Target`'s current Health. Unlike every other stacking the count is not something a
    /// build holds: it comes from the fight (see [`Stacking::health_steps`]), so a calculator that has no Health
    /// leaves the effect out and lists it as conditional.
    PerHealthPct { of: HealthOf, part: HealthPart, step_pct: u8 },
}

impl Stacking {
    /// The most whole steps a health gradient has (`100 / step_pct`; a step of 0 is read as 1).
    pub fn max_health_steps(step_pct: u8) -> u32 {
        100 / u32::from(step_pct.max(1))
    }

    /// For [`Stacking::PerHealthPct`], the number of whole steps `ctx` puts the Health at: `missing / step` or
    /// `current / step`. `None` for any other stacking, or when the context does not know that Health.
    pub fn health_steps(&self, ctx: &dyn ConditionContext) -> Option<u32> {
        let Stacking::PerHealthPct { of, part, step_pct } = *self else { return None };
        let pct = u32::from(ctx.health_pct_of(of)?.min(100));
        let amount = match part {
            HealthPart::Missing => 100 - pct,
            HealthPart::Current => pct,
        };
        Some(amount / u32::from(step_pct.max(1)))
    }

    /// Whether the number of stacks is read from the fight and not held by the build.
    pub fn is_health_gradient(&self) -> bool {
        matches!(self, Stacking::PerHealthPct { .. })
    }

    /// For a health gradient, what it scales with in words ("per 10% of the enemy's current Health"), for the
    /// calculator's list of conditional effects.
    pub fn gradient_text(&self) -> Option<String> {
        let Stacking::PerHealthPct { of, part, step_pct } = *self else { return None };
        let whose = match of {
            HealthOf::Own => "your",
            HealthOf::Target => "the enemy's",
            HealthOf::HealTarget => "the healed one's",
        };
        let part = match part {
            HealthPart::Missing => "missing",
            HealthPart::Current => "current",
        };
        Some(format!("per {step_pct}% of {whose} {part} Health"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ChannelValue {
    pub channel: Channel,
    pub value: f64,
    #[serde(default)]
    pub value_per_stack: f64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct EffectDef {
    /// The id of the ability record the effect comes from. There are no synthetic ids.
    pub id: u32,
    pub name: String,
    pub source: EffectSource,
    pub channels: Vec<ChannelValue>,
    #[serde(default)]
    pub condition: Condition,
    #[serde(default)]
    pub stacking: Stacking,
    /// The game's Major/Minor buff code (`major_minor_id`): different abilities that give the same
    /// buff share it (Minor Force `61746` and its set copy `193448`), and the buff counts once however
    /// many of them are active. `None` for everything that is not such a buff.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buff_code: Option<u8>,
    /// `None` = permanent while the condition holds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u32>,
    /// For procs; only the simulator uses it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cooldown_ms: Option<u32>,
    /// What fires the effect, for a proc ("when you deal critical damage, 33% chance, once every 5 seconds"). An
    /// effect with a trigger is not on because of the build: no sheet counts it, and the calculator lists it as
    /// conditional with the trigger in words. Only a simulator, which sees the events, switches it on (for
    /// `duration_ms`, at most once every `cooldown_ms` when the trigger has none of its own). `None` = not a proc.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<Trigger>,
    /// Research notes carried over from the hand-written tables.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl EffectDef {
    /// Number of stacks that actually count: `Unique` is always 1, `PerStack` is capped.
    pub fn effective_stacks(&self, stacks: u32) -> u32 {
        match self.stacking {
            Stacking::Unique => stacks.min(1),
            Stacking::PerStack { max } => stacks.min(max),
            Stacking::Additive => stacks,
            Stacking::PerHealthPct { step_pct, .. } => stacks.min(Stacking::max_health_steps(step_pct)),
        }
    }

    /// Value of `channel` at `stacks` stacks (0.0 if the effect has no such channel or
    /// `stacks` is 0).
    pub fn value_at(&self, channel: Channel, stacks: u32) -> f64 {
        self.channels
            .iter()
            .filter(|cv| cv.channel == channel)
            .map(|cv| self.channel_value_at(cv, stacks))
            .sum()
    }

    /// Value of one entry at `stacks` stacks.
    pub fn channel_value_at(&self, cv: &ChannelValue, stacks: u32) -> f64 {
        let n = self.effective_stacks(stacks);
        if n == 0 {
            return 0.0;
        }
        match self.stacking {
            Stacking::Unique => cv.value,
            Stacking::PerStack { .. } => cv.value + cv.value_per_stack * n as f64,
            Stacking::Additive | Stacking::PerHealthPct { .. } => cv.value * n as f64,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ResistableDamageType;

    fn def(stacking: Stacking, value: f64, per: f64) -> EffectDef {
        EffectDef {
            id: 1,
            name: "t".into(),
            source: EffectSource::Food,
            channels: vec![ChannelValue { channel: Channel::CriticalChance, value, value_per_stack: per }],
            condition: Condition::Always,
            stacking,
            buff_code: None,
            duration_ms: None,
            cooldown_ms: None,
            trigger: None,
            note: None,
        }
    }

    #[test]
    fn unique_ignores_extra_sources() {
        let d = def(Stacking::Unique, 100.0, 0.0);
        assert_eq!(d.value_at(Channel::CriticalChance, 1), 100.0);
        assert_eq!(d.value_at(Channel::CriticalChance, 3), 100.0);
        assert_eq!(d.value_at(Channel::CriticalChance, 0), 0.0);
    }

    #[test]
    fn per_stack_scales_and_caps() {
        let d = def(Stacking::PerStack { max: 10 }, 0.0, 34.62);
        assert!((d.value_at(Channel::CriticalChance, 10) - 346.2).abs() < 1e-9);
        assert!((d.value_at(Channel::CriticalChance, 99) - 346.2).abs() < 1e-9);
        assert_eq!(d.value_at(Channel::Armour(ResistableDamageType::All), 10), 0.0);
    }

    #[test]
    fn additive_counts_every_source() {
        let d = def(Stacking::Additive, 50.0, 0.0);
        assert_eq!(d.value_at(Channel::CriticalChance, 3), 150.0);
    }
}
