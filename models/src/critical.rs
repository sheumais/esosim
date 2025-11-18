use crate::{Percent, CRITICAL_DAMAGE_DEFAULT, CRITICAL_DAMAGE_MAXIMUM, EFFECTIVE_LEVEL};

pub struct CriticalDamage {
    additive: Percent,
    multiplicative: Percent
}

impl CriticalDamage {
    pub fn calculate(&self) -> f32 {
        let base = 
            self.additive.to_f32()
            + CRITICAL_DAMAGE_DEFAULT;
        
        base * (1.0 + self.multiplicative.to_f32()).min(CRITICAL_DAMAGE_MAXIMUM)
    }
}

impl Default for CriticalDamage {
    fn default() -> Self {
        Self {
            additive: Percent::default(),
            multiplicative: Percent::default() 
        }
    }
}

pub struct CriticalChance {
    additive: u16,
    multiplicative: Percent,
}

impl CriticalChance {
    pub fn calculate(&self) -> f32 {       
        self.calculate_with_level(EFFECTIVE_LEVEL)
    }

    pub fn calculate_with_level(&self, level: u8) -> f32 {
        let level_const = 2 * level as u32 * (100 + level as u32);
        ((self.additive as f32 / level_const as f32) + 0.1 + self.multiplicative.to_f32()).min(1.0)
    }
}

impl Default for CriticalChance {
    fn default() -> Self {
        Self {
            additive: 0,
            multiplicative: Percent::default() 
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
        crit_chance.multiplicative += Percent::from_f32(0.072);
        assert_eq!(crit_chance.calculate() * 100.0, 48.4249);
    }

    #[test]
    fn test_calculate_crit_damage() {
        let mut crit_damage = CriticalDamage::default();
        crit_damage.additive
        +=Percent::from_u8(10) // animal companions (advanced species)
        + Percent::from_u8(2) // 1x medium armour
        + Percent::from_u8(12) // khajiit passive
        + Percent::from_u8(10); // velothi amulet
        assert_eq!(crit_damage.calculate() * 100.0, 84.0);
    }
}
