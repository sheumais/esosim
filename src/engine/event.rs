use crate::engine::{ID, STACKS};

#[derive(Debug)]
pub enum Event<'a> {
    EquipChanged,
    Tick { delta_ms: u64 },
    SkillUsed { user: &'a ID, skill_id: u32 },
    PlayerUpdated,
    BarSwapped,
    BuffGained,
    BuffFaded,
    BuffStacksUpdated,
    ExternalResourceSource { health: u32, magicka: u32, stamina: u32 },
}

pub trait Context {
    fn player_has_item(&self, player_id: &ID, item_id: &u32) -> bool;
    fn player_set_piece_count(&self, player_id: &ID, set_id: &u16) -> u8;
    fn add_player_buff(&mut self, player_id: &ID, buff_id: ID, stacks: STACKS);
    fn remove_player_buff(&mut self, player_id: &ID, buff_id: &ID);
}

#[derive(Clone)]
pub struct SetDescriptor {
    pub id: u16,
    pub min_pieces: u8,
    pub instance_factory: fn() -> Box<dyn SetInstance>,
    pub priority: u16,
}

pub trait SetInstance: Send {
    fn on_activate(&mut self, player: &ID, ctx: &mut dyn Context);

    fn on_deactivate(&mut self, player: &ID, ctx: &mut dyn Context);

    fn on_event(&mut self, player: &ID, event: &Event, ctx: &mut dyn Context);
}