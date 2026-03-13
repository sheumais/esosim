use std::collections::HashMap;

use crate::{engine::{ID, STACKS, player::{character::Character, sets::SET_REGISTRY_MAP}}, models::player::ActiveBar};

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
        choice: Option<ActiveBar>
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

impl World for HashMap<UnitId, Character> {
    fn character(&self, id: UnitId) -> &Character {
        &self[&id]
    }

    fn character_mut(&mut self, id: UnitId) -> &mut Character {
        self.get_mut(&id).expect("invalid UnitId")
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
        let mut sets_to_activate = Vec::new();
        let mut sets_to_deactivate = Vec::new();

        {
            for (set_id, reference) in &SET_REGISTRY_MAP {
                let pieces = self.players.character(player).get_set_piece_count(&reference.id);
                let active_sets = self
                    .set_manager
                    .active_sets
                    .entry(player)
                    .or_default();
                let active = active_sets.contains_key(&reference.id);

                if pieces >= reference.min_pieces && !active {
                    sets_to_activate.push(*set_id);
                } else if pieces < reference.min_pieces && active {
                    sets_to_deactivate.push(*set_id);
                }
            }
        }

        for set_id in sets_to_activate {
            let reference = &SET_REGISTRY_MAP[&set_id];
            let mut inst = (reference.instance_factory)();
            inst.on_activate(player, &mut self.players);
            self.set_manager.active_sets.get_mut(&player).unwrap().insert(reference.id, inst);
        }

        for set_id in sets_to_deactivate {
            let reference = &SET_REGISTRY_MAP[&set_id];
            let mut inst = self.set_manager.active_sets.get_mut(&player).unwrap().remove(&reference.id).unwrap();
            inst.on_deactivate(player, &mut self.players);
        }
    }

    pub fn handle_event(&mut self, event: Event) {
        match event {
            Event::SkillUsed { caster, target, skill_id } => {
                // self.resolve_skill(caster, target, skill_id);
            }

            Event::EquipChanged { player }
            | Event::PlayerUpdated { player } => {
                self.evaluate_sets_for_player(player);
                self.players.character_mut(player).recompute_all_supplemental_state();
            }

            Event::BarSwapped { player, ref choice } => {
                self.players.character_mut(player).swap_bars(choice.as_ref());
            }


            Event::BuffGained { target, buff_id, stacks, .. } => {
                self.players.character_mut(target).add_buff(buff_id, stacks);
            }

            Event::BuffFaded { target, buff_id } => {
                self.players.character_mut(target).remove_buff(buff_id);
            }

            Event::Tick { .. } => {}
        }

        self.emit_event_to_sets(&event);
    }

    pub fn emit_event_to_sets(&mut self, event: &Event) {
        let players: Vec<UnitId> = self.set_manager.active_sets.keys().copied().collect();
        for player in players {
            let sets: Vec<u16> = self.set_manager.active_sets[&player].keys().copied().collect();
            for set_id in sets {
                if let Some(set_inst) = self.set_manager.active_sets.get_mut(&player).and_then(|m| m.get_mut(&set_id)) {
                    set_inst.on_event(player, event, &mut self.players);
                }
            }
        }
    }
}
