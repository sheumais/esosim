use std::collections::HashMap;

use crate::data::item_type::GearSlot;
use crate::engine::sets::SET_REGISTRY;
use crate::engine::{ID, STACKS};
use crate::engine::armour::{Armour, Penetration};
use crate::engine::critical::{CriticalDamage, CriticalDamageTaken, CriticalChance};
use crate::engine::event::{Context, Event, SetInstance};
use crate::engine::power::Power;
use crate::engine::resource::Resources as ResourceModel;
use crate::models::damage::DamageType;
use crate::models::player::{ActiveBar, GearPiece, Player as PlayerModel};

pub struct CharacterContext<'a> {
    player: &'a mut PlayerModel,
}

pub struct Character {
    player: PlayerModel,
    active_sets: HashMap<u16, Box<dyn SetInstance>>,
    critical_damage_done: CriticalDamage,
    power: Power,
    armour: Armour,
    critical_damage_taken: CriticalDamageTaken,
    resources: ResourceModel,
    critical_chance: CriticalChance,
    penetration: Penetration,
}

impl Character {
    pub fn new(id: u32) -> Self {
        Self {
            player: PlayerModel::new(id),
            active_sets: HashMap::new(),
            critical_damage_done: CriticalDamage::new(),
            power: Power::new(),
            armour: Armour::new(),
            critical_damage_taken: CriticalDamageTaken::new(),
            resources: ResourceModel::new(),
            critical_chance: CriticalChance::new(),
            penetration: Penetration::new(),
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        self.emit_event_to_sets(&event);

        match event {
            Event::EquipChanged | Event::PlayerUpdated | Event::BarSwapped => {
                self.evaluate_sets();
                self.recompute_all_supplemental_state();
            }
            Event::BuffFaded | Event::BuffGained | Event::BuffStacksUpdated => {
                self.recompute_buff_supplemental_state();
            }
            // Event::ExternalResourceSource { health, magicka, stamina } => {
            //     self.handle_external_resource_source(health, magicka, stamina);
            // }
            _ => {}
        }
    }

    pub fn add_buff(&mut self, id: ID, stacks: STACKS) {
        self.player.add_buff(id, stacks);
        self.armour.add_source_checked(id, Some(stacks));
        self.critical_damage_done.add_source_checked(id, Some(stacks));
        self.critical_damage_taken.add_source_checked(id, Some(stacks));
        self.power.add_source_checked(id, Some(stacks));
        self.resources.add_source_checked(id, Some(stacks));
        self.critical_chance.add_source_checked(id, Some(stacks));
        self.penetration.add_source_checked(id, Some(stacks));
        self.handle_event(Event::BuffGained);
    }

    pub fn remove_buff(&mut self, id: ID) {
        self.player.remove_buff(&id);
        self.armour.remove_source(&id);
        self.critical_damage_done.remove_source(&id);
        self.critical_damage_taken.remove_source(&id);
        self.power.remove_source(&id);
        self.resources.remove_source(&id);
        self.critical_chance.remove_source(&id);
        self.penetration.remove_source(&id);
        self.handle_event(Event::BuffFaded);
    }

    pub fn get_critical_damage_done(&self) -> u16 {
        self.critical_damage_done.calculate()
    }

    pub fn get_critical_damage_taken(&self) -> u8 {
        self.critical_damage_taken.calculate()
    }

    pub fn get_critical_damage_uncapped(&self) -> u16 {
        self.critical_damage_done.calculate_uncapped()
    }

    pub fn get_power(&mut self) -> u32 {
        self.power.calculate()
    }

    pub fn get_armour(&self, damage_type: &DamageType) -> u32 {
        self.armour.calculate(damage_type)
    }

    pub fn get_max_health(&self) -> u32 {
        self.resources.get_max_health()
    }

    pub fn get_max_magicka(&self) -> u32 {
        self.resources.get_max_magicka()
    }

    pub fn get_max_stamina(&self) -> u32 {
        self.resources.get_max_stamina()
    }

    pub fn get_critical_chance(&self) -> f32 {
        self.critical_chance.calculate()
    }

    pub fn get_critical_chance_raw(&self) -> u32 {
        self.critical_chance.get_raw()
    }

    pub fn get_penetration(&self) -> u32 {
        self.penetration.calculate()
    }

    pub fn swap_bars(&mut self, choice: Option<&ActiveBar>) {
        self.player.swap_bars(choice);
        self.handle_event(Event::BarSwapped);
    }

    pub fn set_gear_piece(&mut self, slot: &GearSlot, gear: GearPiece) {
        self.player.set_gear_piece(slot, gear);
        self.handle_event(Event::EquipChanged);
    }

    pub fn set_skills_on_bar(&mut self, bar: &ActiveBar, skills: Vec<u32>) {
        self.player.set_skills(&bar, skills);
        self.handle_event(Event::EquipChanged);
    }

    pub fn get_bar_of_skill_id(&self, skill: &ID) -> Option<&ActiveBar> {
        self.player.get_bar_of_skill_id(skill)
    }

    pub fn get_active_bar(&self) -> &ActiveBar {
        self.player.get_active_bar()
    }

    pub fn evaluate_sets(&mut self) {
        let player_id = *self.player.id();

        let mut ctx = CharacterContext {
            player: &mut self.player,
        };

        for desc in SET_REGISTRY {
            let pieces = ctx.player_set_piece_count(&player_id, &desc.id);
            let active = self.active_sets.contains_key(&desc.id);

            if pieces >= desc.min_pieces && !active {
                let mut instance = (desc.instance_factory)();
                instance.on_activate(&player_id, &mut ctx);
                self.active_sets.insert(desc.id, instance);
            } else if pieces < desc.min_pieces && active {
                if let Some(mut inst) = self.active_sets.remove(&desc.id) {
                    inst.on_deactivate(&player_id, &mut ctx);
                }
            }
        }
        self.recompute_all_supplemental_state();
    }

    fn emit_event_to_sets(&mut self, event: &Event) {
        let player_id = *self.player.id();

        let mut ctx = CharacterContext {
            player: &mut self.player,
        };

        for inst in self.active_sets.values_mut() {
            inst.on_event(&player_id, event, &mut ctx);
        }
    }

    fn recompute_all_supplemental_state(&mut self) {
        self.critical_damage_done.update_from_player(&self.player);
        self.power.update_from_player(&self.player);
        self.armour.update_from_player(&self.player);
        self.critical_damage_taken.update_from_player(&self.player);
        self.resources.update_from_player(&self.player);
        self.critical_chance.update_from_player(&self.player);
        self.penetration.update_from_player(&self.player);
    }

    fn recompute_buff_supplemental_state(&mut self) {
        if self.critical_damage_done.is_dirty {self.critical_damage_done.refresh()};
        if self.power.is_dirty {self.power.refresh()};
        // self.armour.refresh(); todo
        if self.armour.is_dirty {self.armour.update_from_player(&self.player)};
        if self.critical_damage_done.is_dirty {self.critical_damage_done.refresh()};
        if self.critical_damage_taken.is_dirty {self.critical_damage_taken.refresh()};
        if self.resources.is_dirty {self.resources.refresh()};
        if self.critical_chance.is_dirty {self.critical_chance.refresh()};
        if self.penetration.is_dirty {self.penetration.refresh()};
    }

    // fn handle_external_resource_source(&mut self, health: u32, magicka: u32, stamina: u32) {
    //     if let Some((h, m, s)) = self.resources.calculate_attributes((health, magicka, stamina)) {
    //         self.set_attributes(h, m, s);
    //     }
    // }

    pub fn set_attributes(&mut self, health: u8, magicka: u8, stamina: u8) {
        self.player.set_attributes((health, magicka, stamina));
    }
}

impl<'a> Context for CharacterContext<'a> {
    fn player_set_piece_count(&self, _player_id: &ID, set_id: &u16) -> u8 {
        self.player.get_number_of_equipped_set(set_id)
    }

