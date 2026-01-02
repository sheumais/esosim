use crate::models::LEVEL;

#[derive(Default)]
pub struct Power {
    additive: u32,
    multiplicative: f32,
    bloodthirsty: u32,
}

impl Power {
    /// Assumes level 50
    pub fn calculate(&self) -> u32 {
        self.calculate_with_level(LEVEL)
    }

    pub fn calculate_with_level(&self, level: u8) -> u32 {
        let base =
            level as u32 * 20
            + self.additive;
        (base as f32 * (1.0 + self.multiplicative)).round() as u32 + self.bloodthirsty
    }

    pub fn add_to_additive(&mut self, value: u32) {
        self.additive += value;
    }

    pub fn add_to_multiplicative(&mut self, value: f32) {
        self.multiplicative += value;
    }

    pub fn set_bloodthirsty(&mut self, value: u32) {
        self.bloodthirsty = value;
    }

    pub fn reset(&mut self) {
        self.additive = 0;
        self.multiplicative = 0.0;
        self.bloodthirsty = 0;
    }

    pub fn new() -> Self {
        Self {
            additive: 0,
            multiplicative: 0.0,
            bloodthirsty: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_default_power() {
        let power = Power::new();
        assert_eq!(power.calculate(), LEVEL as u32 * 20);
    }

    #[test]
    fn test_dual_wield() {
        let mut power = Power::new();
        power.additive = ((0.1765 + 0.06) as f32 * 1335.0).round() as u32 + 1535; // offhand and passive plus mainhand
        assert_eq!(power.calculate(), 2851);
    }

    #[test]
    fn test_dual_wield_fleshed_out_example() {
        let mut power = Power::new();
        power.additive = ((0.1765 + 0.06) as f32 * 1335.0).round() as u32 + 1535;
        power.multiplicative = 0.03; // one fighters guild skill
        assert_eq!(power.calculate(), 2937);
        power.multiplicative += 0.12; // 6 medium pieces
        assert_eq!(power.calculate(), 3279);
        power.additive += 174 * 3; // jewellery with weapon damage enchants
        power.additive += 129 * 2; // 2x set lines
        assert_eq!(power.calculate(), 4176);
        power.multiplicative += 0.2; // major brutality
        assert_eq!(power.calculate(), 4902);
        power.additive += 205; // wrathful strikes CP <!> Doesn't show up on character sheet
        assert_eq!(power.calculate(), 5179);
    }
}