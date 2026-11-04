//! Loading: skill file + language file -> an in-memory `Corpus`.
//!
//! Everything downstream (graph, resolve, probes) works off `Corpus` so it can
//! also be built synthetically in unit tests via [`Corpus::from_skills`].

use csv::ReaderBuilder;
use eso_skill_data::{SkillData34, load_skill_data};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::path::{Path, PathBuf};

/// Table ids inside `en.lang.csv` (first column).
pub const LANG_ABILITY_NAMES: u32 = 198_758_357;
/// Set names, keyed by set id (read by the `sets` command).
pub const LANG_SET_NAMES: u32 = 38_727_365;
pub const LANG_ABILITY_TOOLTIPS_A: u32 = 206_046_340;
pub const LANG_ABILITY_TOOLTIPS_B: u32 = 132_143_172;

pub struct Corpus {
    /// Every parsed record, keyed (and therefore iterated) by ability id.
    pub skills: BTreeMap<u32, SkillData34>,
    pub names: BTreeMap<u32, String>,
    pub tooltips: BTreeMap<u32, String>,
    /// Every tooltip text of an ability, table A first then table B (`tooltips` keeps only
    /// the winner). Placeholders may be spread over several texts, so classification reads all.
    pub tooltip_texts: BTreeMap<u32, Vec<String>>,
    /// File name only (no directory) so bundle metadata stays machine-independent.
    pub source_file: String,
    pub duplicate_records: usize,
}

impl Corpus {
    pub fn from_skills(list: Vec<SkillData34>) -> Self {
        let mut skills = BTreeMap::new();
        let mut duplicate_records = 0;
        for s in list {
            let id = s.ability_id1;
            if skills.insert(id, s).is_some() {
                duplicate_records += 1;
            }
        }
        Corpus {
            skills,
            names: BTreeMap::new(),
            tooltips: BTreeMap::new(),
            tooltip_texts: BTreeMap::new(),
            source_file: String::new(),
            duplicate_records,
        }
    }

    pub fn load(skill_dir: &str, lang_file: &str) -> Result<Corpus, String> {
        let file = find_skill_data_file(Path::new(skill_dir))
            .map_err(|e| format!("could not find skill data file in '{skill_dir}': {e}"))?;
        let file_str = file
            .to_str()
            .ok_or_else(|| format!("skill data path is not valid UTF-8: {file:?}"))?;
        eprintln!("Found skill data file {file_str}");

        let state = load_skill_data(file_str)
            .map_err(|e| format!("failed to load skill data '{file_str}': {e}"))?;
        let mut corpus = Corpus::from_skills(state.skills);
        corpus.source_file = file
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        if corpus.duplicate_records > 0 {
            eprintln!(
                "warning: {} duplicate ability ids in the skill file (last record wins)",
                corpus.duplicate_records
            );
        }

        // One pass over the (large) CSV for all three tables.
        let mut lang = load_lang(
            lang_file,
            &[LANG_ABILITY_NAMES, LANG_ABILITY_TOOLTIPS_A, LANG_ABILITY_TOOLTIPS_B],
        );
        corpus.names = lang.remove(&LANG_ABILITY_NAMES).unwrap_or_default();
        let tips_b = lang.remove(&LANG_ABILITY_TOOLTIPS_B).unwrap_or_default();
        let tips_a = lang.remove(&LANG_ABILITY_TOOLTIPS_A).unwrap_or_default();
        // Tables A and B are different tables, but many ids carry the same string in both:
        // keep each distinct text once (A first, then B).
        for (id, t) in tips_a.iter().chain(tips_b.iter()) {
            let v = corpus.tooltip_texts.entry(*id).or_default();
            if !v.contains(t) {
                v.push(t.clone());
            }
        }
        // `tooltips` keeps one text per id: B first, then A on top, so A wins (old behaviour).
        let mut tooltips = tips_b;
        tooltips.extend(tips_a);
        corpus.tooltips = tooltips;

        if corpus.names.is_empty() {
            eprintln!("warning: no ability names loaded from '{lang_file}' (wrong path?)");
        }
        eprintln!(
            "Loaded {} ability records, {} names, {} tooltips",
            corpus.skills.len(),
            corpus.names.len(),
            corpus.tooltips.len()
        );
        Ok(corpus)
    }

    /// All tooltip texts of an ability (falls back to the single `tooltips` entry, which is
    /// what synthetic corpora in tests fill).
    pub fn texts_for(&self, id: u32) -> Vec<&str> {
        match self.tooltip_texts.get(&id) {
            Some(v) if !v.is_empty() => v.iter().map(String::as_str).collect(),
            _ => self.tooltips.get(&id).map(|t| vec![t.as_str()]).unwrap_or_default(),
        }
    }

    /// Display name: language table first, then the name embedded in the record.
    pub fn name_of(&self, id: u32) -> String {
        if let Some(n) = self.names.get(&id) {
            return n.clone();
        }
        match self.skills.get(&id) {
            Some(s) if !s.name.is_empty() => s.name.clone(),
            _ => "?".to_string(),
        }
    }
}

/// Reads `path` once and returns `table_id -> (key -> text)` for the wanted tables.
pub fn load_lang(path: &str, wanted: &[u32]) -> HashMap<u32, BTreeMap<u32, String>> {
    let mut out: HashMap<u32, BTreeMap<u32, String>> =
        wanted.iter().map(|&w| (w, BTreeMap::new())).collect();

    let reader = ReaderBuilder::new()
        .has_headers(false)
        .flexible(true)
        .from_path(path);
    let mut reader = match reader {
        Ok(r) => r,
        Err(e) => {
            eprintln!("warning: cannot open language file '{path}': {e}");
            return out;
        }
    };

    for result in reader.records() {
        let Ok(record) = result else { continue };
        let Some(table) = record
            .get(0)
            .and_then(|s| s.trim_matches('"').parse::<u32>().ok())
        else {
            continue;
        };
        let Some(bucket) = out.get_mut(&table) else { continue };
        let Some(key) = record.get(2).and_then(|s| s.parse::<u32>().ok()) else {
            continue;
        };
        let Some(value) = record.get(4) else { continue };
        bucket.insert(key, value.to_string());
    }
    out
}

/// Largest `*_Uncompressed.EsoFileData` in `dir` (the skill file is the biggest one).
pub fn find_skill_data_file(dir: &Path) -> std::io::Result<PathBuf> {
    let suffix = "_Uncompressed.EsoFileData";
    let mut best: Option<(PathBuf, u64)> = None;
    for entry in fs::read_dir(dir)?.flatten() {
        let name = entry.file_name().to_string_lossy().into_owned();
        if !name.ends_with(suffix) {
            continue;
        }
        let size = entry.metadata().map(|m| m.len()).unwrap_or(0);
        if best.as_ref().map_or(true, |(_, s)| size > *s) {
            best = Some((entry.path(), size));
        }
    }
    best.map(|(p, _)| p).ok_or_else(|| {
        std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("no files matching '*{suffix}'"),
        )
    })
}