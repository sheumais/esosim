//! `overrides.toml`: hand-written `EffectDef`s that always beat auto-classification
//! (roadmap 2.0 item 2, decision D4).
//!
//! Everything the classifier cannot see (conditions, stacking, procs, durations) is written here
//! by a person. The file is the `EffectDef` of `esosim-types` as TOML plus a few fields that only
//! the compiler uses (`game_name`, `source_url`, `check`); see the header of `data/overrides.toml`.

use esosim_types::{Application, ChannelValue, Condition, EffectDef, EffectSource, Stacking, Trigger};
use serde::Deserialize;
use std::collections::BTreeSet;

/// The embedded default, `data/overrides.toml`.
pub const OVERRIDES_TOML: &str = include_str!("../data/overrides.toml");

/// Which number of the ability record an override's value must equal (`verify`).
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValueCheck {
    /// `value0`, `value1` or `value2`.
    pub field: String,
    /// `channels[channel].value` must equal the record's field times this. Default 1.
    #[serde(default = "one")]
    pub scale: f64,
    /// Index into `channels`. Default 0.
    #[serde(default)]
    pub channel: usize,
    /// Another ability whose same field is subtracted first: `(this - that) x scale`. For a tier that
    /// holds only the increase over the tier below (True-Sworn Fury's 75 % tier is record 150704 minus
    /// the base record 150703).
    #[serde(default)]
    pub minus: Option<u32>,
}

fn one() -> f64 {
    1.0
}

/// One `[[effect]]` table, as written.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawOverride {
    id: u32,
    name: String,
    #[serde(default)]
    game_name: Option<String>,
    source: EffectSource,
    /// Empty is fine for a proc that only applies other effects (`applies`).
    #[serde(default)]
    channels: Vec<ChannelValue>,
    #[serde(default)]
    condition: Condition,
    #[serde(default)]
    stacking: Stacking,
    #[serde(default)]
    buff_code: Option<u8>,
    #[serde(default)]
    duration_ms: Option<u32>,
    #[serde(default)]
    cooldown_ms: Option<u32>,
    /// What fires a proc: `trigger = { event = { Cast = { SkillLine = "Psijic Order" } }, chance_pct = 33 }`.
    #[serde(default)]
    trigger: Option<Trigger>,
    /// Effects this one puts on someone: `applies = [{ effect_id = 61743, recipient = "Target", duration_ms = 4000 }]`
    /// ("when you deal critical damage you apply Major Breach to the enemy"). The applied effect is written as an
    /// `[[effect]]` of its own (a `Debuff` or `Buff` source), once, however many abilities apply it.
    #[serde(default)]
    applies: Vec<Application>,
    #[serde(default)]
    note: Option<String>,
    #[serde(default)]
    source_url: Option<String>,
    #[serde(default)]
    check: Option<ValueCheck>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFile {
    #[serde(default)]
    effect: Vec<RawOverride>,
}

/// An override: the effect, and what `verify` should hold it to.
#[derive(Debug, Clone, PartialEq)]
pub struct Override {
    pub def: EffectDef,
    /// The ability's name in the game (defaults to `def.name`).
    pub game_name: String,
    pub source_url: Option<String>,
    pub check: Option<ValueCheck>,
}

/// Ids from here up cannot be an ability's. Nothing may use them: an effect with no ability behind it is a gap in
/// the architecture, to be flagged in the roadmap, not given an invented id.
const NO_ABILITY_HAS_AN_ID_FROM: u32 = 0x8000_0000;

