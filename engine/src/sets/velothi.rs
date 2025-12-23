use esosim_data::major_minor::FORCE_MINOR_ID;

use crate::{ID, event::{Context, Event, SetDescriptor, SetInstance}};

pub struct VelothiInstance;

impl VelothiInstance {
    pub fn new() -> Self { VelothiInstance }
}

impl SetInstance for VelothiInstance {
    fn on_activate(&mut self, player: &ID, ctx: &mut dyn Context) {
        ctx.add_player_buff(player, FORCE_MINOR_ID, 1);
    }

    fn on_deactivate(&mut self, player: &ID, ctx: &mut dyn Context) {
        ctx.remove_player_buff(player, &FORCE_MINOR_ID);
    }

    fn on_event(&mut self, _player: &ID, _event: &Event, _ctx: &mut dyn Context) {
        //
    }
}

pub static VELOTHI_DESCRIPTOR: SetDescriptor = SetDescriptor {
    id: 694,
    min_pieces: 1,
    priority: 50,
    instance_factory: || Box::new(VelothiInstance::new()),
};