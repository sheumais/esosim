use std::collections::HashMap;

use esosim_data::item_type::GearSlot;
use esosim_models::{player::{ActiveBar, GearPiece, Player as PlayerModel}};

use crate::{ID, STACKS, critical::CriticalDamage, event::{Context, Event, SetInstance}, power::Power, sets::SET_REGISTRY};

pub struct CharacterContext<'a> {
    player: &'a mut PlayerModel,
}

pub struct Character {
    player: PlayerModel,
    active_sets: HashMap<u16, Box<dyn SetInstance>>,
    critical_damage_done: CriticalDamage,
    power: Power,
}

impl Character {
    pub fn new(id: u32) -> Self {
        Self {
            player: PlayerModel::new(id),
            active_sets: HashMap::new(),
            critical_damage_done: CriticalDamage::new(),
            power: Power::new(),
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        self.emit_event_to_sets(&event);

        match event {
            Event::EquipChanged | Event::PlayerUpdated | Event::BarSwapped | Event::BuffFaded | Event::BuffGained | Event::BuffStacksUpdated => {
                self.evaluate_sets();
                self.recompute_all_supplemental_state();
            }
            _ => {}
        }
    }

    pub fn add_buff(&mut self, id: ID, stacks: STACKS) {
        self.player.add_buff(id, stacks);
        self.handle_event(Event::BuffGained);
    }

    pub fn remove_buff(&mut self, id: ID) {
        self.player.remove_buff(&id);
        self.handle_event(Event::BuffFaded);
    }

    pub fn get_critical_damage_done(&mut self) -> u8 {
        self.critical_damage_done.calculate()
    }

    pub fn get_power(&mut self) -> u32 {
        self.power.calculate()
    }

    pub fn swap_bars(&mut self, choice: Option<ActiveBar>) {
        self.player.swap_bars(choice);
        self.handle_event(Event::BarSwapped);
    }

    pub fn set_gear_piece(&mut self, slot: &GearSlot, gear: GearPiece) {
        self.player.set_gear_piece(slot, gear);
        self.handle_event(Event::EquipChanged);
    }

    pub fn set_skills_on_bar(&mut self, bar: ActiveBar, skills: Vec<u32>) {
        self.player.set_skills(&bar, skills);
        self.handle_event(Event::EquipChanged);
    }

    pub fn get_bar_of_skill_id(&self, skill: &ID) -> Option<ActiveBar> {
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

    pub fn emit_event_to_sets(&mut self, event: &Event) {
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
    use super::*;
    use esosim_models::player::{GearEnchant, GearPiece};
    use esosim_data::{critical_damage::*, item_type::{EnchantType, GearTrait, ItemQuality}, major_minor::{COURAGE_MAJOR_ID, COURAGE_MINOR_ID, FORCE_MAJOR_ID, FORCE_MINOR_ID, SORCERY_MAJOR_ID}, skill::HARNESSED_QUINTESSENCE_ID};

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

        // let mut power = Power::new();
        // power.additive = ((0.1765 + 0.06) as f32 * 1335.0).round() as u32 + 1535;
        // power.multiplicative = 0.03; // one fighters guild skill
        // assert_eq!(power.calculate(), 2937);
        // power.multiplicative += 0.12; // 6 medium pieces
        // assert_eq!(power.calculate(), 3279);
        // power.additive += 174 * 3; // jewellery with weapon damage enchants
        // power.additive += 129 * 2; // 2x set lines
        // assert_eq!(power.calculate(), 4176);
        // power.multiplicative += 0.2; // major brutality
        // assert_eq!(power.calculate(), 4902);

        character.add_buff(COURAGE_MINOR_ID, 1);
        character.add_buff(COURAGE_MAJOR_ID, 1);
        character.add_buff(SORCERY_MAJOR_ID, 1);
        character.add_buff(HARNESSED_QUINTESSENCE_ID, 1);

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
                enchant: Some(GearEnchant {
                    glyph: EnchantType::IncreasePhysicalDamage,
                    effective_level: 66,
                    quality: ItemQuality::Legendary,
                }),
            },
        );

        let power = character.get_power();

        assert!(power == 4307, "power incorrect (is {})", power);
    }
}