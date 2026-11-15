use std::collections::HashMap;

use super::effect::Channel;

#[derive(Default, Clone, Copy, Debug)]
pub struct Aggregator {
    additive: i64,
    /// What the whole numbers in `additive` left out: the sum of `value - value.trunc()` over every value added
    /// with [`Aggregator::add_additive_exact`]. Always 0 for a channel that rounds (`Channel::keeps_fraction`).
    additive_fraction: f64,
    /// Basis points (1 = 0.01%)
    multiplicative_bps: i64,
}

impl Aggregator {
    pub fn add_additive(&mut self, v: i64) {
        self.additive += v;
    }

    /// Add `v` keeping its fraction: the whole part (towards zero) goes into `additive`, the rest is remembered
    /// for [`Aggregator::additive_exact`].
    pub fn add_additive_exact(&mut self, v: f64) {
        let whole = v.trunc();
        self.additive += whole as i64;
        self.additive_fraction += v - whole;
    }

    pub fn add_multiplicative_bps(&mut self, v: i64) {
        self.multiplicative_bps += v;
    }

    pub fn additive(&self) -> i64 {
        self.additive
    }

    /// The additive total with the fractions of the exact values in it.
    pub fn additive_exact(&self) -> f64 {
        self.additive as f64 + self.additive_fraction
    }

    pub fn multiplicative_bps(&self) -> i64 {
        self.multiplicative_bps
    }
}

pub trait IntoAdditive { fn into_additive(self) -> i64; }
impl IntoAdditive for i64 { fn into_additive(self) -> i64 { self } }
impl IntoAdditive for i32 { fn into_additive(self) -> i64 { self as i64 } }
impl IntoAdditive for u32 { fn into_additive(self) -> i64 { self as i64 } }
impl IntoAdditive for f32 { fn into_additive(self) -> i64 { self.round() as i64 } }
impl IntoAdditive for f64 { fn into_additive(self) -> i64 { self.round() as i64 } }


#[derive(Default, Clone, Debug)]
pub struct ChannelSet(HashMap<Channel, Aggregator>);

impl ChannelSet {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn add_additive(&mut self, channel: Channel, value: impl IntoAdditive) {
        self.0.entry(channel).or_default().add_additive(value.into_additive());
    }

    /// Add a value that has a fractional part. A channel that keeps fractions (`Channel::keeps_fraction`) adds the
    /// whole part to [`ChannelSet::get`] and remembers the rest for [`ChannelSet::get_exact`]; every other channel
    /// rounds to the nearest whole number, exactly as `add_additive` does with an `f64`.
    pub fn add_additive_f64(&mut self, channel: Channel, value: f64) {
        let agg = self.0.entry(channel).or_default();
        if channel.keeps_fraction() {
            agg.add_additive_exact(value);
        } else {
            agg.add_additive(value.round() as i64);
        }
    }

    pub fn add_multiplicative_bps(&mut self, channel: Channel, value_bps: impl IntoAdditive) {
        self.0
            .entry(channel)
            .or_default()
            .add_multiplicative_bps(value_bps.into_additive());
    }

    pub fn get(&self, channel: Channel) -> i64 {
        self.0.get(&channel).map(|a| a.additive()).unwrap_or(0)
    }

    /// The additive total of `channel` with the fractions of its sources in it. Equal to `get` as a float for a
    /// channel that rounds its sources.
    pub fn get_exact(&self, channel: Channel) -> f64 {
        self.0.get(&channel).map(|a| a.additive_exact()).unwrap_or(0.0)
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
            entry.additive_fraction += agg.additive_fraction;
            entry.add_multiplicative_bps(agg.multiplicative_bps());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use esosim_types::ResistableDamageType;

    #[test]
    fn a_channel_that_keeps_fractions_keeps_them_across_sources_and_merges() {
        let mut a = ChannelSet::new();
        a.add_additive_f64(Channel::CriticalChance, 657.36);
        a.add_additive_f64(Channel::CriticalChance, 2629.44);
        let mut b = ChannelSet::new();
        b.add_additive_f64(Channel::CriticalChance, 438.24);
        b.add_additive_f64(Channel::CriticalChance, -1314.72);
        a.merge(&b);
        // the integer view is the sum of the whole parts, the exact view has every fraction
        assert_eq!(a.get(Channel::CriticalChance), 657 + 2629 + 438 - 1314);
        assert!((a.get_exact(Channel::CriticalChance) - (657.36 + 2629.44 + 438.24 - 1314.72)).abs() < 1e-9);
    }

    #[test]
    fn every_other_channel_still_rounds_each_source() {
        let armour = Channel::Armour(ResistableDamageType::All);
        let mut c = ChannelSet::new();
        c.add_additive_f64(armour, 10.5);
        c.add_additive_f64(armour, 10.4);
        assert_eq!((c.get(armour), c.get_exact(armour)), (21, 21.0));
        assert_eq!(c.get_exact(Channel::CriticalDamage), 0.0);
    }
}