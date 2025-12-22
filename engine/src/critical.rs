use std::collections::HashMap;
use esosim_data::{critical_damage::{CRITICAL_DAMAGE_DONE_BY_ID, CRITICAL_DAMAGE_TAKEN_BY_ID, DEXTERITY_ID, FORCE_MINOR_ID, HEAVY_WEAPONS_ID, THE_SHADOW_ID, TWIN_BLADE_AND_BLUNT_ID, VELOTHI_UR_MAGES_AMULET_ID, VELOTHI_UR_MAGES_AMULET_ITEM_ID}, item_type::ItemType};
use esosim_models::{critical::CriticalDamage as CriticalDamageModel, player::Player};

use crate::{ID, STACKS};

pub struct CriticalDamage {
    pub sources: HashMap<ID, STACKS>,
    critical_damage: CriticalDamageModel,
}

impl CriticalDamage {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            critical_damage: CriticalDamageModel::default(),
        }
    }

    pub fn add_source(&mut self, id: ID, stacks: Option<STACKS>) {
        self.sources.insert(id, stacks.unwrap_or(1));
        self.refresh();
    }

    pub fn remove_source(&mut self, id: &ID) {
        self.sources.remove(id);
        self.refresh();
    }

    fn refresh(&mut self) {
        self.critical_damage.reset();
        let mut has_seen_minor_force = false;
        for (id, stacks) in &self.sources {
            if matches!(id, &FORCE_MINOR_ID | &VELOTHI_UR_MAGES_AMULET_ID) {
                match has_seen_minor_force {
                    true => continue,
                    false => has_seen_minor_force = true,
                }
            }
            if let Some(buff) = CRITICAL_DAMAGE_DONE_BY_ID.get(&id) {
                self.critical_damage.add_to_additive((buff.value + buff.value_per_stack * *stacks as i32) as u16);
            }
            // Malacath's add to multiplicative
        }
    }

    pub fn is_valid_source(id: &ID) -> bool {
        CRITICAL_DAMAGE_DONE_BY_ID.get(id).is_some()
    }

    pub fn calculate(&self) -> u8 {
        self.critical_damage.calculate()
    }

    pub fn update_from_player(&mut self, player: &Player) {
        for buff in player.get_buffs().iter() {
            if Self::is_valid_source(buff.0) {
                match buff.0 {
                    &THE_SHADOW_ID => {
                        self.add_source(*buff.0, Some(7)) // magic number of divines pieces on a dps for now
                    },
                    _ => self.add_source(*buff.0, Some(*buff.1)),
                }
            }
        }

        let medium = player.get_number_of_equipped_item_type(&ItemType::Medium);
        let axes = player.get_number_of_equipped_item_type(&ItemType::Axe);
        let two_handed_axe = player.get_number_of_equipped_item_type(&ItemType::TwoHandedAxe);
        let is_velothi_equipped = player.is_specific_item_equipped(VELOTHI_UR_MAGES_AMULET_ITEM_ID);

        self.add_source(TWIN_BLADE_AND_BLUNT_ID, Some(axes));
        self.add_source(HEAVY_WEAPONS_ID, Some(two_handed_axe));
        self.add_source(DEXTERITY_ID, Some(medium));
        if is_velothi_equipped {self.add_source(VELOTHI_UR_MAGES_AMULET_ID, None);}
    }
}

pub struct CriticalDamageTaken {
    pub sources: HashMap<ID, STACKS>,
}

impl CriticalDamageTaken {
    pub fn add_source(&mut self, id: ID, stacks: Option<STACKS>) {
        self.sources.insert(id, stacks.unwrap_or(1));
    }

    pub fn remove_source(&mut self, id: ID) {
        self.sources.remove(&id);
    }

    pub fn is_valid_source(id: &ID) -> bool {
        CRITICAL_DAMAGE_TAKEN_BY_ID.get(id).is_some()
    }

    pub fn calculate(&self) -> u8 {
        let mut value: u8 = 0;
        for (id, stacks) in &self.sources {
            if let Some(buff) = CRITICAL_DAMAGE_TAKEN_BY_ID.get(&id) {
                value += buff.value as u8 + buff.value_per_stack as u8 * stacks
            }
        }
        value
    }
}