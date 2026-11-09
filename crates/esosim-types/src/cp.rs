//! Champion Point stars as the game stores them (roadmap 2.2a).
//!
//! One [`CpStar`] per record of the CP table (manifest id `0x6000_01AB`). The *effect* of a star
//! is not here: like any other buff it is the ability `ability_id` (tooltip, coefficients and all
//! live in `bundle.jsonl`). What the table adds is the spending rules: how many stages, what each
//! stage costs, and the number each stage is worth.

use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

/// How many slottable stars one tree can have slotted at once. The game has had four for ten years;
/// it is written here, not read from the table, which does not say.
pub const CP_SLOTS_PER_TREE: usize = 4;

/// The game's `perk_type` byte of a CP record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CpPerk {
    /// `0`: a plain star.
    Passive,
    /// `1`: a slottable star (the table calls it "active").
    Active,
    /// `2`: the always-on "base active" stars (Fortified, Boundless Vitality, ...).
    BaseActive,
}

impl CpPerk {
    /// `None` for a `perk_type` the parser does not know.
    pub fn from_game(perk_type: u8) -> Option<CpPerk> {
        match perk_type {
            0 => Some(CpPerk::Passive),
            1 => Some(CpPerk::Active),
            2 => Some(CpPerk::BaseActive),
            _ => None,
        }
    }
}

/// One star of the CP table: a record with an ability and a position on the map.
/// (The table's other records cannot be shown in the CP menu and are not stars.)
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CpStar {
    /// The record's own id (`skill_id1`).
    pub id: u32,
    /// The ability's name: the table's own name field is always empty.
    pub name: String,
    /// The ability that carries the effect.
    pub ability_id: u32,
    pub perk: CpPerk,
    pub max_stages: u32,
    pub cost_per_stage: u32,
    /// What one stage is worth, as the game stores it.
    pub value_per_stage: f32,
    /// Prerequisite links (`link_ids`, then `link_ids2`), star ids.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<u32>,
    /// The tree the star is in (`cp_trees.toml`, worked out when the data is synced). Empty when
    /// no trees are configured, in which case the tree rules cannot be checked.
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub tree: String,
    /// A star that can be bought with no connected star (a way into its tree; `cp_trees.toml`).
    /// Stars with no links at all need no connection either.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub entry: bool,
    #[serde(default)]
    pub pos_x: f32,
    #[serde(default)]
    pub pos_y: f32,
}

impl CpStar {
    /// Points needed to buy every stage.
    pub fn max_points(&self) -> u32 {
        self.max_stages * self.cost_per_stage
    }

    /// Stages bought by `points` spent on this star (never above `max_stages`).
    pub fn stages_for(&self, points: u32) -> u32 {
        if self.cost_per_stage == 0 { 0 } else { (points / self.cost_per_stage).min(self.max_stages) }
    }
}

/// What a player has done on the CP map (roadmap 2.2b): points spent per star, and which
/// slottable stars sit in the bar. Plain input data; the rules that turn it into stages are
/// [`CpLoadout::stages_in_effect`], and [`CpLoadout::issues`] reports what the game would not allow.
/// Like set issues, these are reported and never enforced: what-if builds are the point of the calculator.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct CpLoadout {
    /// Points spent on each star, keyed by star id (`CpStar::id`). A star that is absent has none.
    #[serde(default)]
    pub points: BTreeMap<u32, u32>,
    /// Star ids in the slots. Only [`CpPerk::Active`] stars care about this.
    #[serde(default)]
    pub slotted: Vec<u32>,
}

impl CpLoadout {
    /// Stages of `star` that bought points give (never above `max_stages`), slotted or not.
    pub fn stages_bought(&self, star: &CpStar) -> u32 {
        star.stages_for(self.points.get(&star.id).copied().unwrap_or(0))
    }

    pub fn is_slotted(&self, star_id: u32) -> bool {
        self.slotted.contains(&star_id)
    }

