use std::collections::HashMap;

use eso_skill_data::enums::{mechanic::Mechanic::{Stamina, Magicka}, skill_line::SkillLine};
use esosim_types::{AggKind::{self, Multiplicative}, DamageDoneContext, channel::StatusEffectContext};

use crate::{data::{enums::{cca::CoreCombatAbility::{self, Bash, Block, DodgeRoll, Sprint}, damage::ResistableDamageType::{self, All as ResistableDamageTypeAll, *}, gear::{EnchantType, GearSlot, GearTrait, ItemType}}, tables::{enchant::get_armour_enchant_multiplier, power::OFFHAND_MULTIPLIER, sets::SETS}}, engine::{active_effects::ActiveEffects, aggregator::ChannelSet, effect::{AggKind::Additive, NoContext, Channel::{self, *}, DamageDoneContext::{Bash as BashDamage, *}, ResourceKind::{Health, Magicka as Mag, Stamina as Stam}}, grant_rule::GrantRule, registry::EffectRegistry}, entity::{gear::{GearPiece, Loadout, PrismaticValueWrapper}, player::ActiveBar::*}};

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
    pub attributes: (u8, u8, u8),
    /// The stones picked, in pick order (the first is the one that survives without Twice-Born Star).
    /// Kept here, not only in `active_effects`, so a bar swap that loses the second stone's slot and
    /// then gets it back re-applies the same stone (`recompute` gates by `mundus_slots`).
    pub mundus_pick: esosim_types::MundusLoadout,
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
            attributes: (0, 0, 0),
            mundus_pick: esosim_types::MundusLoadout::default(),
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
        // Only as many stones as the gear on the active bar has room for, in pick order. The picks
        // themselves are never forgotten, so swapping to a bar with fewer than 5 Twice-Born Star pieces
        // switches the second stone off and swapping back switches the same stone on again.
        let slots = self.mundus_slots();
        let stones = self.mundus_pick.distinct().into_iter().filter(|id| {
            registry.get(*id).is_some_and(|d| matches!(d.source, crate::engine::effect::EffectSource::Mundus { .. }))
        });
        for id in stones.skip(slots) {
            active.set(id, 0);
        }
        for rule in grant_rules {
            if let Some(stacks) = rule.evaluate(self) {
                active.set(rule.effect_id, stacks);
            }
        }
        // Gear and sets first: their `MundusBoost` (Divines) scales the mundus stones, which are
        // worth `value * (1 + boost)` (roadmap 2.3), so the boost has to be known before the effects
        // are added. `aggregate_with_boost` leaves the boost out of its own `MundusBoost`, so it is
        // counted once.
        let mut channels = ChannelSet::new();
        self.apply_gear(&mut channels);
        self.apply_set_bonuses(&mut channels);
        let mundus_boost_bps = channels.get_multiplicative_bps(MundusBoost);
        channels.merge(&active.aggregate_with_boost(registry, &NoContext, mundus_boost_bps));
        self.channels = channels;
    }

    pub fn apply_gear(&self, channels: &mut ChannelSet) { // todo add tests
        let mut divines_seen = 0u32;
        for (slot, gear) in self.get_active_gear_with_slots() {
            let power = gear.get_weapon_power();
            if power > 0f32 {
                let m = 1.0 + match () {
                    _ if !slot.is_offhand() => 0.0,
                    _ if self.has_buff(45477) => OFFHAND_MULTIPLIER + 0.06,
                    _ if self.has_buff(30873) => OFFHAND_MULTIPLIER + 0.03,
                    _ => OFFHAND_MULTIPLIER,
                };
                let p = (power * m).round();
                channels.add_additive(Power(DamageDoneContext::Type(ResistableDamageTypeAll), Additive), p);
            } 
            channels.add_additive(Armour(ResistableDamageTypeAll), gear.get_armour_value(&slot));
            if let Some(gear_trait) = gear.get_item_trait() {
                let wrapper = gear.get_trait_value();
                match wrapper {
                    PrismaticValueWrapper::F32(v) => {
                        match gear_trait {
                            GearTrait::JewelryBloodthirsty => {},
                            GearTrait::JewelryHarmony => {
                                channels.add_additive(SynergyRestore(Health, Additive), v);
                                channels.add_additive(SynergyRestore(Mag, Additive), v);
                                channels.add_additive(SynergyRestore(Stam, Additive), v);
                            },
                            GearTrait::JewelryProtective => {}, // handle in armour code
                            GearTrait::JewelrySwift => {channels.add_additive(MovementSpeed, v);},
                            GearTrait::JewelryTriune => {}, // handle below
                            GearTrait::JewelryInfused => {}, // handle in enchantment code
                            GearTrait::JewelryArcane => {channels.add_additive(Resource(Mag, Additive), v)},
                            GearTrait::JewelryRobust => {channels.add_additive(Resource(Stam, Additive), v)},
                            GearTrait::JewelryHealthy => {channels.add_additive(Resource(Health, Additive), v)},

                            GearTrait::ArmorSturdy => {channels.add_multiplicative_bps(CostReductionCCA(Block, Multiplicative), (v * 10_000.0).round());},
                            GearTrait::ArmorImpenetrable => {channels.add_additive(CriticalResistance, v);},
                            GearTrait::ArmorReinforced => {}, // handle in armour code
                            GearTrait::ArmorWellFitted => {
                                let bps = (v * 10_000.0).round();
                                channels.add_multiplicative_bps(CostReductionCCA(DodgeRoll, Multiplicative), bps);
                                channels.add_multiplicative_bps(CostReductionCCA(Sprint, Multiplicative), bps);
                            },
                            GearTrait::ArmorDivines => {
                                // The character sheet counts the second Divines piece at a whole percent
                                // (9.1 % shows as 9.0 %), the others exactly: eight Legendary pieces are
                                // +72.7 %, not 72.8 % (checked against the game's sheet, roadmap 2.3).
                                let exact = (v * 10_000.0).round();
                                let bps = if divines_seen == 1 { (v * 100.0).round() * 100.0 } else { exact };
                                divines_seen += 1;
                                channels.add_multiplicative_bps(MundusBoost, bps);
                            },
                            GearTrait::ArmorNirnhoned => {}, // handle in armour code
                            GearTrait::ArmorInfused => {}, // handle in enchantment code
                            GearTrait::ArmorTraining => {channels.add_multiplicative_bps(KillExperience, (v * 10_000.0).round());},
                            GearTrait::ArmorInvigorating => {
                                {channels.add_additive(Recovery(Mag, Additive), v)};
                                {channels.add_additive(Recovery(Stam, Additive), v)};
                                {channels.add_additive(Recovery(Health, Additive), v)};
                            },

                            GearTrait::WeaponInfused => {}, // handled in enchantment code
                            GearTrait::WeaponNirnhoned => {}, // handled in weapon code
                            GearTrait::WeaponCharged => {channels.add_multiplicative_bps(StatusEffectChance(StatusEffectContext::All), (v * 10_000.0).round());},
                            GearTrait::WeaponDecisive => {channels.add_multiplicative_bps(DecisiveUltimateChance, (v * 10_000.0).round());},
                            GearTrait::WeaponDefending => {}, // handle in armour code
                            GearTrait::WeaponPowered => {channels.add_multiplicative_bps(HealingDone, (v * 10_000.0).round());},
                            GearTrait::WeaponPrecise => {channels.add_additive(CriticalChance, v)},
                            GearTrait::WeaponSharpened => {channels.add_additive(Penetration(ResistableDamageTypeAll), v)},
                            GearTrait::WeaponTraining => {channels.add_multiplicative_bps(KillExperience, (v * 10_000.0).round());},
                            _ => {},
                        }
                    }
                    PrismaticValueWrapper::Prismatic(v) => {
                        match gear_trait {
                            GearTrait::JewelryTriune => {
                                channels.add_additive(Resource(Health, Additive), v.0);
                                channels.add_additive(Resource(Mag, Additive), v.1);
                                channels.add_additive(Resource(Stam, Additive), v.2);
                            },
                            _ => {},
                        }
                    }
                }
            }
            if let Some(e) = gear.get_enchant() {
                let wrapper = gear.get_enchant_value().expect("An enchant should always have a value");
                let infused_multiplier = gear.get_infused_value();
                let slot_multiplier = get_armour_enchant_multiplier(&slot);
                match wrapper {
                    PrismaticValueWrapper::F32(v_) => {
                        let v = v_ * infused_multiplier * slot_multiplier;
                        match e.glyph {
                            EnchantType::DiseaseResistance => {channels.add_additive(Armour(Disease), v)},
                            EnchantType::FlameResistance => {channels.add_additive(Armour(Flame), v)},
                            EnchantType::FrostResistance => {channels.add_additive(Armour(Frost), v)},
                            EnchantType::Health => {channels.add_additive(Resource(Health, Additive), v)},
                            EnchantType::HealthRegen => {channels.add_additive(Recovery(Health, Additive), v)},
                            EnchantType::IncreaseBashDamage => {channels.add_additive(DamageDone(BashDamage, Additive), v);},
                            EnchantType::IncreasePhysicalDamage => {channels.add_additive(Power(DamageDoneContext::Type(ResistableDamageTypeAll), AggKind::Additive), v); channels.add_additive(Recovery(Stam, Additive), 10. * infused_multiplier * slot_multiplier)},
                            EnchantType::IncreasePotionEffectiveness => {channels.add_additive(PotionEffectiveness, v);},
                            EnchantType::IncreaseSpellDamage => {channels.add_additive(Power(DamageDoneContext::Type(ResistableDamageTypeAll), AggKind::Additive), v); channels.add_additive(Recovery(Mag, Additive), 10. * infused_multiplier * slot_multiplier)},
                            EnchantType::Magicka => {channels.add_additive(Resource(Mag, Additive), v)},
                            EnchantType::MagickaRegen => {channels.add_additive(Recovery(Mag, Additive), v)},
                            EnchantType::PhysicalResistance => {channels.add_additive(Armour(Martial), v)},
                            EnchantType::PoisonResistance => {channels.add_additive(Armour(Poison), v)},
                            EnchantType::PrismaticDefense => {}, // handle below
                            EnchantType::PrismaticRecovery => {}, // handle below
                            EnchantType::ReduceBlockCost => {channels.add_additive(CostReductionCCA(Block, Additive), v);},
                            EnchantType::ReduceFeatCost => {channels.add_additive(CostReduction(Stamina), v);},
                            EnchantType::ReducePotionCooldown => {channels.add_additive(PotionCooldown, v);},
                            EnchantType::ReduceSpellCost => {channels.add_additive(CostReduction(Magicka), v);},
                            EnchantType::ShockResistance => {channels.add_additive(Armour(Shock), v)},
                            EnchantType::SpellResistance => {channels.add_additive(Armour(Spell), v)},
                            EnchantType::Stamina => {channels.add_additive(Resource(Stam, Additive), v)},
                            EnchantType::StaminaRegen => {channels.add_additive(Recovery(Stam, Additive), v)},
                            _ => {},
                        }
                    },
                    PrismaticValueWrapper::Prismatic(v_) => {
                        let v = (v_.0 * infused_multiplier * slot_multiplier, v_.1 * infused_multiplier * slot_multiplier, v_.2 * infused_multiplier * slot_multiplier);
                        match e.glyph {
                            EnchantType::PrismaticDefense => {
                                channels.add_additive(Resource(Health, Additive), v.0);
                                channels.add_additive(Resource(Mag, Additive), v.1);
                                channels.add_additive(Resource(Stam, Additive), v.2);
                            },
                            EnchantType::PrismaticRecovery => {
                                channels.add_additive(Recovery(Health, Additive), v.0);
                                channels.add_additive(Recovery(Mag, Additive), v.1);
                                channels.add_additive(Recovery(Stam, Additive), v.2);
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
            let Some(set) = SETS.get(set_id) else { continue };
            for row in set.bonuses.iter().take(count as usize) {
                for bonus in row.iter() {
                    // `None`: a stat the compiler has no channel for yet (see sets.jsonl)
                    if let Some(channel) = bonus.channel {
                        channels.add_additive(channel, bonus.value);
                    }
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
                if let Some(unperfected) = SETS.unperfected_of(set_id) {
                    *sets.entry(unperfected).or_insert(0) += contribution;
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

    pub fn set_attributes(&mut self, magicka: u8, health: u8, stamina: u8) {
        debug_assert!(magicka + health + stamina <= 64);
        self.attributes = (magicka, health, stamina);
    }
}