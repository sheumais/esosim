use std::collections::HashMap;

use crate::engine::{ID, STACKS, player::{character::Character, sets::SET_REGISTRY_MAP}};

pub type UnitId = ID;

pub trait World {
    fn character(&self, id: UnitId) -> &Character;
    fn character_mut(&mut self, id: UnitId) -> &mut Character;

    fn add_buff(&mut self, target: UnitId, buff: ID, stacks: STACKS);
    fn remove_buff(&mut self, target: UnitId, buff: ID);
}


#[derive(Debug)]
pub enum Event {
    Tick { delta_ms: u64 },

    SkillUsed {
        caster: UnitId,
        target: UnitId,
        skill_id: u32,
    },

    BuffGained {
        source: Option<UnitId>,
        target: UnitId,
        buff_id: ID,
        stacks: STACKS,
    },

    BuffFaded {
        target: UnitId,
        buff_id: ID,
    },

    EquipChanged {
        player: UnitId,
    },
    PlayerUpdated {
        player: UnitId,
    },
    BarSwapped {
        player: UnitId,
    },
}

pub struct SetDescriptor {
    pub id: u16,
    pub min_pieces: u8,
    pub instance_factory: fn() -> Box<dyn SetInstance>,
    pub priority: u16,
}

pub trait SetInstance: Send {
    fn on_activate(&mut self, owner: UnitId, world: &mut dyn World);
    fn on_deactivate(&mut self, owner: UnitId, world: &mut dyn World);

    fn on_event(
        &mut self,
        owner: UnitId,
        event: &Event,
        world: &mut dyn World,
    );
}

pub struct SetManager {
    active_sets: HashMap<UnitId, HashMap<u16, Box<dyn SetInstance>>>,
}

pub struct GameState {
    players: HashMap<UnitId, Character>,
    set_manager: SetManager,
}

impl World for GameState {
    fn character(&self, id: UnitId) -> &Character {
        &self.players[&id]
    }

    fn character_mut(&mut self, id: UnitId) -> &mut Character {
        self.players.get_mut(&id).expect("invalid UnitId")
    }

    fn add_buff(&mut self, target: UnitId, buff_id: ID, stacks: STACKS) {
        let c = self.character_mut(target);
        c.add_buff(buff_id, stacks);
        c.recompute_buff_supplemental_state();
    }

    fn remove_buff(&mut self, target: UnitId, buff_id: ID) {
        let c = self.character_mut(target);
        c.remove_buff(buff_id);
        c.recompute_buff_supplemental_state();
    }
}


impl GameState {
    pub fn evaluate_sets_for_player(&mut self, player: UnitId) {
        let active_sets = self
            .set_manager
            .active_sets
            .entry(player)
            .or_default();

        for (set_id, reference) in &SET_REGISTRY_MAP {
            let pieces = self.character(player).get_set_piece_count(&reference.id);
            let active = active_sets.contains_key(&reference.id);

            if pieces >= reference.min_pieces && !active {
                let mut inst = (reference.instance_factory)();
                inst.on_activate(player, self);
                active_sets.insert(reference.id, inst);
            } else if pieces < reference.min_pieces && active {
                let mut inst = active_sets.remove(&reference.id).unwrap();
                inst.on_deactivate(player, self);
            }
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::SkillUsed { caster, target, skill_id } => {
                self.resolve_skill(caster, target, skill_id);
            }

            Event::EquipChanged { player }
            | Event::PlayerUpdated { player }
            | Event::BarSwapped { player } => {
                self.evaluate_sets_for_player(player);
                self.character_mut(player).recompute_all_supplemental_state();
            }

            Event::BuffGained { target, buff_id, stacks, .. } => {
                self.add_buff(target, buff_id, stacks);
            }

            Event::BuffFaded { target, buff_id } => {
                self.remove_buff(target, buff_id);
            }

            Event::Tick { .. } => {}
        }

        self.emit_event_to_sets(&event);
    }

    fn resolve_skill(&mut self, caster: UnitId, target: UnitId, skill_id: u32) {
        //
    }

}