    /// Stages of `star` that actually apply. Passive and base-active stars apply for every
    /// stage bought; a slottable star only while it is slotted. Nothing else is checked here (not
    /// the slot cap, not the prerequisites): see [`CpLoadout::issues`].
    pub fn stages_in_effect(&self, star: &CpStar) -> u32 {
        match star.perk {
            CpPerk::Passive | CpPerk::BaseActive => self.stages_bought(star),
            CpPerk::Active if self.is_slotted(star.id) => self.stages_bought(star),
            CpPerk::Active => 0,
        }
    }

    /// What the game would not let the player do, in a fixed order: unknown stars and wasted points
    /// by star id, then slots in slot order, then too many slots per tree, then missing
    /// prerequisites by star id. Empty for a legal loadout.
    ///
    /// * Prerequisites: a star with points needs a linked star that has at least one stage bought
    ///   (and that star must itself be connected, back to a way in), unless it is an `entry` star or
    ///   has no links at all (the always-on "base" stars). A tree is only checked when it names at
    ///   least one entry star: without one there is no way into it and every purchase would fail.
    /// * Slots: at most [`CP_SLOTS_PER_TREE`] per tree; only trees known from `tree` are checked.
    pub fn issues(&self, stars: &BTreeMap<u32, CpStar>) -> Vec<CpIssue> {
        let mut issues = Vec::new();

        for (&star_id, &spent) in &self.points {
            match stars.get(&star_id) {
                None => issues.push(CpIssue::UnknownStar { star_id }),
                Some(star) if spent > star.max_points() => {
                    issues.push(CpIssue::PointsOverMax { star_id, spent, max: star.max_points() })
                }
                Some(_) => {}
            }
        }

        let mut seen = BTreeSet::new();
        let mut per_tree: BTreeMap<&str, usize> = BTreeMap::new();
        for &star_id in &self.slotted {
            if !seen.insert(star_id) {
                continue;
            }
            let Some(star) = stars.get(&star_id) else {
                issues.push(CpIssue::UnknownStar { star_id });
                continue;
            };
            if star.perk != CpPerk::Active {
                issues.push(CpIssue::SlottedNotSlottable { star_id });
            } else if self.stages_bought(star) == 0 {
                issues.push(CpIssue::SlottedWithoutStages { star_id });
            } else if !star.tree.is_empty() {
                *per_tree.entry(star.tree.as_str()).or_insert(0) += 1;
            }
        }
        for (tree, slotted) in per_tree {
            if slotted > CP_SLOTS_PER_TREE {
                issues.push(CpIssue::TooManySlotted { tree: tree.to_string(), slotted, max: CP_SLOTS_PER_TREE });
            }
        }

        issues.extend(self.missing_prerequisites(stars).into_iter().map(|star_id| CpIssue::MissingPrerequisite { star_id }));
        issues
    }

    /// Stars with points that cannot be reached from a way into their tree, by star id.
    fn missing_prerequisites(&self, stars: &BTreeMap<u32, CpStar>) -> Vec<u32> {
        let checked_trees: BTreeSet<&str> =
            stars.values().filter(|s| s.entry && !s.tree.is_empty()).map(|s| s.tree.as_str()).collect();
        let checked = |s: &CpStar| checked_trees.contains(s.tree.as_str());

        // Stars whose first stage is bought and that are connected to a way in, grown outwards from
        // the stars that need no connection. Two stars that only link to each other never count.
        let mut reached: BTreeSet<u32> = stars
            .values()
            .filter(|s| checked(s) && (s.entry || s.links.is_empty()) && self.stages_bought(s) >= 1)
            .map(|s| s.id)
            .collect();
        loop {
            let more: Vec<u32> = stars
                .values()
                .filter(|s| checked(s) && !reached.contains(&s.id) && self.stages_bought(s) >= 1)
                .filter(|s| s.links.iter().any(|l| reached.contains(l)))
                .map(|s| s.id)
                .collect();
            if more.is_empty() {
                break;
            }
            reached.extend(more);
        }

        stars
            .values()
            .filter(|s| checked(s) && !s.entry && !s.links.is_empty())
            .filter(|s| self.points.get(&s.id).copied().unwrap_or(0) > 0)
            .filter(|s| !s.links.iter().any(|l| reached.contains(l)))
            .map(|s| s.id)
            .collect()
    }
}

