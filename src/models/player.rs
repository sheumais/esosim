use std::collections::HashMap;

use crate::data::{armour::*, item_type::*, power::*, skill::*, traits::*};

pub struct Player {
    id: u32,
    gear: Loadout,
    primary_abilities: Vec<u32>,
    backup_abilities: Vec<u32>,
    active_bar: ActiveBar,
    buffs: HashMap<u32, u8>,
    attributes: (u8, u8, u8),
}

impl Player {
    pub fn new(id: u32) -> Self {
        Self {
            id,
            gear: Loadout::default(),
            primary_abilities: Vec::new(),
            backup_abilities: Vec::new(),
            active_bar: ActiveBar::Primary,
            buffs: HashMap::new(),
            attributes: (0, 0, 0)
        }
    }

    pub fn swap_bars(&mut self, choice: Option<ActiveBar>) {
        if let Some(choice) = choice {
            self.active_bar = choice;
        } else {
            self.active_bar = match self.active_bar {
                ActiveBar::Primary => ActiveBar::Backup,
                ActiveBar::Backup => ActiveBar::Primary,
            }
        }
    }

    pub fn get_skills_on_current_bar(&self) -> Vec<&u32> {
        let skills = match self.active_bar {
            ActiveBar::Primary => &self.primary_abilities,
            ActiveBar::Backup => &self.backup_abilities,
        };
        skills.iter().map(|f| f).collect()
    }

    pub fn get_active_gear(&self) -> Vec<&GearPiece> {
        self.gear.get_active_gear(&self.active_bar)
    }

    pub fn get_active_bar(&self) -> &ActiveBar {
        &self.active_bar
    }

    pub fn is_specific_item_equipped(&self, item_id: &u32) -> bool {
        for item in self.get_active_gear() {
            if &item.item_id == item_id {return true}
        }
        false
    }

    pub fn get_number_of_equipped_item_type(&self, item_type: &ItemType) -> u8 {
        self.gear.get_number_of_item_type(item_type, &self.active_bar)
    }

    pub fn get_number_of_equipped_trait(&self, item_trait: &GearTrait) -> u8 {
        self.gear.get_number_of_trait(item_trait, &self.active_bar)
    }

    pub fn get_number_of_equipped_set(&self, set_id: &u16) -> u8 {
        self.gear.get_number_of_set(set_id, &self.active_bar)
    }

    pub fn set_gear_piece(&mut self, slot: &GearSlot, gear: GearPiece) {
        self.gear.set_gear_piece(slot, gear);
    }

    pub fn set_skills(&mut self, bar: &ActiveBar, skills: Vec<u32>) {
        match bar {
            ActiveBar::Primary => self.primary_abilities = skills,
            ActiveBar::Backup => self.backup_abilities = skills,
        }
    }

    pub fn get_buffs(&self) -> &HashMap<u32, u8> {
        &self.buffs
    }
    
    pub fn add_buff(&mut self, id: u32, stacks: u8) {
        self.buffs.insert(id, stacks);
    }

    pub fn remove_buff(&mut self, id: &u32) {
        self.buffs.remove(id);
    }

    pub fn modify_stacks(&mut self, id: u32, stacks: u8) {
        self.add_buff(id, stacks);
    }

    pub fn get_bar_of_skill_id(&self, skill: &u32) -> Option<ActiveBar> {
        let in_primary = self.primary_abilities.contains(skill);
        let in_backup = self.backup_abilities.contains(skill);

        match (in_primary, in_backup) {
            (true, false) => Some(ActiveBar::Primary),
            (false, true) => Some(ActiveBar::Backup),
            _ => None,
        }
    }

    pub fn id(&self) -> &u32 {
        &self.id
    }

    pub fn get_gear_piece(&self, slot: &GearSlot) -> Option<&GearPiece> {
        self.gear.get_gear_piece(slot)
    }

    pub fn get_number_of_active_skills_from_skill_line(&self, skill_line: &SkillLine) -> u8 {
        self.get_skills_on_current_bar()
            .iter()
            .filter_map(|s| ability_id_to_skill_line(s))
            .filter(|a| a == skill_line)
            .count() as u8
    }

    pub fn has_buff(&self, ability_id: &u32) -> bool {
        self.buffs.get(ability_id).is_some()
    }

    pub fn get_total_armour(&self) -> u32 {
        self.gear.get_total_armour(&self.active_bar)
    }

    pub fn set_attributes(&mut self, attributes: (u8, u8, u8)) {
        debug_assert!(attributes.0 + attributes.1 + attributes.2 <= 64);
        self.attributes = attributes
    }

