//! esosim data compiler: `eso.mnf` in, the data files the engine needs out.
//!
//! ```no_run
//! let bundle = esosim_data_compiler::compile(std::path::Path::new("eso.mnf"))?;
//! bundle.write_to(std::path::Path::new("bundle"))?;
//! # Ok::<(), String>(())
//! ```
//!
//! The compiler is deliberately a lean library (decision recorded in `PHASE1-PROGRESS.md`):
//! [`compile`] reads the game, and everything a person writes to the game data is held by
//! [`verify`], which works on the generated *files* only, so it runs on a machine with no game
//! install. The binary adds three commands on top: `compile` (the default), `verify` and
//! `suggest-effects`.

mod behaviour;
mod classify;
mod compile;
mod corpus;
pub mod cp;
pub mod effects;
mod graph;
pub mod items;
mod manifest;
pub mod overrides;
pub mod resolved;
pub mod roots;
mod setdata;
pub mod suggest;
pub mod triggers;
pub mod verify;

/// Compile just `item_types.csv` (item id -> weapon type, armour weight, ring, necklace) from a raw item
/// table file, with the `eso-item-data` crate (roadmap 2.8).
pub fn item_types_from_table(table: &[u8]) -> Result<String, String> {
    items::item_types_from_table(table)
}

/// Compile just `set_items.csv` from a raw set table file (the `.EsoFileData` extract of manifest id 0x16).
pub fn set_items_from_table(table: &[u8]) -> Result<String, String> {
    Ok(setdata::items_csv(&setdata::parse(table)?))
}

use std::collections::BTreeSet;
use std::fs;
use std::path::Path;

use corpus::Corpus;
use graph::Graph;
use resolved::RootCategory;
use roots::{RootConfig, Roots};
use verify::{VerifyInput, VerifyReport};

/// The hand-written inputs of a compile: which extra roots to add, which effects were written by
/// a person, and which effect files outside the compiler hold more hand-written ids.
/// `Options::default()` is what ships: the embedded `data/roots.toml` and `data/overrides.toml`.
#[derive(Debug, Clone)]
pub struct Options {
    /// Text of `roots.toml`.
    pub roots_toml: String,
    /// Text of `overrides.toml`.
    pub overrides_toml: String,
    /// Text of `cp_effects.toml`: which channel each champion point star feeds (roadmap 2.2c).
    pub cp_effects_toml: String,
    /// Text of `cp_trees.toml`: which tree each champion point star is in, and its way in (roadmap 2.2b).
    pub cp_trees_toml: String,
    pub hand_written: Vec<(String, String)>,
}

impl Default for Options {
    fn default() -> Self {
        Options {
            roots_toml: roots::ROOTS_TOML.to_string(),
            overrides_toml: overrides::OVERRIDES_TOML.to_string(),
            cp_effects_toml: cp::CP_EFFECTS_TOML.to_string(),
            cp_trees_toml: cp::CP_TREES_TOML.to_string(),
            hand_written: Vec::new(),
        }
    }
}

impl Options {
    /// The ids of every hand-written effect file.
    fn hand_written_ids(&self) -> Result<BTreeSet<u32>, String> {
        let mut ids = BTreeSet::new();
        for (label, text) in &self.hand_written {
            ids.extend(verify::ids_of_jsonl(text).map_err(|e| format!("{label} {e}"))?);
        }
        Ok(ids)
    }

    fn verify_input<'a>(
        &'a self,
        abilities_jsonl: &'a str,
        sets_jsonl: &'a str,
        cp_jsonl: &'a str,
    ) -> VerifyInput<'a> {
        VerifyInput {
            abilities_jsonl,
            sets_jsonl,
            cp_jsonl,
            overrides_toml: &self.overrides_toml,
            roots_toml: &self.roots_toml,
            other_effects: self.hand_written.iter().map(|(l, t)| (l.clone(), t.as_str())).collect(),
        }
    }
}

/// `verify` on generated files (`bundle.jsonl`, `sets.jsonl`) and the hand-written inputs in `opts`.
pub fn verify_files(abilities_jsonl: &str, sets_jsonl: &str, cp_jsonl: &str, opts: &Options) -> VerifyReport {
    verify::verify(&opts.verify_input(abilities_jsonl, sets_jsonl, cp_jsonl))
}

