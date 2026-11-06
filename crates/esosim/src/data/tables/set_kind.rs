//! What kind of set each set is (roadmap 2.1d): monster, mythic, arena weapon or regular.
//!
//! The data is `set_behaviour.jsonl` (written by the compiler, see `SetKind` in `esosim-types`
//! for how the kinds are inferred); this module only indexes it by set id.

use std::collections::HashMap;
use std::sync::LazyLock;

use esosim_types::{SetBehaviour, SetKind};

/// Every set that is not `Regular`. A perfected set has its regular set's kind.
pub static SET_KINDS: LazyLock<HashMap<u16, SetKind>> = LazyLock::new(|| {
    set_kinds_from(esosim_data::SET_BEHAVIOUR_JSONL)
        .unwrap_or_else(|e| panic!("esosim-data set_behaviour.jsonl: {e}"))
});

/// The kind of set `set_id` (`Regular` for an id that is not in the table).
pub fn set_kind(set_id: u16) -> SetKind {
    SET_KINDS.get(&set_id).copied().unwrap_or_default()
}

pub fn set_kinds_from(jsonl: &str) -> Result<HashMap<u16, SetKind>, String> {
    let mut out = HashMap::new();
    for (i, line) in jsonl.lines().enumerate().filter(|(_, l)| !l.trim().is_empty()) {
        let set: SetBehaviour = serde_json::from_str(line).map_err(|e| format!("line {}: {e}", i + 1))?;
        let id = u16::try_from(set.id).map_err(|_| format!("line {}: set id {} does not fit u16", i + 1, set.id))?;
        if !set.kind.is_regular() {
            out.insert(id, set.kind);
        }
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::data::tables::set_ids::*;
    use crate::data::tables::sets::SETS;

    fn count(kind: SetKind) -> usize {
        SET_KINDS.values().filter(|k| **k == kind).count()
    }

    /// Pinned so that a change in the data that moves a set between kinds is a loud, reviewed one.
    /// 60 arena weapons are the 30 regular sets and their 30 perfected versions.
    #[test]
    fn the_shipped_counts_of_each_kind() {
        assert_eq!((count(SetKind::Monster), count(SetKind::Mythic), count(SetKind::ArenaWeapon)), (74, 38, 60));
    }

    #[test]
    fn kinds_that_were_checked_against_the_game() {
        // monster sets (head + shoulders): Mighty Chudan, Slimecraw, Immolator Charr, Iceheart
        for id in [MIGHTY_CHUDAN_SET_ID, SLIMECRAW_SET_ID, IMMOLATOR_CHARR_SET_ID] {
            assert_eq!(set_kind(id), SetKind::Monster, "{id}");
        }
        // mythics, including the two the shape alone gets wrong or cannot see
        for id in [OAKENSOUL_RING_SET_ID, GAZE_OF_SITHIS_SET_ID, VELOTHI_UR_MAGES_AMULET_SET_ID, SHAPESHIFTERS_CHAIN_SET_ID] {
            assert_eq!(set_kind(id), SetKind::Mythic, "{id}");
        }
        assert_eq!(set_kind(675), SetKind::Mythic, "Stormweaver's Cavort looks like a monster set but is a mythic");
        // arena weapons, regular and perfected
        for id in [CAUSTIC_ARROW_SET_ID, PERFECTED_CAUSTIC_ARROW_SET_ID, DISCIPLINED_SLASH_SET_ID, PERFECTED_DISCIPLINED_SLASH_SET_ID] {
            assert_eq!(set_kind(id), SetKind::ArenaWeapon, "{id}");
        }
        assert_eq!(set_kind(ANSUULS_TORMENT_SET_ID), SetKind::Regular);
        assert_eq!(set_kind(u16::MAX), SetKind::Regular);
    }

    #[test]
    fn every_listed_set_is_in_the_set_table() {
        for id in SET_KINDS.keys() {
            assert!(SETS.get(*id).is_some(), "set {id} has a kind but no row in sets.jsonl");
        }
    }

    #[test]
    fn a_perfected_set_has_its_regular_sets_kind() {
        for set in SETS.iter() {
            if let Some(regular) = set.perfected_of {
                assert_eq!(set_kind(set.id), set_kind(regular), "{} ({})", set.id, set.name);
            }
        }
    }

    #[test]
    fn bad_lines_are_errors_with_a_line_number() {
        assert!(set_kinds_from("{\"id\":1,\"class\":\"stat_only\",\"kind\":\"wizard\"}\n").unwrap_err().contains("line 1"));
        assert!(set_kinds_from("{\"id\":70000,\"class\":\"stat_only\",\"kind\":\"mythic\"}\n").unwrap_err().contains("u16"));
    }
}
