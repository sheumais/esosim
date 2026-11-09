//! Root categories (roadmap 2.0 item 1): which abilities the bundle is built from.
//!
//! The bundle is the closure of a set of *roots* over the ability graph. Three categories are
//! structural and come straight from the game tables:
//!
//! * `skills`: every ability with a `player_skill_index`;
//! * `sets`: every ability block / keystone of the set table;
//! * `cp`: the ability of every star of the champion point table.
//!
//! The others (`mundus`, `consumables`, `racial`, `buffs`) are listed in `data/roots.toml`
//! by id, by exact name or by name prefix. Each ability in the bundle then carries the categories
//! whose closure it belongs to, so "why is this ability here?" has an answer.

use crate::compile::is_root;
use crate::corpus::Corpus;
use crate::resolved::{ResolvedAbility, RootCategory};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

/// The embedded default, `data/roots.toml`.
pub const ROOTS_TOML: &str = include_str!("../data/roots.toml");

/// What `roots.toml` says about one category.
#[derive(Debug, Default, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CategorySpec {
    #[serde(default)]
    pub ids: Vec<u32>,
    /// Exact ability names (lang file).
    #[serde(default)]
    pub names: Vec<String>,
    /// Every ability whose name starts with this.
    #[serde(default)]
    pub name_prefix: Option<String>,
    /// How many roots `names` / `name_prefix` should find; another count is reported.
    #[serde(default)]
    pub expect: Option<usize>,
    /// `buffs` only: ids applied to the enemy. They are roots too.
    #[serde(default)]
    pub debuff_ids: Vec<u32>,
    /// Every ability carrying one of these `ability_tags` (`SkillData34::ability_tags`).
    #[serde(default)]
    pub tags: Vec<u16>,
}

/// The parsed `roots.toml`.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RootConfig {
    pub categories: BTreeMap<RootCategory, CategorySpec>,
}

impl RootConfig {
    pub fn embedded() -> Result<RootConfig, String> {
        parse_config(ROOTS_TOML)
    }

    /// The configured ids of `category` (`debuff_ids` included), sorted.
    pub fn listed_ids(&self, category: RootCategory) -> BTreeSet<u32> {
        match self.categories.get(&category) {
            Some(spec) => spec.ids.iter().chain(spec.debuff_ids.iter()).copied().collect(),
            None => BTreeSet::new(),
        }
    }

    /// Ids of `buffs` that are debuffs.
    pub fn debuff_ids(&self) -> BTreeSet<u32> {
        match self.categories.get(&RootCategory::Buffs) {
            Some(spec) => spec.debuff_ids.iter().copied().collect(),
            None => BTreeSet::new(),
        }
    }
}

/// Parse `roots.toml`. Tables are named after the categories; `skills` and `sets` are structural
/// and rejected, as is anything unknown (a typo must not silently drop a category).
pub fn parse_config(text: &str) -> Result<RootConfig, String> {
    let raw: BTreeMap<String, CategorySpec> = toml::from_str(text).map_err(|e| format!("roots.toml: {e}"))?;
    let mut categories = BTreeMap::new();
    for (name, spec) in raw {
        let Some(category) = RootCategory::parse(&name) else {
            return Err(format!("roots.toml: unknown category `{name}`"));
        };
        if matches!(category, RootCategory::Skills | RootCategory::Sets | RootCategory::Cp) {
            return Err(format!("roots.toml: `{name}` comes from the game tables and cannot be listed"));
        }
        if !spec.debuff_ids.is_empty() && category != RootCategory::Buffs {
            return Err(format!("roots.toml: `debuff_ids` only makes sense in [buffs], not [{name}]"));
        }
        categories.insert(category, spec);
    }
    Ok(RootConfig { categories })
}

/// The root ids of every category.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Roots {
    pub by_category: BTreeMap<RootCategory, BTreeSet<u32>>,
}

/// What building the roots found out: how many per category, and what did not line up.
#[derive(Debug, Default, Clone, PartialEq)]
pub struct RootsReport {
    pub counts: BTreeMap<RootCategory, usize>,
    /// Listed ids that have no record in the ability table.
    pub missing: BTreeMap<RootCategory, Vec<u32>>,
    /// `(category, expected, found)` where `expect` in `roots.toml` is not met.
    pub unexpected: Vec<(RootCategory, usize, usize)>,
}

impl RootsReport {
    /// A few lines for the console.
    pub fn summary(&self) -> String {
        let counts: Vec<String> = self.counts.iter().map(|(c, n)| format!("{} {n}", c.name())).collect();
        let mut out = format!("roots: {}", counts.join(", "));
        for (category, ids) in &self.missing {
            let shown: Vec<String> = ids.iter().take(10).map(u32::to_string).collect();
            let more = ids.len().saturating_sub(10);
            out.push_str(&format!(
                "\nroots: {} listed id(s) of `{}` are not in the game data: {}{}",
                ids.len(),
                category.name(),
                shown.join(", "),
                if more > 0 { format!(" (+{more} more)") } else { String::new() }
            ));
        }
        for (category, expected, found) in &self.unexpected {
            out.push_str(&format!("\nroots: `{}` expected {expected} roots, found {found}", category.name()));
        }
        out
    }
}

