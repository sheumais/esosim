use std::collections::HashMap;

use eso_skill_data::enums::skill_line::SkillLine;

use crate::{data::{enums::{damage::ResistableDamageType, gear::{EnchantType, GearSlot, GearTrait, ItemType}}, tables::{power::OFFHAND_MULTIPLIER, sets::SET_BONUSES}}, engine::{active_effects::ActiveEffects, aggregator::ChannelSet, effect::Channel, grant_rule::GrantRule, registry::EffectRegistry}, entity::{gear::{GearPiece, Loadout}, player::ActiveBar::*}};

pub enum ActiveBar {
    Front,
    Backup,
}
impl ActiveBar {
    pub fn to_bool(&self) -> bool {
        match &self {
            Front => false,
            Backup => true,
        }
    }
}

pub struct Player {
    pub active_effects: ActiveEffects,
    pub channels: ChannelSet,
    pub gear: Loadout,
    pub active_bar: ActiveBar,
    pub primary_abilities: Vec<u32>,
    pub backup_abilities: Vec<u32>,
    pub werewolf_abilities: Vec<u32>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            active_effects: ActiveEffects::new(),
            channels: ChannelSet::new(),
            gear: Loadout::default(),
            active_bar: Front,
            primary_abilities: Vec::new(),
            backup_abilities: Vec::new(),
            werewolf_abilities: Vec::new(),
        }
    }

    pub fn get_active_gear(&self) -> Vec<&GearPiece> {
        self.gear.get_active_gear(self.get_active_bar_as_bool())
    }

    pub fn get_active_gear_with_slots(&self) -> impl Iterator<Item = (GearSlot, &GearPiece)> {
        self.gear.iter_active_gear(self.get_active_bar_as_bool())
    }

    pub fn get_active_bar(&self) -> &ActiveBar {
        &self.active_bar
    }

    pub fn get_active_bar_as_bool(&self) -> bool {
        self.get_active_bar().to_bool()
    }

    pub fn add_buff(&mut self, id: u32, stacks: u8) {
        self.active_effects.set(id, stacks as u32);
    }
    pub fn remove_buff(&mut self, id: u32) {
        self.active_effects.set(id, 0);
    }
    pub fn has_buff(&self, id: u32) -> bool {
        self.active_effects.has(id)
    }

    pub fn is_specific_item_equipped(&self, item_id: &u32) -> bool {
        for item in self.get_active_gear() {
            if &item.item_id == item_id {return true}
        }
        false
    }

    pub fn get_number_of_equipped_item_type(&self, item_type: &ItemType) -> u8 {
        self.gear.get_number_of_item_type(item_type, self.get_active_bar_as_bool())
    }

    pub fn get_number_of_equipped_trait(&self, item_trait: &GearTrait) -> u8 {
        self.gear.get_number_of_trait(item_trait, self.get_active_bar_as_bool())
    }

    pub fn set_gear_piece(&mut self, slot: GearSlot, gear: GearPiece) {
        self.gear.set_gear_piece(slot, gear);
    }

    pub fn set_skills(&mut self, backbar: bool, skills: Vec<u32>) {
        match backbar {
            false => self.primary_abilities = skills,
            true => self.backup_abilities = skills,
        }
    }

    pub fn get_gear_piece(&self, slot: &GearSlot) -> Option<&GearPiece> {
        self.gear.get_gear_piece(slot)
    }

    pub fn recompute(&mut self, registry: &EffectRegistry, grant_rules: &[GrantRule]) {
        let mut active = self.active_effects.clone();
        for rule in grant_rules {
            if let Some(stacks) = rule.evaluate(self) {
                active.set(rule.effect_id, stacks);
            }
        }
        let mut channels = active.aggregate(registry);
        self.apply_gear(&mut channels);
        self.apply_set_bonuses(&mut channels);
        self.channels = channels;
    }

    pub fn apply_gear(&self, channels: &mut ChannelSet) {
        for (slot, gear) in self.get_active_gear_with_slots() {
            let power = gear.get_weapon_power();
            if power > 0f32 {
                let m = match slot.is_offhand() {
                    true => OFFHAND_MULTIPLIER, // todo add "get player offhand multipler, with +0.06"
                    false => 1.0,
                };
                let p = (power * m).round() as i64;
                channels.add_additive(Channel::Power, p);
            } 
            channels.add_additive(Channel::Armour(ResistableDamageType::All), gear.get_armour_value(&slot) as i64);
            if let Some(v) = gear.get_item_trait() {
                match v {
                    GearTrait::JewelryBloodthirsty => {},
                    GearTrait::JewelryHarmony => {},
                    GearTrait::JewelryProtective => {},
                    GearTrait::JewelrySwift => {},
                    GearTrait::JewelryTriune => {},
                    GearTrait::JewelryInfused => {},
                    GearTrait::JewelryArcane => {},
                    GearTrait::JewelryRobust => {},
                    GearTrait::JewelryHealthy => {},

                    GearTrait::ArmorSturdy => {},
                    GearTrait::ArmorImpenetrable => {},
                    GearTrait::ArmorReinforced => {},
                    GearTrait::ArmorWellFitted => {},
                    GearTrait::ArmorDivines => {},
                    GearTrait::ArmorNirnhoned => {},
                    GearTrait::ArmorInfused => {},
                    GearTrait::ArmorTraining => {},
                    GearTrait::ArmorInvigorating => {},
                    GearTrait::ArmorIntricate => {},
                    GearTrait::ArmorOrnate => {},

                    GearTrait::WeaponInfused => {},
                    GearTrait::WeaponNirnhoned => {},
                    GearTrait::WeaponCharged => {},
                    GearTrait::WeaponDecisive => {},
                    GearTrait::WeaponDefending => {},
                    GearTrait::WeaponPowered => {},
                    GearTrait::WeaponPrecise => {},
                    GearTrait::WeaponSharpened => {},
                    GearTrait::WeaponTraining => {},
                    GearTrait::WeaponIntricate => {},
                    GearTrait::WeaponOrnate => {},
                    _ => {},
                }
            }
            if let Some(e) = gear.get_enchant() {
                let v = gear.get_trait_value().unwrap_or(0f32);
                match e.glyph {
                    EnchantType::DiseaseResistance => {},
                    EnchantType::FireResistance => {},
                    EnchantType::FrostResistance => {},
                    EnchantType::Health => {},
                    EnchantType::HealthRegen => {},
                    EnchantType::IncreaseBashDamage => {},
                    EnchantType::IncreasePhysicalDamage => {},
                    EnchantType::IncreasePotionEffectiveness => {},
                    EnchantType::IncreaseSpellDamage => {},
                    EnchantType::Magicka => {},
                    EnchantType::MagickaRegen => {},
                    EnchantType::PhysicalResistance => {},
                    EnchantType::PoisonResistance => {},
                    EnchantType::PrismaticDefense => {},
                    EnchantType::PrismaticRecovery => {},
                    EnchantType::ReduceBlockAndBash => {},
                    EnchantType::ReduceFeatCost => {},
                    EnchantType::ReducePotionCooldown => {},
                    EnchantType::ReduceSpellCost => {},
                    EnchantType::ShockResistance => {},
                    EnchantType::SpellResistance => {},
                    EnchantType::Stamina => {},
                    EnchantType::StaminaRegen => {},
                    _ => {},
                }
            }
        }
    }

    pub fn apply_set_bonuses(&self, channels: &mut ChannelSet) {
        for (set_id, count) in self.get_active_sets_counts() {
            let Some(set) = SET_BONUSES.get(&set_id) else { continue };
            for row in set.bonuses.iter().take(count as usize) {
                for bonus in row.iter() {
                    channels.add_additive(bonus.channel, bonus.value as i64);
                }
            }
        }
    }

    pub fn get_active_sets_counts(&self) -> HashMap<u16, u8> {
        let mut sets = HashMap::new();
        for piece in self.get_active_gear() {
            if let Some(set_id) = piece.set_id {
                *sets.entry(set_id).or_insert(0) += 1 + piece.is_two_handed_weapon() as u8;
            }
        }
        sets
    }

    pub fn get_number_of_equipped_set(&self, set_id: u16) -> u8 {
        self.get_active_sets_counts().get(&set_id).copied().unwrap_or(0)
    }

    pub fn get_number_of_active_skills_from_skill_line(&self, skill_line: &SkillLine) -> u8 {
        todo!()
    }
}