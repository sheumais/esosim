use std::collections::HashMap;

use crate::data::critical_chance::{CRITICAL_CHANCE_BOTH, CRITICAL_CHANCE_SPELL, CRITICAL_CHANCE_WEAPON};
use crate::data::critical_damage::{CRITICAL_DAMAGE_DONE_BY_ID, CRITICAL_DAMAGE_TAKEN_BY_ID, DEXTERITY_ID, HEAVY_WEAPONS_ID, THE_SHADOW_ID, TWIN_BLADE_AND_BLUNT_ID};
use crate::data::item_type::{GearTrait, ItemType, is_two_handed_weapon_option};
use crate::data::sets::{SetBonusType, get_total_bonus};
use crate::data::traits::get_weapon_precise_value;
use crate::engine::{ID, STACKS};
use crate::models::critical::{CriticalDamage as CriticalDamageModel, CriticalChance as CriticalChanceModel};
use crate::models::player::Player;

pub struct CriticalDamage {
    pub sources: HashMap<ID, STACKS>,
    critical_damage: CriticalDamageModel,
    pub is_dirty: bool,
}

impl CriticalDamage {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            critical_damage: CriticalDamageModel::default(),
            is_dirty: false,
        }
    }

    pub fn add_source(&mut self, id: ID, stacks: Option<STACKS>) {
        self.sources.insert(id, stacks.unwrap_or(1));
        self.is_dirty = true;
    }

    pub fn add_source_checked(&mut self, id: ID, stacks: Option<STACKS>) {
        if Self::is_valid_source(&id) {
            self.add_source(id, stacks);
        }
    }

    pub fn add_raw_stat_unchecked(&mut self, value: u16) {
        self.critical_damage.add_percent(value);
    }

    pub fn remove_source(&mut self, id: &ID) {
        self.is_dirty = self.sources.remove(id).is_some();
    }

    pub fn refresh(&mut self) {
        self.critical_damage.reset();
        for (id, stacks) in &self.sources {
            if let Some(buff) = CRITICAL_DAMAGE_DONE_BY_ID.get(&id) {
                self.critical_damage.add_percent((buff.value + buff.value_per_stack * *stacks as f64) as u16);
            }
            // Malacath's add to multiplicative
        }
        self.is_dirty = false;
    }

    pub fn is_valid_source(id: &ID) -> bool {
        CRITICAL_DAMAGE_DONE_BY_ID.get(id).is_some()
    }

    pub fn calculate(&self) -> u16 {
        self.critical_damage.calculate()
    }

    pub fn calculate_uncapped(&self) -> u16 {
        self.critical_damage.calculate_uncapped()
    }

    pub fn update_from_player(&mut self, player: &Player) {
        self.critical_damage.reset();
        self.sources.clear();
        let divines = player.get_number_of_equipped_trait(&GearTrait::ArmorDivines);
        for (id, stacks) in player.get_buffs().iter() {
            if Self::is_valid_source(id) {
                match id {
                    &THE_SHADOW_ID => {
                        self.add_source(*id, Some(divines)) // assumes cp160 gold gear
                    },
                    _ => self.add_source(*id, Some(*stacks)),
                }
            }
        }

        let medium = player.get_number_of_equipped_item_type(&ItemType::Medium);
        let axes = player.get_number_of_equipped_item_type(&ItemType::Axe);
        let two_handed_axe = player.get_number_of_equipped_item_type(&ItemType::TwoHandedAxe);

        if axes > 0 {self.add_source(TWIN_BLADE_AND_BLUNT_ID, Some(axes))};
        if two_handed_axe > 0 {self.add_source(HEAVY_WEAPONS_ID, Some(two_handed_axe))};
        if medium > 0 {self.add_source(DEXTERITY_ID, Some(medium))};
        self.refresh();
    }
}

pub struct CriticalDamageTaken {
    pub sources: HashMap<ID, STACKS>,
    critical_damage_taken: u8,
    pub is_dirty: bool,
}

impl CriticalDamageTaken {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            critical_damage_taken: 0,
            is_dirty: false,
        }
    }

    pub fn add_source(&mut self, id: ID, stacks: Option<STACKS>) {
        self.sources.insert(id, stacks.unwrap_or(1));
        self.is_dirty = true;
    }
    
    pub fn add_source_checked(&mut self, id: ID, stacks: Option<STACKS>) {
        if Self::is_valid_source(&id) {
            self.add_source(id, stacks);
        }
    }

    pub fn add_raw_stat_unchecked(&mut self, value: u8) {
        self.critical_damage_taken += value;
    }

    pub fn remove_source(&mut self, id: &ID) {
        self.is_dirty = self.sources.remove(id).is_some();
    }

    pub fn refresh(&mut self) {
        self.critical_damage_taken = 0;
        for (id, stacks) in &self.sources {
            if let Some(buff) = CRITICAL_DAMAGE_TAKEN_BY_ID.get(&id) {
                self.critical_damage_taken += (buff.value + buff.value_per_stack * *stacks as f64).round() as u8;
            }
        }
        self.is_dirty = false;
    }

    pub fn is_valid_source(id: &ID) -> bool {
        CRITICAL_DAMAGE_TAKEN_BY_ID.get(id).is_some()
    }

    pub fn calculate(&self) -> u8 {
        self.critical_damage_taken
    }

    pub fn update_from_player(&mut self, player: &Player) {
        self.critical_damage_taken = 0;
        self.sources.clear();
        for (id, stacks) in player.get_buffs() {
            if Self::is_valid_source(id) {
                self.add_source(*id, Some(*stacks));
            }
        }

        self.refresh();
    }
}

