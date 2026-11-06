//! esosim data compiler: `eso.mnf` in, the data files the engine needs out.
//!
//! ```no_run
//! let bundle = esosim_data_compiler::compile(std::path::Path::new("eso.mnf"))?;
//! bundle.write_to(std::path::Path::new("bundle"))?;
//! # Ok::<(), String>(())
//! ```

mod behaviour;
mod classify;
mod compile;
mod corpus;
mod graph;
mod manifest;
pub mod resolved;
mod setdata;

/// Compile just `set_items.csv` from a raw set table file (the `.EsoFileData` extract of manifest id 0x16).
pub fn set_items_from_table(table: &[u8]) -> Result<String, String> {
    Ok(setdata::items_csv(&setdata::parse(table)?))
}

use std::fs;
use std::path::Path;

use corpus::Corpus;
use graph::Graph;

/// The compiler's output, as file contents.
#[derive(Debug)]
pub struct Bundle {
    /// `bundle.jsonl`: one ability per line (player skills, set abilities, and what they reference).
    pub abilities_jsonl: String,
    /// `sets.jsonl`: one set per line.
    pub sets_jsonl: String,
    /// `set_ids.rs`: one `pub const <NAME>_SET_ID: u16` per set.
    pub set_ids_rs: String,
    /// `set_behaviour.jsonl`: one line per set with its behaviour class (roadmap 2.1b).
    pub set_behaviour_jsonl: String,
    /// `set_items.csv`: `set id,item id,...` one line per set (roadmap 2.1e).
    pub set_items_csv: String,
    /// One-paragraph report of the classification (counts per class, unclassified blocks).
    pub behaviour_report: String,
}

impl Bundle {
    /// Write `bundle.jsonl`, `sets.jsonl`, `set_ids.rs` and `set_behaviour.jsonl` into `dir` (created if missing).
    pub fn write_to(&self, dir: &Path) -> Result<(), String> {
        fs::create_dir_all(dir).map_err(|e| format!("creating {dir:?}: {e}"))?;
        for (name, text) in [
            ("bundle.jsonl", &self.abilities_jsonl),
            ("sets.jsonl", &self.sets_jsonl),
            ("set_ids.rs", &self.set_ids_rs),
            ("set_behaviour.jsonl", &self.set_behaviour_jsonl),
            ("set_items.csv", &self.set_items_csv),
        ] {
            let path = dir.join(name);
            fs::write(&path, text).map_err(|e| format!("writing {path:?}: {e}"))?;
        }
        Ok(())
    }
}

/// Read the game's ability table, set table and lang from the manifest at `mnf` and compile them.
pub fn compile(mnf: &Path) -> Result<Bundle, String> {
    build(manifest::read(mnf)?)
}

