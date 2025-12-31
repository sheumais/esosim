use crate::ARMOUR_MAXIMUM;

#[derive(Default)]
pub struct Armour {
    additive: u32,
}

impl Armour {
    pub fn calculate(&self) -> u32 {
        self.additive
    }

    pub fn add_to_additive(&mut self, value: u32) {
        self.additive += value;
    }

    pub fn reset(&mut self) {
        self.additive = 0;
    }

    pub fn new() -> Self {
        Self {
            additive: 0,
        }
    }
}