/// `suggest-effects <category>`: `overrides.toml` skeletons for every root of `category` that has no
/// override and is not hand-written elsewhere. Works on generated files, like `verify`.
/// Returns the text and the number of skeletons.
pub fn suggest_effects(
    category: &str,
    abilities_jsonl: &str,
    sets_jsonl: &str,
    cp_jsonl: &str,
    opts: &Options,
) -> Result<(String, usize), String> {
    let category = RootCategory::parse(category).ok_or_else(|| {
        let known: Vec<&str> = RootCategory::ALL.iter().map(|c| c.name()).collect();
        format!("unknown category `{category}` (one of: {})", known.join(", "))
    })?;
    let abilities = verify::parse_abilities(abilities_jsonl)?;
    let config = roots::parse_config(&opts.roots_toml)?;
    let root_ids = roots::root_ids_from_files(category, &abilities, sets_jsonl, cp_jsonl, &config)?;

    let mut reviewed = opts.hand_written_ids()?;
    reviewed.extend(overrides::parse(&opts.overrides_toml)?.iter().map(|o| o.def.id));
    if matches!(category, RootCategory::Cp) {
        // a star written in cp_effects.toml is reviewed
        let stars = cp::stars_of_jsonl(cp_jsonl)?;
        for m in cp::parse_effects(&opts.cp_effects_toml)? {
            reviewed.extend(stars.iter().filter(|s| s.id == m.id).map(|s| s.ability_id));
        }
    }
    Ok(suggest::suggest(&abilities, category, &root_ids, &reviewed))
}

/// The compiler's output, as file contents.
#[derive(Debug)]
pub struct Bundle {
    /// `bundle.jsonl`: one ability per line (player skills, set abilities, and what they reference).
    pub abilities_jsonl: String,
    /// `sets.jsonl`: one set per line.
    pub sets_jsonl: String,
    /// `cp.jsonl`: one champion point star per line (`esosim_types::CpStar`), sorted by id (roadmap 2.2a).
    pub cp_jsonl: String,
    /// `set_ids.rs`: one `pub const <NAME>_SET_ID: u16` per set.
    pub set_ids_rs: String,
    /// `set_behaviour.jsonl`: one line per set with its behaviour class (roadmap 2.1b).
    pub set_behaviour_jsonl: String,
    /// `set_items.csv`: `set id,item id,...` one line per set (roadmap 2.1e).
    pub set_items_csv: String,
    /// `skill_lines.csv`: `skill line id,ability id,...` one line per skill line (roadmap 1.4).
    pub skill_lines_csv: String,
    /// `effects.jsonl`: every override plus the high-confidence auto-classified effects, each with
    /// its `provenance` (roadmap 2.0 item 4).
    pub effects_jsonl: String,
    pub cp_effects_jsonl: String,
    /// `item_types.csv`: `KIND,item id,...` one line per weapon type, armour weight, ring and necklace (roadmap 2.8).
    pub item_types_csv: String,
    /// One-paragraph report of the classification (counts per class, unclassified blocks).
    pub behaviour_report: String,
    /// What building the roots found: counts per category, listed ids the game does not have.
    pub roots_report: String,
}

impl Bundle {
    /// Hold the compiled files and the hand-written inputs in `opts` against each other (`verify`).
    pub fn verify(&self, opts: &Options) -> VerifyReport {
        verify_files(&self.abilities_jsonl, &self.sets_jsonl, &self.cp_jsonl, opts)
    }

    /// Write every output file into `dir` (created if missing).
    pub fn write_to(&self, dir: &Path) -> Result<(), String> {
        fs::create_dir_all(dir).map_err(|e| format!("creating {dir:?}: {e}"))?;
        for (name, text) in [
            ("bundle.jsonl", &self.abilities_jsonl),
            ("sets.jsonl", &self.sets_jsonl),
            ("cp.jsonl", &self.cp_jsonl),
            ("set_ids.rs", &self.set_ids_rs),
            ("set_behaviour.jsonl", &self.set_behaviour_jsonl),
            ("set_items.csv", &self.set_items_csv),
            ("skill_lines.csv", &self.skill_lines_csv),
            ("effects.jsonl", &self.effects_jsonl),
            ("cp_effects.jsonl", &self.cp_effects_jsonl),
            ("item_types.csv", &self.item_types_csv),
        ] {
            let path = dir.join(name);
            fs::write(&path, text).map_err(|e| format!("writing {path:?}: {e}"))?;
        }
        Ok(())
    }
}

/// Read the game's ability table, set table, champion point table and lang from the manifest at `mnf` and compile them.
pub fn compile(mnf: &Path) -> Result<Bundle, String> {
    compile_with(mnf, &Options::default())
}

/// [`compile`] with explicit hand-written inputs.
pub fn compile_with(mnf: &Path, opts: &Options) -> Result<Bundle, String> {
    build(manifest::read(mnf)?, opts)
}

