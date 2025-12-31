use std::collections::HashMap;
use esosim_data::{armour::{ARMOUR_ALL_BY_ID, FROST_RESISTANCE_BY_ID, PHYSICAL_RESISTANCE_BY_ID, POISON_DISEASE_RESISTANCE_BY_ID, SPELL_RESISTANCE_BY_ID}, enchant::{get_enchant_jewellery_increase_disease_resistance, get_enchant_jewellery_increase_fire_resistance, get_enchant_jewellery_increase_frost_resistance, get_enchant_jewellery_increase_physical_resistance, get_enchant_jewellery_increase_poison_resistance, get_enchant_jewellery_increase_shock_resistance, get_enchant_jewellery_increase_spell_resistance}, item_type::{EnchantType, GearTrait, ItemType}, skill::{FROZEN_ARMOUR_ID, SkillLine}};
use esosim_models::{armour::Armour as ArmourModel, damage::DamageType, player::Player};

use crate::{ID, STACKS};

    // BLEED,
    // COLD,
    // DISEASE,
    // FIRE,
    // MAGIC,
    // OBLIVION,
    // PHYSICAL,
    // POISON,
    // SHOCK,

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
        }
    }

    pub fn add_source(&mut self, id: ID, stacks: Option<STACKS>) {
        self.sources.insert(id, stacks.unwrap_or(1));
    }

    pub fn remove_source(&mut self, id: &ID) {
        self.sources.remove(id);
    }

    fn refresh(&mut self) {
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

        self.player_armour = player_armour;

        self.refresh();

        for gear in player.get_active_gear() {
            if let Some(enchant) = &gear.enchant {
                match enchant.glyph {
                    EnchantType::FrostResistance    => self.cold.add_to_additive(get_enchant_jewellery_increase_frost_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    EnchantType::FireResistance     => self.fire.add_to_additive(get_enchant_jewellery_increase_fire_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    EnchantType::ShockResistance    => self.shock.add_to_additive(get_enchant_jewellery_increase_shock_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    EnchantType::PoisonResistance   => self.poison.add_to_additive(get_enchant_jewellery_increase_poison_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    EnchantType::DiseaseResistance  => self.disease.add_to_additive(get_enchant_jewellery_increase_disease_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    EnchantType::PhysicalResistance => self.physical.add_to_additive(get_enchant_jewellery_increase_physical_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    EnchantType::SpellResistance    => self.spell.add_to_additive(get_enchant_jewellery_increase_spell_resistance(&enchant.effective_level, &enchant.quality) as u32),
                    _ => {},
                }
            }
        }
    }
}
