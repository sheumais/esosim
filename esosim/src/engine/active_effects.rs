use std::collections::HashMap;

use super::aggregator::ChannelSet;
use super::effect::AggKind;
use super::registry::EffectRegistry;

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

    pub fn has(&self, id: u32) -> bool {
        self.stacks.contains_key(&id)
    }

    pub fn stacks_of(&self, id: u32) -> u32 {
        self.stacks.get(&id).copied().unwrap_or(0)
    }

    pub fn ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.stacks.keys().copied()
    }

    pub fn aggregate(&self, registry: &EffectRegistry) -> ChannelSet {
        let mut out = ChannelSet::new();
        for (&id, &stacks) in &self.stacks {
            let Some(fx) = registry.get(id) else { continue };
            let value = fx.value + fx.value_per_stack * stacks as f64;
            for &ch in fx.channels {
                match ch.agg_kind() {
                    AggKind::Additive => out.add_additive(ch, value.round() as i64),
                    AggKind::Multiplicative => {
                        out.add_multiplicative_bps(ch, (value * 10_000.0).round() as i64)
                    }
                }
            }
        }
        out
    }
}