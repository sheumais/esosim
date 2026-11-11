//! The owned, data-driven description of "something that changes your stats"
//! (roadmap 1.5, Appendix A.1). Replaces the old `&'static Effect`.

use crate::{Channel, condition::Condition};
use serde::{Deserialize, Serialize};

/// Where an effect comes from. Drives UI grouping ("why do I have +2629 crit?") and the
/// per-source breakdown in the calculator.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EffectSource {
    Set { set_id: u16, pieces_required: u8 },
    ChampionPoint { cp_id: u32 },
    Mundus { ability_id: u32 },
    Race { name: String },
    Class { name: String },
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
    /// Ability id when it has one, otherwise a synthetic id (>= 0x8000_0000).
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
            Stacking::Additive => cv.value * n as f64,
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
