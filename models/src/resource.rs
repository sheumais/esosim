use crate::{Percent, LEVEL};

pub enum PlayerAttributeType {
    Health,
    Magicka,
    Stamina,
}

pub struct PlayerMaxResource {
    resource_type: PlayerAttributeType,
    attribute: u16,
    additive: u16,
    multiplicative: Percent,
}

impl PlayerMaxResource {
    pub fn calculate(&self) -> u32 {
        let (level_coeff, attr_coeff) = match self.resource_type {
            PlayerAttributeType::Health => (300, 122),
            PlayerAttributeType::Magicka | PlayerAttributeType::Stamina => (220, 111),
        };

        let base = level_coeff * LEVEL as u16
            + 1000
            + attr_coeff * self.attribute
            + self.additive;

        (base as f32 * (1.0 + self.multiplicative.to_f32())).round() as u32
    }

    pub fn new(resource_type: PlayerAttributeType) -> Self {
        PlayerMaxResource {
            resource_type,
            attribute: 0,
            additive: 0,
            multiplicative: Percent::new(),
        }
    }

    pub fn get_type(self) -> PlayerAttributeType {
        self.resource_type
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_default_max_health() {
        let health = PlayerMaxResource::new(PlayerAttributeType::Health);
        assert_eq!(health.calculate(), 16000u32);
    }

    #[test]
    fn test_calculate_default_max_magicka() {
        let magicka = PlayerMaxResource::new(PlayerAttributeType::Magicka);
        assert_eq!(magicka.calculate(), 12000u32);
    }

    #[test]
    fn test_calculate_default_max_stamina() {
        let stamina = PlayerMaxResource::new(PlayerAttributeType::Stamina);
        assert_eq!(stamina.calculate(), 12000u32);
    }

    #[test]
    fn test_calculate_max_health_tank() {
        let mut h = PlayerMaxResource::new(PlayerAttributeType::Health);
        h.attribute = 32; 
        h.additive = 4462 // random crown tri max food 
            + 477 * 2 + 193 * 3 + 560 // hakeijo enchants + hero's vigor CP
            + 1206*2 // pearlescent ward passive
            + 1000; // nord passive
        h.multiplicative = Percent::from_f32(0.1 + 0.02); // undaunted passive + heavy armour passive x5
        assert_eq!(h.calculate(), 33456u32); // compare with tested value in game
    }

    #[test]
    fn test_calculate_max_stamina_dps() {
        todo!();
    }

    #[test]
    fn test_calculate_max_magicka_healer() {
        todo!();
    }
}