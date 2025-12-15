use std::ops::{Add, AddAssign};

mod resource;
mod critical;
mod player;
mod damage_taken;
mod damage_done;
mod power;
mod monster;
mod damage;
mod fight;
mod buff;

const LEVEL: u8 = 50;
const EFFECTIVE_LEVEL: u8 = 66;
const CRITICAL_DAMAGE_DEFAULT: u32 = 50;
const CRITICAL_DAMAGE_MAXIMUM: f32 = 1.25;
const CRITICAL_CHANCE_DEFAULT: f32 = 0.1;
const ARMOUR_MAXIMUM: f32 = 0.5;