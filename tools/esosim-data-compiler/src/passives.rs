//! Passive families from `base_ability_id` (roadmap 2.4).
//!
//! Every rank of a passive above the first carries the ability id of its rank 1 in the first entry of its
//! ability record's list 25 (`SkillData34::base_ability_id`, the same field that links a morph to its base
//! skill). The ranks of one passive are therefore the abilities that share a base, plus the base itself.
//! This replaces the old guess ("records of the same name and the same skill line, in id order"), which
//! `passive_ranks.csv` was written with by script.
//!
//! * [`families`] groups the ability table by base.
//! * [`ranks_csv`] writes `passive_ranks.csv`: only ranks that have an effect written, and only families with at
//!   least two of those (one rank leaves nothing to choose between).
//!
//! A family is left out when any of its members is a player skill (`player_skill_index != 0`): those are the
//! morphs of an active skill, which share the same link.
//!
//! Ranks are ordered by the `rank_index` in the same list 25 entry (1 for the base, which names itself as its
//! base; 2, 3, ... for the later ranks), with the ability id breaking a tie. A base with no list 25 entry counts
//! as rank 0, so it still comes first.

use std::collections::{BTreeMap, BTreeSet, HashMap};

use eso_skill_data::SkillData34;

/// One passive and its ranks.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PassiveFamily {
    /// The ability id of rank 1 (what every other rank's `base_ability_id` names).
    pub base: u32,
    /// `skill_line_id` of the base record, 0 if it has none or is not in the table.
    pub skill_line: u32,
    /// Ability ids of every rank, the base included, in rank order (`rank_index`, then id).
    pub ranks: Vec<u32>,
}

/// The rank 1 of `id`: follow `base_ability_id` until an ability has none. A cycle or a very long chain stops
/// where it is.
fn root_of(base_of: &HashMap<u32, u32>, id: u32) -> u32 {
    let mut cur = id;
    for _ in 0..16 {
        match base_of.get(&cur) {
            Some(&b) if b != cur => cur = b,
            _ => break,
        }
    }
    cur
}

/// Every group of abilities that share a base, passives only, ordered by base id.
pub fn families(skills: &[SkillData34]) -> Vec<PassiveFamily> {
    let base_of: HashMap<u32, u32> =
        skills.iter().filter_map(|s| s.base_ability_id().map(|b| (s.ability_id1, b))).collect();
    let by_id: HashMap<u32, &SkillData34> = skills.iter().map(|s| (s.ability_id1, s)).collect();
    let rank_of = |id: u32| by_id.get(&id).and_then(|s| s.list25.first()).map_or(0, |e| e.rank_index);

    let mut groups: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    for &id in base_of.keys() {
        let root = root_of(&base_of, id);
        let g = groups.entry(root).or_default();
        g.insert(root);
        g.insert(id);
    }

    groups
        .into_iter()
        .filter(|(_, ids)| !ids.iter().any(|id| by_id.get(id).is_some_and(|s| s.base_data.player_skill_index != 0)))
        .map(|(base, ids)| PassiveFamily {
            base,
            skill_line: by_id.get(&base).map_or(0, |s| s.base_data.skill_line_id),
            ranks: {
                let mut r: Vec<u32> = ids.into_iter().collect();
                r.sort_by_key(|&id| (rank_of(id), id));
                r
            },
        })
        .collect()
}

/// What [`ranks_csv`] left out, for the compile report.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct RanksReport {
    pub families: usize,
    pub rows: usize,
    /// Families with exactly one written rank: nothing to choose between, not written.
    pub single: usize,
    /// `(base id, ability id)` of a rank with no effect in a family that has others: a build at that rank gets
    /// nothing.
    pub gaps: Vec<(u32, u32)>,
    /// `(base id, ability id)` of a rank already in an earlier row; the family is skipped, as the engine
    /// refuses an ability in two rows.
    pub conflicts: Vec<(u32, u32)>,
}

impl RanksReport {
    pub fn summary(&self) -> String {
        let mut s = format!(
            "passive ranks: {} base_ability_id family(ies), {} row(s) written, {} with one written rank only",
            self.families, self.rows, self.single
        );
        if !self.gaps.is_empty() {
            s.push_str(&format!("\npassive ranks with no effect (base id, ability id): {:?}", self.gaps));
        }
        if !self.conflicts.is_empty() {
            s.push_str(&format!("\npassive ranks in two families, skipped (base id, ability id): {:?}", self.conflicts));
        }
        s
    }
}

