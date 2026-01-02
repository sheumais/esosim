use std::collections::HashMap;

use crate::{data::{item_type::{EnchantType, GearTrait}, resource::{FOOD_BUFFS, RESOURCE_HEALTH_ADDITIVE, RESOURCE_HEALTH_MULTIPLICATIVE, RESOURCE_MAGICKA_ADDITIVE, RESOURCE_MAGICKA_MULTIPLICATIVE, RESOURCE_STAMINA_ADDITIVE, RESOURCE_STAMINA_MULTIPLICATIVE}, traits::get_armor_infused_value}, engine::{ID, STACKS}, models::{player::{Player, get_armour_enchant_multiplier}, resource::{PlayerAttributeType, PlayerMaxResource}}};
use crate::data::enchant::*;

pub struct Resources {
    sources: HashMap<ID, STACKS>,
    health: u32,
    max_health: PlayerMaxResource,
    magicka: u32,
    max_magicka: PlayerMaxResource,
    stamina: u32,
    max_stamina: PlayerMaxResource,
    gear_stats: (u32, u32, u32),
    food: Option<u32>,
}

impl Resources {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            health: 0,
            max_health: PlayerMaxResource::new(PlayerAttributeType::Health),
            magicka: 0,
            max_magicka: PlayerMaxResource::new(PlayerAttributeType::Magicka),
            stamina: 0,
            max_stamina: PlayerMaxResource::new(PlayerAttributeType::Stamina),
            gear_stats: (0, 0, 0),
            food: None,
        }
    }

    pub fn add_source(&mut self, id: ID, stacks: Option<STACKS>) {
        self.sources.insert(id, stacks.unwrap_or(1));
    }

    pub fn remove_source(&mut self, id: &ID) {
        self.sources.remove(id);
    }

    pub fn is_valid_source(id: &ID) -> bool {
        RESOURCE_HEALTH_ADDITIVE.get(id).is_some() | RESOURCE_HEALTH_MULTIPLICATIVE.get(id).is_some() | RESOURCE_MAGICKA_ADDITIVE.get(id).is_some() | RESOURCE_MAGICKA_MULTIPLICATIVE.get(id).is_some() | RESOURCE_STAMINA_ADDITIVE.get(&id).is_some() | RESOURCE_STAMINA_MULTIPLICATIVE.get(&id).is_some()
    }

    pub fn refresh(&mut self) {
        self.max_health.reset();
        self.max_magicka.reset();
        self.max_stamina.reset();
        for (id, stacks) in &self.sources {
            if let Some(ha) = RESOURCE_HEALTH_ADDITIVE.get(id) {
                self.max_health.add_to_additive((ha.value + ha.value_per_stack * *stacks as f64) as u32);
            }
            if let Some(hm) = RESOURCE_HEALTH_MULTIPLICATIVE.get(id) {
                self.max_health.add_to_multiplicative((hm.value + hm.value_per_stack * *stacks as f64) as f32 / 100.0);
            }
            if let Some(ma) = RESOURCE_MAGICKA_ADDITIVE.get(id) {
                self.max_magicka.add_to_additive((ma.value + ma.value_per_stack * *stacks as f64) as u32);
            }
            if let Some(mm) = RESOURCE_MAGICKA_MULTIPLICATIVE.get(id) {
                self.max_magicka.add_to_multiplicative((mm.value + mm.value_per_stack * *stacks as f64) as f32 / 100.0);
            }
            if let Some(sa) = RESOURCE_STAMINA_ADDITIVE.get(id) {
                self.max_stamina.add_to_additive((sa.value + sa.value_per_stack * *stacks as f64) as u32);
            }
            if let Some(sm) = RESOURCE_STAMINA_MULTIPLICATIVE.get(id) {
                self.max_stamina.add_to_multiplicative((sm.value + sm.value_per_stack * *stacks as f64) as f32 / 100.0);
            }
        }
        if let Some(id) = self.food {
            if let Some(food_buff) = FOOD_BUFFS.get(&id) {
                if let Some(h) = food_buff.max_health {
                    self.max_health.add_to_additive(h);
                };
                if let Some(m) = food_buff.max_magicka {
                    self.max_magicka.add_to_additive(m);
                };
                if let Some(s) = food_buff.max_stamina {
                    self.max_stamina.add_to_additive(s);
                };
            }
        }
        let (ref health, ref magicka, ref stamina) = self.gear_stats;
        self.max_health.add_to_additive(*health);
        self.max_magicka.add_to_additive(*magicka);
        self.max_stamina.add_to_additive(*stamina);
    }

    pub fn update_from_player(&mut self, player: &Player) {
        self.sources.clear();
        let player_attributes = player.get_attributes();
        self.max_health.set_attribute(player_attributes.0);
        self.max_magicka.set_attribute(player_attributes.1);
        self.max_stamina.set_attribute(player_attributes.2);
        for (id, stacks) in player.get_buffs() {
            if Self::is_valid_source(id) {
                self.add_source(*id, Some(*stacks));
            }
            if FOOD_BUFFS.get(id).is_some() {
                self.food = Some(*id);
            }
        }
        self.gear_stats = (0, 0, 0);
        let (ref mut health, ref mut magicka, ref mut stamina) = self.gear_stats;
        for (slot, gear_piece) in player.get_active_gear_with_slots() {
            let infused_multiplier = match gear_piece.gear_trait {
                Some(GearTrait::ArmorInfused) => {get_armor_infused_value(&gear_piece.quality)},
                Some(_) => {1.0},
                None => {1.0},
            };
            let slot_multiplier = get_armour_enchant_multiplier(&slot);
            let multi = infused_multiplier * slot_multiplier;
            if let Some(enchant) = &gear_piece.enchant {
                match enchant.glyph {
                    EnchantType::Magicka => *magicka += (multi * get_enchant_armour_magicka_value(&enchant.effective_level, &enchant.quality)) as u32,
                    EnchantType::Stamina => *stamina += (multi * get_enchant_armour_stamina_value(&enchant.effective_level, &enchant.quality)) as u32,
                    EnchantType::Health => *health += (multi * get_enchant_armour_health_value(&enchant.effective_level, &enchant.quality)) as u32,
                    EnchantType::PrismaticDefense => {
                        let (h, m, s) = get_enchant_armour_prismatic_values(&enchant.effective_level, &enchant.quality);
                        *health += (multi * h) as u32;
                        *magicka += (multi * m) as u32;
                        *stamina += (multi * s) as u32;
                    },
                    _ => {},
                }
            }
        }
        self.refresh();
    }

    pub fn get_max_health(&self) -> u32 {
        self.max_health.calculate()
    }

    pub fn get_max_magicka(&self) -> u32 {
        self.max_magicka.calculate()
    }

    pub fn get_max_stamina(&self) -> u32 {
        self.max_stamina.calculate()
    }
}