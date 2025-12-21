pub mod resource;
pub mod critical;
pub mod damage_taken;
pub mod damage_done;
pub mod power;
pub mod monster;
pub mod damage;
pub mod player;

const LEVEL: u8 = 50;
const EFFECTIVE_LEVEL: u8 = 66;
const CRITICAL_DAMAGE_DEFAULT: u8 = 50;
const CRITICAL_DAMAGE_MAXIMUM: u8 = 125;
const CRITICAL_CHANCE_DEFAULT: f32 = 0.1;
const ARMOUR_MAXIMUM: f32 = 0.5;