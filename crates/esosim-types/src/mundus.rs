//! Mundus stones as plain data (roadmap 2.3).
//!
//! A stone is an ordinary `EffectDef` whose source is `EffectSource::Mundus` (its ability id is the
//! game's "Boon: The Thief" and so on), so there is no stone type here: a *selection* of stones is
//! a list of those ability ids. The numbers are not in the game's ability records (the tooltip says
//! "by <<1>>" and the record carries no usable magnitude), so they are hand-written in the compiler's
//! `overrides.toml`.
//!
//! The calculator never refuses a loadout (what-if builds are the point), so the rules of the stones
//! are reported by [`MundusLoadout::issues`], not enforced, like `CpLoadout::issues`.

use serde::{Deserialize, Serialize};

/// How many stones a character has without any help: the one it last prayed at.
pub const BASE_MUNDUS_STONES: usize = 1;

/// How many stones the character has while the set that allows a second one is worn (Twice-Born
/// Star, set 161: "You can have two Mundus Stone boons at the same time").
pub const MUNDUS_STONES_WITH_EXTRA_SLOT: usize = 2;

/// The stones a build has, as ability ids (`Boon: The Thief` = 13975), in the order they were picked.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct MundusLoadout {
    #[serde(default)]
    pub stones: Vec<u32>,
}

/// What the game would not let a [`MundusLoadout`] do.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MundusIssue {
    /// The id is not a mundus stone of the data.
    NotAStone { id: u32 },
    /// The same stone twice: the game wants two *different* stones, and a repeat adds nothing.
    Duplicate { id: u32 },
    /// More stones than the build has room for.
    TooMany { stones: usize, max: usize },
}

impl MundusLoadout {
    pub fn new(stones: impl IntoIterator<Item = u32>) -> Self {
        Self { stones: stones.into_iter().collect() }
    }

    /// The stones that count: each known stone once, in pick order. A stone past `max_stones`
    /// still counts (see [`MundusIssue::TooMany`]); so does a repeat, once.
    pub fn distinct(&self) -> Vec<u32> {
        let mut seen = Vec::with_capacity(self.stones.len());
        for id in &self.stones {
            if !seen.contains(id) {
                seen.push(*id);
            }
        }
        seen
    }

    /// Everything wrong with the loadout, in pick order: unknown ids, repeats, then too many.
    /// `is_stone` says whether an ability id is a stone; `max_stones` is what the build has room for.
    pub fn issues(&self, is_stone: impl Fn(u32) -> bool, max_stones: usize) -> Vec<MundusIssue> {
        let mut issues = Vec::new();
        let mut seen: Vec<u32> = Vec::new();
        for &id in &self.stones {
            if !is_stone(id) {
                issues.push(MundusIssue::NotAStone { id });
            } else if seen.contains(&id) {
                issues.push(MundusIssue::Duplicate { id });
            }
            seen.push(id);
        }
        let distinct = self.distinct().into_iter().filter(|id| is_stone(*id)).count();
        if distinct > max_stones {
            issues.push(MundusIssue::TooMany { stones: distinct, max: max_stones });
        }
        issues
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn known(id: u32) -> bool {
        matches!(id, 1 | 2 | 3)
    }

    #[test]
    fn a_legal_loadout_has_no_issues() {
        assert_eq!(MundusLoadout::default().issues(known, 1), []);
        assert_eq!(MundusLoadout::new([1]).issues(known, 1), []);
        assert_eq!(MundusLoadout::new([1, 2]).issues(known, 2), []);
    }

    #[test]
    fn a_second_stone_needs_room_for_it() {
        assert_eq!(
            MundusLoadout::new([1, 2]).issues(known, BASE_MUNDUS_STONES),
            [MundusIssue::TooMany { stones: 2, max: 1 }]
        );
    }

    #[test]
    fn repeats_and_unknown_ids_are_reported_and_only_known_stones_are_counted() {
        let l = MundusLoadout::new([1, 1, 9, 2]);
        assert_eq!(
            l.issues(known, 2),
            [MundusIssue::Duplicate { id: 1 }, MundusIssue::NotAStone { id: 9 }]
        );
        assert_eq!(l.distinct(), [1, 9, 2]);
        assert_eq!(l.issues(known, 1), [
            MundusIssue::Duplicate { id: 1 },
            MundusIssue::NotAStone { id: 9 },
            MundusIssue::TooMany { stones: 2, max: 1 },
        ]);
    }

    #[test]
    fn a_loadout_round_trips_and_a_missing_list_is_empty() {
        let l = MundusLoadout::new([13975, 13984]);
        let json = serde_json::to_string(&l).unwrap();
        assert_eq!(json, r#"{"stones":[13975,13984]}"#);
        assert_eq!(serde_json::from_str::<MundusLoadout>(&json).unwrap(), l);
        assert_eq!(serde_json::from_str::<MundusLoadout>("{}").unwrap(), MundusLoadout::default());
    }
}
