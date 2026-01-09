mod event;
mod critical;
pub mod character;
mod sets;
mod power;
mod armour;
mod resource;

pub use event::Event::ExternalResourceSource;

type ID = u32;
type STACKS = u8;