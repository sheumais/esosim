use std::collections::HashMap;

use esosim_data::item_type::{ITEM_TYPES, ItemQuality, ItemType};

pub struct Player {
    gear: Loadout,
    primary_abilities: [Option<u32>; 6],
    backup_abilities: [Option<u32>; 6],
    active_bar: ActiveBar,
    buffs: HashMap<u32, u8>,
}

impl Player {
    pub fn new() -> Self {
        Self {
            gear: Loadout::default(),
            primary_abilities: [None; 6],
            backup_abilities: [None; 6],
            active_bar: ActiveBar::Primary,
            buffs: HashMap::new(),
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
        skills.iter().filter_map(|s| s.as_ref()).collect()
    }

    pub fn get_active_gear(&self) -> Vec<&GearPiece> {
        self.gear.get_active_gear(&self.active_bar)
    }

    pub fn get_number_of_equipped_item_type(&self, item_type: &ItemType) -> u8 {
        self.gear.get_number_of_item_type(item_type, &self.active_bar)
    }

    pub fn set_gear_piece(&mut self, slot: &GearSlot, gear: GearPiece) {
        self.gear.set_gear_piece(slot, gear);
    }

    pub fn set_skills(&mut self, bar: &ActiveBar, skills: [Option<u32>; 6]) {
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

    pub fn get_number_of_item_type(&self, item_type: &ItemType, active_bar: &ActiveBar) -> u8 {
        self.get_active_gear(active_bar)
            .iter()
            .filter_map(|g| g.get_item_type())
            .filter(|i| *i == item_type)
            .count() as u8
    }
}

#[derive(Debug, PartialEq)]
pub enum GearSlot {
    Head,
    Shoulders,
    Chest,
    Hands,
    Waist,
    Legs,
    Feet,
    Necklace,
    Ring1,
    Ring2,
    MainHand,
    MainHandBackup,
    Poison,
    OffHand,
    OffHandBackup,
    BackupPoison,
}

#[derive(Debug, PartialEq)]
pub enum GearTrait {
    Powered,
    Charged,
    Precise,
    Infused,
    Defending,
    Training,
    Sharpened,
    Decisive,
    Sturdy,
    Impenetrable,
    Reinforced,
    WellFitted,
    Invigorating,
    Divines,
    Nirnhoned,
    Healthy,
    Arcane,
    Robust,
    Bloodthirsty,
    Harmony,
    Protective,
    Swift,
    Triune,
    Intricate,
}

#[derive(Debug, PartialEq)]
pub enum EnchantType {
    AbsorbHealth,
    AbsorbMagicka,
    AbsorbStamina,
    BefouledWeapon,
    Beserker,
    ChargedWeapon,
    DamageShield,
    DiseaseResistance,
    FieryWeapon,
    FrozenWeapon,
    Health,
    HealthRegen,
    IncreaseBashDamage,
    IncreasePhysicalDamage,
    IncreasePotionEffectiveness,
    IncreaseSpellDamage,
    Magicka,
    MagickaRegen,
    OblivionDamage,
    PoisonedWeapon,
    PrismaticDefense,
    PrismaticOnslaught,
    PrismaticRecovery,
    ReduceArmor,
    ReduceBlockAndBash,
    ReduceFeatCost,
    ReducePotionCooldown,
    ReducePower,
    ReduceSpellCost,
    Stamina,
    StaminaRegen,
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
}

#[derive(Debug, PartialEq)]
pub enum ActiveBar {
    Primary,
    Backup,
}
