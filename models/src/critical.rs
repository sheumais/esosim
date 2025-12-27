use crate::{CRITICAL_CHANCE_DEFAULT, CRITICAL_DAMAGE_DEFAULT, CRITICAL_DAMAGE_MAXIMUM, EFFECTIVE_LEVEL};

pub struct CriticalDamage {
    additive: u16,
    multiplicative: f32
}

impl CriticalDamage {
    pub fn calculate(&self) -> u8 {
        let base =
            self.additive
            + CRITICAL_DAMAGE_DEFAULT as u16;

        CRITICAL_DAMAGE_MAXIMUM.min((base as f32 * (1.0 + self.multiplicative)) as u8)
    }

    pub fn calculate_uncapped(&self) -> u16 {
        let base =
            self.additive
            + CRITICAL_DAMAGE_DEFAULT as u16;

        (base as f32 * (1.0 + self.multiplicative)) as u16
    }

    pub fn add_to_additive(&mut self, value: u16) {
        self.additive += value;
    }

    pub fn add_to_multiplicative(&mut self, value: f32) {
        self.multiplicative += value;
    }

    pub fn reset(&mut self) {
        self.additive = 0;
        self.multiplicative = 0.0;
    }
}

impl Default for CriticalDamage {
    fn default() -> Self {
        Self {
            additive: 0,
            multiplicative: 0.0
        }
    }
}

pub struct CriticalChance {
    additive: u32,
    multiplicative: f32,
}

impl CriticalChance {
    pub fn calculate(&self) -> f32 {       
        self.calculate_with_level(EFFECTIVE_LEVEL)
    }

    pub fn calculate_with_level(&self, level: u8) -> f32 {
        let level_const = 2 * level as u32 * (100 + level as u32);
        ((self.additive as f32 / level_const as f32) + CRITICAL_CHANCE_DEFAULT + self.multiplicative).min(1.0)
    }

    pub fn add_to_additive(&mut self, value: u32) {
        self.additive += value;
    }

    pub fn add_to_multiplicative(&mut self, value: f32) {
        self.multiplicative += value;
    }
}

impl Default for CriticalChance {
    fn default() -> Self {
        Self {
            additive: 0,
            multiplicative: 0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_defaults() {
        let crit_dmg = CriticalDamage::default();
        assert_eq!(crit_dmg.calculate(), crate::CRITICAL_DAMAGE_DEFAULT);
        let crit_chance = CriticalChance::default();
        assert_eq!(crit_chance.calculate(), crate::CRITICAL_CHANCE_DEFAULT);
    }

    #[test]
    fn test_calculate_crit_chance() {
        let mut crit_chance = CriticalChance::default();
        crit_chance.additive 
        +=2629 // major savagery
        + 1922 // thief mundus
        + 219 * 6 // light armour
        + 320 // precision cp
        + 657; // slimecraw 1pc
        crit_chance.multiplicative += 0.072;
        assert_eq!(crit_chance.calculate() * 100.0, 48.4249);
    }

    #[test]
    fn test_calculate_crit_damage() {
        let mut crit_damage = CriticalDamage::default();
        crit_damage.additive
        += 10 // animal companions (advanced species)
        + 2 // 1x medium armour
        + 12 // khajiit passive
        + 10; // velothi amulet/minor force
        assert_eq!(crit_damage.calculate(), 84u8);
    }
}
