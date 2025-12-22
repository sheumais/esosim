use esosim_models::player::{ActiveBar, GearPiece, GearSlot, Player as PlayerModel};

use crate::{ID, STACKS, critical::CriticalDamage};

pub struct Character {
    player: PlayerModel,
    critical_damage_done: CriticalDamage,
}

impl Character {
    pub fn new() -> Self {
        Self {
            player: PlayerModel::new(),
            critical_damage_done: CriticalDamage::new(),
        }
    }

    pub fn add_buff(&mut self, id: ID, stacks: STACKS) {
        self.player.add_buff(id, stacks);
    }

    pub fn remove_buff(&mut self, id: ID) {
        self.player.remove_buff(&id);
    }

    pub fn get_critical_damage_done(&mut self) -> u8 {
        self.critical_damage_done.update_from_player(&self.player);
        self.critical_damage_done.calculate()
    }

    pub fn swap_bars(&mut self, choice: Option<ActiveBar>) {
        self.player.swap_bars(choice);
    }

    pub fn set_gear_piece(&mut self, slot: &GearSlot, gear: GearPiece) {
        self.player.set_gear_piece(slot, gear);
    }

    pub fn set_skills_on_bar(&mut self, bar: ActiveBar, skills: Vec<u32>) {
        self.player.set_skills(&bar, skills)
    }

    pub fn get_bar_of_skill_id(&self, skill: &ID) -> Option<ActiveBar> {
        self.player.get_bar_of_skill_id(skill)
    }

    pub fn get_active_bar(&self) -> &ActiveBar {
        self.player.get_active_bar()
    }
}

#[cfg(test)]
mod character_integration_test {
    use super::*;
    use esosim_models::player::{GearPiece, GearSlot, GearTrait};
    use esosim_data::{critical_damage::*, item_type::ItemQuality, major_minor::{FORCE_MAJOR_ID, FORCE_MINOR_ID}, sets::mythic::velothi::VELOTHI_UR_MAGES_AMULET_ITEM_ID};

    #[test]
    fn full_character_with_many_buffs_and_items_calculates_critical_damage() {
        let mut character = Character::new();

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
                item_id: VELOTHI_UR_MAGES_AMULET_ITEM_ID,
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

        assert!(crit_damage == 94, "crit damage incorrect (is {}%)", crit_damage); // fails crit damage is 125%
    }
}