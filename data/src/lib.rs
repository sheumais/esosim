pub mod item_type;
pub mod power;
pub mod critical_damage;

#[derive(Eq, PartialEq, Hash, Debug)]
pub struct StatBuff {
    pub id: u32,
    pub value: i32,
    pub value_per_stack: i32,
}