    pub fn get_attributes(&self) -> (u8, u8, u8) {
        self.attributes
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

    pub fn set_gear_piece(&mut self, slot: &GearSlot, gear: GearPiece) {
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

    pub fn get_active_gear(&self, active_bar: &ActiveBar) -> Vec<&GearPiece> {
        let slots: &[GearSlot] = match active_bar {
            ActiveBar::Primary => &[
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
            ActiveBar::Backup => &[
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

    pub fn iter_active_gear(&self, active_bar: &ActiveBar) -> impl Iterator<Item = (GearSlot, &GearPiece)> {
        let slots: &[GearSlot] = match active_bar {
            ActiveBar::Primary => &[
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
            ActiveBar::Backup => &[
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

    pub fn get_number_of_item_type(&self, item_type: &ItemType, active_bar: &ActiveBar) -> u8 {
        self.get_active_gear(active_bar)
            .iter()
            .filter_map(|g| g.get_item_type())
            .filter(|i| *i == item_type)
            .count() as u8
    }

    pub fn get_number_of_trait(&self, item_trait: &GearTrait, active_bar: &ActiveBar) -> u8 {
        self.get_active_gear(active_bar)
            .iter()
            .filter_map(|g| g.get_item_trait())
            .filter(|i| *i == item_trait)
            .count() as u8
    }

    pub fn get_number_of_set(&self, set_id: &u16, active_bar: &ActiveBar) -> u8 {
        self.get_active_gear(active_bar)
            .iter()
            .filter_map(|g| g.set_id)
            .filter(|i| i == set_id)
            .count() as u8
    }

    pub fn get_total_armour(&self, active_bar: &ActiveBar) -> u32 {
        self.iter_active_gear(active_bar).map(|(slot, gear_piece)| gear_piece.get_armour_value(&slot)).sum()
    }
}

#[derive(Debug, PartialEq)]
pub struct GearEnchant {
    pub glyph: EnchantType,
    pub effective_level: u8,
    pub quality: ItemQuality,
}

#[derive(Debug, PartialEq)]
pub struct GearPiece {
    pub item_id: u32,
    pub effective_level: u8,
    pub gear_trait: Option<GearTrait>,
    pub quality: ItemQuality,
    pub set_id: Option<u16>,
    pub enchant: Option<GearEnchant>,
}

impl GearPiece {
    pub fn get_item_type(&self) -> Option<&ItemType> {
        ITEM_TYPES.get(&self.item_id)
    }

    pub fn get_item_trait(&self) -> Option<&GearTrait> {
        self.gear_trait.as_ref()
    }

    pub fn get_trait_value(&self) -> Option<f32> {
        let trait_opt = &self.gear_trait;
        let item_type = self.get_item_type()?; // todo fix jewellery
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

        if is_weapon(item_type)
            && is_two_handed_weapon(item_type)
            && weapon_trait_doubles(&trait_)
        {
            value *= 2.0;
        }

        Some(value)
    }

    pub fn get_weapon_power(&self, gear_slot: &GearSlot) -> u32 {
        let base_power = self
            .get_item_type()
            .and_then(|item_type| power_from_weapon_type(item_type, &self.quality))
            .unwrap_or(0) as f32;

        let trait_multiplier = match self.gear_trait {
            Some(GearTrait::WeaponNirnhoned) => get_weapon_nirnhoned_value(&self.quality),
            _ => 1.0,
        };

        let multiplier = match gear_slot {
            GearSlot::OffHand | GearSlot::OffHandBackup => OFFHAND_MULTIPLIER + 0.06,
            _ => 1.0
        };

        (base_power * trait_multiplier * multiplier).round() as u32
    }

    pub fn get_armour_value(&self, gear_slot: &GearSlot) -> u32 {
        let armour_value = if let Some(item_type) = self.get_item_type() {
            armour_from_armour_piece(item_type, gear_slot, &self.quality).unwrap_or(0).into()
        } else {
            0
        };
        let new_armour_value = if let Some(trait_) = &self.gear_trait {
            match trait_ {
                GearTrait::ArmorReinforced => (armour_value as f32 * get_armor_reinforced_value(&self.quality)) as u32,
                GearTrait::ArmorNirnhoned => (armour_value as f32 + get_armor_nirnhoned_value(&self.quality)) as u32,
                GearTrait::JewelryProtective => (get_jewelry_protective_value(&self.quality)) as u32,
                _ => armour_value,
            }
        } else {
            armour_value
        };
        return new_armour_value
    }
}

pub fn get_trait_value_for_item(gear: &GearPiece) -> Option<f32> {
    let trait_opt = gear.get_item_trait();
    let item_type = gear.get_item_type()?;
    let quality = &gear.quality;

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

    if is_weapon(item_type)
        && is_two_handed_weapon(item_type)
        && weapon_trait_doubles(&trait_)
    {
        value *= 2.0;
    }

    Some(value)
}

#[derive(Debug, PartialEq)]
pub enum ActiveBar {
    Primary,
    Backup,
}
