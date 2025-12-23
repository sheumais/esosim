use std::collections::HashMap;

use esosim_models::player::{ActiveBar, GearPiece, GearSlot, Player as PlayerModel};

use crate::{ID, STACKS, critical::CriticalDamage, event::{Context, Event, SetInstance}, sets::SET_REGISTRY};

pub struct CharacterContext<'a> {
    player: &'a mut PlayerModel,
}

pub struct Character {
    player: PlayerModel,
    active_sets: HashMap<u16, Box<dyn SetInstance>>,
    critical_damage_done: CriticalDamage,
}

impl Character {
    pub fn new(id: u32) -> Self {
        Self {
            player: PlayerModel::new(id),
            active_sets: HashMap::new(),
            critical_damage_done: CriticalDamage::new(),
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
    use esosim_models::player::{GearPiece, GearSlot, GearTrait};
    use esosim_data::{critical_damage::*, item_type::ItemQuality, major_minor::{FORCE_MAJOR_ID, FORCE_MINOR_ID}};

    #[test]
    fn full_character_with_many_buffs_and_items_calculates_critical_damage() {
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
                gear_trait: Some(GearTrait::Nirnhoned),
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
                gear_trait: Some(GearTrait::Precise),
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
                gear_trait: Some(GearTrait::Bloodthirsty),
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
}