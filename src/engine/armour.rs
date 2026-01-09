use std::collections::HashMap;

use crate::data::armour::*;
use crate::data::critical_damage::TWIN_BLADE_AND_BLUNT_ID;
use crate::data::item_type::{EnchantType, GearTrait, ItemType, is_two_handed_weapon_option};
use crate::data::sets::{SetBonusType, get_total_bonus};
use crate::data::skill::{FROZEN_ARMOUR_ID, SPLINTERED_SECRETS_ID, SkillLine};
use crate::data::enchant::*;
use crate::data::traits::get_weapon_sharpened_value;
use crate::engine::{ID, STACKS};
use crate::models::armour::{Armour as ArmourModel, Penetration as PenetrationModel};
use crate::models::damage::DamageType;
use crate::models::player::Player;

pub struct Armour {
    pub sources: HashMap<ID, STACKS>,
    player_armour: u32,
    bleed: ArmourModel,
    cold: ArmourModel,
    disease: ArmourModel,
    fire: ArmourModel,
    magic: ArmourModel,
    oblivion: ArmourModel,
    physical: ArmourModel,
    poison: ArmourModel,
    shock: ArmourModel,
    spell: ArmourModel,
    pub is_dirty: bool,
}

impl Armour {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            player_armour: 0,
            bleed: ArmourModel::default(),
            cold: ArmourModel::default(),
            disease: ArmourModel::default(),
            fire: ArmourModel::default(),
            magic: ArmourModel::default(),
            oblivion: ArmourModel::default(),
            physical: ArmourModel::default(),
            poison: ArmourModel::default(),
            shock: ArmourModel::default(),
            spell: ArmourModel::default(),
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
        self.reset_all();

