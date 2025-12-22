use esosim_models::player::{ActiveBar, GearPiece, GearSlot, Player as PlayerModel};

use crate::{ID, STACKS, critical::{self, CriticalDamage}};

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
        self.critical_damage_done.update_from_player(&self.player);
    }

    pub fn remove_buff(&mut self, id: ID) {
        self.player.remove_buff(&id);
        self.critical_damage_done.update_from_player(&self.player);
    }

    pub fn get_critical_damage_done(&mut self) -> u8 {
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
}