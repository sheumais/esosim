use crate::{damage_taken::Resistance, LEVEL};

pub struct Monster {
    level: u8,
    effective_level: u8,
    max_health: u32,
    resistance: Resistance,
}
impl Monster {
    pub fn new_trial_dummy() -> Self {
        let mut dummy = Self {
            level: LEVEL,
            effective_level: LEVEL,
            max_health: 21_002_944,
            resistance: Resistance::new(),
        };
        dummy.resistance.add_to_additive(18200);
        dummy
    }
}