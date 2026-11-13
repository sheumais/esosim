//! The input of the calculator: everything a person chooses about a character (roadmap 1.1,
//! Appendix A). Plain data with JSON in and out; nothing here looks anything up. Turning a `Build`
//! into a `Player` is `Build::to_player` (roadmap 1.3), the checks are `Build::validate` (4.2).
//!
//! A hand-written file only has to say race, class, level and attributes: every other field has an
//! empty default, which is what a golden fixture (roadmap 3.1) starts from.

use serde::{Deserialize, Serialize};

use esosim_types::{CpLoadout, MundusLoadout, PlayerClass, Race};

use crate::entity::{enemy::EnemyPreset, gear::Loadout};
use crate::stats::sheet::Attributes;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Build {
    pub race: Race,
    pub class: PlayerClass,
    pub level: u8,
    pub attributes: Attributes,
    /// Ability ids of the stones, in pick order.
    #[serde(default)]
    pub mundus: MundusLoadout,
    #[serde(default)]
    pub champion: CpLoadout,
    #[serde(default)]
    pub gear: Loadout,
    /// Ability ids slotted on the front bar.
    #[serde(default)]
    pub front_bar: Vec<u32>,
    #[serde(default)]
    pub back_bar: Vec<u32>,
    #[serde(default)]
    pub food: Option<u32>,
    #[serde(default)]
    pub potion: Option<u32>,
    /// Buffs the group provides, as ability ids.
    #[serde(default)]
    pub external_buffs: Vec<u32>,
    /// Passive abilities the character has unlocked, as ability ids: one id per passive, however many effects it has
    /// (a tooltip with two numbers is one entry), at the rank that is unlocked (Dexterity I, II and III are three
    /// records). Nothing is on unless it is listed here: racial, class, armour-weight, weapon, world and guild passives
    /// (`EffectSource::Race` / `Class` / `Passive`, roadmap 2.4) all work this way. A race or class passive also has to
    /// be the build's own. Its own condition (armour pieces, weapon) then decides whether it counts.
    #[serde(default)]
    pub passives: Vec<u32>,
    #[serde(default)]
    pub enemy: EnemyPreset,
}

impl Build {
    pub fn to_json(&self) -> Result<String, String> {
        serde_json::to_string_pretty(self).map_err(|e| e.to_string())
    }

    pub fn from_json(s: &str) -> Result<Self, String> {
        serde_json::from_str(s).map_err(|e| e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use super::*;
    use crate::data::enums::gear::{EnchantType, GearSlot, GearTrait, ItemQuality, ItemType};
    use crate::entity::gear::{GearEnchant, GearPiece};

    fn sample() -> Build {
        let enchant = GearEnchant { glyph: EnchantType::Health, effective_level: 66, quality: ItemQuality::Legendary };
        let mut gear = Loadout::default();
        gear.set_gear_piece(
            GearSlot::Chest,
            GearPiece::new(ItemType::Heavy, 66, Some(GearTrait::ArmorDivines), ItemQuality::Legendary, Some(161), Some(enchant)),
        );
        Build {
            race: Race::Dunmer,
            class: PlayerClass::Nightblade,
            level: 50,
            attributes: Attributes { health: 12, magicka: 0, stamina: 52 },
            mundus: MundusLoadout::new([13975]),
            champion: CpLoadout { points: BTreeMap::from([(1, 10)]), slotted: vec![1] },
            gear,
            front_bar: vec![1, 2],
            back_bar: vec![3],
            food: Some(4),
            potion: None,
            external_buffs: vec![5],
            passives: vec![],
            enemy: EnemyPreset::Custom { max_health: 21_000_000.0, resistance: 18_200, debuffs: vec![61743] },
        }
    }

    #[test]
    fn a_build_survives_json() {
        let build = sample();
        assert_eq!(Build::from_json(&build.to_json().unwrap()).unwrap(), build);
    }

    #[test]
    fn only_race_class_level_and_attributes_are_required() {
        let json = r#"{"race":"Nord","class":"Templar","level":50,"attributes":{"health":0,"magicka":64,"stamina":0}}"#;
        let build = Build::from_json(json).unwrap();
        assert_eq!(build.gear, Loadout::default());
        assert_eq!(build.enemy, EnemyPreset::TrialDummy);
        assert!(build.front_bar.is_empty() && build.food.is_none());
    }
}
