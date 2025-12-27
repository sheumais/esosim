use crate::ARMOUR_MAXIMUM;

#[derive(Default)]
pub struct Armour {
    additive: u32,
    multiplicative: f32,
}

impl Armour {
    pub fn calculate(&self) -> u32 {
        (self.additive as f32 * (1.0 + self.multiplicative)).round() as u32
    }

    pub fn add_to_additive(&mut self, value: u32) {
        self.additive += value;
    }

    pub fn add_to_multiplicative(&mut self, value: f32) {
        self.multiplicative += value;
    }

    pub fn reset(&mut self) {
        self.additive = 0;
        self.multiplicative = 0.0;
    }

    pub fn new() -> Self {
        Self {
            additive: 0,
            multiplicative: 0.0,
        }
    }
}