pub mod item_type;
pub mod power;
pub mod critical_damage;
pub mod major_minor;
pub mod skill;
pub mod traits;
pub mod armour;
pub mod resource;
pub mod enchant;

#[derive(PartialEq, Debug)]
pub struct StatBuff {
    pub id: u32,
    pub value: f64,
    pub value_per_stack: f64,
}