    fn add_player_buff(&mut self, _player_id: &ID, buff_id: ID, stacks: STACKS) {
        self.player.add_buff(buff_id, stacks);
    }

    fn remove_player_buff(&mut self, _player_id: &ID, buff_id: &ID) {
        self.player.remove_buff(buff_id);
    }

    fn player_has_item(&self, _player_id: &ID, item_id: &u32) -> bool {
        self.player.is_specific_item_equipped(item_id)
    }
}


#[cfg(test)]
mod character_integration_test {
    use crate::{data::{critical_damage::*, item_type::{EnchantType, GearTrait, ItemQuality}, major_minor::*, skill::{CAMOUFLAGED_HUNTER_ID, SOUL_SIPHON_ID, UNDAUNTED_METTLE_ID}}, models::player::GearEnchant};

    use super::*;

    #[test]
    fn critical_damage() {
        let mut character = Character::new(0);

        character.add_buff(FORCE_MINOR_ID, 1);
        character.add_buff(FORCE_MAJOR_ID, 1);
        character.add_buff(HEMORRHAGE_PASSIVE_ID, 1);
        character.add_buff(FELINE_AMBUSH_ID, 1);
        character.add_buff(FATED_FORTUNE_ID, 1);
        character.add_buff(LUCENT_ECHOES_ID, 1);

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

        let crit_damage = character.get_critical_damage_done();

        assert!(crit_damage == 125, "crit damage incorrect (is {}%)", crit_damage);

        character.remove_buff(FORCE_MAJOR_ID);
        character.remove_buff(LUCENT_ECHOES_ID);
        character.remove_buff(FATED_FORTUNE_ID);

        let crit_damage = character.get_critical_damage_done();

        assert!(crit_damage == 94, "crit damage incorrect (is {}%)", crit_damage);
    }