        self.bleed.add_to_additive(self.player_armour.clone());
        self.cold.add_to_additive(self.player_armour.clone());
        self.disease.add_to_additive(self.player_armour.clone());
        self.fire.add_to_additive(self.player_armour.clone());
        self.magic.add_to_additive(self.player_armour.clone());
        self.physical.add_to_additive(self.player_armour.clone());
        self.poison.add_to_additive(self.player_armour.clone());
        self.shock.add_to_additive(self.player_armour.clone());
        self.spell.add_to_additive(self.player_armour.clone());
        for (id, stacks) in &self.sources {
            if let Some(buff) = ARMOUR_ALL_BY_ID.get(&id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as u32;
                self.bleed.add_to_additive(value.clone());
                self.cold.add_to_additive(value.clone());
                self.disease.add_to_additive(value.clone());
                self.fire.add_to_additive(value.clone());
                self.magic.add_to_additive(value.clone());
                self.physical.add_to_additive(value.clone());
                self.poison.add_to_additive(value.clone());
                self.shock.add_to_additive(value.clone());
                self.spell.add_to_additive(value.clone());
            }

            if let Some(buff) = SPELL_RESISTANCE_BY_ID.get(&id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as u32;
                self.cold.add_to_additive(value.clone());
                self.fire.add_to_additive(value.clone());
                self.magic.add_to_additive(value.clone());
                self.shock.add_to_additive(value.clone());
                self.spell.add_to_additive(value.clone());
            }

            if let Some(buff) = PHYSICAL_RESISTANCE_BY_ID.get(&id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as u32;
                self.bleed.add_to_additive(value.clone());
                self.disease.add_to_additive(value.clone());
                self.physical.add_to_additive(value.clone());
                self.poison.add_to_additive(value.clone());
            }

            if let Some(buff) = FROST_RESISTANCE_BY_ID.get(&id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as u32;
                self.cold.add_to_additive(value.clone());
            }

            if let Some(buff) = POISON_DISEASE_RESISTANCE_BY_ID.get(&id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as u32;
                self.poison.add_to_additive(value.clone());
                self.disease.add_to_additive(value.clone());
            }
        }
        self.is_dirty = false;
    }

    fn reset_all(&mut self) {
        self.bleed.reset();
        self.cold.reset();
        self.disease.reset();
        self.fire.reset();
        self.magic.reset();
        self.oblivion.reset();
        self.physical.reset();
        self.poison.reset();
        self.shock.reset();
        self.spell.reset();
        self.is_dirty = true;
    }

    pub fn is_valid_source(id: &ID) -> bool {
        ARMOUR_ALL_BY_ID.get(id).is_some() || SPELL_RESISTANCE_BY_ID.get(id).is_some() || PHYSICAL_RESISTANCE_BY_ID.get(id).is_some() || FROST_RESISTANCE_BY_ID.get(id).is_some() || POISON_DISEASE_RESISTANCE_BY_ID.get(id).is_some()
    }

    pub fn calculate(&self, damage_type: &DamageType) -> u32 {
        match damage_type {
            DamageType::BLEED => self.bleed.calculate(),
            DamageType::DISEASE => self.disease.calculate(),
            DamageType::COLD => self.cold.calculate(),
            DamageType::FIRE => self.fire.calculate(),
            DamageType::MAGIC => self.magic.calculate(),
            DamageType::OBLIVION => self.oblivion.calculate(),
            DamageType::PHYSICAL => self.physical.calculate(),
            DamageType::POISON => self.poison.calculate(),
            DamageType::SHOCK => self.shock.calculate(),
        }
    }

    pub fn update_from_player(&mut self, player: &Player) {
        self.reset_all();
        self.sources.clear();

        let mut player_armour = player.get_total_armour();
        let heavy = player.get_number_of_equipped_item_type(&ItemType::Heavy);
        let light = player.get_number_of_equipped_item_type(&ItemType::Light);
        let warden = player.get_number_of_active_skills_from_skill_line(&SkillLine::WintersEmbrace);
        let ice_staves_shields = player.get_number_of_equipped_item_type(&ItemType::FrostStaff) + player.get_number_of_equipped_item_type(&ItemType::Shield);

        for (id, stacks) in player.get_buffs().iter() {
            if Self::is_valid_source(id) {
                self.add_source(*id, Some(*stacks))
            }
        }

        if heavy > 0 {self.add_source(45533, Some(heavy))};
        if light > 0 {self.add_source(45559, Some(light))};
        if warden > 0 {self.add_source(FROZEN_ARMOUR_ID, Some(warden))};
        if heavy >= 4 {
            if ice_staves_shields > 0 {self.add_source(64079, Some(1))}; // Assume players have this because it is shown for only the person logging
            player_armour += (34.62f32 * 50.0).round() as u32; // Assume players wearing heavy armour (tanks) always have this CP because it is never shown on logs so there is no way to know until reverse engineering the damage taken numbers.
        }
        for set in player.get_active_sets_counts() {
            player_armour += get_total_bonus(&set, &SetBonusType::Armour(None));
        }

        self.player_armour = player_armour;

        self.refresh();

        for gear in player.get_active_gear() {
            if let Some(enchant) = &gear.enchant {
                match enchant.glyph {
                    EnchantType::FrostResistance => self.cold.add_to_additive(get_enchant_jewellery_increase_frost_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    EnchantType::FireResistance => self.fire.add_to_additive(get_enchant_jewellery_increase_fire_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    EnchantType::ShockResistance => self.shock.add_to_additive(get_enchant_jewellery_increase_shock_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    EnchantType::PoisonResistance => self.poison.add_to_additive(get_enchant_jewellery_increase_poison_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    EnchantType::DiseaseResistance => self.disease.add_to_additive(get_enchant_jewellery_increase_disease_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    EnchantType::PhysicalResistance => self.physical.add_to_additive(get_enchant_jewellery_increase_physical_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    EnchantType::SpellResistance => self.spell.add_to_additive(get_enchant_jewellery_increase_spell_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    _ => {},
                }
            }
        }
    }
}

pub struct Penetration {
    pub sources: HashMap<ID, STACKS>,
    physical: PenetrationModel,
    spell: PenetrationModel,
    gear_source: u32,
    pub is_dirty: bool,
}

impl Penetration {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            physical: PenetrationModel::new(),
            spell: PenetrationModel::new(),
            gear_source: 0,
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
        self.reset_all();
        for (id, stacks) in &self.sources {
            if let Some(buff) = PENETRATION_ADDITIVE.get(&id) {
                let value = (buff.value + buff.value_per_stack * *stacks as f64) as u32;
                self.physical.add_to_additive(value);
                self.spell.add_to_additive(value);
            }
        }
        self.physical.add_to_additive(self.gear_source);
        self.spell.add_to_additive(self.gear_source);
        self.is_dirty = false;
    }

    fn reset_all(&mut self) {
        self.physical.reset();
        self.spell.reset();
        self.is_dirty = true;
    }

    pub fn is_valid_source(id: &ID) -> bool {
        PENETRATION_ADDITIVE.contains_key(id)
    }

    pub fn calculate(&self) -> u32 {
        self.physical.calculate().max(self.spell.calculate())
    }

    pub fn update_from_player(&mut self, player: &Player) {
        self.reset_all();
        self.sources.clear();
        self.gear_source = 0;
        for (id, stacks) in player.get_buffs().iter() {
            if Self::is_valid_source(id) {
                self.add_source(*id, Some(*stacks))
            }
        }
        let light = player.get_number_of_equipped_item_type(&ItemType::Light);
        let maces = player.get_number_of_equipped_item_type(&ItemType::Mace);
        let hott = player.get_number_of_active_skills_from_skill_line(&SkillLine::HeraldOfTheTome);
        if light > 0 {self.add_source(45562, Some(light))};
        if maces > 0 {self.add_source(TWIN_BLADE_AND_BLUNT_ID, Some(maces))};
        if hott > 0 {self.add_source(SPLINTERED_SECRETS_ID, Some(hott))};
        self.add_source(141895, Some(2)); // pen CP, assumed to exist

        for set in player.get_active_sets_counts() {
            self.gear_source += get_total_bonus(&set, &SetBonusType::Penetration(None));
        }
        for gear in player.get_active_gear() {
            if let Some(trait_) = &gear.gear_trait {
                match trait_ {
                    GearTrait::WeaponSharpened => {{let value = get_weapon_sharpened_value(&gear.quality); if is_two_handed_weapon_option(gear.get_item_type()) {self.gear_source += value as u32} else {self.gear_source += (value / 2.0).round() as u32}}},
                    _ => {},
                }
            }
        }

        self.refresh();
    }
}