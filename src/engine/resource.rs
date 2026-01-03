use std::collections::HashMap;

use crate::{data::{item_type::{EnchantType, GearTrait, ItemType}, resource::{DARK_VIGOR, FOOD_BUFFS, JUGGERNAUT, RESOURCE_HEALTH_ADDITIVE, RESOURCE_HEALTH_MULTIPLICATIVE, RESOURCE_MAGICKA_ADDITIVE, RESOURCE_MAGICKA_MULTIPLICATIVE, RESOURCE_STAMINA_ADDITIVE, RESOURCE_STAMINA_MULTIPLICATIVE}, sets::{SET_HEALTH_MAX, SET_MAGICKA_MAX, SET_STAMINA_MAX, SetBonusType, get_number_of_bonus_type_from_active_set}, skill::{SkillLine, UNDAUNTED_METTLE_ID}, traits::{get_armor_infused_value, get_jewelry_arcane_value, get_jewelry_healthy_value, get_jewelry_robust_value}}, engine::{ID, STACKS}, models::{player::{Player, get_armour_enchant_multiplier}, resource::{PlayerAttributeType, PlayerMaxResource}}};
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
    pub is_dirty: bool,
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
            is_dirty: false,
        }
    }

    pub fn add_source(&mut self, id: ID, stacks: Option<STACKS>) {
        self.sources.insert(id, stacks.unwrap_or(1));
        self.is_dirty = true;
    }

    pub fn add_source_checked(&mut self, id: ID, stacks: Option<STACKS>) {
        if Self::is_valid_source(&id) {
            self.add_source(id, stacks);
        }
    }

    pub fn remove_source(&mut self, id: &ID) {
        self.is_dirty = self.sources.remove(id).is_some();
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

        self.is_dirty = false;
    }

    pub fn update_from_player(&mut self, player: &Player) {
        self.sources.clear();
        let player_attributes = player.get_attributes();
        self.max_health.set_attribute(player_attributes.0);
        self.max_magicka.set_attribute(player_attributes.1);
        self.max_stamina.set_attribute(player_attributes.2);
        let (light, medium, heavy) = ((player.get_number_of_equipped_item_type(&ItemType::Light) > 0) as u8, (player.get_number_of_equipped_item_type(&ItemType::Medium) > 0) as u8, (player.get_number_of_equipped_item_type(&ItemType::Heavy) > 0) as u8);
        for (id, stacks) in player.get_buffs() {
            if Self::is_valid_source(id) {
                self.add_source(*id, Some(*stacks));
                match id {
                    &UNDAUNTED_METTLE_ID => self.add_source(*id, Some(light+medium+heavy)),
                    _ => {},
                }
            }
            if FOOD_BUFFS.get(id).is_some() {
                self.food = Some(*id);
            }
        }
        let shadow = player.get_number_of_active_skills_from_skill_line(&SkillLine::Shadow);
        self.add_source(DARK_VIGOR.id, Some(shadow));
        let heavy = player.get_number_of_equipped_item_type(&ItemType::Heavy);
        self.add_source(JUGGERNAUT.id, Some(heavy));

        self.gear_stats = (0, 0, 0);
        let (ref mut health, ref mut magicka, ref mut stamina) = self.gear_stats;
        for (slot, gear_piece) in player.get_active_gear_with_slots() {
            let infused_multiplier = match gear_piece.gear_trait {
                Some(GearTrait::ArmorInfused) => {get_armor_infused_value(&gear_piece.quality)},
                Some(_) => {1.0},
                None => {1.0},
            };
            match gear_piece.gear_trait {
                Some(GearTrait::JewelryArcane) => {*magicka += get_jewelry_arcane_value(&gear_piece.quality) as u32},
                Some(GearTrait::JewelryHealthy) => {*health += get_jewelry_healthy_value(&gear_piece.quality) as u32},
                Some(GearTrait::JewelryRobust) => {*stamina += get_jewelry_robust_value(&gear_piece.quality) as u32},
                _ => {},
            }
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
        for set in player.get_active_sets_counts() {
            *health += SET_HEALTH_MAX * get_number_of_bonus_type_from_active_set(&set, &SetBonusType::Health) as u32;
            *magicka += SET_MAGICKA_MAX * get_number_of_bonus_type_from_active_set(&set, &SetBonusType::Magicka) as u32;
            *stamina += SET_STAMINA_MAX * get_number_of_bonus_type_from_active_set(&set, &SetBonusType::Stamina) as u32;
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