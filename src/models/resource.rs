use crate::models::LEVEL;

pub enum PlayerAttributeType {
    Health,
    Magicka,
    Stamina,
}

pub struct PlayerMaxResource {
    resource_type: PlayerAttributeType,
    attribute: u8,
    additive: u32,
    multiplicative: f32,
}

impl PlayerMaxResource {
    pub fn calculate(&self) -> u32 {
        let (level_coeff, attr_coeff) = match self.resource_type {
            PlayerAttributeType::Health => (300, 122),
            PlayerAttributeType::Magicka | PlayerAttributeType::Stamina => (220, 111),
        };

        let base = level_coeff * LEVEL as u32
            + 1000
            + attr_coeff * self.attribute as u32
            + self.additive;

        (base as f32 * (1.0 + self.multiplicative)).round() as u32
    }

    pub fn add_to_additive(&mut self, value: u32) {
        self.additive += value;
    }

    pub fn add_to_multiplicative(&mut self, value: f32) {
        self.multiplicative += value;
    }

    pub fn set_attribute(&mut self, value: u8) {
        self.attribute = value;
    }

    pub fn get_attribute(&self) -> u8 {
        self.attribute
    }

    pub fn reset(&mut self) {
        self.additive = 0;
        self.multiplicative = 0.0;
    }

    pub fn new(resource_type: PlayerAttributeType) -> Self {
        PlayerMaxResource {
            resource_type,
            attribute: 0,
            additive: 0,
            multiplicative: 0.0,
        }
    }

    pub fn get_type(self) -> PlayerAttributeType {
        self.resource_type
    }

    pub fn calculate_attribute_count_from_target(&mut self, target_resource_amount: u32) -> Option<u8> {
        for i in 0..=64 {
            self.attribute = i;
            if self.calculate() == target_resource_amount {
                return Some(i);
            }
        }
        None
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
        h.multiplicative = 0.12; // undaunted passive + heavy armour passive x5
        assert_eq!(h.calculate(), 33456u32); // compared with tested value in game
    }
}