impl Roots {
    /// Roots of every category: the player skills, `set_roots` and `cp_roots` (ids with no record
    /// are dropped), and what `config` lists.
    pub fn build(
        corpus: &Corpus,
        set_roots: &BTreeSet<u32>,
        cp_roots: &BTreeSet<u32>,
        config: &RootConfig,
    ) -> (Roots, RootsReport) {
        let mut roots = Roots::default();
        let mut report = RootsReport::default();

        roots.by_category.insert(
            RootCategory::Skills,
            corpus.skills.iter().filter(|(_, s)| is_root(s)).map(|(id, _)| *id).collect(),
        );
        roots.by_category.insert(
            RootCategory::Sets,
            set_roots.iter().copied().filter(|id| corpus.skills.contains_key(id)).collect(),
        );
        let (cp_found, cp_missing): (BTreeSet<u32>, BTreeSet<u32>) =
            cp_roots.iter().copied().partition(|id| corpus.skills.contains_key(id));
        if !cp_missing.is_empty() {
            report.missing.insert(RootCategory::Cp, cp_missing.into_iter().collect());
        }
        roots.by_category.insert(RootCategory::Cp, cp_found);

        for (category, spec) in &config.categories {
            let mut found: BTreeSet<u32> = BTreeSet::new();

            let listed = config.listed_ids(*category);
            for id in &listed {
                if corpus.skills.contains_key(id) {
                    found.insert(*id);
                } else {
                    report.missing.entry(*category).or_default().push(*id);
                }
            }

            let mut by_name: BTreeSet<u32> = BTreeSet::new();
            if !spec.names.is_empty() || spec.name_prefix.is_some() {
                for (id, name) in &corpus.names {
                    if !corpus.skills.contains_key(id) {
                        continue;
                    }
                    let exact = spec.names.iter().any(|n| n == name);
                    let prefixed = spec.name_prefix.as_deref().is_some_and(|p| name.starts_with(p));
                    if exact || prefixed {
                        by_name.insert(*id);
                    }
                }
                found.extend(by_name.iter().copied());
            }
            if !spec.tags.is_empty() {
            for (id, s) in &corpus.skills {
                    if s.ability_tags.iter().any(|t| spec.tags.contains(t)) {
                        found.insert(*id);
                    }
                }
            }
            if let Some(expected) = spec.expect {
                if by_name.len() != expected {
                    report.unexpected.push((*category, expected, by_name.len()));
                }
            }

            roots.by_category.insert(*category, found);
        }

        for (category, ids) in &roots.by_category {
            report.counts.insert(*category, ids.len());
        }
        (roots, report)
    }

    /// Every root of every category.
    pub fn all(&self) -> BTreeSet<u32> {
        self.by_category.values().flatten().copied().collect()
    }
}

/// The root ids of `category` as far as they can be told from compiled files alone (no game
/// data): player skills from the abilities, `sets` from the `abilities` / `keystone` of
/// `sets.jsonl`, `cp` from `cp.jsonl`, the rest from `config`. Used by `verify` and
/// `suggest-effects`.
pub fn root_ids_from_files(
    category: RootCategory,
    abilities: &[ResolvedAbility],
    sets_jsonl: &str,
    cp_jsonl: &str,
    config: &RootConfig,
) -> Result<BTreeSet<u32>, String> {
    match category {
        RootCategory::Skills => {
            Ok(abilities.iter().filter(|a| a.player_skill_index != 0).map(|a| a.id).collect())
        }
        RootCategory::Sets => set_ability_ids(sets_jsonl),
        RootCategory::Cp => crate::cp::ability_ids_of_jsonl(cp_jsonl),
        other => {
            let mut ids = config.listed_ids(other);
            if let Some(spec) = config.categories.get(&other) {
                ids.extend(abilities.iter().filter(|a| matches_rules(spec, a)).map(|a| a.id));
            }
            Ok(ids)
        }
    }
}

/// Every `abilities[].ability_id` and `keystone` of a `sets.jsonl` (0 is not an id).
pub fn set_ability_ids(sets_jsonl: &str) -> Result<BTreeSet<u32>, String> {
    let mut ids = BTreeSet::new();
    for (i, line) in sets_jsonl.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        let v: serde_json::Value =
            serde_json::from_str(line).map_err(|e| format!("sets.jsonl line {}: {e}", i + 1))?;
        if let Some(blocks) = v.get("abilities").and_then(|a| a.as_array()) {
            for b in blocks {
                if let Some(id) = b.get("ability_id").and_then(|x| x.as_u64()) {
                    ids.insert(id as u32);
                }
            }
        }
        if let Some(id) = v.get("keystone").and_then(|x| x.as_u64()) {
            ids.insert(id as u32);
        }
    }
    ids.remove(&0);
    Ok(ids)
}

