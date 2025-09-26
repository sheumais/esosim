pub struct Penetration {
    item: u16,
    set: u32,
    skill: u16,
    cp: u16,
    buff: u16,
    mundus: u16,
}

impl Penetration {
    fn calculate(&self) -> u32 {
        (self.item + self.skill + self.cp + self.buff + self.mundus) as u32 + self.set
    }

    fn new() -> Self {
        Self {
            item: 0,
            set: 0,
            skill: 0,
            cp: 0,
            buff: 0,
            mundus: 0,
        }
    }
}