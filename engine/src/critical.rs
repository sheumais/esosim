use std::collections::HashMap;
use esosim_data::critical_damage::{CRITICAL_DAMAGE_BUFF_BY_ID, CRITICAL_DAMAGE_DEBUFF_BY_ID};
use esosim_models::critical::CriticalDamage as CriticalDamageModel;

use crate::{ID, STACKS};

pub struct CriticalDamage {
    pub sources: HashMap<ID, STACKS>,
    critical_damage: CriticalDamageModel,
}

impl CriticalDamage {
    pub fn add_source(&mut self, id: ID, stacks: Option<STACKS>) {
        self.sources.insert(id, stacks.unwrap_or(1));
    }

    pub fn remove_source(&mut self, id: ID) {
        self.sources.remove(&id);
    }

    pub fn is_valid_source(id: &ID) -> bool {
        CRITICAL_DAMAGE_BUFF_BY_ID.get(id).is_some()
    }

    pub fn calculate(&mut self) -> u8 {
        for (id, stacks) in &self.sources {
            if let Some(buff) = CRITICAL_DAMAGE_BUFF_BY_ID.get(&id) {
                self.critical_damage.add_to_additive((buff.value + buff.value_per_stack * *stacks as i32) as u16);
            }
            // Malacath's add to multiplicative
        }
        self.critical_damage.calculate()
    }
}

pub struct CriticalDamageTaken {
    pub sources: HashMap<ID, STACKS>,
    critical_damage_taken: u8,
}

impl CriticalDamageTaken {
    pub fn add_source(&mut self, id: ID, stacks: Option<STACKS>) {
        self.sources.insert(id, stacks.unwrap_or(1));
    }

    pub fn remove_source(&mut self, id: ID) {
        self.sources.remove(&id);
    }

    pub fn is_valid_source(id: &ID) -> bool {
        CRITICAL_DAMAGE_DEBUFF_BY_ID.get(id).is_some()
    }

    pub fn calculate(&mut self) -> u8 {
        let mut value: u8 = 0;
        for (id, stacks) in &self.sources {
            if let Some(buff) = CRITICAL_DAMAGE_DEBUFF_BY_ID.get(&id) {
                value += buff.value as u8 + buff.value_per_stack as u8 * stacks
            }
        }
        value
    }
}