fn build(data: manifest::GameData) -> Result<Bundle, String> {
    let lang = data.lang;
    let set_names = lang.set_names.clone();
    if lang.ability_names.is_empty() {
        return Err(format!(
            "lang (manifest id 0x{:X}) has no ability names: wrong file or changed table id",
            manifest::LANG_MANIFEST_ID
        ));
    }
    if lang.set_names.is_empty() {
        return Err(format!(
            "lang (manifest id 0x{:X}) has no set names: the set name table id has probably changed",
            manifest::LANG_MANIFEST_ID
        ));
    }
    if data.skills.is_empty() {
        return Err("ability table holds no records".into());
    }

    let sets = setdata::build(&data.set_table, &lang.set_names).map_err(|e| {
        format!("manifest id 0x{:X} is not a valid set table ({e}); the id has probably changed", manifest::SET_MANIFEST_ID)
    })?;

    let kind_overrides = behaviour::parse_kind_overrides(behaviour::KIND_OVERRIDES)?;
    let set_inputs = behaviour::inputs_from_records(&sets.records, &set_names, &sets.perfected, &kind_overrides);

    let corpus = Corpus::build(data.skills, lang);
    let graph = Graph::build(&corpus);
    let abilities = compile::compile(&corpus, &graph, &sets.ability_roots);
    let (set_behaviours, mut report) = behaviour::classify(&set_inputs, &abilities);
    report.dangling_kind_overrides =
        kind_overrides.keys().copied().filter(|id| !sets.records.iter().any(|r| r.id == *id)).collect();

    Ok(Bundle {
        abilities_jsonl: compile::to_jsonl(&abilities)?,
        sets_jsonl: sets.jsonl,
        set_ids_rs: sets.ids_rs,
        set_behaviour_jsonl: behaviour::to_jsonl(&set_behaviours)?,
        set_items_csv: sets.items_csv,
        behaviour_report: report.summary(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use corpus::LangTexts;
    use eso_skill_data::SkillData34;

    /// A one-set table whose single ability block points at `ability`.
    fn set_table(set_id: u32, ability: u32) -> Vec<u8> {
        let mut p = set_id.to_be_bytes().to_vec();
        p.resize(33, 0);
        p.push(0); // no bonus entries
        p.extend([1, 0]);
        p.extend(ability.to_be_bytes());
        p.extend(0u32.to_be_bytes());
        p.extend([5, 2, 0, 3]);
        p.extend([0u8; 50]);
        let size = p.len() as u32;
        let mut d = vec![0xFA, 0xFA, 0xEB, 0xEB];
        for w in [25u32, 1, 28] {
            d.extend(w.to_be_bytes());
        }
        d.extend(*b"####");
        for w in [1u32, size, size, 0, set_id, 0, size] {
            d.extend(w.to_be_bytes());
        }
        d.extend(p);
        d
    }

    fn skill(id: u32) -> SkillData34 {
        let mut s = SkillData34::default();
        s.ability_id1 = id;
        s
    }

    fn data() -> manifest::GameData {
        let mut lang = LangTexts::default();
        lang.ability_names.insert(5, "Set Proc".into());
        lang.set_names.insert(20, "Archer's Mind".into());
        manifest::GameData {
            skills: vec![skill(5), skill(6)],
            set_table: set_table(20, 5),
            lang,
        }
    }

    #[test]
    fn set_abilities_become_roots_and_everything_is_named() {
        let b = build(data()).unwrap();
        let ids: Vec<u64> = b
            .abilities_jsonl
            .lines()
            .map(|l| serde_json::from_str::<serde_json::Value>(l).unwrap()["id"].as_u64().unwrap())
            .collect();
        assert_eq!(ids, [5]); // 6 is neither a player skill nor reachable from a root
        assert!(b.abilities_jsonl.contains("Set Proc"));
        assert!(b.sets_jsonl.contains("\"keystone\":5"));
        assert!(b.set_ids_rs.contains("pub const ARCHERS_MIND_SET_ID: u16 = 20;"));
        // the set's one block is a plain passive (no trigger, no buff): a static rule. The test set
        // has no stat rows and one block, so it is a 1-item set: a mythic, block at 1 piece
        assert_eq!(
            b.set_behaviour_jsonl,
            "{\"id\":20,\"name\":\"Archer's Mind\",\"class\":\"static_conditional\",\"kind\":\"mythic\",\"blocks\":[{\"ability_id\":5,\"class\":\"static_conditional\",\"pieces\":1}]}\n"
        );
        assert!(b.behaviour_report.contains("1 static_conditional"));
    }

    #[test]
    fn missing_lang_tables_are_errors_not_unknown_sets() {
        let mut d = data();
        d.lang.set_names.clear();
        assert!(build(d).unwrap_err().contains("set names"));
        let mut d = data();
        d.lang.ability_names.clear();
        assert!(build(d).unwrap_err().contains("ability names"));
    }

    #[test]
    fn a_bad_set_table_names_the_manifest_id() {
        let mut d = data();
        d.set_table[0] = 0;
        let e = build(d).unwrap_err();
        assert!(e.contains(&format!("0x{:X}", manifest::SET_MANIFEST_ID)), "{e}");
    }

    #[test]
    fn write_to_creates_the_four_files() {
        let dir = std::env::temp_dir().join(format!("esosim-compiler-test-{}", std::process::id()));
        build(data()).unwrap().write_to(&dir).unwrap();
        for f in ["bundle.jsonl", "sets.jsonl", "set_ids.rs", "set_behaviour.jsonl"] {
            assert!(dir.join(f).is_file(), "{f}");
        }
        fs::remove_dir_all(&dir).unwrap();
    }
}
