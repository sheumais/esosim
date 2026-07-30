use std::collections::HashMap;

use eso_skill_data::enums::skill_line::SkillLine;

use crate::{data::{enums::{cca::CoreCombatAbility, damage::ResistableDamageType, gear::{EnchantType, GearSlot, GearTrait, ItemType}}, tables::{power::OFFHAND_MULTIPLIER, sets::{PERFECTED_TO_SET, SET_BONUSES}}}, engine::{active_effects::ActiveEffects, aggregator::ChannelSet, effect::{AggKind, Channel, ResourceKind}, grant_rule::GrantRule, registry::EffectRegistry}, entity::{gear::{GearPiece, Loadout, PrismaticValueWrapper}, player::ActiveBar::*}};

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

    pub fn apply_gear(&self, channels: &mut ChannelSet) { // todo add tests
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
            if let Some(gear_trait) = gear.get_item_trait() {
                let wrapper = gear.get_trait_value();
                match wrapper {
                    PrismaticValueWrapper::F32(v) => {
                        match gear_trait {
                            GearTrait::JewelryBloodthirsty => {},
                            GearTrait::JewelryHarmony => {
                                channels.add_additive(Channel::SynergyRestore(ResourceKind::Health, AggKind::Additive), v as i64);
                                channels.add_additive(Channel::SynergyRestore(ResourceKind::Magicka, AggKind::Additive), v as i64);
                                channels.add_additive(Channel::SynergyRestore(ResourceKind::Stamina, AggKind::Additive), v as i64);
                            },
                            GearTrait::JewelryProtective => {channels.add_additive(Channel::Armour(ResistableDamageType::All), v as i64)},
                            GearTrait::JewelrySwift => {channels.add_additive(Channel::MovementSpeed, v as i64);},
                            GearTrait::JewelryTriune => {}, // handle below
                            GearTrait::JewelryInfused => {}, // handle in enchantment code
                            GearTrait::JewelryArcane => {channels.add_additive(Channel::Resource(ResourceKind::Magicka, AggKind::Additive), v as i64)},
                            GearTrait::JewelryRobust => {channels.add_additive(Channel::Resource(ResourceKind::Stamina, AggKind::Additive), v as i64)},
                            GearTrait::JewelryHealthy => {channels.add_additive(Channel::Resource(ResourceKind::Health, AggKind::Additive), v as i64)},

                            GearTrait::ArmorSturdy => {channels.add_multiplicative_bps(Channel::CostReductionCCA(CoreCombatAbility::Block), (v * 10_000.0).round() as i64);},
                            GearTrait::ArmorImpenetrable => {channels.add_additive(Channel::CriticalResistance, v as i64);},
                            GearTrait::ArmorReinforced => {}, // handle in armour code
                            GearTrait::ArmorWellFitted => {
                                let bps = (v * 10_000.0).round() as i64;
                                channels.add_multiplicative_bps(Channel::CostReductionCCA(CoreCombatAbility::DodgeRoll), bps);
                                channels.add_multiplicative_bps(Channel::CostReductionCCA(CoreCombatAbility::Sprint), bps);
                            },
                            GearTrait::ArmorDivines => {channels.add_multiplicative_bps(Channel::MundusBoost, (v * 10_000.0).round() as i64);},
                            GearTrait::ArmorNirnhoned => {channels.add_additive(Channel::Armour(ResistableDamageType::All), v as i64)},
                            GearTrait::ArmorInfused => {}, // handle in enchantment code
                            GearTrait::ArmorTraining => {channels.add_multiplicative_bps(Channel::KillExperience, (v * 10_000.0).round() as i64);},
                            GearTrait::ArmorInvigorating => {
                                {channels.add_additive(Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), v as i64)};
                                {channels.add_additive(Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), v as i64)};
                                {channels.add_additive(Channel::Recovery(ResourceKind::Health, AggKind::Additive), v as i64)};
                            },

                            GearTrait::WeaponInfused => {}, // handled in enchantment code
                            GearTrait::WeaponNirnhoned => {}, // handled in weapon code
                            GearTrait::WeaponCharged => {channels.add_multiplicative_bps(Channel::StatusEffectChance, (v * 10_000.0).round() as i64);},
                            GearTrait::WeaponDecisive => {channels.add_multiplicative_bps(Channel::DecisiveUltimateChance, (v * 10_000.0).round() as i64);},
                            GearTrait::WeaponDefending => {channels.add_additive(Channel::Armour(ResistableDamageType::All), v as i64)},
                            GearTrait::WeaponPowered => {channels.add_multiplicative_bps(Channel::HealingDone, (v * 10_000.0).round() as i64);},
                            GearTrait::WeaponPrecise => {channels.add_additive(Channel::CriticalChance, v as i64)},
                            GearTrait::WeaponSharpened => {channels.add_additive(Channel::Penetration(ResistableDamageType::All), v as i64)},
                            GearTrait::WeaponTraining => {channels.add_multiplicative_bps(Channel::KillExperience, (v * 10_000.0).round() as i64);},
                            _ => {},
                        }
                    }
                    PrismaticValueWrapper::Prismatic(v) => {
                        match gear_trait {
                            GearTrait::JewelryTriune => {
                                channels.add_additive(Channel::Resource(ResourceKind::Health, AggKind::Additive), v.0 as i64);
                                channels.add_additive(Channel::Resource(ResourceKind::Magicka, AggKind::Additive), v.1 as i64);
                                channels.add_additive(Channel::Resource(ResourceKind::Stamina, AggKind::Additive), v.2 as i64);
                            },
                            _ => {},
                        }
                    }
                }
            }
            if let Some(e) = gear.get_enchant() {
                let wrapper = gear.get_enchant_value().expect("An enchant should always have a value");
                let infused_multiplier = gear.get_infused_value();
                match wrapper {
                    PrismaticValueWrapper::F32(v_) => {
                        let v = v_ * infused_multiplier;
                        match e.glyph {
                            EnchantType::DiseaseResistance => {channels.add_additive(Channel::Armour(ResistableDamageType::Disease), v as i64)},
                            EnchantType::FlameResistance => {channels.add_additive(Channel::Armour(ResistableDamageType::Flame), v as i64)},
                            EnchantType::FrostResistance => {channels.add_additive(Channel::Armour(ResistableDamageType::Frost), v as i64)},
                            EnchantType::Health => {channels.add_additive(Channel::Resource(ResourceKind::Health, AggKind::Additive), v as i64)},
                            EnchantType::HealthRegen => {channels.add_additive(Channel::Recovery(ResourceKind::Health, AggKind::Additive), v as i64)},
                            EnchantType::IncreaseBashDamage => {todo!()},
                            EnchantType::IncreasePhysicalDamage => {channels.add_additive(Channel::Power, v as i64); channels.add_additive(Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), 10i64)},
                            EnchantType::IncreasePotionEffectiveness => {todo!()},
                            EnchantType::IncreaseSpellDamage => {channels.add_additive(Channel::Power, v as i64); channels.add_additive(Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), 10i64)},
                            EnchantType::Magicka => {channels.add_additive(Channel::Resource(ResourceKind::Magicka, AggKind::Additive), v as i64)},
                            EnchantType::MagickaRegen => {channels.add_additive(Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), v as i64)},
                            EnchantType::PhysicalResistance => {channels.add_additive(Channel::Armour(ResistableDamageType::Martial), v as i64)},
                            EnchantType::PoisonResistance => {channels.add_additive(Channel::Armour(ResistableDamageType::Poison), v as i64)},
                            EnchantType::PrismaticDefense => {},
                            EnchantType::PrismaticRecovery => {},
                            EnchantType::ReduceBlockAndBash => {todo!()},
                            EnchantType::ReduceFeatCost => {
                                todo!()
                            },
                            EnchantType::ReducePotionCooldown => {todo!()},
                            EnchantType::ReduceSpellCost => {todo!()},
                            EnchantType::ShockResistance => {channels.add_additive(Channel::Armour(ResistableDamageType::Shock), v as i64)},
                            EnchantType::SpellResistance => {channels.add_additive(Channel::Armour(ResistableDamageType::Spell), v as i64)},
                            EnchantType::Stamina => {channels.add_additive(Channel::Resource(ResourceKind::Stamina, AggKind::Additive), v as i64)},
                            EnchantType::StaminaRegen => {channels.add_additive(Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), v as i64)},
                            _ => {},
                        }
                    },
                    PrismaticValueWrapper::Prismatic(v_) => {
                        let v = (v_.0 * infused_multiplier, v_.1 * infused_multiplier, v_.2 * infused_multiplier);
                        match e.glyph {
                            EnchantType::PrismaticDefense => {
                                channels.add_additive(Channel::Resource(ResourceKind::Health, AggKind::Additive), v.0 as i64);
                                channels.add_additive(Channel::Resource(ResourceKind::Magicka, AggKind::Additive), v.1 as i64);
                                channels.add_additive(Channel::Resource(ResourceKind::Stamina, AggKind::Additive), v.2 as i64);
                            },
                            EnchantType::PrismaticRecovery => {
                                channels.add_additive(Channel::Recovery(ResourceKind::Health, AggKind::Additive), v.0 as i64);
                                channels.add_additive(Channel::Recovery(ResourceKind::Magicka, AggKind::Additive), v.1 as i64);
                                channels.add_additive(Channel::Recovery(ResourceKind::Stamina, AggKind::Additive), v.2 as i64);
                            },
                            _ => {},
                        }
                    }
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
            let contribution = 1 + piece.is_two_handed_weapon() as u8;
            if let Some(set_id) = piece.set_id {
                *sets.entry(set_id).or_insert(0) += contribution;
                if let Some(unperfected) = PERFECTED_TO_SET.get(&set_id) {
                    *sets.entry(*unperfected).or_insert(0) += contribution;
                }
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