/// Parse `overrides.toml`, sorted by id. Rejects duplicate ids, unknown fields, an empty
/// `channels` list and a `check` that points at nothing.
pub fn parse(text: &str) -> Result<Vec<Override>, String> {
    let raw: RawFile = toml::from_str(text).map_err(|e| format!("overrides.toml: {e}"))?;
    let mut seen = BTreeSet::new();
    let mut out = Vec::with_capacity(raw.effect.len());
    for r in raw.effect {
        let at = |msg: &str| format!("overrides.toml: effect {} (`{}`): {msg}", r.id, r.name);
        if !seen.insert(r.id) {
            return Err(at("duplicate id"));
        }
        if r.id >= NO_ABILITY_HAS_AN_ID_FROM {
            return Err(at("synthetic ids are not allowed: write the effect against the ability record it comes from, or flag the gap in the roadmap"));
        }
        if r.channels.is_empty() && r.applies.is_empty() {
            return Err(at("has no channels and applies nothing"));
        }
        if let Some(a) = r.applies.iter().find(|a| a.effect_id == r.id || a.stacks == 0) {
            return Err(at(&format!("applies {}: an effect cannot apply itself, and an application adds at least 1 stack", a.effect_id)));
        }
        if let Some(t) = &r.trigger {
            if t.chance_pct.is_some_and(|p| p == 0 || p > 100) {
                return Err(at("trigger.chance_pct must be 1 to 100 (leave it out for \"always\")"));
            }
        }
        if let Some(c) = &r.check {
            if !matches!(c.field.as_str(), "value0" | "value1" | "value2") {
                return Err(at("check.field must be value0, value1 or value2"));
            }
            if c.channel >= r.channels.len() {
                return Err(at("check.channel is past the end of channels"));
            }
        }
        let game_name = r.game_name.clone().unwrap_or_else(|| r.name.clone());
        out.push(Override {
            def: EffectDef {
                id: r.id,
                name: r.name,
                source: r.source,
                channels: r.channels,
                condition: r.condition,
                stacking: r.stacking,
                buff_code: r.buff_code,
                duration_ms: r.duration_ms,
                cooldown_ms: r.cooldown_ms,
                trigger: r.trigger,
                applies: r.applies,
                note: r.note,
            },
            game_name,
            source_url: r.source_url,
            check: r.check,
        });
    }
    out.sort_by_key(|o| o.def.id);
    Ok(out)
}

/// The embedded `overrides.toml`, parsed.
pub fn embedded() -> Result<Vec<Override>, String> {
    parse(OVERRIDES_TOML)
}

#[cfg(test)]
mod tests {
    use super::*;
    use esosim_types::{Channel, Recipient, ResistableDamageType};

    const SAMPLE: &str = r#"
[[effect]]
id = 13975
name = "Thief (Mundus)"
game_name = "Boon: The Thief"
source = { Mundus = { ability_id = 13975 } }
channels = [{ channel = "CriticalChance", value = 1212 }]
check = { field = "value1" }
source_url = "https://en.uesp.net/wiki/Online:The_Thief_(Mundus_Stone)"

[[effect]]
id = 142035
name = "Fortified"
source = { Buff = { ability_id = 142035 } }
channels = [{ channel = { Armour = "All" }, value = 0.0, value_per_stack = 34.62 }]
stacking = { PerStack = { max = 10 } }
condition = { All = [{ SetPieces = { set_id = 19, at_least = 5 } }, "InCombat"] }
duration_ms = 6000
note = "Red CP"
"#;

    #[test]
    fn the_shipped_file_parses() {
        // no entries yet; the point is that the header and the schema stay valid
        embedded().unwrap();
    }

    #[test]
    fn effects_are_read_with_every_enum_shape_and_sorted_by_id() {
        let v = parse(SAMPLE).unwrap();
        assert_eq!(v.iter().map(|o| o.def.id).collect::<Vec<_>>(), [13975, 142035]);

        let thief = &v[0];
        assert_eq!(thief.game_name, "Boon: The Thief");
        assert_eq!(thief.def.source, EffectSource::Mundus { ability_id: 13975 });
        assert_eq!(thief.def.channels[0].channel, Channel::CriticalChance);
        assert_eq!(thief.def.channels[0].value, 1212.0, "an integer is fine for a float field");
        assert_eq!(thief.check, Some(ValueCheck { field: "value1".into(), scale: 1.0, channel: 0, minus: None }));

        let fort = &v[1];
        assert_eq!(fort.game_name, "Fortified", "defaults to name");
        assert_eq!(fort.def.channels[0].channel, Channel::Armour(ResistableDamageType::All));
        assert_eq!(fort.def.stacking, Stacking::PerStack { max: 10 });
        assert_eq!(fort.def.duration_ms, Some(6000));
        assert_eq!(
            fort.def.condition,
            Condition::All(vec![Condition::SetPieces { set_id: 19, at_least: 5 }, Condition::InCombat])
        );
    }