/// `passive_ranks.csv`: `ability id,ability id,...` one row per passive, lowest rank first, rows by first id.
/// `written` are the ability ids that have an effect; a rank not in it is left out of its row.
pub fn ranks_csv(families: &[PassiveFamily], written: &BTreeSet<u32>) -> (String, RanksReport) {
    let mut report = RanksReport { families: families.len(), ..RanksReport::default() };
    let mut seen: BTreeSet<u32> = BTreeSet::new();
    let mut rows: Vec<Vec<u32>> = Vec::new();
    for f in families {
        let kept: Vec<u32> = f.ranks.iter().copied().filter(|id| written.contains(id)).collect();
        match kept.len() {
            0 => continue,
            1 => {
                report.single += 1;
                continue;
            }
            _ => {}
        }
        if let Some(&dup) = kept.iter().find(|id| seen.contains(id)) {
            report.conflicts.push((f.base, dup));
            continue;
        }
        report.gaps.extend(f.ranks.iter().filter(|id| !written.contains(id)).map(|&id| (f.base, id)));
        seen.extend(kept.iter().copied());
        rows.push(kept);
    }
    rows.sort();
    report.rows = rows.len();
    let mut out = String::new();
    for row in rows {
        let cells: Vec<String> = row.iter().map(u32::to_string).collect();
        out.push_str(&cells.join(","));
        out.push('\n');
    }
    (out, report)
}

#[cfg(test)]
mod tests {
    use super::*;
    use eso_skill_data::List25Entry;

    /// An ability, with a base if `base` is not 0.
    fn skill(id: u32, base: u32) -> SkillData34 {
        ranked(id, base, 0)
    }

    /// An ability with a base and a `rank_index`.
    fn ranked(id: u32, base: u32, rank: u8) -> SkillData34 {
        let mut s = SkillData34::default();
        s.ability_id1 = id;
        if base != 0 {
            s.list25 = vec![List25Entry { base_ability_id: base, rank_index: rank, ..Default::default() }];
        }
        s
    }

    #[test]
    fn ranks_follow_rank_index_not_id_order() {
        // the base names itself (rank 1), as in Recipe Improvement 44590: ranks 2.. name it
        let f = families(&[ranked(70, 90, 3), ranked(90, 90, 1), ranked(80, 90, 2)]);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].base, f[0].ranks.clone()), (90, vec![90, 80, 70]));
    }

    #[test]
    fn the_recipe_improvement_ranks_come_out_in_order() {
        let f = families(&[
            ranked(44599, 44590, 5), ranked(44590, 44590, 1), ranked(44595, 44590, 2),
            ranked(44598, 44590, 4), ranked(44597, 44590, 3),
        ]);
        assert_eq!(f[0].ranks, [44590, 44595, 44597, 44598, 44599]);
    }

    #[test]
    fn ranks_share_the_base_and_the_base_is_rank_one() {
        let f = families(&[skill(45564, 29743), skill(29743, 0), skill(45563, 29743), skill(7, 0)]);
        assert_eq!(f.len(), 1);
        assert_eq!((f[0].base, f[0].ranks.clone()), (29743, vec![29743, 45563, 45564]));
    }

    #[test]
    fn a_chain_of_bases_ends_in_one_family() {
        // 3 names 2, 2 names 1: all one passive
        let f = families(&[skill(1, 0), skill(2, 1), skill(3, 2)]);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].ranks, [1, 2, 3]);
    }

    #[test]
    fn a_family_with_a_player_skill_is_not_a_passive() {
        let mut base = skill(10, 0);
        base.base_data.player_skill_index = 4;
        let f = families(&[base, skill(11, 10), skill(20, 0), skill(21, 20)]);
        assert_eq!(f.len(), 1);
        assert_eq!(f[0].base, 20);
    }

    #[test]
    fn a_scribed_ability_is_not_a_rank() {
        let mut s = skill(11, 10);
        s.base_data.scribing_index = 3; // `base_ability_id()` is None for these
        assert!(families(&[skill(10, 0), s]).is_empty());
    }

    #[test]
    fn the_csv_keeps_written_ranks_and_reports_what_it_drops() {
        let f = families(&[
            skill(29743, 0), skill(45563, 29743), skill(45564, 29743),
            skill(29388, 0), skill(45443, 29388),
            skill(18929, 0), skill(45476, 18929), // only one rank written
            skill(100, 0), skill(101, 100),       // none written
        ]);
        let written: BTreeSet<u32> = [29743, 45563, 45564, 29388, 45443, 18929].into();
        let (csv, r) = ranks_csv(&f, &written);
        assert_eq!(csv, "29388,45443\n29743,45563,45564\n");
        assert_eq!((r.families, r.rows, r.single), (4, 2, 1));
        assert!(r.gaps.is_empty() && r.conflicts.is_empty());
    }

    #[test]
    fn a_rank_without_an_effect_is_a_gap_not_a_row_entry() {
        let f = families(&[skill(1, 0), skill(2, 1), skill(3, 1)]);
        let (csv, r) = ranks_csv(&f, &[1, 3].into());
        assert_eq!(csv, "1,3\n");
        assert_eq!(r.gaps, [(1, 2)]);
        assert!(r.summary().contains("(1, 2)"));
    }
}