pub fn matches_rules(spec: &CategorySpec, a: &ResolvedAbility) -> bool {
    spec.names.iter().any(|n| *n == a.name)
        || spec.name_prefix.as_deref().is_some_and(|p| a.name.starts_with(p))
        || a.tags.iter().any(|t| spec.tags.contains(t))
}

#[cfg(test)]
mod tests {
    use super::*;
    use eso_skill_data::SkillData34;

    fn skill(id: u32) -> SkillData34 {
        let mut s = SkillData34::default();
        s.ability_id1 = id;
        s
    }

    fn corpus(ids: &[u32], names: &[(u32, &str)]) -> Corpus {
        let mut c = Corpus::from_skills(ids.iter().map(|id| skill(*id)).collect());
        for (id, n) in names {
            c.names.insert(*id, (*n).to_string());
        }
        c
    }

    #[test]
    fn the_shipped_roots_file_parses_and_lists_the_major_minor_buffs() {
        let cfg = RootConfig::embedded().unwrap();
        let buffs = cfg.listed_ids(RootCategory::Buffs);
        assert!(buffs.contains(&76617), "Minor Slayer"); // the id the roadmap wanted always present
        assert!(buffs.contains(&61747), "Major Force");
        assert!(cfg.debuff_ids().contains(&61743), "Major Breach is a debuff");
        assert!(!cfg.debuff_ids().contains(&76617));
        assert_eq!(cfg.categories[&RootCategory::Mundus].expect, Some(14));
        assert!(cfg.categories.get(&RootCategory::Cp).is_none(), "cp comes from the CP table");
    }

    #[test]
    fn structural_and_unknown_categories_are_rejected() {
        assert!(parse_config("[skills]\nids = [1]\n").unwrap_err().contains("game tables"));
        assert!(parse_config("[cp]\nids = [1]\n").unwrap_err().contains("game tables"));
        assert!(parse_config("[mundis]\nids = [1]\n").unwrap_err().contains("unknown category"));
        assert!(parse_config("[mundus]\ndebuff_ids = [1]\n").unwrap_err().contains("only makes sense"));
        assert!(parse_config("[mundus]\nidz = [1]\n").is_err(), "unknown field");
    }

    #[test]
    fn ids_names_and_prefixes_become_roots_and_gaps_are_reported() {
        let c = corpus(
            &[10, 11, 12, 13, 14],
            &[(11, "Boon: The Thief"), (12, "Boon: The Lady"), (13, "Other"), (14, "Fortified")],
        );
        let cfg = parse_config(
            "[mundus]\nname_prefix = \"Boon: \"\nexpect = 3\n[consumables]\nids = [14, 999]\nnames = [\"Other\"]\n",
        )
        .unwrap();
        let (roots, report) = Roots::build(&c, &BTreeSet::new(), &BTreeSet::new(), &cfg);
        assert_eq!(roots.by_category[&RootCategory::Mundus], BTreeSet::from([11, 12]));
        assert_eq!(roots.by_category[&RootCategory::Consumables], BTreeSet::from([13, 14]));
        assert_eq!(report.missing[&RootCategory::Consumables], vec![999]);
        assert_eq!(report.unexpected, vec![(RootCategory::Mundus, 3, 2)]);
        let text = report.summary();
        assert!(text.contains("`consumables` are not in the game data: 999"), "{text}");
        assert!(text.contains("`mundus` expected 3 roots, found 2"), "{text}");
    }

    #[test]
    fn player_skills_and_set_abilities_are_roots_and_unknown_set_ids_are_dropped() {
        let mut s = skill(1);
        s.base_data.player_skill_index = 5;
        let c = Corpus::from_skills(vec![s, skill(2), skill(3)]);
        let (roots, report) =
            Roots::build(&c, &BTreeSet::from([2, 77]), &BTreeSet::from([3, 88]), &RootConfig::default());
        assert_eq!(roots.by_category[&RootCategory::Skills], BTreeSet::from([1]));
        assert_eq!(roots.by_category[&RootCategory::Sets], BTreeSet::from([2]));
        assert_eq!(roots.by_category[&RootCategory::Cp], BTreeSet::from([3]));
        assert_eq!(roots.all(), BTreeSet::from([1, 2, 3]));
        assert_eq!(report.counts[&RootCategory::Sets], 1);
    }

    #[test]
    fn set_ability_ids_read_blocks_and_keystone() {
        let text = "{\"id\":1,\"abilities\":[{\"tag\":1,\"ability_id\":5},{\"tag\":2,\"ability_id\":0}],\"keystone\":6}\n\n{\"id\":2}\n";
        assert_eq!(set_ability_ids(text).unwrap(), BTreeSet::from([5, 6]));
        assert!(set_ability_ids("{nope").unwrap_err().contains("line 1"));
    }
}
