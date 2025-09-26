use std::ops::Add;

mod resource;
mod critical;
mod penetration;
mod player;
mod damage_done;
mod damage_taken;
mod power;

const LEVEL: u8 = 50;
const EFFECTIVE_LEVEL: u8 = 66;
const CRITICAL_DAMAGE_DEFAULT: f32 = 0.5;
const CRITICAL_DAMAGE_MAXIMUM: f32 = 1.25;
const CRITICAL_CHANCE_MAXIMUM: f32 = 21912.000833;
const CRITICAL_CHANCE_STEP: f32 = 219.12000833;
const CRITICAL_CHANCE_DEFAULT: f32 = 0.1;
const ARMOUR_MAXIMUM: f32 = 0.5;
const ARMOUR_MAXIMUM_INT: u16 = 33000;

#[derive(Clone, Copy)]
enum Percent {
    U8(u8),
    F32(f32),
}

impl Percent {
    fn to_f32(&self) -> f32 {
        match self {
            Percent::U8(v) => *v as f32 / 100.0,
            Percent::F32(v) => *v,
        }
    }

    fn to_u8(&self) -> u8 {
        match self {
            Percent::U8(v) => *v,
            Percent::F32(v) => (*v * 100.0) as u8,
        }
    }
}

impl Add for Percent {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Percent::F32(self.to_f32() + rhs.to_f32())
    }
}