pub const CRIT_DAMAGE_SCALE: u16 = 66;
const CRITICAL_DAMAGE_DEFAULT: u16 = 50;
const CRITICAL_DAMAGE_MAXIMUM_DEFAULT: u16 = 125;
const CRITICAL_CHANCE_DEFAULT: f32 = 0.1;
const EFFECTIVE_LEVEL: u8 = 66;

pub struct CriticalDamage {
    additive_scaled: u16,
}

impl CriticalDamage {
    pub fn calculate(&self) -> u16 {
        self.additive_scaled
            + (CRITICAL_DAMAGE_DEFAULT * CRIT_DAMAGE_SCALE)
    }

    pub fn add_percent(&mut self, percent: u16) {
        self.additive_scaled += percent * CRIT_DAMAGE_SCALE;
    }

    pub fn reset(&mut self) {
        self.additive_scaled = 0;
    }
}

impl Default for CriticalDamage {
    fn default() -> Self {
        Self {
            additive_scaled: 0,
        }
    }
}

pub struct CriticalChance {
    additive: u32,
}

impl CriticalChance {
    pub fn calculate(&self) -> f32 {
        self.calculate_with_level(EFFECTIVE_LEVEL)
    }

    pub fn calculate_with_level(&self, level: u8) -> f32 {
        let level_const = 2 * level as u32 * (100 + level as u32); // 21912
        ((self.additive as f32 / level_const as f32) + CRITICAL_CHANCE_DEFAULT).min(1.0)
    }

    pub fn add_to_additive(&mut self, value: u32) {
        self.additive += value;
    }

    pub fn get_raw(&self) -> u32 {
        self.additive
    }

    pub fn reset(&mut self) {
        self.additive = 0;
    }
}

impl Default for CriticalChance {
    fn default() -> Self {
        Self {
            additive: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_defaults() {
        let crit_dmg = CriticalDamage::default();
        assert_eq!(crit_dmg.calculate(), CRITICAL_DAMAGE_DEFAULT * CRIT_DAMAGE_SCALE);
        let crit_chance = CriticalChance::default();
        assert_eq!(crit_chance.calculate(), CRITICAL_CHANCE_DEFAULT);
    }

    #[test]
    fn test_calculate_crit_chance() {
        let mut crit_chance = CriticalChance::default();
        crit_chance.add_to_additive(
        2629 // major savagery
        + 1922 // thief mundus
        + 219 * 6 // light armour
        + 320 // precision cp
        + 657 // slimecraw 1pc
        + 1579); // precise weapon
        assert!(crit_chance.calculate() - (CRITICAL_CHANCE_DEFAULT + 0.384309959) < 1e-7); // read from /script d(GetCriticalStrikeChance(8421))
    }

    #[test]
    fn test_calculate_crit_damage() {
        let mut crit_damage = CriticalDamage::default();
        crit_damage.add_percent(
        10 // animal companions (advanced species)
        + 2 // 1x medium armour
        + 12 // khajiit passive
        + 10); // velothi amulet/minor force
        assert_eq!(crit_damage.calculate(), 84 * CRIT_DAMAGE_SCALE);
    }
}
