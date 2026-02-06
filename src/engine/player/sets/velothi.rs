use crate::data::major_minor::FORCE_MINOR_ID;

use crate::engine::ID;
use crate::engine::world::event::{Event, SetDescriptor, SetInstance, World};

pub struct VelothiInstance;

impl VelothiInstance {
    pub fn new() -> Self { VelothiInstance }
}

impl SetInstance for VelothiInstance {
    fn on_activate(&mut self, owner: ID, world: &mut dyn World) {
        world.add_buff(owner, FORCE_MINOR_ID, 1);
        world.add_buff(owner, 193447, 1);
    }

    fn on_deactivate(&mut self, owner: ID, world: &mut dyn World) {
        world.remove_buff(owner, FORCE_MINOR_ID);
        world.remove_buff(owner, 193447);
    }

    fn on_event(&mut self, _owner: ID, _event: &Event, _world: &mut dyn World) {
        //
    }
}

pub static VELOTHI_DESCRIPTOR: SetDescriptor = SetDescriptor {
    id: 694,
    min_pieces: 1,
    priority: 50,
    instance_factory: || Box::new(VelothiInstance::new()),
};