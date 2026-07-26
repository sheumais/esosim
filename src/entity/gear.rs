use crate::{data::{enums::gear::{EnchantType, GearSlot, GearTrait, ItemQuality, ItemType}, tables::{armour::armour_from_armour_piece, enchant::*, item_type::*, power::power_from_weapon_type, traits::*}}, entity::gear::EnchantValueWrapper::{F32, Prismatic}};


#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GearEnchant {
    pub glyph: EnchantType,
    pub effective_level: u8,
    pub quality: ItemQuality,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct GearPiece {
    pub item_id: u32,
    pub item_type: ItemType,
    pub effective_level: u8,
    pub gear_trait: Option<GearTrait>,
    pub quality: ItemQuality,
    pub set_id: Option<u16>,
    pub enchant: Option<GearEnchant>,
}

pub enum EnchantValueWrapper {
    F32(f32),
    /// Health, Magicka, Stamina
    Prismatic((f32, f32, f32)),
}

impl GearPiece {
    pub fn new(item_type: ItemType, level: u8, gear_trait: Option<GearTrait>, quality: ItemQuality, set_id: Option<u16>, enchant: Option<GearEnchant>) -> Self {
        return Self {
            item_id: 0,
            item_type,
            effective_level: level,
            gear_trait,
            quality,
            set_id,
            enchant,
        }
    }

    pub fn get_item_type(&self) -> ItemType {
        self.item_type.clone()
    }

    pub fn get_quality(&self) -> ItemQuality {
        self.quality.clone()
    }

    pub fn get_item_trait(&self) -> Option<GearTrait> {
        self.gear_trait
    }

    pub fn get_enchant(&self) -> Option<GearEnchant> {
        self.enchant
    }

    pub fn get_effective_level(&self) -> u8 {
        self.effective_level
    }

    pub fn get_effective_enchant_quality(&self) -> Option<ItemQuality> {
        if let Some(e) = self.get_enchant() {
            Some(e.quality.find_lower_quality(self.get_quality()))
        } else {
            None
        }
    }

    pub fn get_trait_value(&self) -> Option<f32> {
        let trait_opt = &self.gear_trait;
        let item_type = self.get_item_type(); // todo fix jewellery
        let quality = &self.quality;

        let trait_ = if let Some(trait_) = trait_opt {
            trait_
        } else {
            return None;
        };

        let mut value = match trait_ {
            GearTrait::WeaponPowered => get_weapon_powered_value(quality),
            GearTrait::WeaponCharged => get_weapon_charged_value(quality),
            GearTrait::WeaponPrecise => get_weapon_precise_value(quality),
            GearTrait::WeaponInfused => get_weapon_infused_value(quality),
            GearTrait::WeaponDefending => get_weapon_defending_value(quality),
            GearTrait::WeaponTraining => get_weapon_training_value(quality),
            GearTrait::WeaponSharpened => get_weapon_sharpened_value(quality),
            GearTrait::WeaponDecisive => get_weapon_decisive_value(quality),
            GearTrait::WeaponNirnhoned => get_weapon_nirnhoned_value(quality),

            GearTrait::ArmorSturdy => get_armor_sturdy_value(quality),
            GearTrait::ArmorImpenetrable => get_armor_impenetrable_value(quality),
            GearTrait::ArmorReinforced => get_armor_reinforced_value(quality),
            GearTrait::ArmorWellFitted => get_armor_well_fitted_value(quality),
            GearTrait::ArmorTraining => get_armor_training_value(quality),
            GearTrait::ArmorInfused => get_armor_infused_value(quality),
            GearTrait::ArmorInvigorating => get_armor_invigorating_value(quality),
            GearTrait::ArmorDivines => get_armor_divines_value(quality),
            GearTrait::ArmorNirnhoned => get_armor_nirnhoned_value(quality),

            GearTrait::JewelryHealthy => get_jewelry_healthy_value(quality),
            GearTrait::JewelryArcane => get_jewelry_arcane_value(quality),
            GearTrait::JewelryRobust => get_jewelry_robust_value(quality),
            GearTrait::JewelryBloodthirsty => get_jewelry_bloodthirsty_value(quality),
            GearTrait::JewelryHarmony => get_jewelry_harmony_value(quality),
            GearTrait::JewelryInfused => get_jewelry_infused_value(quality),
            GearTrait::JewelryProtective => get_jewelry_protective_value(quality),
            GearTrait::JewelrySwift => get_jewelry_swift_value(quality),

            GearTrait::JewelryTriune => return None,

            _ => return None,
        };

        if is_weapon(&item_type)
            && is_two_handed_weapon(&item_type)
            && weapon_trait_doubles(&trait_)
        {
            value *= 2.0;
        }
        if trait_.is_infused() {
            value += 1.0;
        }

        Some(value)
    }

