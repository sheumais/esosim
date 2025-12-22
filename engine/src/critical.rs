use std::collections::HashMap;
use esosim_data::{critical_damage::*, item_type::ItemType, major_minor::FORCE_MINOR_ID, sets::mythic::velothi::{VELOTHI_UR_MAGES_AMULET_ID, VELOTHI_UR_MAGES_AMULET_ITEM_ID}};
use esosim_models::{critical::CriticalDamage as CriticalDamageModel, player::{GearTrait, Player}};

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

    pub fn add_source_without_refresh(&mut self, id: ID, stacks: Option<STACKS>) {
        self.sources.insert(id, stacks.unwrap_or(1));
    }

    pub fn remove_source(&mut self, id: &ID) {
        self.sources.remove(id);
        self.refresh();
    }

    fn refresh(&mut self) {
        self.critical_damage.reset();
        let mut has_seen_minor_force = false;
        for (id, stacks) in &self.sources {
            // println!("ID: {}, stacks: {}", id, stacks);
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
        self.critical_damage.reset();
        self.sources.clear();
        let divines = player.get_number_of_equipped_trait(&GearTrait::Divines);
        for buff in player.get_buffs().iter() {
            if Self::is_valid_source(buff.0) {
                match buff.0 {
                    &THE_SHADOW_ID => {
                        self.add_source_without_refresh(*buff.0, Some(divines)) // assumes cp160 gold gear
                    },
                    _ => self.add_source_without_refresh(*buff.0, Some(*buff.1)),
                }
            }
        }

        let medium = player.get_number_of_equipped_item_type(&ItemType::Medium);
        let axes = player.get_number_of_equipped_item_type(&ItemType::Axe);
        let two_handed_axe = player.get_number_of_equipped_item_type(&ItemType::TwoHandedAxe);
        let is_velothi_equipped = player.is_specific_item_equipped(VELOTHI_UR_MAGES_AMULET_ITEM_ID);

        if axes > 0 {self.add_source_without_refresh(TWIN_BLADE_AND_BLUNT_ID, Some(axes))};
        if two_handed_axe > 0 {self.add_source_without_refresh(HEAVY_WEAPONS_ID, Some(two_handed_axe))};
        if medium > 0 {self.add_source_without_refresh(DEXTERITY_ID, Some(medium))};
        if is_velothi_equipped {self.add_source_without_refresh(FORCE_MINOR_ID, None);}
        self.refresh();
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{character::Character, critical::{CriticalDamage, CriticalDamageTaken}};
    use esosim_models::player::{GearPiece, GearSlot, GearTrait};
    use esosim_data::{item_type::ItemQuality, major_minor::{BRITTLE_MAJOR_ID, BRITTLE_MINOR_ID}, sets::mythic::velothi::VELOTHI_UR_MAGES_AMULET_ITEM_ID};

    #[test]
    fn critical_damage_done_ids_are_registered() {
        assert!(CRITICAL_DAMAGE_DONE_BY_ID.contains_key(&FORCE_MINOR_ID));
        assert!(CRITICAL_DAMAGE_DONE_BY_ID.contains_key(&THE_SHADOW_ID));
        assert!(CRITICAL_DAMAGE_DONE_BY_ID.contains_key(&DEXTERITY_ID));
    }

    #[test]
    fn critical_damage_taken_ids_are_registered() {
        assert!(CRITICAL_DAMAGE_TAKEN_BY_ID.contains_key(&BRITTLE_MINOR_ID));
        assert!(CRITICAL_DAMAGE_TAKEN_BY_ID.contains_key(&BRITTLE_MAJOR_ID));
    }

    #[test]
    fn add_and_remove_source_updates_value() {
        let mut crit = CriticalDamage::new();

        crit.add_source(FORCE_MINOR_ID, None);
        let with_force = crit.calculate();
        assert!(with_force == 60);

        crit.remove_source(&FORCE_MINOR_ID);
        let without_force = crit.calculate();
        assert!(without_force < with_force);
    }

    #[test]
    fn stacking_buffs_scale_correctly() {
        let mut crit = CriticalDamage::new();

        crit.add_source(DEXTERITY_ID, Some(6));
        assert_eq!(crit.calculate(), 62);
    }

    #[test]
    fn minor_force_and_velothi_do_not_stack() {
        let mut crit = CriticalDamage::new();

        crit.add_source(FORCE_MINOR_ID, None);
        let force_only = crit.calculate();

        crit.add_source(VELOTHI_UR_MAGES_AMULET_ID, None);
        let both = crit.calculate();

        assert_eq!(force_only, both);
    }

    #[test]
    fn twin_blade_and_blunt_scales_with_axes() {
        let mut character = Character::new();

        character.set_gear_piece(
        &GearSlot::MainHand,
        GearPiece {
                item_id: 172034,
                effective_level: 66,
                gear_trait: Some(GearTrait::Nirnhoned),
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            },
        );
        character.set_gear_piece(
        &GearSlot::OffHand,
        GearPiece {
                item_id: 172034,
                effective_level: 66,
                gear_trait: Some(GearTrait::Precise),
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            },
        );

        let crit = character.get_critical_damage_done();
        assert!(crit == 62);
    }

    #[test]
    fn heavy_weapons_applies_for_two_handed_axe() {
        let mut character = Character::new();

        character.set_gear_piece(
        &GearSlot::MainHand,
        GearPiece {
                item_id: 131073,
                effective_level: 66,
                gear_trait: Some(GearTrait::Precise),
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            },
        );

        let crit = character.get_critical_damage_done();
        assert!(crit == 62);
    }

    #[test]
    fn velothi_item_adds_minor_force_if_equipped() {
        let mut character = Character::new();

        character.set_gear_piece(
            &GearSlot::Necklace,
            GearPiece {
                item_id: VELOTHI_UR_MAGES_AMULET_ITEM_ID,
                effective_level: 66,
                gear_trait: Some(GearTrait::Bloodthirsty),
                quality: ItemQuality::Legendary,
                set_id: Some(694),
                enchant: None,
            },
        );

        let crit = character.get_critical_damage_done();
        assert!(crit == 60);
    }

    #[test]
    fn critical_damage_taken_accumulates_sources() {
        let mut taken = CriticalDamageTaken {
            sources: HashMap::new(),
        };

        taken.add_source(BRITTLE_MINOR_ID, None);
        taken.add_source(FLAME_WEAKNESS_ID, None);

        assert_eq!(taken.calculate(), 15);
    }

    #[test]
    fn removing_taken_source_updates_value() {
        let mut taken = CriticalDamageTaken {
            sources: HashMap::new(),
        };

        taken.add_source(BRITTLE_MAJOR_ID, None);
        assert_eq!(taken.calculate(), 20);

        taken.remove_source(BRITTLE_MAJOR_ID);
        assert_eq!(taken.calculate(), 0);
    }
}