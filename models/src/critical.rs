use crate::{Percent, CRITICAL_CHANCE_MAXIMUM, CRITICAL_DAMAGE_DEFAULT, CRITICAL_DAMAGE_MAXIMUM};

pub struct CriticalDamage {
    cp_stat_type: Percent,
    skill: Percent,
    cp: Percent,
    mundus: Percent,
    set: Percent,
    item: Percent,
    buff: Percent,
    skill2: Percent
}

impl CriticalDamage {
    fn calculate(&self) -> f32 {
        let base = 
            (self.cp_stat_type
            + self.skill
            + self.cp
            + self.mundus
            + self.set
            + self.item
            + self.buff).to_f32() 
            + CRITICAL_DAMAGE_DEFAULT;
        
        base * (1.0 + self.skill2.to_f32()).min(CRITICAL_DAMAGE_MAXIMUM)
    }

    fn new() -> Self {
        Self {
            cp_stat_type: Percent::U8(0), 
            skill: Percent::U8(0),
            cp: Percent::U8(0),
            mundus: Percent::U8(0),
            set: Percent::U8(0),
            item: Percent::U8(0),
            buff: Percent::U8(0),
            skill2: Percent::U8(0),
        }
    }
}

pub struct CriticalChance {
    set: u16,
    skill2: u16,
    buff: u16,
    cp: u16,
    mundus: u16,
    item: f32,
    skill: f32,
}

impl CriticalChance {
    /// \[0 -> 1\] as a percentage
    fn calculate(&self) -> f32 {
    
    // (Set.SpellCrit + Skill2.SpellCrit + Buff.SpellCrit + CP.SpellCrit + Mundus.SpellCrit)*(1/(2*EffectiveLevel*(100 + EffectiveLevel))) + 0.10 + Item.SpellCrit + Skill.SpellCrit 
        let base =
            self.set
            + self.skill2
            + self.buff
            + self.cp
            + self.mundus;
        
        let level_const = 1.0 / 871200 as f32; // hardcoded for effective level of 66: (2 * 66 * (100 * 66))

        ((base as f32 * level_const) + 0.1 + self.item + self.skill).min(CRITICAL_CHANCE_MAXIMUM)
    }

    fn new() -> Self {
        Self {
            set: 0,
            skill2: 0,
            buff: 0,
            cp: 0,
            mundus: 0,
            item: 0.0,
            skill: 0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_default_critical_damage() {
        let crit_dmg = CriticalDamage::new();
        assert_eq!(crit_dmg.calculate(), crate::CRITICAL_DAMAGE_DEFAULT);
    }

    #[test]
    fn test_calculate_default_critical_chance() {
        let crit_chance = CriticalChance::new();
        assert_eq!(crit_chance.calculate(), crate::CRITICAL_CHANCE_DEFAULT);
    }
}
