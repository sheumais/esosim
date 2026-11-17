//! `suggest-effects <category>`: ready-to-paste `overrides.toml` skeletons (roadmap 2.0 item 3).
//!
//! The classifier covers only a small part of the abilities, but where it has an opinion it is a
//! good first draft. So it is used as a *drafting tool*: for every root of a category that has no
//! override yet, print a `[[effect]]` table prefilled with what the data says (channels from the
//! classifier, `value` from `value1`, a trigger hint from `list26`) and everything it does not
//! know as a `TODO`, for a person to review.
//!
//! A skeleton that has not been finished does not load: an unclassified ability gets an empty
//! `channels` list, which `overrides::parse` rejects.

use crate::resolved::{ClassStatus, ResolvedAbility, RootCategory};
use crate::triggers;
use esosim_types::{Channel, Condition, Trigger, TriggerEvent};
use std::collections::BTreeSet;

/// How much of a tooltip goes into the comment above a skeleton.
const TOOLTIP_CHARS: usize = 240;

/// `s` as a TOML basic string, escaped.
fn toml_string(s: &str) -> String {
    let mut out = String::from('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04X}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// A JSON value as an inline TOML value. Enough for the externally tagged enums of
/// `esosim-types` (strings, numbers, arrays, one-key objects).
fn json_to_toml(v: &serde_json::Value) -> String {
    use serde_json::Value;
    match v {
        Value::Null => "\"\"".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => n.to_string(),
        Value::String(s) => toml_string(s),
        Value::Array(items) => {
            let parts: Vec<String> = items.iter().map(json_to_toml).collect();
            format!("[{}]", parts.join(", "))
        }
        Value::Object(map) if map.is_empty() => "{}".to_string(),
        Value::Object(map) => {
            let parts: Vec<String> = map
                .iter()
                .map(|(k, v)| {
                    let bare = !k.is_empty() && k.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-');
                    let key = if bare { k.clone() } else { toml_string(k) };
                    format!("{key} = {}", json_to_toml(v))
                })
                .collect();
            format!("{{ {} }}", parts.join(", "))
        }
    }
}

/// `channel` as an inline TOML value (`"CriticalChance"`, `{ Armour = "All" }`).
fn toml_channel(channel: &Channel) -> String {
    match serde_json::to_value(channel) {
        Ok(v) => json_to_toml(&v),
        Err(_) => format!("\"TODO\" # could not write {channel:?} as TOML"),
    }
}

/// What the data says about *when* an ability's effect applies, for the `condition = ` and `trigger = ` lines.
#[derive(Debug, Default, PartialEq)]
pub struct Timing {
    /// The condition every clause of the classification agrees on; `Always` when there is none or they disagree.
    pub condition: Condition,
    /// Conditions of the other clauses when they disagree (shown as comments; a person picks).
    pub other_conditions: Vec<Condition>,
    /// The proc's event: the first event a clause's `On(..)` cue names, else the first kind of the ability's
    /// `list26` that is a combat event.
    pub event: Option<TriggerEvent>,
    /// Whether a clause of the tooltip names `event` (written as `trigger = ...`), or only the game's trigger list
    /// does (written as a comment: a buff's record can carry kinds that are not its own proc).
    pub event_in_text: bool,
    /// Further events (other clauses, other `list26` kinds), as comments.
    pub other_events: Vec<TriggerEvent>,
}

/// Read `condition` and the proc event out of an ability's evidence and `list26` kinds.
pub fn timing(a: &ResolvedAbility) -> Timing {
    let mut t = Timing::default();
    let evidence: &[crate::resolved::ResolvedEvidence] = match &a.classification {
        Some(c) if c.status == ClassStatus::Classified => &c.evidence,
        _ => &[],
    };
    // conditions: the clauses that carry one; all alike -> the condition, else the first and the rest as candidates
    let mut conditions: Vec<Condition> = Vec::new();
    for e in evidence {
        if !conditions.contains(&e.condition) {
            conditions.push(e.condition.clone());
        }
    }
    // a clause with no circumstance next to one with a circumstance is a disagreement too: keep `Always` out of the
    // pick when anything else is there, and say so through `other_conditions`
    let mut specific = conditions.iter().filter(|c| **c != Condition::Always);
    if let Some(first) = specific.next() {
        t.condition = first.clone();
        t.other_conditions = specific.cloned().collect();
        if conditions.contains(&Condition::Always) {
            // some clause has no circumstance: the person decides whether the first one holds for all of it
            t.other_conditions.push(Condition::Always);
        }
    }
    // events: clause events first, then the game's own trigger kinds
    let mut events: Vec<TriggerEvent> = Vec::new();
    for ev in evidence.iter().flat_map(|e| e.events.iter()).copied() {
        if !events.contains(&ev) {
            events.push(ev);
        }
    }
    t.event_in_text = !events.is_empty();
    for tr in &a.triggers {
        if let Some(ev) = tr.event {
            if !events.contains(&ev) {
                events.push(ev);
            }
        }
    }
    let mut it = events.into_iter();
    t.event = it.next();
    t.other_events = it.collect();
    t
}

/// `value` as TOML through its serde form (`"InCombat"`, `{ SelfHealthBelowPct = 50 }`).
fn to_toml<T: serde::Serialize>(value: &T) -> String {
    match serde_json::to_value(value) {
        Ok(v) => json_to_toml(&v),
        Err(_) => "\"TODO\"".to_string(),
    }
}

/// The `source = ...` line a skeleton of `category` starts from. What the table cannot know is
/// left as `TODO` / 0 and called out in a comment.
fn source_line(category: RootCategory, id: u32) -> String {
    match category {
        RootCategory::Buffs => format!("source = {{ Buff = {{ ability_id = {id} }} }}"),
        RootCategory::Mundus => format!("source = {{ Mundus = {{ ability_id = {id} }} }}"),
        RootCategory::Cp => format!("# champion points are written in cp_effects.toml (by star id), not here; ability {id}"),
        RootCategory::Skills => format!("source = {{ ActiveSkill = {{ ability_id = {id} }} }} # or Passive"),
        RootCategory::Racial => format!("source = {{ Race = {{ name = \"TODO\", ability_id = {id} }} }}"),
        RootCategory::Consumables => "source = \"Food\" # or \"Potion\"".to_string(),
        RootCategory::Sets => "source = { Set = { set_id = 0, pieces_required = 0 } } # TODO: set id and piece count".to_string(),
        RootCategory::Class => format!("source = {{ Class = {{ name = \"TODO\", ability_id = {id} }} }}"),
        RootCategory::Armour | RootCategory::Weapon => format!("source = {{ Passive = {{ ability_id = {id} }} }}"),
        // like every passive: on only when the build lists the ability; a second number of the same passive keeps this ability id
        RootCategory::World => format!("source = {{ Passive = {{ ability_id = {id} }} }}"),
        _ => format!(""),
    }
}

/// One skeleton.
pub fn skeleton(a: &ResolvedAbility, category: RootCategory) -> String {
    let mut out = String::new();

    // --- what the data says, as comments
    out.push_str(&format!("# {} ({})\n", a.name, a.id));
    if let Some(class) = &a.classification {
        let status = match class.status {
            ClassStatus::Classified => "classified",
            ClassStatus::Conflict => "conflict between tooltips",
            ClassStatus::Unclassified => "unclassified",
        };
        out.push_str(&format!("# classifier: {status}"));
        if let Some(reason) = &class.reason {
            out.push_str(&format!(" ({reason})"));
        }
        out.push('\n');
        for clause in class.unmatched_clauses.iter().take(3) {
            out.push_str(&format!("#   unmatched clause: {clause}\n"));
        }
    } else {
        out.push_str("# classifier: no opinion (the ability carries no value)\n");
    }
    if let Some(text) = &a.tooltip_text {
        let shown: String = text.chars().take(TOOLTIP_CHARS).collect();
        out.push_str(&format!("# tooltip: {shown}\n"));
    }
    out.push_str(&format!(
        "# record: value0 = {}, value1 = {}, value2 = {}, duration_ms = {}, {} coefficient row(s)\n",
        a.value0,
        a.value1,
        a.value2,
        a.duration_ms,
        a.coef.len()
    ));
    for t in &a.triggers {
        let kind = t.u2.first().copied().unwrap_or(0);
        if kind != 0 {
            out.push_str(&format!(
                "# trigger hint: list26 kind {kind}: a combat trigger, so it needs a `condition`, `duration_ms` and `cooldown_ms`\n"
            ));
        }
    }

    // what the tooltip says about *when* (see `triggers.rs`); only ever comments
    if let Some(text) = &a.tooltip_text {
        let cues = triggers::cues_in(text);
        if !cues.is_empty() {
            out.push_str(&format!("# condition cues: {}\n", triggers::summary(&cues)));
            let candidates: Vec<String> =
                cues.iter().filter_map(|m| m.cue.to_condition()).map(|c| format!("{c:?}")).collect();
            if !candidates.is_empty() {
                out.push_str(&format!("# condition candidates (build a `condition` from these): {}\n", candidates.join(", ")));
            }
        }
    }

    // --- the table
    out.push_str("[[effect]]\n");
    out.push_str(&format!("id = {}\n", a.id));
    out.push_str(&format!("name = {}\n", toml_string(&a.name)));
    out.push_str(&source_line(category, a.id));
    out.push('\n');

    let channels: Vec<Channel> = match &a.classification {
        Some(c) if c.status == ClassStatus::Classified => c.channels.clone(),
        _ => Vec::new(),
    };
    if channels.is_empty() {
        out.push_str("channels = []  # TODO: e.g. [{ channel = \"CriticalDamage\", value = 0.0 }]\n");
    } else {
        out.push_str("channels = [\n");
        for ch in &channels {
            // `value1` is the number as the game stores it; check the unit (rating vs percent) by hand
            out.push_str(&format!("    {{ channel = {}, value = {:?} }},\n", toml_channel(ch), a.value1 as f64));
        }
        out.push_str("]\n");
    }
    out.push_str("stacking = \"Unique\"\n");
    if a.duration_ms != 0 {
        out.push_str(&format!("# duration_ms = {}\n", a.duration_ms));
    }
    let timing = timing(a);
    if timing.condition == Condition::Always {
        out.push_str("# condition = \"Always\"\n");
    } else {
        out.push_str(&format!("condition = {}  # from the tooltip's clause; check it\n", to_toml(&timing.condition)));
    }
    for c in &timing.other_conditions {
        out.push_str(&format!("# other clause: condition = {}\n", to_toml(c)));
    }
    if let Some(event) = timing.event {
        let chance = a.tooltip_text.as_deref().is_some_and(|t| t.to_lowercase().contains("chance"));
        let (lead, why) = if timing.event_in_text {
            ("", "a proc: no sheet counts it, the calculator lists it as conditional")
        } else {
            ("# ", "from the ability's trigger list only: uncomment if this ability is the proc")
        };
        out.push_str(&format!(
            "{lead}trigger = {}  # {why}{}\n",
            to_toml(&Trigger::on(event)),
            if chance { "; the tooltip names a chance: add chance_pct" } else { "" }
        ));
        if a.duration_ms == 0 && timing.event_in_text {
            out.push_str("# an instant effect (a heal, a restore) has no duration_ms\n");
        }
    }
    for ev in &timing.other_events {
        out.push_str(&format!("# other event: trigger = {}\n", to_toml(&Trigger::on(*ev))));
    }
    if timing.event.is_some() {
        out.push_str("# cooldown_ms = 0  # the internal cooldown, when the tooltip names one\n");
    }
    out.push_str("# check = { field = \"value1\", scale = 1.0 }  # have `verify` hold the value to the record\n");
    out.push('\n');
    out
}

/// Skeletons for every root of `category` (`root_ids`) that is in `abilities` and not in
/// `reviewed` (ids that already have an override or are hand-written), sorted by id.
/// Returns the text and how many skeletons it holds.
pub fn suggest(
    abilities: &[ResolvedAbility],
    category: RootCategory,
    root_ids: &BTreeSet<u32>,
    reviewed: &BTreeSet<u32>,
) -> (String, usize) {
    let mut picked: Vec<&ResolvedAbility> = abilities
        .iter()
        .filter(|a| root_ids.contains(&a.id) && !reviewed.contains(&a.id))
        .collect();
    picked.sort_by_key(|a| a.id);

    let mut out = format!(
        "# suggest-effects {}: {} unreviewed of {} root(s). Review each, finish the TODOs, paste into overrides.toml.\n\n",
        category.name(),
        picked.len(),
        root_ids.len()
    );
    for a in &picked {
        out.push_str(&skeleton(a, category));
    }
    (out, picked.len())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::overrides;

    fn ability(json: &str) -> ResolvedAbility {
        serde_json::from_str(json).unwrap()
    }

    const CLASSIFIED: &str = r#"{"id":76617,"name":"Minor Slayer","value1":5,"duration_ms":4000,"tooltip_text":"Increases damage done to Dungeon, Trial and Arena monsters by <<1>>%.","classification":{"status":"classified","channels":[{"DamageDone":["Monsters", "Multiplicative"]}],"direction":"raises","evidence":[{"parent":76617,"placeholder":1,"clause":"increases damage done to dungeon, trial and arena monsters by @%","rule":"damage_done_monsters","channels":[{"DamageDone":["Monsters","Multiplicative"]}],"direction":"raises","cues":[{"cue":"Against(Monsters)","rule":"against_monsters","text":"done to dungeon, trial and arena monsters","condition":{"TargetIs":"Monster"}}],"condition":{"TargetIs":"Monster"}}]}}"#;

    const UNCLASSIFIED: &str = r#"{"id":50,"name":"Mystery \"X\"","triggers":[{"u2":[78,0,0,0],"event":"Bash"},{"u2":[0,0,0,0]}],"classification":{"status":"unclassified","reason":"no_rule_matched","unmatched_clauses":["does a thing"]}}"#;

    #[test]
    fn a_classified_ability_gives_a_skeleton_that_loads() {
        let text = skeleton(&ability(CLASSIFIED), RootCategory::Buffs);
        assert!(text.contains("# classifier: classified\n"), "{text}");
        assert!(text.contains("# tooltip: Increases damage"), "{text}");
        assert!(text.contains("DamageDone"), "{text}");
        assert!(text.contains("value = 5.0"), "{text}");
        assert!(text.contains("# duration_ms = 4000"), "{text}");

        // pasted as it is, the table is a valid override
        let parsed = overrides::parse(&text).unwrap();
        assert_eq!(parsed.len(), 1);
        assert_eq!(parsed[0].def.id, 76617);
        assert_eq!(parsed[0].def.name, "Minor Slayer");
        assert_eq!(parsed[0].def.channels.len(), 1);
        assert_eq!(parsed[0].def.channels[0].value, 5.0);
        // the evidence's condition is written, not left as a comment
        assert_eq!(parsed[0].def.condition, Condition::TargetIs(esosim_types::TargetKind::Monster));
        assert!(parsed[0].def.trigger.is_none());
    }

    /// A clause that names an event gives a `trigger` line that loads; an event only in the game's trigger list is
    /// offered as a comment.
    #[test]
    fn a_clause_event_becomes_a_trigger_and_a_list26_kind_only_a_comment() {
        let with_event = CLASSIFIED
            .replace(r#""condition":{"TargetIs":"Monster"}}]}}"#, r#""condition":"InCombat","events":["DrinkPotion"]}]}}"#);
        let text = skeleton(&ability(&with_event), RootCategory::Sets);
        assert!(text.contains("\ncondition = \"InCombat\""), "{text}");
        assert!(text.contains("\ntrigger = { event = \"DrinkPotion\" }"), "{text}");
        let parsed = overrides::parse(&text).unwrap();
        assert_eq!(parsed[0].def.condition, Condition::InCombat);
        assert_eq!(parsed[0].def.trigger, Some(Trigger::on(TriggerEvent::DrinkPotion)));

        // list26 only (the unclassified sample's kind 78 = Bash): a comment, so the skeleton is not made a proc
        let text = skeleton(&ability(UNCLASSIFIED), RootCategory::Sets);
        assert!(text.contains("\n# trigger = { event = \"Bash\" }"), "{text}");
        assert!(!text.contains("\ntrigger = "), "{text}");
    }

    #[test]
    fn disagreeing_clauses_pick_the_first_specific_condition_and_list_the_rest() {
        let a = ability(CLASSIFIED);
        assert_eq!(timing(&a).condition, Condition::TargetIs(esosim_types::TargetKind::Monster));
        assert!(timing(&a).event.is_none());
        let none = ability(UNCLASSIFIED);
        let t = timing(&none);
        assert_eq!((t.condition, t.event, t.event_in_text), (Condition::Always, Some(TriggerEvent::Bash), false));
    }

    #[test]
    fn an_unclassified_ability_gives_a_skeleton_that_must_be_finished() {
        let text = skeleton(&ability(UNCLASSIFIED), RootCategory::Sets);
        assert!(text.contains("# classifier: unclassified (no_rule_matched)"), "{text}");
        assert!(text.contains("unmatched clause: does a thing"), "{text}");
        assert!(text.contains("trigger hint: list26 kind 78"), "{text}");
        assert_eq!(text.matches("trigger hint").count(), 1, "kind 0 is the always-on marker, not a trigger");
        assert!(text.contains("name = \"Mystery \\\"X\\\"\""), "the name is escaped: {text}");
        assert!(text.contains("TODO"), "{text}");
        // unfinished skeletons do not load
        assert!(overrides::parse(&text).unwrap_err().contains("has no channels"));
    }

    #[test]
    fn the_tooltip_says_when_as_comments_and_the_skeleton_still_loads() {
        let json = r#"{"id":61,"name":"Proc","tooltip_text":"When you deal critical damage while in combat, gain <<1>>. Only while you are under 50% health.","classification":{"status":"unclassified","reason":"no_rule_matched"}}"#;
        let text = skeleton(&ability(json), RootCategory::Sets);
        assert!(text.contains("# condition cues: On(DealCritical), While(InCombat), While(ResourceBelow(Health, Some(50)))"), "{text}");
        assert!(text.contains("# condition candidates (build a `condition` from these): InCombat, SelfHealthBelowPct(50)"), "{text}");
        // comments only: finishing the TODOs is still a person's job
        assert!(overrides::parse(&text).unwrap_err().contains("has no channels"));
        // "against monsters" is a condition now: TargetIs(Monster)
        let slayer = skeleton(&ability(CLASSIFIED), RootCategory::Buffs);
        assert!(slayer.contains("# condition candidates (build a `condition` from these): TargetIs(Monster)"), "{slayer}");
        // a tooltip with no circumstance adds no line
        let plain = r#"{"id":62,"name":"Plain","tooltip_text":"Increases your maximum health by <<1>>.","classification":{"status":"unclassified","reason":"no_rule_matched"}}"#;
        assert!(!skeleton(&ability(plain), RootCategory::Buffs).contains("condition candidates"));
    }

    #[test]
    fn only_unreviewed_roots_are_suggested_in_id_order() {
        let abilities = vec![ability(CLASSIFIED), ability(UNCLASSIFIED), ability(r#"{"id":7,"name":"Not a root"}"#)];
        let roots = BTreeSet::from([50, 76617]);

        let (text, n) = suggest(&abilities, RootCategory::Buffs, &roots, &BTreeSet::new());
        assert_eq!(n, 2);
        assert!(text.starts_with("# suggest-effects buffs: 2 unreviewed of 2 root(s)."), "{text}");
        assert!(text.find("id = 50").unwrap() < text.find("id = 76617").unwrap());
        assert!(!text.contains("Not a root"));

        let (text, n) = suggest(&abilities, RootCategory::Buffs, &roots, &BTreeSet::from([50]));
        assert_eq!(n, 1);
        assert!(!text.contains("id = 50\n"), "{text}");
    }

    #[test]
    fn the_source_follows_the_category() {
        let a = ability(CLASSIFIED);
        assert!(skeleton(&a, RootCategory::Mundus).contains("Mundus = { ability_id = 76617 }"));
        assert!(skeleton(&a, RootCategory::Cp).contains("cp_effects.toml"));
        assert!(skeleton(&a, RootCategory::Racial).contains("Race = { name = \"TODO\", ability_id ="));
    }
}
