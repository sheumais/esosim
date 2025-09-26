use crate::{Percent, ARMOUR_MAXIMUM, ARMOUR_MAXIMUM_INT};

struct Resistance {
    item: u16,
    skill2: u16,
    mundus: u16,
    set: u16,
    skill: u16,
    cp: u16,
    buff: f32,
}

impl Resistance {
    // PhysicalResist = (Item.PhysicalResist + Skill2.PhysicalResist + Mundus.PhysicalResist + Set.PhysicalResist + Skill.PhysicalResist + CP.PhysicalResist)*(1 + Buff.PhysicalResist)
    fn calculate(&self) -> f32 {
        let base = 
            self.item
            + self.skill2
            + self.mundus
            + self.set
            + self.skill
            + self.cp;

        ((base as f32 / ARMOUR_MAXIMUM_INT as f32) * (1.0 + self.buff)).min(ARMOUR_MAXIMUM)
    }
    
    fn new() -> Self {
        Self {
            item: 0,
            skill2: 0,
            mundus: 0,
            set: 0,
            skill: 0,
            cp: 0,
            buff: 0.0,
        }
    }
}