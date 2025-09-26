use crate::{Percent, LEVEL};

pub struct Power {
    item: u16,
    set: u16,
    skill2: u16,
    mundus: u16,
    cp: u16,
    skill: Percent,
    buff: Percent,
    bloodthirsty: u16,
}

impl Power {
    fn calculate(&self) -> u16 {
        let base = 
            LEVEL as u16 * 20
            + self.item
            + self.set
            + self.skill2 
            + self.mundus
            + self.cp;
        
        (base as f32 * (1.0 + self.skill.to_f32() + self.buff.to_f32())) as u16 + self.bloodthirsty
    }

    fn new() -> Self {
        Self {
            item: 0,
            set: 0,
            skill2: 0,
            mundus: 0,
            cp: 0,
            skill: Percent::U8(0),
            buff: Percent::U8(0),
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
}