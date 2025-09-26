use crate::Percent;

pub struct DamageDoneType {
    cp: Percent,
    skill: Percent,
    buff: Percent,
    item: Percent,
    set: Percent,
}

impl DamageDoneType {
    fn calculate(self) -> f32 {
        (self.cp + self.skill + self.buff + self.item + self.set).to_f32()
    }

    fn new() -> Self {
        Self { 
            cp: Percent::U8(0),
            skill: Percent::U8(0),
            buff: Percent::U8(0), 
            item: Percent::U8(0), 
            set: Percent::U8(0),
        }
    }
}