fn build(data: manifest::GameData, opts: &Options) -> Result<Bundle, String> {
    let config: RootConfig = roots::parse_config(&opts.roots_toml)?;
    let overrides = overrides::parse(&opts.overrides_toml)?;

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
    let mut cp_stars = cp::stars(&data.cp, |id| corpus.names.get(&id).cloned())?;
    let cp_trees_report = cp::assign_trees(&mut cp_stars, &cp::parse_trees(&opts.cp_trees_toml)?)?;
    let one_way = cp::one_way_links(&cp_stars);
    // a star's effect is keyed by its ability id: it must not collide with a hand-written effect
    let cp_effects = cp::effects(&cp_stars, &cp::parse_effects(&opts.cp_effects_toml)?)?;
    let mut taken = opts.hand_written_ids()?;
    taken.extend(overrides.iter().map(|o| o.def.id));
    if let Some(d) = cp_effects.iter().find(|d| taken.contains(&d.id)) {
        return Err(format!(
            "champion point `{}` (ability {}) is also written by hand in overrides.toml or an engine effect file; the effect would count twice",
            d.name, d.id
        ));
    }
    let (roots, roots_report) =
        Roots::build(&corpus, &sets.ability_roots, &cp::ability_roots(&cp_stars), &config);
    let abilities = compile::compile(&corpus, &graph, &roots);

    // an override wins over the classifier, and an id written by hand elsewhere is never generated
    taken.extend(cp_effects.iter().map(|d| d.id));
    let auto = effects::auto_effects(&abilities, &config, &taken);
    let effects_jsonl = effects::to_jsonl(&overrides, &auto)?;

    let (set_behaviours, mut report) = behaviour::classify(&set_inputs, &abilities);
    report.dangling_kind_overrides =
        kind_overrides.keys().copied().filter(|id| !sets.records.iter().any(|r| r.id == *id)).collect();

    Ok(Bundle {
        abilities_jsonl: compile::to_jsonl(&abilities)?,
        sets_jsonl: sets.jsonl,
        cp_jsonl: cp::to_jsonl(&cp_stars)?,
        set_ids_rs: sets.ids_rs,
        set_behaviour_jsonl: behaviour::to_jsonl(&set_behaviours)?,
        set_items_csv: sets.items_csv,
        skill_lines_csv: compile::skill_lines_csv(&abilities),
        effects_jsonl,
        cp_effects_jsonl: cp::effects_to_jsonl(&cp_effects)?,
        item_types_csv: data.item_types_csv,
        behaviour_report: report.summary(),
        roots_report: format!(
            "{}\ncp table: {} record(s) read, {} real star(s) (a star has an ability and a map position)\ncp {cp_trees_report}{}",
            roots_report.summary(),
            data.cp.len(),
            cp_stars.len(),
            if one_way.is_empty() {
                String::new()
            } else {
                format!("\ncp links listed by one star only (prerequisites follow each star's own list): {one_way:?}")
            }
        ),
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

    /// The shipped inputs, except for the champion point ones: the test games have no CP table, and
    /// the shipped mapping and trees name the real stars.
    fn no_cp() -> Options {
        Options { cp_effects_toml: String::new(), cp_trees_toml: String::new(), ..Options::default() }
    }

    fn data() -> manifest::GameData {
        let mut lang = LangTexts::default();
        lang.ability_names.insert(5, "Set Proc".into());
        lang.set_names.insert(20, "Archer's Mind".into());
        manifest::GameData {
            skills: vec![skill(5), skill(6)],
            set_table: set_table(20, 5),
            cp: Vec::new(),
            lang,
            item_types_csv: String::new(),
        }
    }

    #[test]
    fn set_abilities_become_roots_and_everything_is_named() {
        let b = build(data(), &no_cp()).unwrap();
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

    /// Inputs with no extra roots and the given overrides.
    fn opts(overrides: &str) -> Options {
        Options { overrides_toml: overrides.into(), roots_toml: String::new(), ..no_cp() }
    }

    const PROC_OVERRIDE: &str = "[[effect]]\nid = 5\nname = \"Set Proc\"\nsource = { Passive = { ability_id = 5 } }\nchannels = [{ channel = \"CriticalChance\", value = 0.0 }]\n";

    #[test]
    fn roots_toml_adds_roots_and_labels_every_ability() {
        let o = Options { roots_toml: "[buffs]\nids = [6]\n".into(), ..opts("") };
        let b = build(data(), &o).unwrap();
        let labels: Vec<String> = b
            .abilities_jsonl
            .lines()
            .map(|l| serde_json::from_str::<serde_json::Value>(l).unwrap()["categories"].to_string())
            .collect();
        assert_eq!(labels, ["[\"sets\"]", "[\"buffs\"]"]); // 5 is a set ability, 6 a listed buff
        assert!(b.roots_report.contains("buffs 1") && b.roots_report.contains("sets 1"), "{}", b.roots_report);
        assert_eq!(b.effects_jsonl, "", "nothing is written by hand and the classifier is not sure of anything");
        assert!(b.verify(&o).ok(), "{}", b.verify(&o).summary());
    }

    #[test]
    fn a_listed_root_the_game_does_not_have_is_reported_and_fails_verify() {
        let o = Options { roots_toml: "[buffs]\nids = [6, 4242]\n".into(), ..opts("") };
        let b = build(data(), &o).unwrap();
        assert!(b.roots_report.contains("`buffs` are not in the game data: 4242"), "{}", b.roots_report);
        assert!(b.verify(&o).errors.iter().any(|e| e.contains("[buffs] lists ability 4242")));
    }

    #[test]
    fn overrides_land_in_effects_jsonl_and_verify_holds_them_to_the_bundle() {
        let o = opts(PROC_OVERRIDE);
        let b = build(data(), &o).unwrap();
        assert!(b.effects_jsonl.starts_with("{\"id\":5,\"name\":\"Set Proc\""), "{}", b.effects_jsonl);
        assert!(b.effects_jsonl.ends_with("\"provenance\":\"override\"}\n"), "{}", b.effects_jsonl);
        assert!(b.verify(&o).ok(), "{}", b.verify(&o).summary());

        // a patch renames the ability: the override is stale and verify says so
        let mut renamed = data();
        renamed.lang.ability_names.insert(5, "Renamed".into());
        let b = build(renamed, &o).unwrap();
        let r = b.verify(&o);
        assert!(r.errors.iter().any(|e| e.contains("the game calls ability 5 `Renamed`")), "{:?}", r.errors);
    }

    #[test]
    fn an_override_may_not_reuse_a_hand_written_id() {
        let o = Options { hand_written: vec![("legacy.jsonl".into(), "{\"id\":5,\"name\":\"x\"}\n".into())], ..opts(PROC_OVERRIDE) };
        let b = build(data(), &o).unwrap();
        assert!(b.verify(&o).errors.iter().any(|e| e.contains("also written by hand in legacy.jsonl")));
    }

    #[test]
    fn suggest_effects_skips_reviewed_ids_and_names_unknown_categories() {
        let o = opts(PROC_OVERRIDE);
        let b = build(data(), &o).unwrap();

        // the one set ability (5) already has an override
        let (text, n) = suggest_effects("sets", &b.abilities_jsonl, &b.sets_jsonl, &b.cp_jsonl, &opts("")).unwrap();
        assert_eq!(n, 1, "{text}");
        assert!(text.contains("id = 5\n") && text.contains("Set = { set_id = 0"), "{text}");
        let (_, n) = suggest_effects("sets", &b.abilities_jsonl, &b.sets_jsonl, &b.cp_jsonl, &o).unwrap();
        assert_eq!(n, 0);

        let e = suggest_effects("mundis", &b.abilities_jsonl, &b.sets_jsonl, &b.cp_jsonl, &o).unwrap_err();
        assert!(e.contains("unknown category `mundis`") && e.contains("mundus"), "{e}");
    }

    #[test]
    fn the_shipped_defaults_parse_and_name_the_effects_the_roadmap_wants() {
        let o = Options::default();
        let cfg = roots::parse_config(&o.roots_toml).unwrap();
        let buffs = cfg.listed_ids(RootCategory::Buffs);
        assert!(buffs.contains(&76617), "Minor Slayer");
        assert!(buffs.contains(&61746), "Minor Force (Velothi Ur-Mage's Amulet)");
        assert!(cfg.listed_ids(RootCategory::Cp).is_empty(), "cp roots come from the CP table, not roots.toml");
        assert!(overrides::parse(&o.overrides_toml).is_ok());
    }

    #[test]
    fn missing_lang_tables_are_errors_not_unknown_sets() {
        let mut d = data();
        d.lang.set_names.clear();
        assert!(build(d, &Options::default()).unwrap_err().contains("set names"));
        let mut d = data();
        d.lang.ability_names.clear();
        assert!(build(d, &Options::default()).unwrap_err().contains("ability names"));
    }

    #[test]
    fn a_bad_set_table_names_the_manifest_id() {
        let mut d = data();
        d.set_table[0] = 0;
        let e = build(d, &Options::default()).unwrap_err();
        assert!(e.contains(&format!("0x{:X}", manifest::SET_MANIFEST_ID)), "{e}");
    }

    #[test]
    fn write_to_creates_every_output_file() {
        let dir = std::env::temp_dir().join(format!("esosim-compiler-test-{}", std::process::id()));
        build(data(), &no_cp()).unwrap().write_to(&dir).unwrap();
        for f in ["bundle.jsonl", "sets.jsonl", "cp.jsonl", "set_ids.rs", "set_behaviour.jsonl", "cp_effects.jsonl", "skill_lines.csv", "item_types.csv"] {
            assert!(dir.join(f).is_file(), "{f}");
        }
        fs::remove_dir_all(&dir).unwrap();
    }
}
