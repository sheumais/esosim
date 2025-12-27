use std::collections::HashMap;
use esosim_data::item_type::{EnchantType, GearSlot, GearTrait, ItemType};
use esosim_data::power::{POWER_INCREASES_ADDITIVE, POWER_INCREASES_MULTIPLICATIVE};
use esosim_data::skill::{EXPERT_MAGE_ID, SLAYER_ID, SkillLine};
use esosim_data::traits::get_jewelry_infused_value;
use esosim_models::{power::Power as PowerModel};
use esosim_models::player::{ActiveBar, Player};

use crate::{ID, STACKS};

pub struct Power {
    pub sources: HashMap<ID, STACKS>,
    gear_source: u32,
    power: PowerModel,
}

impl Power {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            gear_source: 0,
            power: PowerModel::default(),
        }
    }

    pub fn add_source(&mut self, id: ID, stacks: Option<STACKS>) {
        self.sources.insert(id, stacks.unwrap_or(1));
        self.refresh();
    }

    pub fn add_source_without_refresh(&mut self, id: ID, stacks: Option<STACKS>) {
        self.sources.insert(id, stacks.unwrap_or(1));
    }

    pub fn add_raw_additive_unchecked(&mut self, value: u32) {
        self.power.add_to_additive(value.into());
    }
    
    pub fn add_raw_multiplicative_unchecked(&mut self, value: f32) {
        self.power.add_to_multiplicative(value);
    }

    pub fn set_raw_bloodthirsty_unchecked(&mut self, value: u32) {
        self.power.set_bloodthirsty(value);
    }

    pub fn remove_source(&mut self, id: &ID) {
        self.sources.remove(id);
        self.refresh();
    }

    fn refresh(&mut self) {
        self.power.reset();
        for (id, stacks) in &self.sources {
            if let Some(buff) = POWER_INCREASES_ADDITIVE.get(id) {
                self.power.add_to_additive((buff.value + buff.value_per_stack * *stacks as f64) as u32);
            } else if let Some(buff) = POWER_INCREASES_MULTIPLICATIVE.get(id) {
                self.power.add_to_multiplicative((buff.value + buff.value_per_stack * *stacks as f64) as f32 / 100.0);
            }
        }
        self.power.add_to_additive(self.gear_source);
    }

    pub fn is_valid_source(id: &ID) -> bool {
        POWER_INCREASES_ADDITIVE.get(id).is_some()
        || POWER_INCREASES_MULTIPLICATIVE.get(id).is_some()
    }

    pub fn calculate(&self) -> u32 {
        self.power.calculate()
    }

    pub fn update_from_player(&mut self, player: &Player) {
        self.power.reset();
        self.sources.clear();
        for (id, stacks) in player.get_buffs() {
            if Self::is_valid_source(id) {
                self.add_source_without_refresh(*id, Some(*stacks));
            }
        }

        let medium = player.get_number_of_equipped_item_type(&ItemType::Medium);
        if medium > 0 {self.add_source_without_refresh(45572, Some(medium))};
        let fighters = player.get_number_of_active_skills_from_skill_line(&SkillLine::FightersGuild);
        if fighters > 0 {self.add_source_without_refresh(SLAYER_ID, Some(fighters));}
        let sorcerer = player.get_number_of_active_skills_from_skill_line(&SkillLine::DarkMagic) 
            + player.get_number_of_active_skills_from_skill_line(&SkillLine::DaedricSummoning)
            + player.get_number_of_active_skills_from_skill_line(&SkillLine::StormCalling);
        let player_is_appropriate_sorcerer = player.has_buff(&EXPERT_MAGE_ID);
        if player_is_appropriate_sorcerer && sorcerer > 0 {self.add_source_without_refresh(EXPERT_MAGE_ID, Some(sorcerer));}

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
                    self.gear_source += (174.0 * multiplier) as u32;
                }
            }
        }

        self.refresh();
    }
}