    #[test]
    fn mistakes_are_errors_that_name_the_effect() {
        let one = |extra: &str| {
            format!("[[effect]]\nid = 5\nname = \"X\"\nsource = \"Food\"\nchannels = [{{ channel = \"CriticalChance\", value = 1 }}]\n{extra}")
        };
        parse(&one("")).unwrap();
        assert!(parse(&one("stackin = \"Unique\"\n")).is_err(), "unknown field");
        assert!(parse(&format!("{0}{0}", one(""))).unwrap_err().contains("duplicate id"));
        assert!(parse(&one("check = { field = \"value9\" }\n")).unwrap_err().contains("value0, value1 or value2"));
        assert!(parse(&one("check = { field = \"value1\", channel = 1 }\n")).unwrap_err().contains("past the end"));
        let none = "[[effect]]\nid = 6\nname = \"Y\"\nsource = \"Food\"\nchannels = []\n";
        assert!(parse(none).unwrap_err().contains("effect 6 (`Y`): has no channels and applies nothing"));
    }

    #[test]
    fn a_check_can_subtract_another_record() {
        let one = "[[effect]]\nid = 5\nname = \"X\"\nsource = \"Food\"\nchannels = [{ channel = \"CriticalChance\", value = 823 }]\ncheck = { field = \"value1\", minus = 150703 }\n";
        assert_eq!(parse(one).unwrap()[0].check.as_ref().unwrap().minus, Some(150703));
    }

    #[test]
    fn a_trigger_loads_with_its_condition_and_a_bad_chance_is_an_error() {
        let text = |chance: &str| format!(
            "[[effect]]\nid = 75746\nname = \"Proc\"\nsource = {{ Set = {{ set_id = 225, pieces_required = 5 }} }}\n\
             channels = [{{ channel = \"CriticalDamage\", value = 1.0 }}]\ncondition = \"InCombat\"\n\
             trigger = {{ event = {{ Cast = {{ SkillLine = \"Psijic Order\" }} }}{chance}, cooldown_ms = 5000 }}\n"
        );
        let ok = parse(&text(", chance_pct = 33")).unwrap();
        let t = ok[0].def.trigger.expect("trigger");
        assert_eq!((t.chance_pct, t.cooldown_ms), (Some(33), Some(5000)));
        assert_eq!(ok[0].def.condition, Condition::InCombat);
        let err = parse(&text(", chance_pct = 150")).unwrap_err();
        assert!(err.contains("chance_pct") && err.contains("75746"), "{err}");
    }

    #[test]
    fn the_shipped_file_covers_the_set_procs_written_so_far() {
        // 1 (Clever Alchemist) + 26 set procs + 8 that apply a debuff to the target + Contagion (waits for an application); a proc dropped by accident fails here
        let procs: Vec<_> = embedded().unwrap().into_iter().filter(|o| o.def.trigger.is_some()).collect();
        assert_eq!(procs.len(), 36);
        assert!(procs.iter().all(|o| matches!(o.def.source, EffectSource::Set { .. })));
        // a proc with numbers of its own is checked against its record; one that only applies has none to check
        assert!(procs.iter().all(|o| o.check.is_some() != o.def.channels.is_empty()));
        assert_eq!(procs.iter().filter(|o| !o.def.applies.is_empty()).count(), 8);
    }

    #[test]
    fn a_proc_can_apply_a_debuff_to_the_target_and_need_no_channels_of_its_own() {
        let text = |applies: &str| format!(
            "[[effect]]\nid = 34386\nname = \"Night Mother's Gaze (5 pieces)\"\nsource = {{ Set = {{ set_id = 51, pieces_required = 5 }} }}\n\
             channels = []\ntrigger = {{ event = \"DealCritical\" }}\napplies = {applies}\n"
        );
        let ok = parse(&text("[{ effect_id = 61743, recipient = \"Target\", duration_ms = 4000 }]")).unwrap();
        let a = ok[0].def.applies[0];
        assert_eq!((a.effect_id, a.recipient, a.duration_ms, a.stacks), (61743, Recipient::Target, Some(4000), 1));
        assert!(ok[0].def.channels.is_empty());
        assert!(parse(&text("[]")).unwrap_err().contains("applies nothing"));
        assert!(parse(&text("[{ effect_id = 34386, recipient = \"Target\" }]")).unwrap_err().contains("cannot apply itself"));
        assert!(parse(&text("[{ effect_id = 1, recipient = \"Target\", stacks = 0 }]")).unwrap_err().contains("at least 1 stack"));
        assert!(parse(&text("[{ effect_id = 1, recipient = \"Enemy\" }]")).is_err(), "unknown recipient");
    }

    #[test]
    fn synthetic_ids_are_refused() {
        let err = parse(&SAMPLE.replace("id = 142035", "id = 2147483650")).unwrap_err();
        assert!(err.contains("synthetic ids are not allowed"), "{err}");
    }
}
