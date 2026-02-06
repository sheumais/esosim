use std::collections::HashMap;

use crate::engine::world::event::SetDescriptor;
use phf::phf_map;

mod velothi;

pub static SET_REGISTRY_MAP: phf::Map<u16, &'static SetDescriptor> = phf_map! {
    694u16 => &velothi::VELOTHI_DESCRIPTOR,
};