/// Something in a [`CpLoadout`] that the game would not allow (see [`CpLoadout::issues`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CpIssue {
    /// A star id (spent on or slotted) that is not in the table.
    UnknownStar { star_id: u32 },
    /// More points than the star can take: the extra ones do nothing.
    PointsOverMax { star_id: u32, spent: u32, max: u32 },
    /// A slotted star that is passive or always on: only [`CpPerk::Active`] stars have a slot.
    SlottedNotSlottable { star_id: u32 },
    /// A slotted star with no stage bought: it does nothing.
    SlottedWithoutStages { star_id: u32 },
    /// More slottable stars in one tree than it has slots.
    TooManySlotted { tree: String, slotted: usize, max: usize },
    /// Points on a star with no connected star that has a stage.
    MissingPrerequisite { star_id: u32 },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn star(id: u32, perk: CpPerk, cost: u32, stages: u32, tree: &str, links: &[u32]) -> CpStar {
        CpStar {
            id,
            name: format!("star {id}"),
            ability_id: 1000 + id,
            perk,
            max_stages: stages,
            cost_per_stage: cost,
            value_per_stage: 1.0,
            links: links.to_vec(),
            tree: tree.to_string(),
            entry: false,
            pos_x: 1.0,
            pos_y: 1.0,
        }
    }

    fn table(stars: Vec<CpStar>) -> BTreeMap<u32, CpStar> {
        stars.into_iter().map(|s| (s.id, s)).collect()
    }

    fn loadout(points: &[(u32, u32)], slotted: &[u32]) -> CpLoadout {
        CpLoadout { points: points.iter().copied().collect(), slotted: slotted.to_vec() }
    }

    #[test]
    fn stages_are_points_over_cost_and_capped() {
        let s = star(1, CpPerk::Passive, 10, 2, "T", &[]);
        assert_eq!([0, 9, 10, 19, 20, 500].map(|p| s.stages_for(p)), [0, 0, 1, 1, 2, 2]);
        assert_eq!(s.max_points(), 20);
    }

    #[test]
    fn only_slotted_active_stars_apply() {
        let passive = star(1, CpPerk::Passive, 10, 2, "T", &[]);
        let base = star(2, CpPerk::BaseActive, 1, 50, "T", &[]);
        let active = star(3, CpPerk::Active, 10, 5, "T", &[]);
        let l = loadout(&[(1, 20), (2, 30), (3, 50)], &[]);
        assert_eq!([l.stages_in_effect(&passive), l.stages_in_effect(&base), l.stages_in_effect(&active)], [2, 30, 0]);
        let l = loadout(&[(1, 20), (2, 30), (3, 50)], &[3]);
        assert_eq!(l.stages_in_effect(&active), 5);
    }

    #[test]
    fn a_star_serialises_without_its_empty_tree_and_entry_flag() {
        let s = star(1, CpPerk::Passive, 10, 2, "", &[]);
        let json = serde_json::to_string(&s).unwrap();
        assert!(!json.contains("tree") && !json.contains("entry"), "{json}");
        let back: CpStar = serde_json::from_str(&json).unwrap();
        assert_eq!(back, s);
        let mut s = s;
        (s.tree, s.entry) = ("Warfare".into(), true);
        let back: CpStar = serde_json::from_str(&serde_json::to_string(&s).unwrap()).unwrap();
        assert_eq!(back, s);
    }

    /// Four actives and a fifth in one tree, one active in another.
    fn slot_table() -> BTreeMap<u32, CpStar> {
        table(vec![
            star(1, CpPerk::Active, 10, 5, "A", &[]),
            star(2, CpPerk::Active, 10, 5, "A", &[]),
            star(3, CpPerk::Active, 10, 5, "A", &[]),
            star(4, CpPerk::Active, 10, 5, "A", &[]),
            star(5, CpPerk::Active, 10, 5, "A", &[]),
            star(6, CpPerk::Active, 10, 5, "B", &[]),
            star(7, CpPerk::Passive, 10, 2, "B", &[]),
        ])
    }

    #[test]
    fn four_slots_per_tree_is_fine_and_five_is_reported() {
        let stars = slot_table();
        let pts: Vec<(u32, u32)> = (1..=6).map(|i| (i, 10)).collect();
        assert_eq!(loadout(&pts, &[1, 2, 3, 4, 6]).issues(&stars), vec![]);
        assert_eq!(
            loadout(&pts, &[1, 2, 3, 4, 5, 6]).issues(&stars),
            vec![CpIssue::TooManySlotted { tree: "A".into(), slotted: 5, max: 4 }]
        );
        // slotting the same star twice is one slot
        assert_eq!(loadout(&pts, &[1, 1, 2, 3, 4]).issues(&stars), vec![]);
    }

    #[test]
    fn slots_need_an_active_star_with_a_stage() {
        let stars = slot_table();
        let l = loadout(&[(1, 10), (7, 20)], &[1, 2, 7, 99]);
        assert_eq!(
            l.issues(&stars),
            vec![
                CpIssue::SlottedWithoutStages { star_id: 2 },
                CpIssue::SlottedNotSlottable { star_id: 7 },
                CpIssue::UnknownStar { star_id: 99 },
            ]
        );
    }

    #[test]
    fn unknown_stars_and_wasted_points_are_reported_in_star_order() {
        let stars = slot_table();
        let l = loadout(&[(7, 21), (1, 51), (42, 5)], &[]);
        assert_eq!(
            l.issues(&stars),
            vec![
                CpIssue::PointsOverMax { star_id: 1, spent: 51, max: 50 },
                CpIssue::PointsOverMax { star_id: 7, spent: 21, max: 20 },
                CpIssue::UnknownStar { star_id: 42 },
            ]
        );
    }

    /// entry 1 -- 2 -- 3, and a pair 8 -- 9 that only link to each other; 4 is a base star.
    fn prerequisite_table(entry: bool) -> BTreeMap<u32, CpStar> {
        let mut stars = table(vec![
            star(1, CpPerk::Passive, 10, 2, "T", &[2]),
            star(2, CpPerk::Passive, 10, 2, "T", &[1, 3]),
            star(3, CpPerk::Passive, 50, 1, "T", &[2]),
            star(4, CpPerk::BaseActive, 1, 50, "T", &[]),
            star(8, CpPerk::Passive, 10, 2, "T", &[9]),
            star(9, CpPerk::Passive, 10, 2, "T", &[8]),
        ]);
        stars.get_mut(&1).unwrap().entry = entry;
        stars
    }

    #[test]
    fn a_star_needs_a_connected_star_with_a_stage() {
        let stars = prerequisite_table(true);
        // 1 is an entry, 2 hangs off 1, 3 off 2; base stars need nothing
        assert_eq!(loadout(&[(1, 10), (2, 10), (3, 50), (4, 7)], &[]).issues(&stars), vec![]);
        // 3 without 2; 2 without 1
        assert_eq!(
            loadout(&[(1, 10), (3, 50)], &[]).issues(&stars),
            vec![CpIssue::MissingPrerequisite { star_id: 3 }]
        );
        assert_eq!(loadout(&[(2, 10)], &[]).issues(&stars), vec![CpIssue::MissingPrerequisite { star_id: 2 }]);
    }

    #[test]
    fn the_connected_star_needs_a_whole_stage_not_just_points() {
        let stars = prerequisite_table(true);
        // 9 of the 10 points star 1 needs for its first stage
        assert_eq!(loadout(&[(1, 9), (2, 10)], &[]).issues(&stars), vec![CpIssue::MissingPrerequisite { star_id: 2 }]);
    }

    #[test]
    fn stars_that_only_link_to_each_other_are_not_connected() {
        let stars = prerequisite_table(true);
        assert_eq!(
            loadout(&[(8, 10), (9, 10)], &[]).issues(&stars),
            vec![CpIssue::MissingPrerequisite { star_id: 8 }, CpIssue::MissingPrerequisite { star_id: 9 }]
        );
    }

    #[test]
    fn a_tree_with_no_entry_star_is_not_checked() {
        let stars = prerequisite_table(false);
        assert_eq!(loadout(&[(3, 50), (9, 10)], &[]).issues(&stars), vec![]);
    }
}
