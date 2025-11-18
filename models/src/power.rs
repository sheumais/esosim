use crate::{Percent, LEVEL};

pub struct Power {
    additive: u16,
    multiplicative: Percent,
    bloodthirsty: u16,
}

impl Power {
    /// Assumes level 50
    pub fn calculate(&self) -> u16 {
        self.calculate_with_level(LEVEL)
    }

    pub fn calculate_with_level(&self, level: u8) -> u16 {
        let base = 
            level as u16 * 20
            + self.additive;
        (base as f32 * (1.0 + self.multiplicative.to_f32())) as u16 + self.bloodthirsty
    }

    pub fn new() -> Self {
        Self {
            additive: 0,
            multiplicative: Percent::new(),
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
        assert_eq!(power.calculate(), LEVEL as u16 * 20);
    }

    #[test]
    fn test_calculate_power_dps_example() {
        let mut power = Power::new();
        power.additive
            += 129 * 3 // armour lines
            + 1535 // main hand weapon
            + (0.06 * 1335.0) as u16 // offhand
            + 174 * 3 // jewellery
            + 205; // wrathful strikes cp
        power.multiplicative
        += Percent::from_f32(
            0.2 // major brutality
            + 0.02 * 6. // medium armour
            + 0.03 // 1x fighters guild skill
        );
        assert_eq!(power.calculate(), 5076); // comes to 5034, fails test
    }
}