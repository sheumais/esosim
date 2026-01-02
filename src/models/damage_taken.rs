use crate::models::{ARMOUR_MAXIMUM, EFFECTIVE_LEVEL};

pub struct Resistance {
    additive: u32,
    multiplicative: f32,
}

impl Resistance {
    /// Assumes effective level of 66 (player)
    pub fn calculate(&self) -> f32 {
        self.calculate_with_level(EFFECTIVE_LEVEL)
    }

    pub fn calculate_with_level(&self, level: u8) -> f32 {
        let armour_cap = level as u32 * 500;
        ((self.additive as f32 / armour_cap as f32) * (1.0 + self.multiplicative)).min(ARMOUR_MAXIMUM)
    }

    pub fn add_to_additive(&mut self, value: u32) {
        self.additive += value;
    }

    pub fn add_to_multiplicative(&mut self, value: f32) {
        self.multiplicative += value;
    }
    
    pub fn new() -> Self {
        Self {
            additive: 0,
            multiplicative: 0.0
        }
    }
}