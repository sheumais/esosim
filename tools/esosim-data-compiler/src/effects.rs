//! `effects.jsonl`: overrides ∪ high-confidence auto-classified effects (roadmap 2.0 item 4).
//!
//! One `EffectDef` per line, sorted by id, with a `provenance` field saying where it came from.
//! An override always wins: an id that has one is never auto-generated.
//!
//! **Auto rows are deliberately few.** Only abilities listed in `roots.toml` under `buffs`
//! qualify (Major/Minor buffs and debuffs), and only when the classifier is sure: one channel,
//! agreed by every tooltip, no guessed (indirect) evidence, and the record's `value1` points the
//! way the tooltip's verb does. The value is `value1` as the game stores it; whether that is in
//! the engine channel's unit (rating, percentage points) is **not** checked, so every auto row says
//! so in its `note`. Sets, mundus, CP, racial and consumables need a human (`suggest-effects`).

use crate::overrides::Override;
use crate::resolved::{ClassStatus, Direction, ResolvedAbility};
use crate::roots::RootConfig;
use esosim_types::{ChannelValue, Condition, EffectDef, EffectSource, Stacking};
use serde::Serialize;
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum Provenance {
    Override,
    Auto,
}

#[derive(Serialize)]
struct Row<'a> {
    #[serde(flatten)]
    def: &'a EffectDef,
    provenance: Provenance,
}

/// The effects the classifier is confident enough about to write without review.
/// `taken` are ids that have an override (or are hand-written elsewhere) and are skipped.
pub fn auto_effects(abilities: &[ResolvedAbility], config: &RootConfig, taken: &BTreeSet<u32>) -> Vec<EffectDef> {
    let buffs = config.listed_ids(crate::resolved::RootCategory::Buffs);
    let debuffs = config.debuff_ids();

    let mut out = Vec::new();
    for a in abilities {
        if !buffs.contains(&a.id) || taken.contains(&a.id) || a.value1 == 0 {
            continue;
        }
        let Some(class) = &a.classification else { continue };
        if class.status != ClassStatus::Classified || class.channels.len() != 1 {
            continue;
        }
        if class.evidence.is_empty() || class.evidence.iter().any(|e| e.indirect) {
            continue;
        }
        // the sign of `value1` has to agree with the tooltip's verb, otherwise it is a guess
        let agrees = match class.direction {
            Some(Direction::Raises) => a.value1 > 0,
            Some(Direction::Lowers) => a.value1 < 0,
            None => false,
        };
        if !agrees {
            continue;
        }
        let source = if debuffs.contains(&a.id) {
            EffectSource::Debuff { ability_id: a.id }
        } else {
            EffectSource::Buff { ability_id: a.id }
        };
        out.push(EffectDef {
            id: a.id,
            name: a.name.clone(),
            source,
            channels: vec![ChannelValue { channel: class.channels[0], value: a.value1 as f64, value_per_stack: 0.0 }],
            condition: Condition::Always,
            stacking: Stacking::Unique,
            duration_ms: None,
            cooldown_ms: None,
            note: Some(format!("auto: value1 = {} as stored by the game; unit and sign not checked against the engine", a.value1)),
        });
    }
    out.sort_by_key(|d| d.id);
    out
}

