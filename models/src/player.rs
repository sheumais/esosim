use crate::{critical::{CriticalChance, CriticalDamage}, damage_done::DamageDone, damage_taken::Resistance, power::Power, resource::{PlayerAttributeType, PlayerMaxResource}, EFFECTIVE_LEVEL, LEVEL};

pub struct Player {
    level: u8,
    effective_level: u8,
    max_health: PlayerMaxResource,
    max_magicka: PlayerMaxResource,
    max_stamina: PlayerMaxResource,
    critical_damage: CriticalDamage,
    critical_chance: CriticalChance,
    penetration: u32,
    damage_done: DamageDone,
    damage_taken: f32,
    physical_resistance: Resistance,
    spell_resistance: Resistance,
    frost_resistance: u32,
    flame_resistance: u32,
    shock_resistance: u32,
    poison_resistance: u32,
    disease_resistance: u32,
    spell_power: Power,
    weapon_power: Power,
}
impl Player {
    pub fn new() -> Self {
        Self {
            level: LEVEL,
            effective_level: EFFECTIVE_LEVEL,
            max_health: PlayerMaxResource::new(PlayerAttributeType::Health),
            max_magicka: PlayerMaxResource::new(PlayerAttributeType::Magicka),
            max_stamina: PlayerMaxResource::new(PlayerAttributeType::Stamina),
            critical_damage: CriticalDamage::default(),
            critical_chance: CriticalChance::default(),
            penetration: 0,
            damage_done: DamageDone::default(),
            damage_taken: 0.0,
            physical_resistance: Resistance::new(),
            spell_resistance: Resistance::new(),
            frost_resistance: 0,
            flame_resistance: 0,
            shock_resistance: 0,
            poison_resistance: 0,
            disease_resistance: 0,
            spell_power: Power::new(),
            weapon_power: Power::new(),
        }
    }
}