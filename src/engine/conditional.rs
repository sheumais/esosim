use super::effect::Channel;
use super::registry::EffectRegistry;
use crate::entity::enemy::Enemy;

#[derive(Debug, Clone, Copy)]
pub enum TargetCondition {
    HasStatus(u32), // Off Balance, Stagger, etc. — by ability id
    HealthBelowPercent(u8),
    DebuffCount(u8), // Reave-style: scales with # of debuffs on target
    Always,
}

impl TargetCondition {
    pub fn evaluate(&self, target: &Enemy, registry: &EffectRegistry) -> bool {
        match *self {
            TargetCondition::HasStatus(id) => target.active_effects.has(id),
            TargetCondition::HealthBelowPercent(pct) => target.health_percent() < pct as f32,
            TargetCondition::DebuffCount(min) => target.debuff_count(registry) >= min as u32,
            TargetCondition::Always => true,
        }
    }
}

pub struct ConditionalEffect {
    pub channel: Channel,
    pub value: f64,
    pub condition: TargetCondition,
}

pub static CONCUSSION: ConditionalEffect = ConditionalEffect {
    channel: Channel::Power,
    value: 0.10,
    condition: TargetCondition::HasStatus(0 /* OFF_BALANCE_ID, see data/effects */),
};

pub fn resolve_offensive_channel(
    base: &super::aggregator::ChannelSet,
    conditionals: &[ConditionalEffect],
    channel: Channel,
    target: &Enemy,
    registry: &EffectRegistry,
) -> i64 {
    let mut total = base.get(channel);
    for c in conditionals.iter().filter(|c| c.channel == channel) {
        if c.condition.evaluate(target, registry) {
            total += (c.value * base.get(channel) as f64).round() as i64;
        }
    }
    total
}