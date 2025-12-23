use crate::event::SetDescriptor;

mod velothi;

pub static SET_REGISTRY: &[&SetDescriptor] = &[
    &velothi::VELOTHI_DESCRIPTOR,
];