/// `effects.jsonl`: every override, then every auto effect whose id has no override, merged and
/// sorted by id. LF endings, nothing machine specific.
pub fn to_jsonl(overrides: &[Override], auto: &[EffectDef]) -> Result<String, String> {
    let mut rows: Vec<(u32, Row)> = Vec::new();
    for o in overrides {
        rows.push((o.def.id, Row { def: &o.def, provenance: Provenance::Override }));
    }
    let overridden: BTreeSet<u32> = overrides.iter().map(|o| o.def.id).collect();
    for d in auto {
        if !overridden.contains(&d.id) {
            rows.push((d.id, Row { def: d, provenance: Provenance::Auto }));
        }
    }
    rows.sort_by_key(|(id, _)| *id);

    let mut out = String::new();
    for (id, row) in &rows {
        out.push_str(&serde_json::to_string(row).map_err(|e| format!("serialising effect {id}: {e}"))?);
        out.push('\n');
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use esosim_types::Channel;

    fn ability(json: &str) -> ResolvedAbility {
        serde_json::from_str(json).unwrap()
    }

    /// A classified single-channel buff with `value1`.
    fn buff(id: u32, value1: i32, direction: &str, indirect: bool) -> ResolvedAbility {
        ability(&format!(
            r#"{{"id":{id},"name":"B{id}","value1":{value1},"classification":{{"status":"classified","channels":["CriticalDamage"],"direction":"{direction}","evidence":[{{"parent":1,"placeholder":1,"clause":"@","rule":"r","channels":["CriticalDamage"],"direction":"{direction}"{}}}]}}}}"#,
            if indirect { r#","indirect":true"# } else { "" }
        ))
    }

    fn config() -> RootConfig {
        crate::roots::parse_config("[buffs]\nids = [10, 11, 12, 13]\ndebuff_ids = [14]\n").unwrap()
    }

    #[test]
    fn only_confident_listed_buffs_become_auto_effects() {
        let abilities = vec![
            buff(10, 10, "raises", false),  // ok
            buff(11, 10, "lowers", false),  // sign disagrees with the verb
            buff(12, 10, "raises", true),   // guessed evidence
            buff(99, 10, "raises", false),  // not listed in roots.toml
            buff(13, 10, "raises", false),  // has an override
            buff(14, -7, "lowers", false),  // a debuff
        ];
        let taken = BTreeSet::from([13]);
        let auto = auto_effects(&abilities, &config(), &taken);
        assert_eq!(auto.iter().map(|d| d.id).collect::<Vec<_>>(), [10, 14]);

        assert_eq!(auto[0].source, EffectSource::Buff { ability_id: 10 });
        assert_eq!(auto[0].channels[0].channel, Channel::CriticalDamage);
        assert_eq!(auto[0].channels[0].value, 10.0);
        assert!(auto[0].note.as_deref().unwrap().starts_with("auto:"));
        assert_eq!(auto[1].source, EffectSource::Debuff { ability_id: 14 });
        assert_eq!(auto[1].channels[0].value, -7.0);
    }

    #[test]
    fn overrides_win_and_every_row_says_where_it_came_from() {
        let over = crate::overrides::parse(
            "[[effect]]\nid = 10\nname = \"Mine\"\nsource = \"Food\"\nchannels = [{ channel = \"CriticalChance\", value = 1.0 }]\n",
        )
        .unwrap();
        let auto = auto_effects(&[buff(10, 10, "raises", false), buff(11, 5, "raises", false)], &config(), &BTreeSet::new());
        assert_eq!(auto.len(), 2);

        let text = to_jsonl(&over, &auto).unwrap();
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 2, "the auto row for 10 is dropped");
        assert!(lines[0].starts_with("{\"id\":10,\"name\":\"Mine\""), "{}", lines[0]);
        assert!(lines[0].ends_with("\"provenance\":\"override\"}"), "{}", lines[0]);
        assert!(lines[1].contains("\"id\":11") && lines[1].ends_with("\"provenance\":\"auto\"}"), "{}", lines[1]);
    }

    #[test]
    fn a_row_loads_back_as_an_effect_def_in_the_engine() {
        // the engine's loader reads each line as an `EffectDef`; the extra `provenance` field must not hurt
        let auto = auto_effects(&[buff(10, 10, "raises", false)], &config(), &BTreeSet::new());
        let text = to_jsonl(&[], &auto).unwrap();
        let back: EffectDef = serde_json::from_str(text.lines().next().unwrap()).unwrap();
        assert_eq!(back, auto[0]);
    }

    #[test]
    fn no_effects_is_an_empty_file() {
        assert_eq!(to_jsonl(&[], &[]).unwrap(), "");
    }
}
