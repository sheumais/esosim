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
mod skill;

const LEVEL: u8 = 50;
const EFFECTIVE_LEVEL: u8 = 66;
const CRITICAL_DAMAGE_DEFAULT: f32 = 0.5;
const CRITICAL_DAMAGE_MAXIMUM: f32 = 1.25;
const CRITICAL_CHANCE_DEFAULT: f32 = 0.1;
const ARMOUR_MAXIMUM: f32 = 0.5;

#[derive(Clone, Copy, Default)]
pub struct Percent(u32);

impl Percent {
    /// Construct from whole percent (e.g. `Percent::from_u8(12)` = 12%)
    pub fn from_u8(v: u8) -> Self {
        Percent((v as u32) * 100)
    }

    /// Construct from fractional value (e.g. 0.1234 = 12.34%)
    pub fn from_f32(v: f32) -> Self {
        Percent((v * 10000.0).round() as u32)
    }

    /// Convert to f32 (0.0–1.0)
    pub fn to_f32(&self) -> f32 {
        self.0 as f32 / 10000.0
    }

    /// Convert back to whole percent (truncates decimals)
    pub fn to_u8(&self) -> u8 {
        (self.0 / 100) as u8
    }

    /// Zero percent
    pub fn new() -> Self {
        Percent(0)
    }
}

impl Add for Percent {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Percent(self.0 + rhs.0)
    }
}

impl AddAssign for Percent {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}