use crate::LEVEL;

enum PlayerAttributeType {
    Health,
    Magicka,
    Stamina,
}

pub struct PlayerMaxResource {
    resource_type: PlayerAttributeType,
    attribute: u16,
    item: u16,
    set: u16,
    food: u16,
    skill2: u16,
    mundus: u16,
    skill: f32,
    buff: f32,
}

impl PlayerMaxResource {
    fn calculate(&self) -> u32 {
        let (level_coeff, attr_coeff) = match self.resource_type {
            PlayerAttributeType::Health => (300, 122),
            PlayerAttributeType::Magicka | PlayerAttributeType::Stamina => (220, 111),
        };

        let base = level_coeff * LEVEL as u16
            + 1000
            + attr_coeff * self.attribute
            + self.item
            + self.set
            + self.food
            + self.skill2
            + self.mundus;

        (base as f32 * (1.0 + self.skill + self.buff)).round() as u32
    }

    fn new(resource_type: PlayerAttributeType) -> Self {
        PlayerMaxResource {
            resource_type,
            attribute: 0,
            item: 0,
            set: 0,
            food: 0,
            skill2: 0,
            mundus: 0,
            skill: 0.0,
            buff: 0.0,
        }
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
        h.food = 4462; // random crown tri max food 
        h.item = 477 * 2 + 193 * 3 + 560; // hakeijo enchants + hero's vigor CP
        h.set = 1206*2; // pearlescent ward passive
        h.buff = 0.1 + 0.02; // undaunted passive + heavy armour passive x5
        h.skill2 = 1000; // nord passive
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