    pub fn get_infused_value(&self) -> f32 {
        if let Some(i) = self.get_item_trait() {
            if i.is_infused() {
                return self.get_trait_value().unwrap_or(1.0);
            } else {
                return 1f32
            }
        } else {
            return 1.0
        }
    }

    /// multiply by offhand_multipler + 0.06 afterward, then round() and cast to u32
    pub fn get_weapon_power(&self) -> f32 {
        let base_power = power_from_weapon_type(&self.get_item_type(), &self.quality)
            .unwrap_or(0) as f32;

        let trait_multiplier = match self.gear_trait {
            Some(GearTrait::WeaponNirnhoned) => get_weapon_nirnhoned_value(&self.quality),
            _ => 1.0,
        };

        base_power * trait_multiplier
    }

    pub fn get_armour_value(&self, gear_slot: &GearSlot) -> u32 {
        let armour_value = armour_from_armour_piece(&self.get_item_type(), gear_slot, &self.quality).unwrap_or(0).into();
        let new_armour_value = if let Some(trait_) = &self.gear_trait {
            match trait_ {
                GearTrait::ArmorReinforced => (armour_value as f32 * get_armor_reinforced_value(&self.quality)) as u32,
                GearTrait::ArmorNirnhoned => (armour_value as f32 + get_armor_nirnhoned_value(&self.quality)) as u32,
                GearTrait::JewelryProtective => (get_jewelry_protective_value(&self.quality)) as u32,
                GearTrait::WeaponDefending => {match is_two_handed_weapon(&self.get_item_type()) {
                    true => (get_weapon_defending_value(&self.quality) * 2.0) as u32,
                    false => get_weapon_defending_value(&self.quality) as u32,
                }},
                _ => armour_value,
            }
        } else {
            armour_value
        };
        return new_armour_value
    }

    pub fn is_two_handed_weapon(&self) -> bool {
        is_two_handed_weapon(&self.get_item_type())
    }

