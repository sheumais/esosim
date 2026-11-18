//! The input of the calculator: everything a person chooses about a character (roadmap 1.1,
//! Appendix A). Plain data with JSON in and out; nothing here looks anything up. Turning a `Build`
//! into a `Player` is `Build::to_player` (roadmap 1.3), the checks are `Build::validate` (4.2).
//!
//! A hand-written file only has to say race, class, level and attributes: every other field has an
//! empty default, which is what a golden fixture (roadmap 3.1) starts from.

use serde::{Deserialize, Serialize};

use esosim_types::{CpLoadout, Location, MundusLoadout, PlayerClass, Race, SubclassSwap};

use crate::entity::{enemy::EnemyPreset, gear::Loadout};
use crate::stats::sheet::Attributes;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Build {
    pub race: Race,
    pub class: PlayerClass,
    /// Subclassing: own class skill lines given up for a line of another class, at most two, from two different
    /// classes. Empty (and left out of the JSON) = the pure class, which is the only build with Class Mastery.
    /// A passive follows its line, so what a Dragonknight with Shadow gets is Shadow's passives. A swap the game
    /// would not allow is left out of the sheet and named by `Build::subclass_issues`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subclass: Vec<SubclassSwap>,
    /// 50 when the JSON leaves it out (every character with Champion Points is level 50).
    #[serde(default = "default_level")]
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
    /// The ITEM id of the food or drink (not its buff: many foods give the same buff, with a value the server
    /// works out). The GoldenFixture addon fills it in when the buff on the character names exactly one item in
    /// `data::tables::food`; when several foods give that buff it leaves it out and the id is added by hand.
    /// `Build::prepare_for` switches on the buff the item gives.
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
    /// Where the character is: not in a PvP area, in one, or near a Keep (Combat Medic, Battle Resurrection).
    /// `None` = not said, which leaves the location conditions undecided (such an effect is listed as conditional).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    #[serde(default)]
    pub enemy: EnemyPreset,
}

fn default_level() -> u8 {
    50
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
            subclass: vec![],
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
            location: None,
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
        assert_eq!(build.location, None);
        assert!(build.subclass.is_empty(), "no subclassing unless the build says so");
    }

    #[test]
    fn a_subclass_is_optional_and_survives_json() {
        use eso_skill_data::enums::skill_line::SkillLine;
        let line = |name: &str| (0u32..4096).filter_map(|i| SkillLine::from_id(&i)).find(|l| l.as_str() == name).unwrap();
        let mut build = sample();
        assert!(!build.to_json().unwrap().contains("subclass"), "pure class: not written");
        build.subclass = vec![SubclassSwap::new(line("Assassination"), line("Storm Calling"))];
        let json = build.to_json().unwrap();
        assert!(json.contains(r#""replaces": "Assassination""#) && json.contains(r#""with": "Storm Calling""#), "{json}");
        assert_eq!(Build::from_json(&json).unwrap(), build);
        // written by hand: a build file with no subclass key still loads
        let by_hand = r#"{"race":"Nord","class":"Templar","level":50,"attributes":{"health":0,"magicka":64,"stamina":0},
            "subclass":[{"replaces":"Restoring Light","with":"Dark Magic"}]}"#;
        let build = Build::from_json(by_hand).unwrap();
        assert_eq!(build.subclass, [SubclassSwap::new(line("Restoring Light"), line("Dark Magic"))]);
    }

    /// What the GoldenFixture addon (1.1) leaves out of a capture, esosim works out: the item tables give the type
    /// and set of an item id, and CP160 Legendary gear is the default.
    #[test]
    fn a_compact_gear_piece_is_filled_in() {
        let json = r#"{"race":"Nord","class":"Sorcerer","attributes":{"health":64},
            "gear":{"chest":{"item_id":207729,"gear_trait":"ArmorReinforced","enchant":{"glyph":"PrismaticDefense"}},
                    "legs":{"item_id":57714,"gear_trait":"ArmorNirnhoned","quality":"Epic","effective_level":50}}}"#;
        let build = Build::from_json(json).unwrap();
        assert_eq!(build.level, 50, "a level that is left out is 50");
        assert_eq!(build.attributes, Attributes { health: 64, magicka: 0, stamina: 0 });
        let chest = build.gear.chest.unwrap();
        assert_eq!(
            (chest.item_type, chest.set_id, chest.effective_level, chest.quality),
            (ItemType::Heavy, Some(771), 66, ItemQuality::Legendary)
        );
        let enchant = chest.enchant.unwrap();
        assert_eq!(
            (enchant.glyph, enchant.effective_level, enchant.quality),
            (EnchantType::PrismaticDefense, 66, ItemQuality::Legendary)
        );
        let legs = build.gear.legs.unwrap();
        assert_eq!(
            (legs.item_type, legs.set_id, legs.effective_level, legs.quality),
            (ItemType::Heavy, None, 50, ItemQuality::Epic)
        );
    }

    #[test]
    fn a_location_is_optional_and_survives_json() {
        let mut build = sample();
        assert!(!build.to_json().unwrap().contains("location"), "not said, not written");
        build.location = Some(Location::NearKeep);
        assert_eq!(Build::from_json(&build.to_json().unwrap()).unwrap(), build);
    }
}
