use std::collections::HashMap;

use crate::data::critical_damage::{HEAVY_WEAPONS_ID, TWIN_BLADE_AND_BLUNT_ID};
use crate::data::item_type::{EnchantType, GearSlot, GearTrait, ItemType};
use crate::data::power::{POWER_INCREASES_ADDITIVE, POWER_INCREASES_MULTIPLICATIVE, SPELL_POWER_INCREASES_ADDITIVE, SPELL_POWER_INCREASES_MULTIPLICATIVE, WEAPON_POWER_INCREASES_ADDITIVE, WEAPON_POWER_INCREASES_MULTIPLICATIVE};
use crate::data::sets::{SetBonusType, get_total_bonus};
use crate::data::skill::{EXPERT_MAGE_ID, SLAYER_ID, SkillLine};
use crate::data::traits::get_jewelry_infused_value;
use crate::data::enchant::get_enchant_jewellery_increase_weapon_damage;
use crate::engine::{ID, STACKS};
use crate::models::player::{ActiveBar, Player};
use crate::models::power::Power as PowerModel;


pub struct Power {
    pub sources: HashMap<ID, STACKS>,
    gear_source: u32,
    weapon: PowerModel,
    spell: PowerModel,
    pub is_dirty: bool,
}

impl Power {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            gear_source: 0,
            weapon: PowerModel::default(),
            spell: PowerModel::default(),
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
        self.weapon.reset();
        self.spell.reset();
        for (id, stacks) in &self.sources {
            if let Some(buff) = POWER_INCREASES_ADDITIVE.get(id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as u32;
                self.weapon.add_to_additive(value);
                self.spell.add_to_additive(value);
            } else if let Some(buff) = POWER_INCREASES_MULTIPLICATIVE.get(id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as f32 / 100.0;
                self.weapon.add_to_multiplicative(value);
                self.spell.add_to_multiplicative(value);
            } else if let Some(buff) = WEAPON_POWER_INCREASES_ADDITIVE.get(id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as u32;
                self.weapon.add_to_additive(value);
            } else if let Some(buff) = WEAPON_POWER_INCREASES_MULTIPLICATIVE.get(id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as f32 / 100.0;
                self.weapon.add_to_multiplicative(value);
            } else if let Some(buff) = SPELL_POWER_INCREASES_ADDITIVE.get(id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as u32;
                self.spell.add_to_additive(value);
            } else if let Some(buff) = SPELL_POWER_INCREASES_MULTIPLICATIVE.get(id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as f32 / 100.0;
                self.spell.add_to_multiplicative(value);
            }
        }
        self.weapon.add_to_additive(self.gear_source);
        self.spell.add_to_additive(self.gear_source);
        self.is_dirty = false;
    }

    pub fn is_valid_source(id: &ID) -> bool {
        POWER_INCREASES_ADDITIVE.contains_key(id)
        || POWER_INCREASES_MULTIPLICATIVE.contains_key(id)
        || WEAPON_POWER_INCREASES_ADDITIVE.contains_key(id)
        || WEAPON_POWER_INCREASES_MULTIPLICATIVE.contains_key(id)
        || SPELL_POWER_INCREASES_ADDITIVE.contains_key(id)
        || SPELL_POWER_INCREASES_MULTIPLICATIVE.contains_key(id)
    }

    pub fn calculate(&self) -> u32 {
        self.spell.calculate().max(self.weapon.calculate())
    }

    pub fn update_from_player(&mut self, player: &Player) {
        self.spell.reset();
        self.weapon.reset();
        self.sources.clear();
        for (id, stacks) in player.get_buffs() {
            if Self::is_valid_source(id) {
                self.add_source(*id, Some(*stacks));
            }
        }

        let medium = player.get_number_of_equipped_item_type(&ItemType::Medium);
        if medium > 0 {self.add_source(45572, Some(medium))};
        let fighters = player.get_number_of_active_skills_from_skill_line(&SkillLine::FightersGuild);
        if fighters > 0 {self.add_source(SLAYER_ID, Some(fighters));}
        let sorcerer = player.get_number_of_active_skills_from_skill_line(&SkillLine::DarkMagic) 
            + player.get_number_of_active_skills_from_skill_line(&SkillLine::DaedricSummoning)
            + player.get_number_of_active_skills_from_skill_line(&SkillLine::StormCalling);
        let player_is_appropriate_sorcerer = player.has_buff(&EXPERT_MAGE_ID);
        if player_is_appropriate_sorcerer && sorcerer > 0 {self.add_source(EXPERT_MAGE_ID, Some(sorcerer));}
        let swords = player.get_number_of_equipped_item_type(&ItemType::Sword);
        let two_handed_sword = player.get_number_of_equipped_item_type(&ItemType::TwoHandedSword);

        if swords > 0 {self.add_source(TWIN_BLADE_AND_BLUNT_ID, Some(swords))};
        if two_handed_sword > 0 {self.add_source(HEAVY_WEAPONS_ID, Some(two_handed_sword))};

        let (main_slot, off_slot) = match player.get_active_bar() {
            ActiveBar::Primary => (GearSlot::MainHand, GearSlot::OffHand),
            ActiveBar::Backup => (GearSlot::MainHandBackup, GearSlot::OffHandBackup),
        };

        let main_hand_power = player
            .get_gear_piece(&main_slot)
            .map(|mhw| mhw.get_weapon_power(&main_slot))
            .unwrap_or(0);

        let off_hand_power = player
            .get_gear_piece(&off_slot)
            .map(|ohw| ohw.get_weapon_power(&off_slot))
            .unwrap_or(0);

        self.gear_source = (main_hand_power + off_hand_power) as u32;

        for gear in player.get_active_gear().iter() {
            if let Some(enchant) = &gear.enchant {
                if matches!(enchant.glyph, EnchantType::IncreasePhysicalDamage | EnchantType::IncreaseSpellDamage) {
                    let multiplier = if let Some(trait_) = &gear.gear_trait {
                        match trait_ {
                            GearTrait::JewelryInfused => get_jewelry_infused_value(&enchant.quality),
                            _ => 1.0
                        }
                    } else {
                        1.0
                    };
                    self.gear_source += (get_enchant_jewellery_increase_weapon_damage(&enchant.effective_level, &enchant.quality) * multiplier) as u32;
                }
            }
        }
        for set in player.get_active_sets_counts() {
            self.gear_source += get_total_bonus(&set, &SetBonusType::Power(None));
        }

        self.refresh();
    }
}