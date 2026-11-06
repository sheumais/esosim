//! Skill records + language texts -> an in-memory `Corpus`.
//!
//! Everything downstream (graph, resolve, classify) works off `Corpus`, so it can also be
//! built synthetically in unit tests via [`Corpus::from_skills`].

use eso_common::lang::EsoLangFile;
use eso_skill_data::SkillData34;
use std::collections::BTreeMap;

/// Table ids inside the lang file (the record `id`; the record `index` is the key).
pub const LANG_ABILITY_NAMES: u32 = 198_758_357;
/// Set names, keyed by set id.
pub const LANG_SET_NAMES: u32 = 38_727_365;
pub const LANG_ABILITY_TOOLTIPS_A: u32 = 206_046_340;
pub const LANG_ABILITY_TOOLTIPS_B: u32 = 132_143_172;

/// The four lang tables the compiler reads, `key -> text`.
#[derive(Debug, Default)]
pub struct LangTexts {
    pub ability_names: BTreeMap<u32, String>,
    pub tooltips_a: BTreeMap<u32, String>,
    pub tooltips_b: BTreeMap<u32, String>,
    pub set_names: BTreeMap<u32, String>,
}

/// The text as the old CSV export carried it: line breaks stay escaped as the two characters
/// `\n` / `\r`. The classifier and the bundle were built against that form, so keeping it
/// keeps `bundle.jsonl` byte-identical to what the CSV route produced.
fn escape(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    for c in text.chars() {
        match c {
            '\0' => break,
            '\r' => out.push_str("\\r"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
    out
}

impl LangTexts {
    /// One pass over the lang records; later records overwrite earlier ones with the same key.
    pub fn from_lang(lang: &EsoLangFile) -> Self {
        let mut t = LangTexts::default();
        for r in lang.records() {
            let bucket = match r.id {
                LANG_ABILITY_NAMES => &mut t.ability_names,
                LANG_ABILITY_TOOLTIPS_A => &mut t.tooltips_a,
                LANG_ABILITY_TOOLTIPS_B => &mut t.tooltips_b,
                LANG_SET_NAMES => &mut t.set_names,
                _ => continue,
            };
            bucket.insert(r.index, escape(&r.text));
        }
        t
    }
}

pub struct Corpus {
    /// Every parsed record, keyed (and therefore iterated) by ability id.
    pub skills: BTreeMap<u32, SkillData34>,
    pub names: BTreeMap<u32, String>,
    pub tooltips: BTreeMap<u32, String>,
    /// Every tooltip text of an ability, table A first then table B (`tooltips` keeps only
    /// the winner). Placeholders may be spread over several texts, so classification reads all.
    pub tooltip_texts: BTreeMap<u32, Vec<String>>,
}

impl Corpus {
    /// Records only, no lang texts. Duplicate ids: the last record wins.
    pub fn from_skills(list: Vec<SkillData34>) -> Self {
        let skills = list.into_iter().map(|s| (s.ability_id1, s)).collect();
        Corpus {
            skills,
            names: BTreeMap::new(),
            tooltips: BTreeMap::new(),
            tooltip_texts: BTreeMap::new(),
        }
    }

    /// Records plus the ability names and tooltips from the lang file.
    pub fn build(skills: Vec<SkillData34>, texts: LangTexts) -> Self {
        let mut corpus = Corpus::from_skills(skills);
        corpus.names = texts.ability_names;
        let (tips_a, tips_b) = (texts.tooltips_a, texts.tooltips_b);
        // Tables A and B are different tables, but many ids carry the same string in both:
        // keep each distinct text once (A first, then B).
        for (id, t) in tips_a.iter().chain(tips_b.iter()) {
            let v = corpus.tooltip_texts.entry(*id).or_default();
            if !v.contains(t) {
                v.push(t.clone());
            }
        }
        // `tooltips` keeps one text per id: B first, then A on top, so A wins.
        let mut tooltips = tips_b;
        tooltips.extend(tips_a);
        corpus.tooltips = tooltips;
        corpus
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

#[cfg(test)]
mod tests {
    use super::*;

    fn lang(entries: &[(u32, u32, &str)]) -> EsoLangFile {
        let mut l = EsoLangFile::new();
        for (table, key, text) in entries {
            l.add_entry(*table, 0, *key, 0, text);
        }
        l
    }

    #[test]
    fn lang_records_are_sorted_into_their_tables() {
        let t = LangTexts::from_lang(&lang(&[
            (LANG_ABILITY_NAMES, 7, "Fire"),
            (LANG_SET_NAMES, 20, "Archer's Mind"),
            (LANG_ABILITY_TOOLTIPS_A, 7, "a"),
            (LANG_ABILITY_TOOLTIPS_B, 7, "b"),
            (12345, 7, "some other table"),
        ]));
        assert_eq!(t.ability_names.get(&7).map(String::as_str), Some("Fire"));
        assert_eq!(t.set_names.get(&20).map(String::as_str), Some("Archer's Mind"));
        assert_eq!(t.tooltips_a.len() + t.tooltips_b.len(), 2);
    }

    #[test]
    fn line_breaks_stay_escaped_like_the_csv_route() {
        let t = LangTexts::from_lang(&lang(&[(LANG_ABILITY_TOOLTIPS_A, 1, "one\ntwo\r\n\"q\"")]));
        assert_eq!(t.tooltips_a[&1], "one\\ntwo\\r\\n\"q\"");
    }

    #[test]
    fn tooltip_a_wins_and_both_texts_are_kept_once() {
        let mut t = LangTexts::default();
        t.tooltips_a.insert(1, "A".into());
        t.tooltips_b.insert(1, "B".into());
        t.tooltips_a.insert(2, "same".into());
        t.tooltips_b.insert(2, "same".into());
        let c = Corpus::build(vec![], t);
        assert_eq!(c.tooltips[&1], "A");
        assert_eq!(c.tooltip_texts[&1], vec!["A", "B"]);
        assert_eq!(c.tooltip_texts[&2], vec!["same"]);
    }
}
