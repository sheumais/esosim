//! `verify`: hold everything a person wrote to the game data (roadmap 2.0 item 5).
//!
//! It works on files, not on the game: `bundle.jsonl`, `sets.jsonl`, `cp.jsonl`, `overrides.toml` and
//! `roots.toml`, so it runs in `cargo xtask check` on a machine with no game install, and after
//! `compile` on the fresh output. Every failure is a patch (or a typo) that made hand-written
//! data stale:
//!
//! * an override's ability exists in the bundle, and its language name matches (`game_name`);
//! * every id an override refers to exists: its `source` ability, the buffs its `condition`
//!   asks about, the abilities it needs slotted, the sets it needs pieces of;
//! * where the override carries a `check`, its value equals the number in the ability record;
//! * no override repeats an id that is hand-written elsewhere: the effect would be counted twice;
//! * every id and name rule of `roots.toml` still finds what it was written for (once the
//!   bundle carries `categories`, i.e. was written by a compiler that knows root categories).

use crate::overrides::{self, Override};
use crate::resolved::ResolvedAbility;
use crate::roots::{self, RootConfig};
use esosim_types::{Condition, EffectSource};
use std::collections::{BTreeSet, HashMap};

/// The files `verify` reads, as text.
pub struct VerifyInput<'a> {
    pub abilities_jsonl: &'a str,
    pub sets_jsonl: &'a str,
    pub cp_jsonl: &'a str,
    pub overrides_toml: &'a str,
    pub roots_toml: &'a str,
    /// Effect files written by hand elsewhere, as `(label, jsonl)`; an override must not reuse their ids.
    pub other_effects: Vec<(String, &'a str)>,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct VerifyReport {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    /// How many overrides and listed root ids were looked at.
    pub overrides_checked: usize,
    pub roots_checked: usize,
}

impl VerifyReport {
    pub fn ok(&self) -> bool {
        self.errors.is_empty()
    }

    /// The console report: one line per finding, then a verdict.
    pub fn summary(&self) -> String {
        let mut out = String::new();
        for e in &self.errors {
            out.push_str(&format!("error: {e}\n"));
        }
        for w in &self.warnings {
            out.push_str(&format!("warning: {w}\n"));
        }
        out.push_str(&format!(
            "verify: {} ({} override(s), {} root id(s) checked; {} error(s), {} warning(s))",
            if self.ok() { "ok" } else { "FAILED" },
            self.overrides_checked,
            self.roots_checked,
            self.errors.len(),
            self.warnings.len()
        ));
        out
    }
}

/// `{"id":N,...}` lines of a JSONL file as ids; blank lines and `//` comments are skipped.
pub(crate) fn ids_of_jsonl(text: &str) -> Result<BTreeSet<u32>, String> {
    let mut ids = BTreeSet::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.trim();
        if line.is_empty() || line.starts_with("//") {
            continue;
        }
        let v: serde_json::Value = serde_json::from_str(line).map_err(|e| format!("line {}: {e}", i + 1))?;
        match v.get("id").and_then(|x| x.as_u64()) {
            Some(id) => {
                ids.insert(id as u32);
            }
            None => return Err(format!("line {}: no numeric `id`", i + 1)),
        }
    }
    Ok(ids)
}

pub(crate) fn parse_abilities(text: &str) -> Result<Vec<ResolvedAbility>, String> {
    let mut out = Vec::new();
    for (i, line) in text.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        out.push(serde_json::from_str(line).map_err(|e| format!("bundle.jsonl line {}: {e}", i + 1))?);
    }
    Ok(out)
}

/// Visit `c` and everything nested in it.
fn walk(c: &Condition, f: &mut dyn FnMut(&Condition)) {
    f(c);
    match c {
        Condition::All(v) | Condition::Any(v) => v.iter().for_each(|x| walk(x, f)),
        Condition::Not(x) => walk(x, f),
        _ => {}
    }
}

