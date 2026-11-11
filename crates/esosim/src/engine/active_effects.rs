use std::collections::{HashMap, HashSet};

use super::aggregator::ChannelSet;
use super::effect::{Bucket, Channel, Condition, ConditionContext, EffectDef, EffectSource, NoContext};
use super::registry::EffectRegistry;

/// Which effects are on, and with how many stacks. What a stack count means depends on the
/// effect's `Stacking`: `Unique` counts once however often it is set, `PerStack` scales up to
/// its cap, `Additive` counts every instance (use [`ActiveEffects::add`] once per source).
#[derive(Default, Clone, Debug)]
pub struct ActiveEffects {
    stacks: HashMap<u32, u32>,
}

impl ActiveEffects {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, id: u32, stacks: u32) {
        if stacks == 0 {
            self.stacks.remove(&id);
        } else {
            self.stacks.insert(id, stacks);
        }
    }

    /// One more instance/stack of `id`.
    pub fn add(&mut self, id: u32) {
        *self.stacks.entry(id).or_insert(0) += 1;
    }

    pub fn has(&self, id: u32) -> bool {
        self.stacks.contains_key(&id)
    }

    pub fn stacks_of(&self, id: u32) -> u32 {
        self.stacks.get(&id).copied().unwrap_or(0)
    }

    pub fn ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.stacks.keys().copied()
    }

    /// Aggregate with no condition context: only effects whose condition is `Always` apply.
    pub fn aggregate(&self, registry: &EffectRegistry) -> ChannelSet {
        self.aggregate_with(registry, &NoContext)
    }

    /// Aggregate, applying an effect only when its condition evaluates to `Some(true)`.
    /// Undecidable (`None`) conditions are left out. Effects with the same `buff_code` count once.
    pub fn aggregate_with(&self, registry: &EffectRegistry, ctx: &dyn ConditionContext) -> ChannelSet {
        self.aggregate_with_boost(registry, ctx, 0)
    }

    /// Like [`aggregate_with`](Self::aggregate_with), for a build whose gear gives a mundus boost
    /// of its own (`extra_mundus_boost_bps`, in basis points: Divines pieces, roadmap 2.3).
    ///
    /// Every effect whose source is a mundus stone is worth `value * (1 + boost)`, where the boost is
    /// `extra_mundus_boost_bps` plus whatever the other active effects put on `Channel::MundusBoost`
    /// (a fraction, so 9.1 % is `0.091`), truncated to a whole number. Boosts add up (seven 9.1 % Divines pieces are +63.7 %), and a
    /// stone never boosts itself. The returned set holds only what effects produce, so the `extra`
    /// boost is not in its `MundusBoost`: whoever supplied it already has it.
    pub fn aggregate_with_boost(
        &self,
        registry: &EffectRegistry,
        ctx: &dyn ConditionContext,
        extra_mundus_boost_bps: i64,
    ) -> ChannelSet {
        let mut out = ChannelSet::new();
        for (_, part) in self.aggregate_parts(registry, ctx, extra_mundus_boost_bps) {
            out.merge(&part);
        }
        out
    }

    /// [`aggregate_with_boost`](Self::aggregate_with_boost), but one `ChannelSet` per effect that
    /// applies, in id order, so a caller can say where each number came from (roadmap 1.3,
    /// `Player::explain`). Merging the parts gives exactly `aggregate_with_boost`.
    pub fn aggregate_parts<'r>(
        &self,
        registry: &'r EffectRegistry,
        ctx: &dyn ConditionContext,
        extra_mundus_boost_bps: i64,
    ) -> Vec<(&'r EffectDef, ChannelSet)> {
        let mut ids: Vec<u32> = self.stacks.keys().copied().collect();
        ids.sort_unstable();
        let mut parts: Vec<(&'r EffectDef, ChannelSet)> = Vec::new();
        let mut stones: Vec<(&'r EffectDef, u32)> = Vec::new();
        let mut boost_bps = extra_mundus_boost_bps;
        // Major/Minor buffs already counted: several abilities give the same buff, which counts once
        let mut buff_codes: HashSet<u8> = HashSet::new();
        for id in ids {
            let stacks = self.stacks[&id];
            let Some(fx) = registry.get(id) else { continue };
            if fx.condition != Condition::Always && fx.condition.evaluate(ctx) != Some(true) {
                continue;
            }
            if let Some(code) = fx.buff_code {
                if !buff_codes.insert(code) {
                    continue; // the lowest id with this code is the one that counts
                }
            }
            if matches!(fx.source, EffectSource::Mundus { .. }) {
                stones.push((fx, stacks));
                continue;
            }
            let mut part = ChannelSet::new();
            add_effect(&mut part, fx, stacks, 1.0, false);
            boost_bps += part.get_multiplicative_bps(Channel::MundusBoost);
            parts.push((fx, part));
        }
        let scale = 1.0 + boost_bps as f64 / 10_000.0;
        for (fx, stacks) in stones {
            let mut part = ChannelSet::new();
            add_effect(&mut part, fx, stacks, scale, true);
            parts.push((fx, part));
        }
        parts
    }
}

/// Add every channel of `fx` at `stacks` stacks, each value times `scale`. A mundus stone's value is
/// truncated to a whole number, as the character sheet does (2023 x 1.727 = 3493.8 shows 3493); every
/// other effect is rounded.
fn add_effect(out: &mut ChannelSet, fx: &EffectDef, stacks: u32, scale: f64, truncate: bool) {
    for cv in &fx.channels {
        let mut value = fx.channel_value_at(cv, stacks) * scale;
        if truncate {
            value = (value + 1e-6).floor(); // the epsilon keeps 1212 * 1.5 style products from landing just under
        }
        match cv.channel.bucket() {
            Bucket::Additive => out.add_additive(cv.channel, value.round() as i64),
            Bucket::Multiplicative => out.add_multiplicative_bps(cv.channel, (value * 10_000.0).round() as i64),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::bundle::Bundle;
    use crate::engine::effect::{
        Bar, Channel, ChannelValue, EffectDef, EffectSource, ResistableDamageType, Stacking,
    };

    const FORTIFIED_ID: u32 = 142035;

    #[test]
    fn fortified_comes_from_the_cp_table_and_ten_stacks_is_34_61_times_ten() {
        let registry = EffectRegistry::from_bundle(Bundle::embedded());
        let mut active = ActiveEffects::new();
        active.set(FORTIFIED_ID, 10);
        let sheet = active.aggregate(&registry);
        let armour = Channel::Armour(ResistableDamageType::All);
        assert_eq!(sheet.get(armour), 346); // 346.1

        let def = registry.get(FORTIFIED_ID).unwrap();
        assert!((def.value_at(armour, 10) - 346.1).abs() < 1e-9);
    }

    #[test]
    fn per_stack_is_capped() {
        let registry = EffectRegistry::embedded();
        let armour = Channel::Armour(ResistableDamageType::All);
        let at = |stacks| {
            let mut active = ActiveEffects::new();
            active.set(FORTIFIED_ID, stacks);
            active.aggregate(&registry).get(armour)
        };
        assert!(at(50) > at(49), "Fortified has 50 stages in the game's table");
        assert_eq!(at(80), at(50));
    }

    fn test_def(id: u32, stacking: Stacking, condition: Condition) -> EffectDef {
        EffectDef {
            id,
            name: format!("test {id}"),
            source: EffectSource::Buff { ability_id: id },
            channels: vec![ChannelValue { channel: Channel::CriticalChance, value: 100.0, value_per_stack: 0.0 }],
            condition,
            stacking,
            buff_code: None,
            duration_ms: None,
            cooldown_ms: None,
            note: None,
        }
    }

    #[test]
    fn unique_effect_applied_by_two_sources_counts_once() {
        let registry = EffectRegistry::from_defs([test_def(1, Stacking::Unique, Condition::Always)]);
        let mut active = ActiveEffects::new();
        active.add(1);
        active.add(1);
        assert_eq!(active.aggregate(&registry).get(Channel::CriticalChance), 100);
    }

    #[test]
    fn two_sources_of_one_major_or_minor_buff_count_once() {
        // Minor Force and its set copy are two abilities with one code (44); Minor Brutality is another buff (code 1)
        let force = |id| EffectDef { buff_code: Some(44), ..test_def(id, Stacking::Unique, Condition::Always) };
        let brutality = EffectDef { buff_code: Some(1), ..test_def(3, Stacking::Unique, Condition::Always) };
        let registry = EffectRegistry::from_defs([force(1), force(2), brutality]);
        let mut active = ActiveEffects::new();
        active.set(1, 1);
        active.set(2, 1);
        assert_eq!(active.aggregate(&registry).get(Channel::CriticalChance), 100);
        active.set(3, 1);
        assert_eq!(active.aggregate(&registry).get(Channel::CriticalChance), 200);
    }

    #[test]
    fn additive_effect_counts_every_source() {
        let registry = EffectRegistry::from_defs([test_def(1, Stacking::Additive, Condition::Always)]);
        let mut active = ActiveEffects::new();
        active.add(1);
        active.add(1);
        assert_eq!(active.aggregate(&registry).get(Channel::CriticalChance), 200);
    }

    #[test]
    fn conditional_effects_need_a_deciding_context() {
        struct FrontBar;
        impl ConditionContext for FrontBar {
            fn on_bar(&self, bar: Bar) -> Option<bool> { Some(bar == Bar::Front) }
        }
        let registry = EffectRegistry::from_defs([test_def(1, Stacking::Unique, Condition::OnBar(Bar::Front))]);
        let mut active = ActiveEffects::new();
        active.set(1, 1);
        assert_eq!(active.aggregate(&registry).get(Channel::CriticalChance), 0);
        assert_eq!(active.aggregate_with(&registry, &FrontBar).get(Channel::CriticalChance), 100);
    }
}
