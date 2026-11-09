//! Set bonus table.
//!
//! The data is not in this file: it is `crates/esosim-data/data/sets.jsonl`, one set per line,
//! written by `cargo xtask sync-data` from the game's set table (commit it; it is
//! deterministic). This module only parses it. Set *ids* for the hand-written rules that name
//! sets (`grant_rule.rs`) come from the generated `esosim_data::set_ids`, which holds nothing
//! but `u16` constants, so regenerating either file can never stop the compiler from building.

use std::collections::HashMap;
use std::sync::LazyLock;

use serde::Deserialize;

use crate::engine::effect::Channel;

pub use super::set_ids::*;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct SetBonus {
    /// `None`: the compiler saw this stat but has no channel for it yet (the raw stat type
    /// is kept in `sets.jsonl`). The engine ignores it.
    #[serde(default)]
    pub channel: Option<Channel>,
    pub value: f64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Set {
    pub id: u16,
    #[serde(default)]
    pub name: String,
    /// For a perfected set: the id of the regular set it also counts as.
    #[serde(default)]
    pub perfected_of: Option<u16>,
    /// `bonuses[n - 1]` is what the n-th equipped item adds.
    pub bonuses: Vec<Vec<SetBonus>>,
}

#[derive(Debug, Default)]
pub struct SetTable {
    sets: HashMap<u16, Set>,
}

impl SetTable {
    /// Parse `sets.jsonl` (blank lines are skipped; a duplicate id is an error).
    pub fn from_jsonl(text: &str) -> Result<Self, String> {
        let mut sets = HashMap::new();
        for (i, line) in text.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
            let set: Set = serde_json::from_str(line).map_err(|e| format!("line {}: {e}", i + 1))?;
            let id = set.id;
            if sets.insert(id, set).is_some() {
                return Err(format!("line {}: duplicate set id {id}", i + 1));
            }
        }
        Ok(Self { sets })
    }

    pub fn get(&self, id: u16) -> Option<&Set> {
        self.sets.get(&id)
    }

    /// The regular set a perfected set also counts as.
    pub fn unperfected_of(&self, perfected: u16) -> Option<u16> {
        self.sets.get(&perfected)?.perfected_of
    }

    pub fn len(&self) -> usize {
        self.sets.len()
    }

    pub fn is_empty(&self) -> bool {
        self.sets.is_empty()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Set> {
        self.sets.values()
    }
}

pub static SETS: LazyLock<SetTable> = LazyLock::new(|| {
    SetTable::from_jsonl(esosim_data::SETS_JSONL)
        .unwrap_or_else(|e| panic!("esosim-data sets.jsonl: {e}"))
});

#[cfg(test)]
mod tests {
    use esosim_types::DamageDoneContext::All;

use crate::{
        data::enums::gear::{GearSlot, ItemQuality, ItemType},
        engine::{aggregator::ChannelSet, effect::*},
        entity::{gear::GearPiece, player::Player},
    };
    use super::*;

    const SET_CRITICAL_CHANCE_DEFAULT: u32 = 657;
    const SET_POWER_DEFAULT: u32 = 129;

    #[test]
    fn reawakened_hierophant_magicka_bonus_is_summed_correctly() {
        let mut channels = ChannelSet::new();
        let mut player = Player::new();
        let hierophant_item = GearPiece::new(ItemType::Heavy, 66, None, ItemQuality::Legendary, Some(REAWAKENED_HIEROPHANT_SET_ID), None);
        let hierophant_weapon = GearPiece::new(ItemType::LightningStaff, 66, None, ItemQuality::Legendary, Some(REAWAKENED_HIEROPHANT_SET_ID), None);
        player.set_gear_piece(GearSlot::Chest, hierophant_item.clone());
        player.set_gear_piece(GearSlot::Legs, hierophant_item.clone());
        player.set_gear_piece(GearSlot::Hands, hierophant_item.clone());
        player.set_gear_piece(GearSlot::MainHand, hierophant_weapon);
        player.apply_set_bonuses(&mut channels);
        assert_eq!(channels.get(Channel::Resource(ResourceKind::Magicka, AggKind::Additive)), 731 * 3);
    }

    #[test]
    fn reawakened_coral_riptide_perfected_bonus_is_summed_correctly() {
        let mut channels = ChannelSet::new();
        let mut player = Player::new();
        let armour = GearPiece::new(ItemType::Heavy, 66, None, ItemQuality::Legendary, Some(PERFECTED_CORAL_RIPTIDE_SET_ID), None);
        let weapon = GearPiece::new(ItemType::LightningStaff, 66, None, ItemQuality::Legendary, Some(PERFECTED_CORAL_RIPTIDE_SET_ID), None);
        player.set_gear_piece(GearSlot::Chest, armour.clone());
        player.set_gear_piece(GearSlot::Legs, armour.clone());
        player.set_gear_piece(GearSlot::Hands, armour.clone());
        player.set_gear_piece(GearSlot::MainHand, weapon);
        player.apply_set_bonuses(&mut channels);
        assert!(player.get_active_sets_counts().contains_key(&CORAL_RIPTIDE_SET_ID));
        assert_eq!(channels.get(Channel::CriticalChance), (SET_CRITICAL_CHANCE_DEFAULT * 2) as i64);
        assert_eq!(channels.get(Channel::Power(All, AggKind::Additive)), SET_POWER_DEFAULT as i64);
    }

    #[test]
    fn the_shipped_table_parses_and_every_perfected_set_points_at_a_real_set() {
        assert!(!SETS.is_empty());
        for set in SETS.iter() {
            if let Some(base) = set.perfected_of {
                assert!(SETS.get(base).is_some(), "set {} is the perfected version of missing set {base}", set.id);
            }
        }
    }

    fn counts(pieces: &[(GearSlot, ItemType, u16)]) -> HashMap<u16, u8> {
        let mut player = Player::new();
        for (slot, item_type, set_id) in pieces {
            let piece = GearPiece::new(*item_type, 66, None, ItemQuality::Legendary, Some(*set_id), None);
            player.set_gear_piece(*slot, piece);
        }
        player.get_active_sets_counts()
    }

    /// Roadmap 2.1d golden fixture: perfected pieces count as the regular set as well, regular
    /// pieces never count as the perfected set (a two-handed weapon is 2 pieces).
    #[test]
    fn perfected_pieces_count_as_regular_but_not_the_other_way_round() {
        use GearSlot::*;
        let (reg, perf) = (CORAL_RIPTIDE_SET_ID, PERFECTED_CORAL_RIPTIDE_SET_ID);

        let all_regular = counts(&[(Chest, ItemType::Heavy, reg), (Legs, ItemType::Heavy, reg), (Hands, ItemType::Heavy, reg), (MainHand, ItemType::LightningStaff, reg)]);
        assert_eq!((all_regular.get(&reg), all_regular.get(&perf)), (Some(&5), None));

        let all_perfected = counts(&[(Chest, ItemType::Heavy, perf), (Legs, ItemType::Heavy, perf), (Hands, ItemType::Heavy, perf), (MainHand, ItemType::LightningStaff, perf)]);
        assert_eq!((all_perfected.get(&reg), all_perfected.get(&perf)), (Some(&5), Some(&5)));

        let mixed = counts(&[(Chest, ItemType::Heavy, perf), (Legs, ItemType::Heavy, perf), (Hands, ItemType::Heavy, reg)]);
        assert_eq!((mixed.get(&reg), mixed.get(&perf)), (Some(&3), Some(&2)));
    }

    /// Roadmap 2.1d golden fixture: arena weapons are counted on weapons, and a two-handed weapon
    /// is two pieces. Perfected Caustic Arrow (316's perfected set) adds 103 Power at 2 pieces.
    #[test]
    fn a_two_handed_arena_weapon_alone_reaches_the_two_piece_row() {
        use GearSlot::*;
        let (reg, perf) = (CAUSTIC_ARROW_SET_ID, PERFECTED_CAUSTIC_ARROW_SET_ID);
        let two_perfected = |item| {
            let mut p = Player::new();
            for slot in [MainHand, OffHand] {
                p.set_gear_piece(slot, GearPiece::new(item, 66, None, ItemQuality::Legendary, Some(perf), None));
            }
            p
        };

        // one bow: 2 pieces of the perfected set and of the regular set it also counts as
        let mut bow = Player::new();
        bow.set_gear_piece(MainHand, GearPiece::new(ItemType::Bow, 66, None, ItemQuality::Legendary, Some(perf), None));
        let c = bow.get_active_sets_counts();
        assert_eq!((c.get(&reg), c.get(&perf)), (Some(&2), Some(&2)));
        let mut channels = ChannelSet::new();
        bow.apply_set_bonuses(&mut channels);
        assert_eq!(channels.get(Channel::Power(All, AggKind::Additive)), 103);

        // two one-handed weapons also make 2 (dual wield, or weapon + shield)
        let mut channels = ChannelSet::new();
        two_perfected(ItemType::Dagger).apply_set_bonuses(&mut channels);
        assert_eq!(channels.get(Channel::Power(All, AggKind::Additive)), 103);

        // one one-handed perfected weapon is 1 piece: the ability, but not the 2-piece row
        let mut one = Player::new();
        one.set_gear_piece(MainHand, GearPiece::new(ItemType::Dagger, 66, None, ItemQuality::Legendary, Some(perf), None));
        let mut channels = ChannelSet::new();
        one.apply_set_bonuses(&mut channels);
        assert_eq!((one.get_active_sets_counts().get(&reg), channels.get(Channel::Power(All, AggKind::Additive))), (Some(&1), 0));

        // perfected + regular: 2 of the regular set, but only 1 perfected, so no 2-piece row
        let mut mixed = Player::new();
        mixed.set_gear_piece(MainHand, GearPiece::new(ItemType::Dagger, 66, None, ItemQuality::Legendary, Some(perf), None));
        mixed.set_gear_piece(OffHand, GearPiece::new(ItemType::Dagger, 66, None, ItemQuality::Legendary, Some(reg), None));
        let c = mixed.get_active_sets_counts();
        assert_eq!((c.get(&reg), c.get(&perf)), (Some(&2), Some(&1)));
        let mut channels = ChannelSet::new();
        mixed.apply_set_bonuses(&mut channels);
        assert_eq!(channels.get(Channel::Power(All, AggKind::Additive)), 0);
    }

    /// Only perfected sets point at a regular set; never the other way round (roadmap 2.1d).
    #[test]
    fn perfected_links_run_one_way() {
        let mut links = 0;
        for set in SETS.iter() {
            if let Some(regular) = set.perfected_of {
                links += 1;
                assert_eq!(SETS.unperfected_of(regular), None, "set {regular} is both regular and perfected");
            }
        }
        assert_eq!(links, 62);
    }

    #[test]
    fn every_generated_set_id_constant_is_in_the_table() {
        for id in [REAWAKENED_HIEROPHANT_SET_ID, CORAL_RIPTIDE_SET_ID, PERFECTED_CORAL_RIPTIDE_SET_ID] {
            assert!(SETS.get(id).is_some(), "set id {id} missing from sets.jsonl");
        }
    }

    #[test]
    fn parses_typed_channels_and_unmapped_stats() {
        let t = SetTable::from_jsonl(concat!(
            r#"{"id":1,"bonuses":[[],[{"channel":{"Resource":["Health","Additive"]},"value":1206.0}],[{"channel":null,"stat":21,"value":3.0}]]}"#, "\n",
            "\n",
            r#"{"id":2,"perfected_of":1,"name":"Perfected X","bonuses":[[{"channel":{"CostReduction":"Ultimate"},"value":5.0}]]}"#, "\n",
        ))
        .unwrap();
        assert_eq!(t.len(), 2);
        assert_eq!(t.get(1).unwrap().bonuses[1][0].channel, Some(Channel::Resource(ResourceKind::Health, AggKind::Additive)));
        assert_eq!(t.get(1).unwrap().bonuses[2][0].channel, None);
        assert_eq!(t.unperfected_of(2), Some(1));
        assert_eq!(t.unperfected_of(1), None);
        assert_eq!(
            t.get(2).unwrap().bonuses[0][0].channel,
            Some(Channel::CostReduction(eso_skill_data::enums::mechanic::Mechanic::Ultimate))
        );
    }

    #[test]
    fn bad_input_is_an_error_with_a_line_number() {
        let e = SetTable::from_jsonl("{\"id\":1,\"bonuses\":[]}\n{\"id\":1,\"bonuses\":[]}\n").unwrap_err();
        assert!(e.contains("line 2") && e.contains("duplicate"), "{e}");
        let e = SetTable::from_jsonl("{\"id\":1,\"bonuses\":[[{\"channel\":\"NoSuchChannel\",\"value\":1.0}]]}\n").unwrap_err();
        assert!(e.contains("line 1"), "{e}");
    }
}
