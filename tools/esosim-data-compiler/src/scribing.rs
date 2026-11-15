//! Scribing grimoires into skill lines (roadmap 2.5).
//!
//! The game gives a scribed ability no skill line (`skill_line_id` is 0), but it does number the grimoire: every
//! record a grimoire produces (the grimoire, each focus-script variant, the effects they cause) has the same
//! `base_data.scribing_index`, 1..=12. `data/grimoires.csv` says which skill line each index belongs to, and
//! [`lines`] puts every record with that index under that line. The result is merged into `skill_lines.csv`
//! (`compile::skill_lines_csv`), so the engine has no scribing code at all: a grimoire's ability is a member of a
//! skill line like any other, and a patch that adds script variants needs no edit here.
//!
//! The table can only be wrong in ways that are caught while compiling: an index the game has and the table lacks
//! (a new grimoire), a row whose index the game no longer uses, a line the parser does not know, an index listed twice.

use std::collections::{BTreeMap, BTreeSet};

use eso_skill_data::SkillData34;
use eso_skill_data::enums::skill_line::SkillLine;

/// The embedded default, `data/grimoires.csv`.
pub const GRIMOIRES_CSV: &str = include_str!("../data/grimoires.csv");

/// One row of the table.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Grimoire {
    pub index: u32,
    pub name: String,
    /// `skill_line_id`, as written in `skill_lines.csv`.
    pub skill_line: u32,
}

/// What [`lines`] found, for the compile report.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct ScribingReport {
    /// Abilities written into a skill line, per grimoire name.
    pub per_grimoire: Vec<(String, usize)>,
}

impl ScribingReport {
    pub fn summary(&self) -> String {
        let total: usize = self.per_grimoire.iter().map(|(_, n)| n).sum();
        let each: Vec<String> = self.per_grimoire.iter().map(|(g, n)| format!("{g} {n}")).collect();
        format!("scribing: {total} abilit(ies) of {} grimoires put in a skill line ({})", self.per_grimoire.len(), each.join(", "))
    }
}

/// The id of the skill line the parser calls `name` (`SkillLine::as_str`), or `None`.
fn line_id_named(name: &str) -> Option<u32> {
    (0u32..4096).find(|id| SkillLine::from_id(id).is_some_and(|l| l.as_str() == name))
}

/// Every skill line name the parser knows, for an error message.
fn known_line_names() -> Vec<&'static str> {
    (0u32..4096).filter_map(|id| SkillLine::from_id(&id)).map(|l| l.as_str()).collect()
}

/// A skill line cell: the parser's name for it, or the id itself.
fn parse_line(cell: &str) -> Result<u32, String> {
    if let Some(id) = line_id_named(cell) {
        return Ok(id);
    }
    match cell.parse::<u32>() {
        Ok(id) if SkillLine::from_id(&id).is_some() => Ok(id),
        _ => Err(format!("no skill line called {cell:?}; the parser knows: {}", known_line_names().join(", "))),
    }
}

/// Parse `grimoires.csv`. Rows are by index; an index listed twice is an error.
pub fn parse(csv: &str) -> Result<Vec<Grimoire>, String> {
    let mut rows: Vec<Grimoire> = Vec::new();
    for (n, row) in csv.lines().enumerate() {
        let row = row.trim();
        if row.is_empty() || row.starts_with('#') {
            continue;
        }
        let at = |msg: String| format!("grimoires.csv line {}: {msg}", n + 1);
        let mut cells = row.split(',').map(str::trim);
        let index_cell = cells.next().unwrap_or("");
        let index: u32 = index_cell.parse().map_err(|_| at(format!("bad scribing_index {index_cell:?}")))?;
        if index == 0 {
            return Err(at("scribing_index 0 means \"not scribed\"".into()));
        }
        let name = cells.next().filter(|c| !c.is_empty()).ok_or_else(|| at("no grimoire name".into()))?;
        let line_cell = cells.next().filter(|c| !c.is_empty()).ok_or_else(|| at("no skill line".into()))?;
        let skill_line = parse_line(line_cell).map_err(at)?;
        if let Some(prev) = rows.iter().find(|g| g.index == index) {
            return Err(at(format!("scribing_index {index} is already `{}`", prev.name)));
        }
        rows.push(Grimoire { index, name: name.to_string(), skill_line });
    }
    Ok(rows)
}