pub struct CriticalChance {
    pub sources: HashMap<ID, STACKS>,
    weapon_critical: CriticalChanceModel,
    spell_critical: CriticalChanceModel,
    set_additive: u32,
    pub is_dirty: bool,
}

impl CriticalChance {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            weapon_critical: CriticalChanceModel::default(),
            spell_critical: CriticalChanceModel::default(),
            set_additive: 0,
            is_dirty: false,
        }
    }

    pub fn add_source(&mut self, id: ID, stacks: Option<STACKS>) {
        self.sources.insert(id, stacks.unwrap_or(1));
        self.is_dirty = true;
    }

    pub fn add_source_checked(&mut self, id: ID, stacks: Option<STACKS>) {
        if Self::is_valid_source(&id) {
            self.add_source(id, stacks);
        }
    }

    pub fn remove_source(&mut self, id: &ID) {
        self.is_dirty = self.sources.remove(id).is_some();
    }

    pub fn refresh(&mut self) {
        self.weapon_critical.reset();
        self.spell_critical.reset();
        for (id, stacks) in &self.sources {
            if let Some(buff) = CRITICAL_CHANCE_BOTH.get(&id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as u32;
                self.weapon_critical.add_to_additive(value);
                self.spell_critical.add_to_additive(value);
            } else if let Some(buff) = CRITICAL_CHANCE_SPELL.get(&id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as u32;
                self.spell_critical.add_to_additive(value);
            } else if let Some(buff) = CRITICAL_CHANCE_WEAPON.get(&id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as u32;
                self.weapon_critical.add_to_additive(value);
            }
        }
        self.weapon_critical.add_to_additive(self.set_additive);
        self.spell_critical.add_to_additive(self.set_additive);
        self.is_dirty = false;
    }

    pub fn is_valid_source(id: &ID) -> bool {
        CRITICAL_CHANCE_BOTH.contains_key(id) | CRITICAL_CHANCE_SPELL.contains_key(id) | CRITICAL_CHANCE_WEAPON.contains_key(id)
    }

    pub fn calculate(&self) -> f32 {
        self.weapon_critical.calculate().max(self.spell_critical.calculate())
    }

    pub fn get_raw(&self) -> u32 {
        self.weapon_critical.get_raw().max(self.spell_critical.get_raw())
    }

    pub fn update_from_player(&mut self, player: &Player) {
        self.weapon_critical.reset();
        self.spell_critical.reset();
        self.sources.clear();
        self.set_additive = 0;
        
        for (id, stacks) in player.get_buffs().iter() {
            if Self::is_valid_source(id) {
                self.add_source(*id, Some(*stacks));
            }
        }
        self.add_source(141898, Some(2)); // Precision CP
        let light = player.get_number_of_equipped_item_type(&ItemType::Light);
        if light > 0 {self.add_source(45562, Some(light))};
        for set in player.get_active_sets_counts() {
            self.set_additive += get_total_bonus(&set,&SetBonusType::CriticalChance(None));
        }
        for gear_piece in player.get_active_gear() {
            match gear_piece.gear_trait {
                Some(GearTrait::WeaponPrecise) => {let value = get_weapon_precise_value(&gear_piece.quality); if is_two_handed_weapon_option(gear_piece.get_item_type()) {self.set_additive += value as u32} else {self.set_additive += (value / 2.0).round() as u32}},
                _ => {},
            }
        }
        self.refresh();
    }
}

#[cfg(test)]
mod tests {
    use crate::{data::{item_type::{GearSlot, ItemQuality}, major_minor::*}, engine::player::character::Character, models::player::GearPiece};

    use super::*;

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

        crit.add_source_checked(FORCE_MINOR_ID, Some(1));
        crit.refresh();
        let with_force = crit.calculate();
        assert!(with_force == 60, "was {}", crit.calculate());

        crit.remove_source(&FORCE_MINOR_ID);
        crit.refresh();
        let without_force = crit.calculate();
        assert!(without_force < with_force);
    }

    #[test]
    fn stacking_buffs_scale_correctly() {
        let mut crit = CriticalDamage::new();

        crit.add_source(DEXTERITY_ID, Some(6));
        crit.refresh();
        assert_eq!(crit.calculate(), 62);
    }

    #[test]
    fn twin_blade_and_blunt_scales_with_axes() {
        let mut character = Character::new(0);

        character.set_gear_piece(
        &GearSlot::MainHand,
        GearPiece {
                item_id: 172034,
                effective_level: 66,
                gear_trait: Some(GearTrait::WeaponNirnhoned),
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
                gear_trait: Some(GearTrait::WeaponPrecise),
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
        let mut character = Character::new(0);

        character.set_gear_piece(
        &GearSlot::MainHand,
        GearPiece {
                item_id: 131073,
                effective_level: 66,
                gear_trait: Some(GearTrait::WeaponPrecise),
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
        let mut character = Character::new(0);

        character.set_gear_piece(
            &GearSlot::Necklace,
            GearPiece {
                item_id: 194512,
                effective_level: 66,
                gear_trait: Some(GearTrait::JewelryBloodthirsty),
                quality: ItemQuality::Legendary,
                set_id: Some(694),
                enchant: None,
            },
        );

        let crit = character.get_critical_damage_done();
        assert!(crit == 60);
    }
}