    #[test]
    fn power() {
        let mut character = Character::new(0);

        character.add_buff(SORCERY_MAJOR_ID, 1);

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
        character.set_gear_piece(
            &GearSlot::Ring1,
            GearPiece {
                item_id: 0,
                effective_level: 66,
                gear_trait: Some(GearTrait::JewelryBloodthirsty),
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: Some(GearEnchant {
                    glyph: EnchantType::IncreasePhysicalDamage,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            },
        );
        character.set_gear_piece(
            &GearSlot::Ring2,
            GearPiece {
                item_id: 0,
                effective_level: 66,
                gear_trait: Some(GearTrait::JewelryBloodthirsty),
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: Some(GearEnchant {
                    glyph: EnchantType::IncreasePhysicalDamage,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            },
        );
        character.set_gear_piece(
            &GearSlot::Necklace,
            GearPiece {
                item_id: 194512,
                effective_level: 66,
                gear_trait: Some(GearTrait::JewelryBloodthirsty),
                quality: ItemQuality::Legendary,
                set_id: Some(694),
                enchant: Some(GearEnchant {
                    glyph: EnchantType::IncreasePhysicalDamage,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            },
        );
        character.set_gear_piece(
            &GearSlot::Waist,
            GearPiece { 
                item_id: 98304,
                effective_level: 66,
                gear_trait: None,
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            }
        );
        character.set_gear_piece(
            &GearSlot::Chest,
            GearPiece { 
                item_id: 98304,
                effective_level: 66,
                gear_trait: None,
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            }
        );
        character.set_gear_piece(
            &GearSlot::Shoulders,
            GearPiece { 
                item_id: 98304,
                effective_level: 66,
                gear_trait: None,
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            }
        );
        character.set_gear_piece(
            &GearSlot::Hands,
            GearPiece { 
                item_id: 98304,
                effective_level: 66,
                gear_trait: None,
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            }
        );
        character.set_gear_piece(
            &GearSlot::Legs,
            GearPiece { 
                item_id: 98304,
                effective_level: 66,
                gear_trait: None,
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            }
        );
        character.set_gear_piece(
            &GearSlot::Feet,
            GearPiece { 
                item_id: 98304,
                effective_level: 66,
                gear_trait: None,
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            }
        );

        character.set_skills_on_bar(&ActiveBar::Primary, vec![CAMOUFLAGED_HUNTER_ID]);

        let power = character.get_power();

        assert!(power == 4554, "power incorrect (is {})", power);
    }
    
    #[test]
    fn armour() {
        let mut character = Character::new(0);

        character.set_gear_piece( // 1823
            &GearSlot::Head,
            GearPiece {
                item_id: 178633,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorDivines),
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            }
        );
        character.set_gear_piece( // 3215
            &GearSlot::Chest,
            GearPiece {
                item_id: 207729,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorReinforced),
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            }
        );
        character.set_gear_piece( // 1221
            &GearSlot::Shoulders,
            GearPiece {
                item_id: 181695,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorDivines),
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            }
        );
        character.set_gear_piece( // 1386
            &GearSlot::Hands,
            GearPiece {
                item_id: 207742,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorDivines),
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            }
        );
        character.set_gear_piece( // 1039
            &GearSlot::Waist,
            GearPiece {
                item_id: 207775,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorDivines),
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            }
        );
        character.set_gear_piece( // 2813
            &GearSlot::Legs,
            GearPiece {
                item_id: 207761,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorReinforced),
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            }
        );
        character.set_gear_piece( // 2425
            &GearSlot::Feet,
            GearPiece {
                item_id: 207735,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorDivines),
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None,
            }
        );