/// `skill line id -> ability ids` (ascending) for every ability the game numbers as part of a grimoire and puts in no
/// skill line. An ability that already has a line (should the game ever give them one) is left to `skill_lines.csv`.
pub fn lines(skills: &[SkillData34], table: &[Grimoire]) -> Result<(BTreeMap<u32, Vec<u32>>, ScribingReport), String> {
    let mut by_index: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    for s in skills.iter().filter(|s| s.base_data.scribing_index != 0 && s.base_data.skill_line_id == 0) {
        by_index.entry(s.base_data.scribing_index).or_default().insert(s.ability_id1);
    }

    if let Some((index, ids)) = by_index.iter().find(|(i, _)| !table.iter().any(|g| g.index == **i)) {
        return Err(format!(
            "scribing_index {index} ({} abilities, e.g. {}) is not in grimoires.csv: a new grimoire? Add `{index}, <name>, <skill line>`",
            ids.len(),
            ids.iter().next().copied().unwrap_or(0)
        ));
    }

    let mut out: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    let mut report = ScribingReport::default();
    for g in table {
        let Some(ids) = by_index.get(&g.index) else {
            return Err(format!(
                "grimoires.csv lists `{}` as scribing_index {}, but no ability in the game has that index",
                g.name, g.index
            ));
        };
        out.entry(g.skill_line).or_default().extend(ids);
        report.per_grimoire.push((g.name.clone(), ids.len()));
    }
    Ok((out.into_iter().map(|(line, ids)| (line, ids.into_iter().collect())).collect(), report))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scribed(id: u32, index: u32) -> SkillData34 {
        let mut s = SkillData34::default();
        s.ability_id1 = id;
        s.base_data.scribing_index = index;
        s
    }

    fn table() -> Vec<Grimoire> {
        vec![
            Grimoire { index: 1, name: "Vault".into(), skill_line: 32 },
            Grimoire { index: 2, name: "Wield Soul".into(), skill_line: 72 },
            Grimoire { index: 8, name: "Soul Burst".into(), skill_line: 72 },
        ]
    }

    #[test]
    fn the_shipped_table_parses_and_covers_the_twelve_grimoires() {
        let t = parse(GRIMOIRES_CSV).unwrap();
        assert_eq!(t.iter().map(|g| g.index).collect::<Vec<_>>(), (1..=12).collect::<Vec<_>>());
        // Wield Soul and Soul Burst are both Soul Magic: the other ten lines are one grimoire each
        let soul = t.iter().filter(|g| g.index == 2 || g.index == 8).map(|g| g.skill_line).collect::<BTreeSet<_>>();
        assert_eq!(soul.len(), 1);
        assert_eq!(t.iter().map(|g| g.skill_line).collect::<BTreeSet<_>>().len(), 11);
    }

    #[test]
    fn every_record_of_a_grimoire_lands_in_its_line_in_id_order() {
        let skills = [scribed(30, 1), scribed(10, 1), scribed(20, 2), scribed(25, 8), scribed(7, 0)];
        let (by_line, report) = lines(&skills, &table()).unwrap();
        assert_eq!(by_line, BTreeMap::from([(32, vec![10, 30]), (72, vec![20, 25])]));
        assert_eq!(report.per_grimoire, [("Vault".to_string(), 2), ("Wield Soul".to_string(), 1), ("Soul Burst".to_string(), 1)]);
    }

    #[test]
    fn an_ability_the_game_already_gives_a_line_is_left_alone() {
        let mut s = scribed(10, 1);
        s.base_data.skill_line_id = 32;
        let skills = [s, scribed(11, 1), scribed(20, 2), scribed(25, 8)];
        assert_eq!(lines(&skills, &table()).unwrap().0[&32], [11]);
    }

    #[test]
    fn a_grimoire_the_table_lacks_stops_the_compile() {
        let e = lines(&[scribed(1, 1), scribed(2, 2), scribed(3, 8), scribed(99, 13)], &table()).unwrap_err();
        assert!(e.contains("scribing_index 13") && e.contains("grimoires.csv"), "{e}");
    }

    #[test]
    fn a_row_the_game_no_longer_uses_stops_the_compile() {
        let e = lines(&[scribed(1, 1), scribed(2, 2)], &table()).unwrap_err();
        assert!(e.contains("Soul Burst") && e.contains("no ability"), "{e}");
    }

    #[test]
    fn a_bad_row_names_its_line() {
        let e = parse("# header\n1, Vault, Bow\n2, Wield Soul, Not A Line\n").unwrap_err();
        assert!(e.starts_with("grimoires.csv line 3:") && e.contains("no skill line called"), "{e}");
        assert!(parse("x, Vault, Bow\n").is_err());
        assert!(parse("0, Vault, Bow\n").is_err());
        assert!(parse("1, Vault\n").is_err());
        let e = parse("1, Vault, Bow\n1, Again, Bow\n").unwrap_err();
        assert!(e.contains("already `Vault`"), "{e}");
    }

    #[test]
    fn a_line_may_be_given_by_id() {
        let id = line_id_named("Bow").unwrap();
        assert_eq!(parse(&format!("1, Vault, {id}\n")).unwrap()[0].skill_line, id);
    }
}