fn source_ability(s: &EffectSource) -> Option<u32> {
    match s {
        EffectSource::Mundus { ability_id }
        | EffectSource::Passive { ability_id }
        | EffectSource::ActiveSkill { ability_id }
        | EffectSource::Buff { ability_id }
        | EffectSource::Debuff { ability_id } => Some(*ability_id),
        _ => None,
    }
}

fn record_value(a: &ResolvedAbility, field: &str) -> Option<f64> {
    match field {
        "value0" => Some(a.value0 as f64),
        "value1" => Some(a.value1 as f64),
        "value2" => Some(a.value2 as f64),
        _ => None,
    }
}

fn same_name(a: &str, b: &str) -> bool {
    a.trim().eq_ignore_ascii_case(b.trim())
}

pub fn verify(input: &VerifyInput) -> VerifyReport {
    let mut r = VerifyReport::default();

    let abilities = match parse_abilities(input.abilities_jsonl) {
        Ok(a) => a,
        Err(e) => {
            r.errors.push(e);
            return r;
        }
    };
    let by_id: HashMap<u32, &ResolvedAbility> = abilities.iter().map(|a| (a.id, a)).collect();
    let has_categories = abilities.iter().any(|a| !a.categories.is_empty());

    let set_ids = match ids_of_jsonl(input.sets_jsonl) {
        Ok(s) => s,
        Err(e) => {
            r.errors.push(format!("sets.jsonl {e}"));
            BTreeSet::new()
        }
    };

    let mut hand_written: HashMap<u32, String> = HashMap::new();
    for (label, text) in &input.other_effects {
        match ids_of_jsonl(text) {
            Ok(ids) => {
                for id in ids {
                    hand_written.insert(id, label.clone());
                }
            }
            Err(e) => r.errors.push(format!("{label} {e}")),
        }
    }

    // ---- overrides
    let overrides: Vec<Override> = match overrides::parse(input.overrides_toml) {
        Ok(o) => o,
        Err(e) => {
            r.errors.push(e);
            Vec::new()
        }
    };
    let override_ids: BTreeSet<u32> = overrides.iter().map(|o| o.def.id).collect();

    for o in &overrides {
        r.overrides_checked += 1;
        let who = format!("override {} `{}`", o.def.id, o.def.name);

        if let Some(file) = hand_written.get(&o.def.id) {
            r.errors.push(format!("{who}: the id is also written by hand in {file}; the effect would count twice"));
        }

        if !o.is_synthetic() {
            match by_id.get(&o.def.id) {
                None => r.errors.push(format!(
                    "{who}: ability {} is not in the bundle (removed or renumbered by a patch?)",
                    o.def.id
                )),
                Some(a) => {
                    if !same_name(&a.name, &o.game_name) {
                        r.errors.push(format!(
                            "{who}: the game calls ability {} `{}`, the override says `{}`",
                            o.def.id, a.name, o.game_name
                        ));
                    }
                    if let Some(check) = &o.check {
                        // `overrides::parse` already made sure the field name and channel index are valid
                        if let (Some(expected), Some(cv)) = (
                            record_value(a, &check.field).map(|v| v * check.scale),
                            o.def.channels.get(check.channel),
                        ) {
                            if (cv.value - expected).abs() > 1e-6 * expected.abs().max(1.0) {
                                r.errors.push(format!(
                                    "{who}: channel {} has value {} but the record's {} is {} (x {} = {})",
                                    check.channel,
                                    cv.value,
                                    check.field,
                                    record_value(a, &check.field).unwrap_or(0.0),
                                    check.scale,
                                    expected
                                ));
                            }
                        }
                    }
                }
            }
        }

        if let Some(id) = source_ability(&o.def.source) {
            if !by_id.contains_key(&id) {
                r.errors.push(format!("{who}: its source refers to ability {id}, which is not in the bundle"));
            }
        }
        if let EffectSource::Set { set_id, .. } = &o.def.source {
            if !set_ids.contains(&(*set_id as u32)) {
                r.errors.push(format!("{who}: its source refers to set {set_id}, which is not in sets.jsonl"));
            }
        }

        let mut problems: Vec<String> = Vec::new();
        walk(&o.def.condition, &mut |c| match c {
            Condition::SelfBuffActive(id) | Condition::TargetDebuffActive(id) => {
                let known = by_id.contains_key(id) || override_ids.contains(id) || hand_written.contains_key(id);
                if !known {
                    problems.push(format!("its condition asks about buff {id}, which is not in the bundle or any effect file"));
                }
            }
            Condition::AbilitySlotted { ability_id } => {
                if !by_id.contains_key(ability_id) {
                    problems.push(format!("its condition needs ability {ability_id}, which is not in the bundle"));
                }
            }
            Condition::SetPieces { set_id, .. } => {
                if !set_ids.contains(&(*set_id as u32)) {
                    problems.push(format!("its condition needs pieces of set {set_id}, which is not in sets.jsonl"));
                }
            }
            _ => {}
        });
        for p in problems {
            r.errors.push(format!("{who}: {p}"));
        }
    }

    // ---- roots.toml
    let config = match roots::parse_config(input.roots_toml) {
        Ok(c) => c,
        Err(e) => {
            r.errors.push(e);
            RootConfig::default()
        }
    };
    let mut missing_in_old_bundle = 0usize;
    for (category, spec) in &config.categories {
        for id in config.listed_ids(*category) {
            r.roots_checked += 1;
            match by_id.get(&id) {
                None if has_categories => r.errors.push(format!(
                    "roots.toml: [{}] lists ability {id}, which is not in the bundle (removed or renumbered by a patch?)",
                    category.name()
                )),
                None => missing_in_old_bundle += 1,
                Some(a) if has_categories && !a.categories.contains(category) => r.errors.push(format!(
                    "roots.toml: [{}] lists ability {id} (`{}`), but the bundle does not label it `{}`",
                    category.name(),
                    a.name,
                    category.name()
                )),
                Some(_) => {}
            }
        }
        if has_categories && (!spec.names.is_empty() || spec.name_prefix.is_some()) {
            let found = abilities
                .iter()
                .filter(|a| a.categories.contains(category))
                .filter(|a| {
                    spec.names.iter().any(|n| *n == a.name)
                        || spec.name_prefix.as_deref().is_some_and(|p| a.name.starts_with(p))
                })
                .count();
            if let Some(expected) = spec.expect {
                if found != expected {
                    r.errors.push(format!(
                        "roots.toml: [{}] expects {expected} abilities by name, the bundle has {found}",
                        category.name()
                    ));
                }
            }
        }
    }
    if missing_in_old_bundle > 0 {
        r.warnings.push(format!(
            "{missing_in_old_bundle} id(s) listed in roots.toml are not in bundle.jsonl, which has no root categories yet: run `cargo xtask sync-data`"
        ));
    }

    // ---- set abilities that are not in the bundle (known: 259723 of set 856); informative only
    match roots::set_ability_ids(input.sets_jsonl) {
        Ok(ids) => {
            let missing: Vec<u32> = ids.into_iter().filter(|id| !by_id.contains_key(id)).collect();
            if !missing.is_empty() {
                let shown: Vec<String> = missing.iter().take(5).map(u32::to_string).collect();
                r.warnings.push(format!(
                    "{} set ability id(s) are not in the bundle: {}{}",
                    missing.len(),
                    shown.join(", "),
                    if missing.len() > 5 { ", ..." } else { "" }
                ));
            }
        }
        Err(e) => r.errors.push(e),
    }

    // ---- champion point abilities that are not in the bundle; informative only
    match crate::cp::ability_ids_of_jsonl(input.cp_jsonl) {
        Ok(ids) => {
            let missing: Vec<u32> = ids.into_iter().filter(|id| !by_id.contains_key(id)).collect();
            if !missing.is_empty() {
                let shown: Vec<String> = missing.iter().take(5).map(u32::to_string).collect();
                r.warnings.push(format!(
                    "{} champion point ability id(s) are not in the bundle: {}{}",
                    missing.len(),
                    shown.join(", "),
                    if missing.len() > 5 { ", ..." } else { "" }
                ));
            }
        }
        Err(e) => r.errors.push(e),
    }
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    const ABILITIES: &str = concat!(
        r#"{"id":61746,"name":"Minor Force","value1":10,"categories":["buffs"]}"#, "\n",
        r#"{"id":76617,"name":"Minor Slayer","value1":5,"categories":["buffs"]}"#, "\n",
        r#"{"id":13975,"name":"Boon: The Thief","value1":12,"categories":["mundus"]}"#, "\n",
        r#"{"id":13984,"name":"Boon: The Shadow","categories":["mundus"]}"#, "\n",
    );
    const SETS: &str = "{\"id\":19,\"abilities\":[{\"ability_id\":76617}],\"keystone\":76617}\n";
    const ROOTS: &str = "[buffs]\nids = [61746, 76617]\n[mundus]\nname_prefix = \"Boon: \"\nexpect = 2\n";

    fn effect(extra: &str) -> String {
        format!(
            "[[effect]]\nid = 13975\nname = \"Thief (Mundus)\"\ngame_name = \"Boon: The Thief\"\nsource = {{ Mundus = {{ ability_id = 13975 }} }}\nchannels = [{{ channel = \"CriticalChance\", value = 1212.0 }}]\n{extra}"
        )
    }

    fn run(abilities: &str, overrides: &str, roots: &str) -> VerifyReport {
        verify(&VerifyInput {
            abilities_jsonl: abilities,
            sets_jsonl: SETS,
            cp_jsonl: "",
            overrides_toml: overrides,
            roots_toml: roots,
            other_effects: vec![("legacy.jsonl".into(), "{\"id\":61746,\"name\":\"x\"}\n// c\n")],
        })
    }

    fn has_error(r: &VerifyReport, part: &str) -> bool {
        r.errors.iter().any(|e| e.contains(part))
    }

    #[test]
    fn good_data_passes() {
        let r = run(ABILITIES, &effect("check = { field = \"value1\", scale = 101.0 }\n"), ROOTS);
        assert_eq!(r.errors, Vec::<String>::new());
        assert!(r.ok());
        assert_eq!((r.overrides_checked, r.roots_checked), (1, 2));
        assert!(r.summary().contains("verify: ok"), "{}", r.summary());
    }

    #[test]
    fn an_override_whose_id_disappears_in_a_patch_fails() {
        let without = ABILITIES.lines().filter(|l| !l.contains("13975")).collect::<Vec<_>>().join("\n");
        let r = run(&without, &effect(""), ROOTS);
        assert!(has_error(&r, "ability 13975 is not in the bundle"), "{:?}", r.errors);
        assert!(!r.ok());
        assert!(r.summary().contains("verify: FAILED"));
    }

    #[test]
    fn a_renamed_ability_fails() {
        let renamed = ABILITIES.replace("Boon: The Thief", "Boon: The Burglar");
        let r = run(&renamed, &effect(""), ROOTS);
        assert!(has_error(&r, "the game calls ability 13975 `Boon: The Burglar`, the override says `Boon: The Thief`"), "{:?}", r.errors);
    }

    #[test]
    fn the_name_defaults_to_the_effect_name_and_ignores_case() {
        let toml = effect("").replace("game_name = \"Boon: The Thief\"\n", "");
        assert!(has_error(&run(ABILITIES, &toml, ROOTS), "the override says `Thief (Mundus)`"));
        let toml = effect("").replace("Boon: The Thief", "BOON: the thief");
        assert!(run(ABILITIES, &toml, ROOTS).ok());
    }

    #[test]
    fn a_value_that_no_longer_matches_the_record_fails() {
        // the record says 12; 12 * 101 = 1212 passes (see good_data_passes), 12 * 100 does not
        let r = run(ABILITIES, &effect("check = { field = \"value1\", scale = 100.0 }\n"), ROOTS);
        assert!(has_error(&r, "channel 0 has value 1212 but the record's value1 is 12 (x 100 = 1200)"), "{:?}", r.errors);
    }

    #[test]
    fn referenced_ids_must_exist() {
        let cond = "condition = { All = [{ SelfBuffActive = 99 }, { SetPieces = { set_id = 7, at_least = 5 } }, { AbilitySlotted = { ability_id = 98 } }] }\n";
        let r = run(ABILITIES, &effect(cond), ROOTS);
        assert!(has_error(&r, "asks about buff 99"), "{:?}", r.errors);
        assert!(has_error(&r, "pieces of set 7"), "{:?}", r.errors);
        assert!(has_error(&r, "needs ability 98"), "{:?}", r.errors);
        // a buff that is in the bundle, a set that exists, and a buff another effect file defines are fine
        let ok = "condition = { All = [{ SelfBuffActive = 61746 }, { SetPieces = { set_id = 19, at_least = 5 } }] }\n";
        assert!(run(ABILITIES, &effect(ok), ROOTS).ok());

        let src = effect("").replace("Mundus = { ability_id = 13975 }", "Buff = { ability_id = 555 }");
        assert!(has_error(&run(ABILITIES, &src, ROOTS), "source refers to ability 555"));
        let set_src = effect("").replace("Mundus = { ability_id = 13975 }", "Set = { set_id = 8, pieces_required = 5 }");
        assert!(has_error(&run(ABILITIES, &set_src, ROOTS), "source refers to set 8"));
    }

    #[test]
    fn an_id_that_is_also_hand_written_elsewhere_fails() {
        let dup = effect("").replace("id = 13975", "id = 61746").replace("Mundus = { ability_id = 13975 }", "Buff = { ability_id = 61746 }");
        let dup = dup.replace("game_name = \"Boon: The Thief\"", "game_name = \"Minor Force\"");
        let r = run(ABILITIES, &dup, ROOTS);
        assert!(has_error(&r, "also written by hand in legacy.jsonl"), "{:?}", r.errors);
    }

    #[test]
    fn synthetic_ids_are_not_looked_up() {
        let toml = "[[effect]]\nid = 2147483700\nname = \"Synthetic\"\nsource = \"Food\"\nchannels = [{ channel = \"CriticalChance\", value = 1.0 }]\n";
        assert!(run(ABILITIES, toml, ROOTS).ok());
    }

    #[test]
    fn roots_that_a_patch_removed_or_that_the_bundle_does_not_label_fail() {
        let without = ABILITIES.lines().filter(|l| !l.contains("76617")).collect::<Vec<_>>().join("\n");
        assert!(has_error(&run(&without, "", ROOTS), "[buffs] lists ability 76617, which is not in the bundle"));

        let unlabelled = ABILITIES.replace("\"name\":\"Minor Force\",\"value1\":10,\"categories\":[\"buffs\"]", "\"name\":\"Minor Force\",\"value1\":10,\"categories\":[\"cp\"]");
        assert!(has_error(&run(&unlabelled, "", ROOTS), "does not label it `buffs`"));

        let fewer = ROOTS.replace("expect = 2", "expect = 3");
        assert!(has_error(&run(ABILITIES, "", &fewer), "[mundus] expects 3 abilities by name, the bundle has 2"));
    }

    #[test]
    fn a_bundle_without_categories_only_warns_about_roots() {
        let old = "{\"id\":61746,\"name\":\"Minor Force\"}\n";
        let r = run(old, "", ROOTS);
        assert!(r.ok(), "{:?}", r.errors);
        assert!(r.warnings.iter().any(|w| w.contains("no root categories yet")), "{:?}", r.warnings);
        assert!(r.warnings.iter().any(|w| w.contains("1 set ability id(s) are not in the bundle: 76617")), "{:?}", r.warnings);
    }

    #[test]
    fn unreadable_input_is_reported_not_a_panic() {
        assert!(has_error(&run("{nope", "", ROOTS), "bundle.jsonl line 1"));
        assert!(has_error(&run(ABILITIES, "[[effect]]\nid = 1\n", ROOTS), "overrides.toml"));
        assert!(has_error(&run(ABILITIES, "", "[nowhere]\n"), "unknown category"));
    }
}