        character.add_buff(45306, 1); // Nord passive

        character.set_gear_piece(
            &GearSlot::MainHand,
            GearPiece {
                item_id: 187552,
                effective_level: 66,
                gear_trait: Some(GearTrait::WeaponDecisive),
                quality: ItemQuality::Legendary,
                set_id: None,
                enchant: None
            }
        );

        let physical_resistance = character.get_armour(&DamageType::PHYSICAL);
        let spell_resistance = character.get_armour(&DamageType::MAGIC);

        assert!(physical_resistance == 21869, "physical resistance incorrect (is {})", physical_resistance);
        assert!(spell_resistance == 22595, "spell resistance incorrect (is {})", spell_resistance);

        let power = character.get_power();

        assert!(power == 2382, "power is incorrect (is {})", power);
    }

    #[test]
    fn max_magicka() {
        let mut character = Character::new(0);

        character.set_gear_piece(
            &GearSlot::Head,
            GearPiece {
                item_id: 147237,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorDivines),
                quality: ItemQuality::Legendary,
                set_id: Some(436),
                enchant: Some(GearEnchant {
                    glyph: EnchantType::Magicka,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            }
        );
        character.set_gear_piece( // |H1:item:111885:364:50:0:0:0:18:0:0:0:0:0:0:0:2049:29:0:1:0:6706:0|h|h
            &GearSlot::Chest,
            GearPiece {
                item_id: 111885,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorDivines),
                quality: ItemQuality::Legendary,
                set_id: Some(185),
                enchant: Some(GearEnchant {
                    glyph: EnchantType::Magicka,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            }
        );
        character.set_gear_piece( // |H1:item:147238:364:50:0:0:0:18:25:0:0:0:0:0:0:2049:67:0:1:0:8717:0|h|h
            &GearSlot::Shoulders,
            GearPiece {
                item_id: 147238,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorDivines),
                quality: ItemQuality::Legendary,
                set_id: Some(436),
                enchant: Some(GearEnchant {
                    glyph: EnchantType::Magicka,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            }
        );
        character.set_gear_piece( // |H1:item:111887:364:50:0:0:0:18:0:0:0:0:0:0:0:2049:29:0:1:0:4345:0|h|h
            &GearSlot::Hands,
            GearPiece {
                item_id: 111887,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorDivines),
                quality: ItemQuality::Legendary,
                set_id: Some(185),
                enchant: Some(GearEnchant {
                    glyph: EnchantType::Magicka,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            }
        );
        character.set_gear_piece( // |H1:item:111892:364:50:0:0:0:18:36:0:0:0:0:0:0:2049:29:0:1:0:6178:0|h|h
            &GearSlot::Waist,
            GearPiece {
                item_id: 111892,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorDivines),
                quality: ItemQuality::Legendary,
                set_id: Some(185),
                enchant: Some(GearEnchant {
                    glyph: EnchantType::Magicka,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            }
        );
        character.set_gear_piece( // |H1:item:111889:364:50:0:0:0:18:0:0:0:0:0:0:0:2049:29:0:1:0:2065:0|h|h
            &GearSlot::Legs,
            GearPiece {
                item_id: 111889,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorDivines),
                quality: ItemQuality::Legendary,
                set_id: Some(185),
                enchant: Some(GearEnchant {
                    glyph: EnchantType::Magicka,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            }
        );
        character.set_gear_piece( // |H1:item:111886:364:50:0:0:0:18:0:0:0:0:0:0:0:2049:29:0:1:0:5352:0|h|h
            &GearSlot::Feet,
            GearPiece {
                item_id: 111886,
                effective_level: 66,
                gear_trait: Some(GearTrait::ArmorDivines),
                quality: ItemQuality::Legendary,
                set_id: Some(185),
                enchant: Some(GearEnchant {
                    glyph: EnchantType::Magicka,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            }
        );
        character.set_gear_piece( // |H1:item:117088:364:50:45875:370:50:33:29:0:0:0:0:0:0:2049:23:0:1:0:0:0|h|h
            &GearSlot::Necklace,
            GearPiece {
                item_id: 117088,
                effective_level: 66,
                gear_trait: Some(GearTrait::JewelryInfused),
                quality: ItemQuality::Legendary,
                set_id: Some(180),
                enchant: Some(GearEnchant {
                    glyph: EnchantType::ReducePotionCooldown,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            }
        );
        character.set_gear_piece( // |H1:item:117087:364:50:45875:370:50:33:0:0:0:0:0:0:0:2049:23:0:1:0:0:0|h|h
            &GearSlot::Ring1,
            GearPiece {
                item_id: 117087,
                effective_level: 66,
                gear_trait: Some(GearTrait::JewelryInfused),
                quality: ItemQuality::Legendary,
                set_id: Some(180),
                enchant: Some(GearEnchant {
                    glyph: EnchantType::ReducePotionCooldown,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            }
        );
        character.set_gear_piece( // |H1:item:117087:364:50:45875:370:50:33:0:0:0:0:0:0:0:2049:23:0:1:0:0:0|h|h
            &GearSlot::Ring2,
            GearPiece {
                item_id: 117087,
                effective_level: 66,
                gear_trait: Some(GearTrait::JewelryInfused),
                quality: ItemQuality::Legendary,
                set_id: Some(180),
                enchant: Some(GearEnchant {
                    glyph: EnchantType::ReducePotionCooldown,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            }
        );
        character.set_gear_piece( // |H1:item:112009:364:50:0:0:0:0:0:0:0:0:0:0:0:1:29:0:1:0:369:0|h|h
            &GearSlot::MainHand,
            GearPiece {
                item_id: 112009,
                effective_level: 66,
                gear_trait: Some(GearTrait::WeaponDecisive),
                quality: ItemQuality::Legendary,
                set_id: Some(185),
                enchant: Some(GearEnchant {
                    glyph: EnchantType::AbsorbMagicka,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            }
        );

        character.add_buff(84720, 1); // ghastly eye bowl
        character.add_buff(UNDAUNTED_METTLE_ID, 0);
        character.add_buff(45260, 1); // gift of magnus (breton passive)
        character.add_buff(149305, 2); // max mag passive cp
        character.set_attributes(0, 64, 0);
        character.set_skills_on_bar(&ActiveBar::Primary, vec![40094, 40079, 40058, 85840, 42038, SOUL_SIPHON_ID]);
        character.recompute_all_supplemental_state();

        // (12000 + 111*64 + 4008 + 2000 + 4592 + 2192 + 520) * (1 + 6% + 2%)
        
        assert!(character.get_max_magicka() == 35009, "Max magicka incorrect (is {})", character.get_max_magicka())
    }
}