    /// Gets the nominal value of the enchant.
    pub fn get_enchant_value(&self) -> Option<EnchantValueWrapper> {
        if let Some(e) = self.get_enchant() {
            let v = match e.glyph {
                EnchantType::DiseaseResistance => {get_enchant_jewellery_increase_disease_resistance(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                EnchantType::FlameResistance => {get_enchant_jewellery_increase_flame_resistance(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                EnchantType::FrostResistance => {get_enchant_jewellery_increase_frost_resistance(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                EnchantType::Health => {get_enchant_armour_health_value(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                EnchantType::HealthRegen => {get_enchant_jewellery_increase_health_recovery(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                // EnchantType::IncreaseBashDamage => {},
                EnchantType::IncreasePhysicalDamage => {get_enchant_jewellery_increase_weapon_damage(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                // EnchantType::IncreasePotionEffectiveness => {},
                EnchantType::IncreaseSpellDamage => {get_enchant_jewellery_increase_spell_damage(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                EnchantType::Magicka => {get_enchant_armour_magicka_value(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                EnchantType::MagickaRegen => {get_enchant_jewellery_increase_magicka_recovery(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                EnchantType::PhysicalResistance => {get_enchant_jewellery_increase_physical_resistance(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                EnchantType::PoisonResistance => {get_enchant_jewellery_increase_poison_resistance(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                // EnchantType::ReduceBlockAndBash => {},
                // EnchantType::ReduceFeatCost => {},
                // EnchantType::ReducePotionCooldown => {},
                // EnchantType::ReduceSpellCost => {},
                EnchantType::ShockResistance => {get_enchant_jewellery_increase_shock_resistance(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                EnchantType::SpellResistance => {get_enchant_jewellery_increase_spell_resistance(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                EnchantType::Stamina => {get_enchant_armour_stamina_value(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                EnchantType::StaminaRegen => {get_enchant_jewellery_increase_stamina_recovery(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                _ => {
                    let p = match e.glyph {
                        EnchantType::PrismaticDefense => {get_enchant_armour_prismatic_values(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                        EnchantType::PrismaticRecovery => {get_enchant_jewellery_prismatic_recovery_values(self.get_effective_level(), self.get_effective_enchant_quality().unwrap())},
                        _ => {return None},
                    };
                    return Some(Prismatic(p))
                },
            };
            return Some(F32(v))
        }

        return None
    }
}

#[derive(Debug, PartialEq)]
pub struct Loadout {
    pub head: Option<GearPiece>,
    pub shoulders: Option<GearPiece>,
    pub chest: Option<GearPiece>,
    pub hands: Option<GearPiece>,
    pub waist: Option<GearPiece>,
    pub legs: Option<GearPiece>,
    pub feet: Option<GearPiece>,
    pub necklace: Option<GearPiece>,
    pub ring1: Option<GearPiece>,
    pub ring2: Option<GearPiece>,
    pub main_hand: Option<GearPiece>,
    pub main_hand_backup: Option<GearPiece>,
    pub poison: Option<GearPiece>,
    pub off_hand: Option<GearPiece>,
    pub off_hand_backup: Option<GearPiece>,
    pub poison_backup: Option<GearPiece>,
}

impl Loadout {
    pub fn default() -> Self {
        Loadout { 
            head: None,
            shoulders: None,
            chest: None,
            hands: None,
            waist: None,
            legs: None,
            feet: None,
            necklace: None,
            ring1: None,
            ring2: None,
            main_hand: None,
            main_hand_backup: None,
            poison: None,
            off_hand: None,
            off_hand_backup: None,
            poison_backup: None
        }
    }

    pub fn get_gear_piece(&self, slot: &GearSlot) -> Option<&GearPiece> {
        let option = match slot {
            GearSlot::Head => &self.head,
            GearSlot::Shoulders => &self.shoulders,
            GearSlot::Chest => &self.chest,
            GearSlot::Hands => &self.hands,
            GearSlot::Waist => &self.waist,
            GearSlot::Legs => &self.legs,
            GearSlot::Feet => &self.feet,
            GearSlot::Necklace => &self.necklace,
            GearSlot::Ring1 => &self.ring1,
            GearSlot::Ring2 => &self.ring2,
            GearSlot::MainHand => &self.main_hand,
            GearSlot::OffHand => &self.off_hand,
            GearSlot::Poison => &self.poison,
            GearSlot::MainHandBackup => &self.main_hand_backup,
            GearSlot::OffHandBackup => &self.off_hand_backup,
            GearSlot::BackupPoison => &self.poison_backup,
        };
        option.as_ref()
    }

    pub fn set_gear_piece(&mut self, slot: GearSlot, gear: GearPiece) {
        match slot {
            GearSlot::Head => self.head = Some(gear),
            GearSlot::Shoulders => self.shoulders = Some(gear),
            GearSlot::Chest => self.chest = Some(gear),
            GearSlot::Hands => self.hands = Some(gear),
            GearSlot::Waist => self.waist = Some(gear),
            GearSlot::Legs => self.legs = Some(gear),
            GearSlot::Feet => self.feet = Some(gear),
            GearSlot::Necklace => self.necklace = Some(gear),
            GearSlot::Ring1 => self.ring1 = Some(gear),
            GearSlot::Ring2 => self.ring2 = Some(gear),
            GearSlot::MainHand => self.main_hand = Some(gear),
            GearSlot::OffHand => self.off_hand = Some(gear),
            GearSlot::Poison => self.poison = Some(gear),
            GearSlot::MainHandBackup => self.main_hand_backup = Some(gear),
            GearSlot::OffHandBackup => self.off_hand_backup = Some(gear),
            GearSlot::BackupPoison => self.poison_backup = Some(gear),
        };
    }

    pub fn get_active_gear(&self, backbar: bool) -> Vec<&GearPiece> {
        let slots: &[GearSlot] = match backbar {
            false => &[
                GearSlot::Head,
                GearSlot::Shoulders,
                GearSlot::Chest,
                GearSlot::Hands,
                GearSlot::Waist,
                GearSlot::Legs,
                GearSlot::Feet,
                GearSlot::Necklace,
                GearSlot::Ring1,
                GearSlot::Ring2,
                GearSlot::MainHand,
                GearSlot::OffHand,
                GearSlot::Poison,
            ],
            true => &[
                GearSlot::Head,
                GearSlot::Shoulders,
                GearSlot::Chest,
                GearSlot::Hands,
                GearSlot::Waist,
                GearSlot::Legs,
                GearSlot::Feet,
                GearSlot::Necklace,
                GearSlot::Ring1,
                GearSlot::Ring2,
                GearSlot::MainHandBackup,
                GearSlot::OffHandBackup,
                GearSlot::BackupPoison,
            ],
        };

        slots.iter()
            .filter_map(|slot| self.get_gear_piece(slot))
            .collect()
    }

    pub fn iter_active_gear(&self, backbar: bool) -> impl Iterator<Item = (GearSlot, &GearPiece)> {
        let slots: &[GearSlot] = match backbar {
            false => &[
                GearSlot::Head,
                GearSlot::Shoulders,
                GearSlot::Chest,
                GearSlot::Hands,
                GearSlot::Waist,
                GearSlot::Legs,
                GearSlot::Feet,
                GearSlot::Necklace,
                GearSlot::Ring1,
                GearSlot::Ring2,
                GearSlot::MainHand,
                GearSlot::OffHand,
                GearSlot::Poison,
            ],
            true => &[
                GearSlot::Head,
                GearSlot::Shoulders,
                GearSlot::Chest,
                GearSlot::Hands,
                GearSlot::Waist,
                GearSlot::Legs,
                GearSlot::Feet,
                GearSlot::Necklace,
                GearSlot::Ring1,
                GearSlot::Ring2,
                GearSlot::MainHandBackup,
                GearSlot::OffHandBackup,
                GearSlot::BackupPoison,
            ],
        };

        slots.iter().filter_map(|slot| {
            self.get_gear_piece(slot)
                .map(|gear| (*slot, gear))
        })
    }

    pub fn get_number_of_item_type(&self, item_type: &ItemType, backbar: bool) -> u8 {
        self.get_active_gear(backbar)
            .iter()
            .filter(|i| &i.get_item_type() == item_type)
            .count() as u8
    }

    pub fn get_number_of_trait(&self, item_trait: &GearTrait, backbar: bool) -> u8 {
        self.get_active_gear(backbar)
            .iter()
            .filter_map(|i| i.get_item_trait())
            .filter(|i| i == item_trait)
            .count() as u8
    }

    pub fn get_number_of_set(&self, set_id: &u16, backbar: bool) -> u8 {
        self.get_active_gear(backbar)
            .iter()
            .filter_map(|g| g.set_id)
            .filter(|i| i == set_id)
            .count() as u8
    }
}