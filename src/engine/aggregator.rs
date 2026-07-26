use std::collections::HashMap;

use super::effect::Channel;

#[derive(Default, Clone, Copy, Debug)]
pub struct Aggregator {
    additive: i64,
    /// Basis points (1 = 0.01%)
    multiplicative_bps: i64,
}

impl Aggregator {
    pub fn add_additive(&mut self, v: i64) {
        self.additive += v;
    }

    pub fn add_multiplicative_bps(&mut self, v: i64) {
        self.multiplicative_bps += v;
    }

    pub fn additive(&self) -> i64 {
        self.additive
    }

    pub fn multiplicative_bps(&self) -> i64 {
        self.multiplicative_bps
    }
}

#[derive(Default, Clone, Debug)]
pub struct ChannelSet(HashMap<Channel, Aggregator>);

impl ChannelSet {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_additive(&mut self, channel: Channel, value: i64) {
        self.0.entry(channel).or_default().add_additive(value);
    }

    pub fn add_multiplicative_bps(&mut self, channel: Channel, value_bps: i64) {
        self.0
            .entry(channel)
            .or_default()
            .add_multiplicative_bps(value_bps);
    }

    pub fn get(&self, channel: Channel) -> i64 {
        self.0.get(&channel).map(|a| a.additive()).unwrap_or(0)
    }

    pub fn get_multiplicative_bps(&self, channel: Channel) -> i64 {
        self.0
            .get(&channel)
            .map(|a| a.multiplicative_bps())
            .unwrap_or(0)
    }

    pub fn merge(&mut self, other: &ChannelSet) {
        for (&channel, agg) in other.0.iter() {
            let entry = self.0.entry(channel).or_default();
            entry.add_additive(agg.additive());
            entry.add_multiplicative_bps(agg.multiplicative_bps());
        }
    }
}