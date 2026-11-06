//! Loadout rules that come from a set's *kind* (roadmap 2.1d).
//!
//! The calculator never refuses a loadout (what-if builds are the point), so these are reported,
//! not enforced: [`Player::set_issues`] lists what the game would not let the player do.

use crate::data::enums::gear::GearSlot;
use crate::data::tables::set_kind::set_kind;
use crate::entity::player::Player;
use esosim_types::SetKind;

/// Every slot, both bars. (`GearSlot` has no iterator of its own.)
const ALL_SLOTS: [GearSlot; 16] = [
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
    GearSlot::MainHandBackup,
    GearSlot::Poison,
    GearSlot::OffHand,
    GearSlot::OffHandBackup,
    GearSlot::BackupPoison,
];

fn is_weapon_slot(slot: GearSlot) -> bool {
    matches!(slot, GearSlot::MainHand | GearSlot::MainHandBackup | GearSlot::OffHand | GearSlot::OffHandBackup)
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetIssue {
    /// Only one mythic item can be worn at a time. Lists every mythic piece found.
    MoreThanOneMythic { pieces: Vec<(GearSlot, u16)> },
    /// Monster sets exist only as head and shoulder pieces.
    MonsterPieceInWrongSlot { set_id: u16, slot: GearSlot },
    /// Arena weapon sets exist only as weapons and shields.
    ArenaWeaponInWrongSlot { set_id: u16, slot: GearSlot },
}

impl Player {
    /// What is wrong with the equipped sets, on either bar, in slot order. Empty for a legal build.
    pub fn set_issues(&self) -> Vec<SetIssue> {
        let mut issues = Vec::new();
        let mut mythics = Vec::new();
        for slot in ALL_SLOTS {
            let Some(set_id) = self.gear.get_gear_piece(&slot).and_then(|p| p.set_id) else { continue };
            match set_kind(set_id) {
                SetKind::Mythic => mythics.push((slot, set_id)),
                SetKind::Monster if !matches!(slot, GearSlot::Head | GearSlot::Shoulders) => {
                    issues.push(SetIssue::MonsterPieceInWrongSlot { set_id, slot })
                }
                SetKind::ArenaWeapon if !is_weapon_slot(slot) => {
                    issues.push(SetIssue::ArenaWeaponInWrongSlot { set_id, slot })
                }
                _ => {}
            }
        }
        if mythics.len() > 1 {
            issues.push(SetIssue::MoreThanOneMythic { pieces: mythics });
        }
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::enums::gear::{ItemQuality, ItemType};
    use crate::data::tables::set_ids::*;
    use crate::entity::gear::GearPiece;

    fn wear(player: &mut Player, slot: GearSlot, item: ItemType, set_id: Option<u16>) {
        player.set_gear_piece(slot, GearPiece::new(item, 66, None, ItemQuality::Legendary, set_id, None));
    }

    #[test]
    fn an_ordinary_build_has_no_issues() {
        let mut p = Player::new();
        wear(&mut p, GearSlot::Chest, ItemType::Heavy, Some(ANSUULS_TORMENT_SET_ID));
        wear(&mut p, GearSlot::Head, ItemType::Heavy, Some(MIGHTY_CHUDAN_SET_ID));
        wear(&mut p, GearSlot::Shoulders, ItemType::Heavy, Some(MIGHTY_CHUDAN_SET_ID));
        wear(&mut p, GearSlot::Ring1, ItemType::Medium, Some(OAKENSOUL_RING_SET_ID));
        wear(&mut p, GearSlot::MainHand, ItemType::Bow, Some(PERFECTED_CAUSTIC_ARROW_SET_ID));
        wear(&mut p, GearSlot::OffHandBackup, ItemType::Shield, Some(DISCIPLINED_SLASH_SET_ID));
        assert_eq!(p.set_issues(), []);
        assert_eq!(Player::new().set_issues(), []);
    }

    #[test]
    fn two_mythics_are_reported_with_where_they_are() {
        let mut p = Player::new();
        wear(&mut p, GearSlot::Ring1, ItemType::Medium, Some(OAKENSOUL_RING_SET_ID));
        wear(&mut p, GearSlot::Necklace, ItemType::Medium, Some(GAZE_OF_SITHIS_SET_ID));
        assert_eq!(
            p.set_issues(),
            [SetIssue::MoreThanOneMythic {
                pieces: vec![(GearSlot::Necklace, GAZE_OF_SITHIS_SET_ID), (GearSlot::Ring1, OAKENSOUL_RING_SET_ID)]
            }]
        );
        // the same mythic in both ring slots is still two mythic items
        let mut p = Player::new();
        wear(&mut p, GearSlot::Ring1, ItemType::Medium, Some(OAKENSOUL_RING_SET_ID));
        wear(&mut p, GearSlot::Ring2, ItemType::Medium, Some(OAKENSOUL_RING_SET_ID));
        assert_eq!(p.set_issues().len(), 1);
    }

    #[test]
    fn a_monster_piece_is_only_legal_on_head_and_shoulders() {
        let mut p = Player::new();
        wear(&mut p, GearSlot::Chest, ItemType::Heavy, Some(SLIMECRAW_SET_ID));
        wear(&mut p, GearSlot::Head, ItemType::Heavy, Some(SLIMECRAW_SET_ID));
        assert_eq!(p.set_issues(), [SetIssue::MonsterPieceInWrongSlot { set_id: SLIMECRAW_SET_ID, slot: GearSlot::Chest }]);
    }

    #[test]
    fn an_arena_weapon_is_only_legal_in_a_weapon_slot_on_either_bar() {
        let mut p = Player::new();
        for slot in [GearSlot::MainHand, GearSlot::OffHand, GearSlot::MainHandBackup, GearSlot::OffHandBackup] {
            wear(&mut p, slot, ItemType::Dagger, Some(CAUSTIC_ARROW_SET_ID));
        }
        assert_eq!(p.set_issues(), []);
        wear(&mut p, GearSlot::Feet, ItemType::Light, Some(PERFECTED_CAUSTIC_ARROW_SET_ID));
        assert_eq!(
            p.set_issues(),
            [SetIssue::ArenaWeaponInWrongSlot { set_id: PERFECTED_CAUSTIC_ARROW_SET_ID, slot: GearSlot::Feet }]
        );
    }

    #[test]
    fn gear_without_a_set_is_ignored() {
        let mut p = Player::new();
        wear(&mut p, GearSlot::Chest, ItemType::Heavy, None);
        assert_eq!(p.set_issues(), []);
    }
}
