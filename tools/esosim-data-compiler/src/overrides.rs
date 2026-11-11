//! `overrides.toml`: hand-written `EffectDef`s that always beat auto-classification
//! (roadmap 2.0 item 2, decision D4).
//!
//! Everything the classifier cannot see (conditions, stacking, procs, durations) is written here
//! by a person. The file is the `EffectDef` of `esosim-types` as TOML plus a few fields that only
//! the compiler uses (`game_name`, `source_url`, `check`); see the header of `data/overrides.toml`.

use esosim_types::{ChannelValue, Condition, EffectDef, EffectSource, Stacking};
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

impl Override {
    /// Synthetic ids (no ability behind them) start here; they are not looked up in the bundle.
    pub const SYNTHETIC_ID_START: u32 = 0x8000_0000;

    pub fn is_synthetic(&self) -> bool {
        self.def.id >= Self::SYNTHETIC_ID_START
    }
}

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
        if r.channels.is_empty() {
            return Err(at("has no channels"));
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
    use esosim_types::{Channel, ResistableDamageType};

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
        assert!(parse(none).unwrap_err().contains("effect 6 (`Y`): has no channels"));
    }

    #[test]
    fn a_check_can_subtract_another_record() {
        let one = "[[effect]]\nid = 5\nname = \"X\"\nsource = \"Food\"\nchannels = [{ channel = \"CriticalChance\", value = 823 }]\ncheck = { field = \"value1\", minus = 150703 }\n";
        assert_eq!(parse(one).unwrap()[0].check.as_ref().unwrap().minus, Some(150703));
    }

    #[test]
    fn synthetic_ids_are_recognised() {
        let v = parse(&SAMPLE.replace("id = 142035", "id = 2147483650")).unwrap();
        assert!(v.iter().any(Override::is_synthetic));
        assert!(!v.iter().all(Override::is_synthetic));
    }
}
