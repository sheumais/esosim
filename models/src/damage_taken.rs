use crate::{Percent, ARMOUR_MAXIMUM, EFFECTIVE_LEVEL};

pub struct Resistance {
    additive: u16,
    multiplicative: Percent,
}

impl Resistance {
    /// Assumes effective level of 66 (player)
    pub fn calculate(&self) -> f32 {
        self.calculate_with_level(EFFECTIVE_LEVEL)
    }

    pub fn calculate_with_level(&self, level: u8) -> f32 {
        let armour_cap = level as u16 * 500;
        ((self.additive as f32 / armour_cap as f32) * (1.0 + self.multiplicative.to_f32())).min(ARMOUR_MAXIMUM)
    }
    
    pub fn new() -> Self {
        Self {
            additive: 0,
            multiplicative: Percent::new()
        }
    }
}