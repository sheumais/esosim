//! Set table (manifest id 0x16, one record per set) -> `sets.jsonl` + `set_ids.rs`.
//!
//! Container (big-endian u32 unless noted):
//!   0  magic FA FA EB EB | 4 version (25) | 8 record count | 12 unknown (28) | 16 first record
//!
//! Record (`size + 32` bytes, back to back until EOF):
//!   +0  "####"  +4 1-based index  +8 size  +12 size  +16 unknown  +20 set id
//!   +24 running offset  +28 size  +32 payload (`size` bytes, begins with the set id again)
//!
//! Payload (only the part we need; the rest is skipped):
//!   0   u32 set id
//!   4   .. fixed header (name hash / text id, unknown)
//!   33  u8  bonus count
//!   34  `count` x 13-byte stat entries:
//!         u8  slot            (slot N -> `bonuses[N - 1]`, i.e. the N-item bonus)
//!         u32 stat type       (see `stat_info`)
//!         u32 raw             (integer value for flat stats such as Healing Taken)
//!         f32 coefficient     (little-endian; floor(86 * coef) = the value the game shows)
//!   ..  0-2 ability blocks of 10 bytes: u8 tag, u8 flag, u32 ability id, u32 second id
//!       (the last one is the set's 5-piece ability; sets with a single block have only that)
//!   ..  item-list header (u8, u8, u16 item count; the two u8 are not decoded), `count` x u32 item ids
//!       (the items that belong to the set; no item is in two sets), then a 38 byte tail (42 in newer
//!       sets; not decoded)
//!
//! Output (see `generate_with`): one JSON object per set, with the channels typed by the engine's own
//! `Channel`, so the engine loads it without any generated Rust that mentions engine types.

use esosim_types::damage::ResistableDamageType as Dmg;
use esosim_types::{AggKind as Agg, Channel, ResourceKind as Res};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet, HashSet};
use std::fmt::Write as _;

/// `floor(SCALE * coefficient)` is the value shown in game (1.5 -> 129, 14.025 -> 1206, ...).
/// The generated file spells the multiplication out, so this is only used for reporting/tests.
pub const SCALE: f64 = 86.0;

const MAGIC: [u8; 4] = [0xFA, 0xFA, 0xEB, 0xEB];
const RECORD_MARKER: [u8; 4] = *b"####";
const CONTAINER_HEADER_LEN: usize = 16;
const RECORD_HEADER_LEN: usize = 32;
const COUNT_OFFSET: usize = 33;
const ENTRIES_OFFSET: usize = 34;
const ENTRY_LEN: usize = 13;
const BLOCK_LEN: usize = 10;
/// Ability ids are well below this; the item list header that follows the blocks is not.
const ID_LIMIT: u32 = 0x20_0000;

/// Hand-maintained `perfected set id  regular set id` pairs (not in the set table).
const PERFECTED_PAIRS: &str = include_str!("sets_perfected.txt");
/// Hand-maintained `set id  CONST_NAME` pairs (roadmap 1.4). A name here beats the derived one.
const SET_ALIASES: &str = include_str!("set_aliases.txt");

// ---------------------------------------------------------------------------
// Parsing
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub struct RawBonus {
    pub slot: u8,
    pub stat: u32,
    pub raw: u32,
    pub coef: f32,
}

/// `[tag][flag][ability id][second id]`. Meaning of tag/flag/second id is not decoded.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct AbilityBlock {
    pub tag: u8,
    pub flag: u8,
    pub ability_id: u32,
    pub ability_id2: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SetRecord {
    pub id: u32,
    pub bonuses: Vec<RawBonus>,
    pub abilities: Vec<AbilityBlock>,
    /// Item ids of every piece of the set (roadmap 2.1e), in table order.
    pub items: Vec<u32>,
}

fn be32(data: &[u8], off: usize) -> Result<u32, String> {
    data.get(off..off + 4)
        .map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
        .ok_or_else(|| format!("read past end of file at offset {off}"))
}

pub fn parse(data: &[u8]) -> Result<Vec<SetRecord>, String> {
    if data.len() < CONTAINER_HEADER_LEN || data[..4] != MAGIC {
        return Err("not a set table: bad magic (expected FA FA EB EB)".into());
    }
    let declared = be32(data, 8)? as usize;
    let mut records = Vec::with_capacity(declared);
    let mut pos = CONTAINER_HEADER_LEN;

    while pos < data.len() {
        let at = |what: &str| format!("record {} at offset {pos}: {what}", records.len() + 1);
        if data.get(pos..pos + 4) != Some(&RECORD_MARKER[..]) {
            return Err(at("missing #### marker"));
        }
        let index = be32(data, pos + 4)? as usize;
        let size = be32(data, pos + 8)? as usize;
        if index != records.len() + 1 {
            return Err(at(&format!("index {index} out of sequence")));
        }
        if be32(data, pos + 12)? as usize != size || be32(data, pos + 28)? as usize != size {
            return Err(at("size fields disagree"));
        }
        let id = be32(data, pos + 20)?;
        let start = pos + RECORD_HEADER_LEN;
        let payload = data
            .get(start..start + size)
            .ok_or_else(|| at("payload runs past end of file"))?;
        if payload.len() < ENTRIES_OFFSET || be32(payload, 0)? != id {
            return Err(at("payload does not start with the set id"));
        }

        let count = payload[COUNT_OFFSET] as usize;
        let end = ENTRIES_OFFSET + count * ENTRY_LEN;
        if payload.len() < end {
            return Err(at(&format!("{count} bonus entries do not fit in the payload")));
        }
        let mut bonuses = Vec::with_capacity(count);
        for e in payload[ENTRIES_OFFSET..end].chunks_exact(ENTRY_LEN) {
            let slot = e[0];
            if slot == 0 {
                return Err(at("bonus entry with slot 0"));
            }
            bonuses.push(RawBonus {
                slot,
                stat: be32(e, 1)?,
                raw: be32(e, 5)?,
                coef: f32::from_le_bytes([e[9], e[10], e[11], e[12]]),
            });
        }

        // Ability blocks follow directly. They are told apart from the item-list header
        // (u8, u8 <= 3, u16 count, then item ids) by a small flag and ids below ID_LIMIT.
        let mut abilities = Vec::new();
        let mut q = end;
        while q + BLOCK_LEN <= payload.len() {
            let (tag, flag) = (payload[q], payload[q + 1]);
            let (id1, id2) = (be32(payload, q + 2)?, be32(payload, q + 6)?);
            if !((1..=15).contains(&tag) && flag <= 1 && id1 != 0 && id1 < ID_LIMIT && id2 < ID_LIMIT) {
                break;
            }
            abilities.push(AbilityBlock { tag, flag, ability_id: id1, ability_id2: id2 });
            q += BLOCK_LEN;
        }

        // Item-list header: u8, u8, u16 count, then the ids. Only the tail after them is skipped.
        let count_at = q + 2;
        let n = payload
            .get(count_at..count_at + 2)
            .map(|b| usize::from(u16::from_be_bytes([b[0], b[1]])))
            .ok_or_else(|| at("no item-list header after the ability blocks"))?;
        let items_end = count_at + 2 + n * 4;
        if payload.len() < items_end {
            return Err(at(&format!("{n} item ids do not fit in the payload")));
        }
        let items = payload[count_at + 2..items_end]
            .chunks_exact(4)
            .map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
            .collect();

        records.push(SetRecord { id, bonuses, abilities, items });
        pos = start + size;
    }

    if records.len() != declared {
        return Err(format!("header says {declared} records, found {}", records.len()));
    }
    Ok(records)
}

// ---------------------------------------------------------------------------
// Stat types -> sets.rs channels
// ---------------------------------------------------------------------------

/// Stat type -> channel. Derived by matching the table against sets.rs (every code below
/// agrees with the existing file on >= 17 occurrences).
/// Codes 13, 22, 25 and 35 occur 1-2 times (sets 544, 847) and are still unidentified.
fn channel_of(code: u32) -> Option<Channel> {
    use Channel as C;
    Some(match code {
        2 => C::Power,
        3 => C::Armour(Dmg::All),
        4 => C::Resource(Res::Magicka, Agg::Additive),
        7 => C::Resource(Res::Health, Agg::Additive),
        10 => C::HealingTaken,
        12 => C::HealingDone,
        17 => C::Recovery(Res::Magicka, Agg::Additive),
        18 => C::Recovery(Res::Stamina, Agg::Additive),
        19 => C::Recovery(Res::Health, Agg::Additive),
        24 => C::CriticalResistance,
        27 => C::Penetration(Dmg::All),
        28 => C::CriticalChance,
        29 => C::Resource(Res::Stamina, Agg::Additive),
        _ => return None,
    })
}

/// The in-game value: floor(86 * coef). Flat stats (Healing Taken/Done, one-off penetration)
/// keep their value in the integer field with a zero coefficient instead.
pub fn bonus_value(b: &RawBonus) -> u32 {
    if b.coef == 0.0 {
        b.raw
    } else {
        (SCALE * f64::from(b.coef)).floor().max(0.0) as u32
    }
}

/// The same value as an exact `f64`, as stored in `sets.jsonl`: `floor(86 * coef)` computed
/// from the `f32` widened to `f64` (so it equals what the old generated Rust expression
/// `(86.0 * <coef>_f32 as f64).floor()` folded to), or the integer field for flat stats.
pub fn bonus_value_f64(b: &RawBonus) -> f64 {
    if b.coef == 0.0 { f64::from(b.raw) } else { (SCALE * f64::from(b.coef)).floor() }
}

// ---------------------------------------------------------------------------
// Names (from the lang file, table LANG_SET_NAMES, keyed by set id)
// ---------------------------------------------------------------------------

/// `Archer's Mind` -> `ARCHERS_MIND`, `Stone-Talker's Oath` -> `STONE_TALKERS_OATH`.
pub fn const_name(display: &str) -> String {
    // The game appends grammar markers after '^' (e.g. "^F"); they are not part of the name.
    let display = display.split('^').next().unwrap_or("");
    let mut out = String::new();
    for c in display.chars() {
        if c == '\'' || c == '\u{2019}' {
            continue;
        }
        if c.is_ascii_alphanumeric() {
            out.push(c.to_ascii_uppercase());
        } else if !out.ends_with('_') {
            out.push('_');
        }
    }
    let out = out.trim_matches('_').to_string();
    if out.is_empty() {
        out
    } else if out.starts_with(|c: char| c.is_ascii_digit()) {
        format!("SET_{out}")
    } else {
        out
    }
}

struct Naming {
    /// id -> (const name, display name; empty when the lang file has none)
    by_id: BTreeMap<u32, (String, String)>,
    unnamed: Vec<u32>,
}

fn name_sets(records: &[SetRecord], lang: &BTreeMap<u32, String>, aliases: &BTreeMap<u32, String>) -> Naming {
    let mut by_id = BTreeMap::new();
    let mut unnamed = Vec::new();
    // Alias names are reserved up front so a derived name can never take one.
    let mut used: HashSet<String> = aliases.values().cloned().collect();
    for rec in records {
        let display = lang.get(&rec.id).map(|s| s.split('^').next().unwrap_or("").trim().to_string());
        let derived = display.as_deref().map(const_name).unwrap_or_default();
        if let Some(alias) = aliases.get(&rec.id) {
            if derived.is_empty() {
                unnamed.push(rec.id);
            }
            by_id.insert(rec.id, (alias.clone(), display.unwrap_or_default()));
            continue;
        }
        let mut name = derived;
        if name.is_empty() {
            unnamed.push(rec.id);
            name = format!("UNKNOWN_SET_{}", rec.id);
        } else if used.contains(&name) {
            name = format!("{name}_{}", rec.id);
        }
        used.insert(name.clone());
        by_id.insert(rec.id, (name, display.unwrap_or_default()));
    }
    Naming { by_id, unnamed }
}

/// `set id  CONST_NAME  # comment` per line. Rejects bad identifiers and duplicate ids/names.
pub fn parse_aliases(text: &str) -> Result<BTreeMap<u32, String>, String> {
    let mut out = BTreeMap::new();
    let mut names: HashSet<String> = HashSet::new();
    for (n, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut it = line.split_whitespace();
        let (Some(id), Some(name), None) = (it.next(), it.next(), it.next()) else {
            return Err(format!("set_aliases.txt line {}: expected `<set id> <CONST_NAME>`", n + 1));
        };
        let id: u32 = id.parse().map_err(|e| format!("set_aliases.txt line {}: bad set id: {e}", n + 1))?;
        let valid = !name.is_empty()
            && name.chars().all(|c| c.is_ascii_uppercase() || c.is_ascii_digit() || c == '_')
            && !name.starts_with(|c: char| c.is_ascii_digit());
        if !valid {
            return Err(format!("set_aliases.txt line {}: '{name}' is not an UPPER_SNAKE identifier", n + 1));
        }
        if out.insert(id, name.to_string()).is_some() {
            return Err(format!("set_aliases.txt line {}: duplicate set id {id}", n + 1));
        }
        if !names.insert(name.to_string()) {
            return Err(format!("set_aliases.txt line {}: duplicate name {name}", n + 1));
        }
    }
    Ok(out)
}

// ---------------------------------------------------------------------------
// Output
// ---------------------------------------------------------------------------

fn is_zero_u32(v: &u32) -> bool {
    *v == 0
}
fn is_zero_f32(v: &f32) -> bool {
    *v == 0.0
}

/// One bonus of a set. `channel` is what the engine reads (`None`: the stat type is not
/// mapped yet, so the engine ignores it); `stat`/`raw`/`coef` keep the source data, so an
/// unmapped stat is visible in the file instead of silently missing.
#[derive(Debug, Serialize)]
pub struct BonusOut {
    pub channel: Option<Channel>,
    pub stat: u32,
    #[serde(skip_serializing_if = "is_zero_u32")]
    pub raw: u32,
    #[serde(skip_serializing_if = "is_zero_f32")]
    pub coef: f32,
    pub value: f64,
}

/// One line of `sets.jsonl`. The engine's `Set` reads `id`, `name`, `perfected_of` and
/// `bonuses`; the rest is for people and tools.
#[derive(Debug, Serialize)]
pub struct SetOut {
    pub id: u32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// Rust constant name used in `set_ids.rs` (`<const_name>_SET_ID`).
    pub const_name: String,
    /// For a perfected set: the regular set it also counts as.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perfected_of: Option<u32>,
    /// `bonuses[n - 1]` is what the n-th item adds (empty rows pad up to the highest slot).
    pub bonuses: Vec<Vec<BonusOut>>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub abilities: Vec<AbilityBlock>,
    /// The last ability block: the set's 5-piece ability, for looking up its tooltip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub keystone: Option<u32>,
}

#[derive(Default)]
pub struct Report {
    /// Aliases whose set id is not in the table any more (the hand code naming it is stale).
    pub dangling_aliases: Vec<(u32, String)>,
    /// (set id, alias, name the lang file would give now) where the two differ: the alias saved you.
    pub alias_overrides: Vec<(u32, String, String)>,
    pub sets: usize,
    pub bonuses: usize,
    pub abilities: usize,
    /// (set id, slot, stat type, value) for stat types we have no channel for. They are
    /// still written (with `"channel": null`); this list is what to map next.
    pub unmapped: Vec<(u32, u8, u32, u32)>,
    /// Ids that had no name and were emitted as `UNKNOWN_SET_<id>`.
    pub unnamed: Vec<u32>,
    /// Perfected-set pairs that name a set missing from the table.
    pub dangling_perfected: Vec<(u32, u32)>,
}

/// `perfected id -> regular id` from `sets_perfected.txt` (`#` starts a comment).
pub fn parse_perfected(text: &str) -> Result<BTreeMap<u32, u32>, String> {
    let mut out = BTreeMap::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let mut it = line.split_whitespace().map(str::parse::<u32>);
        match (it.next(), it.next(), it.next()) {
            (Some(Ok(p)), Some(Ok(r)), None) => {
                if out.insert(p, r).is_some() {
                    return Err(format!("sets_perfected.txt line {}: duplicate perfected id {p}", i + 1));
                }
            }
            _ => return Err(format!("sets_perfected.txt line {}: expected `<perfected id> <regular id>`", i + 1)),
        }
    }
    // Only the perfected set counts as the regular one, never the other way round: a pair listed
    // in both directions would make regular pieces count towards the perfected set's bonuses.
    if let Some((p, r)) = out.iter().find(|(_, r)| out.contains_key(r)) {
        return Err(format!("sets_perfected.txt: set {r} is both a regular set (of {p}) and listed as perfected"));
    }
    Ok(out)
}

/// One row per bonus slot (`bonuses[slot - 1]`), padded with empty rows up to the highest slot.
fn rows_for(rec: &SetRecord, report: &mut Report) -> Vec<Vec<BonusOut>> {
    let max_slot = rec.bonuses.iter().map(|b| b.slot as usize).max().unwrap_or(0);
    let mut rows: Vec<Vec<BonusOut>> = (0..max_slot).map(|_| Vec::new()).collect();
    for b in &rec.bonuses {
        let channel = channel_of(b.stat);
        if channel.is_some() {
            report.bonuses += 1;
        } else {
            report.unmapped.push((rec.id, b.slot, b.stat, bonus_value(b)));
        }
        rows[b.slot as usize - 1].push(BonusOut {
            channel,
            stat: b.stat,
            raw: b.raw,
            coef: b.coef,
            value: bonus_value_f64(b),
        });
    }
    rows
}

/// `sets.jsonl` (one set per line, sorted by id, LF endings, so diffs between patches show
/// only real changes) and `set_ids.rs` (nothing but `pub const <NAME>_SET_ID: u16`).
pub fn generate(
    records: &[SetRecord],
    lang: &BTreeMap<u32, String>,
    perfected: &BTreeMap<u32, u32>,
) -> (String, String, Report) {
    generate_with(records, lang, perfected, &BTreeMap::new())
}

/// [`generate`] plus frozen constant names (`set_aliases.txt`, roadmap 1.4).
pub fn generate_with(
    records: &[SetRecord],
    lang: &BTreeMap<u32, String>,
    perfected: &BTreeMap<u32, u32>,
    aliases: &BTreeMap<u32, String>,
) -> (String, String, Report) {
    let naming = name_sets(records, lang, aliases);
    let mut report = Report { sets: records.len(), unnamed: naming.unnamed.clone(), ..Report::default() };
    for (id, alias) in aliases {
        if !naming.by_id.contains_key(id) {
            report.dangling_aliases.push((*id, alias.clone()));
        } else if let Some(display) = lang.get(id) {
            let derived = const_name(display.split('^').next().unwrap_or("").trim());
            if !derived.is_empty() && derived != *alias {
                report.alias_overrides.push((*id, alias.clone(), derived));
            }
        }
    }
    for (p, r) in perfected {
        if !naming.by_id.contains_key(p) || !naming.by_id.contains_key(r) {
            report.dangling_perfected.push((*p, *r));
        }
    }

    let mut sorted: Vec<&SetRecord> = records.iter().collect();
    sorted.sort_by_key(|r| r.id);

    let mut jsonl = String::with_capacity(records.len() * 200);
    let mut ids = String::from(
        "// Generated by `esosim-data-compiler` (set ids only; the bonuses are in data/bundle/sets.jsonl).\n// Do not edit by hand.\n\n",
    );
    for rec in sorted {
        let (const_name, display) = &naming.by_id[&rec.id];
        let out = SetOut {
            id: rec.id,
            name: display.clone(),
            const_name: const_name.clone(),
            perfected_of: perfected.get(&rec.id).copied(),
            bonuses: rows_for(rec, &mut report),
            abilities: rec.abilities.clone(),
            keystone: rec.abilities.last().map(|a| a.ability_id),
        };
        report.abilities += rec.abilities.len();
        jsonl.push_str(&serde_json::to_string(&out).expect("SetOut serialises"));
        jsonl.push('\n');
        let _ = writeln!(ids, "pub const {const_name}_SET_ID: u16 = {};", rec.id);
    }
    (jsonl, ids, report)
}

/// Every ability id the set table attaches to a set: all ability blocks, plus the keystone
/// (the last block, already part of `abilities` but added explicitly so the intent is visible).
/// `ability_id2` is deliberately left out: its meaning is not decoded (often equal to the
/// first id), and a root that is not understood would pull in an unexplained closure.
pub fn ability_roots(records: &[SetRecord]) -> BTreeSet<u32> {
    let mut ids = BTreeSet::new();
    for rec in records {
        ids.extend(rec.abilities.iter().map(|a| a.ability_id));
        if let Some(keystone) = rec.abilities.last() {
            ids.insert(keystone.ability_id);
        }
    }
    ids.remove(&0);
    ids
}

/// Everything the set table yields.
pub struct SetOutput {
    /// `sets.jsonl`
    pub jsonl: String,
    /// `set_ids.rs`
    pub ids_rs: String,
    /// Ability ids to use as extra roots for the ability bundle (see [`ability_roots`]).
    pub ability_roots: BTreeSet<u32>,
    /// `set_items.csv`: `set id,item id,item id,...` per set that has items (roadmap 2.1e).
    pub items_csv: String,
    /// The parsed table, for the behaviour classification (2.1b).
    pub records: Vec<SetRecord>,
    /// `sets_perfected.txt` (each pair listed in both directions).
    pub perfected: BTreeMap<u32, u32>,
}

/// `set id,item id,...` one line per set that has items, sets in table order.
pub fn items_csv(records: &[SetRecord]) -> String {
    let mut out = String::new();
    for r in records.iter().filter(|r| !r.items.is_empty()) {
        let _ = write!(out, "{}", r.id);
        for i in &r.items {
            let _ = write!(out, ",{i}");
        }
        out.push('\n');
    }
    out
}

/// Parse the set table once and produce the set files and the ability roots.
/// `names` is the lang table of set names, keyed by set id.
pub fn build(table: &[u8], names: &BTreeMap<u32, String>) -> Result<SetOutput, String> {
    let records = parse(table)?;
    if records.is_empty() {
        return Err("set table holds no sets".into());
    }
    let perfected = parse_perfected(PERFECTED_PAIRS)?;
    let aliases = parse_aliases(SET_ALIASES)?;
    let (jsonl, ids_rs, _report) = generate_with(&records, names, &perfected, &aliases);
    let ability_roots = ability_roots(&records);
    let items_csv = items_csv(&records);
    Ok(SetOutput { jsonl, ids_rs, ability_roots, items_csv, records, perfected })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(slot: u8, stat: u32, raw: u32, coef: f32) -> Vec<u8> {
        let mut e = vec![slot];
        e.extend(stat.to_be_bytes());
        e.extend(raw.to_be_bytes());
        e.extend(coef.to_le_bytes());
        e
    }

    fn block(tag: u8, flag: u8, id: u32, id2: u32) -> Vec<u8> {
        let mut e = vec![tag, flag];
        e.extend(id.to_be_bytes());
        e.extend(id2.to_be_bytes());
        e
    }

    /// Builds a container the way the game writes it. After the blocks comes an item-list
    /// header (tag 5, sub 2, 3 items) and junk standing in for items and tail.
    fn container(sets: &[(u32, Vec<Vec<u8>>, Vec<Vec<u8>>)]) -> Vec<u8> {
        let mut d = MAGIC.to_vec();
        d.extend(25u32.to_be_bytes());
        d.extend((sets.len() as u32).to_be_bytes());
        d.extend(28u32.to_be_bytes());
        for (i, (id, entries, blocks)) in sets.iter().enumerate() {
            let mut p = id.to_be_bytes().to_vec();
            p.resize(COUNT_OFFSET, 0);
            p.push(entries.len() as u8);
            for e in entries.iter().chain(blocks) {
                p.extend(e);
            }
            p.extend([5, 2, 0, 3]);
            p.extend([0, 1, 0x8c, 0x9d, 0, 1, 0x8c, 0x9f, 0, 1, 0x8c, 0x9a]);
            p.extend([0u8; 38]);
            let size = p.len() as u32;
            d.extend(RECORD_MARKER);
            for w in [i as u32 + 1, size, size, 0, *id, 0, size] {
                d.extend(w.to_be_bytes());
            }
            d.extend(p);
        }
        d
    }

    fn lang(pairs: &[(u32, &str)]) -> BTreeMap<u32, String> {
        pairs.iter().map(|(k, v)| (*k, v.to_string())).collect()
    }

    #[test]
    fn parses_entries_and_ability_blocks() {
        let d = container(&[
            (20, vec![entry(2, 18, 0, 1.5), entry(4, 7, 0, 14.025)], vec![block(1, 0, 19000, 0)]),
            (769, vec![], vec![block(2, 0, 147225, 0), block(3, 0, 220010, 0)]),
            (800, vec![], vec![]),
        ]);
        let r = parse(&d).unwrap();
        assert_eq!(r.len(), 3);
        assert_eq!(r[0].bonuses.len(), 2);
        assert_eq!(bonus_value(&r[0].bonuses[0]), 129);
        assert_eq!(bonus_value(&r[0].bonuses[1]), 1206);
        assert_eq!(r[0].abilities, vec![AbilityBlock { tag: 1, flag: 0, ability_id: 19000, ability_id2: 0 }]);
        // the item-list header (5, 2, 0x0003...) must not be mistaken for a block
        assert_eq!(r[1].abilities.len(), 2);
        assert_eq!(r[1].abilities[1].ability_id, 220010);
        assert!(r[2].abilities.is_empty());
        assert_eq!(r[1].items, vec![0x18c9d, 0x18c9f, 0x18c9a]);
        assert_eq!(items_csv(&r).lines().next().unwrap(), "20,101533,101535,101530");
    }

    #[test]
    fn ability_roots_include_every_block_and_skip_zero() {
        let rec = |id, ids: &[u32]| SetRecord {
            id,
            bonuses: vec![],
            items: vec![],
            abilities: ids
                .iter()
                .map(|&a| AbilityBlock { tag: 1, flag: 0, ability_id: a, ability_id2: 7 })
                .collect(),
        };
        let roots = ability_roots(&[rec(1, &[57162]), rec(2, &[100, 200]), rec(3, &[]), rec(4, &[57162])]);
        assert_eq!(roots.into_iter().collect::<Vec<_>>(), vec![100, 200, 57162]); // 7 (id2) is not a root
    }

    #[test]
    fn values_are_floor_of_86_times_the_coefficient() {
        let v = |coef: f32| bonus_value(&RawBonus { slot: 1, stat: 0, raw: 0, coef });
        assert_eq!(v(17.3), 1487); // floor, not round (1487.8)
        assert_eq!(v(12.75), 1096); // 1096.5
        assert_eq!(v(7.64), 657);
        let flat = RawBonus { slot: 3, stat: 10, raw: 4, coef: 0.0 };
        assert_eq!(bonus_value(&flat), 4);
        assert_eq!(bonus_value_f64(&flat), 4.0);
    }

    #[test]
    fn f64_value_equals_the_integer_value_for_every_coefficient() {
        for coef in [1.5_f32, 14.025, 12.75, 7.64, 17.3, 34.59, 171.0 / 86.0, 0.0833, 3.0] {
            let b = RawBonus { slot: 2, stat: 2, raw: 0, coef };
            assert_eq!(bonus_value_f64(&b), f64::from(bonus_value(&b)), "coef {coef}");
        }
    }

    #[test]
    fn rejects_corrupt_input() {
        let mut d = container(&[(20, vec![], vec![])]);
        assert!(parse(&d[1..]).is_err());
        d[CONTAINER_HEADER_LEN] = b'X';
        assert!(parse(&d).is_err());
        let mut d = container(&[(20, vec![entry(2, 2, 0, 1.5)], vec![])]);
        d.truncate(d.len() - 70);
        assert!(parse(&d).is_err());
    }

    #[test]
    fn const_names_follow_the_existing_convention() {
        assert_eq!(const_name("Archer's Mind"), "ARCHERS_MIND");
        assert_eq!(const_name("Stone-Talker\u{2019}s Oath"), "STONE_TALKERS_OATH");
        assert_eq!(const_name("Mad God's Dancing Shoes^n"), "MAD_GODS_DANCING_SHOES");
        assert_eq!(const_name("3 Queens"), "SET_3_QUEENS");
        assert_eq!(const_name("???"), "");
    }

    fn lines(text: &str) -> Vec<serde_json::Value> {
        text.lines().map(|l| serde_json::from_str(l).expect("each line is JSON")).collect()
    }

    #[test]
    fn writes_one_typed_json_line_per_set_sorted_by_id() {
        let d = container(&[
            (21, vec![entry(2, 17, 0, 1.5), entry(3, 10, 4, 0.0), entry(3, 2, 0, 3.0), entry(5, 13, 0, 34.59)], vec![block(1, 0, 19000, 0)]),
            (7, vec![], vec![]),
        ]);
        let recs = parse(&d).unwrap();
        let (jsonl, _, report) = generate(&recs, &lang(&[(21, "Akaviri Dragonguard")]), &BTreeMap::new());
        let v = lines(&jsonl);
        assert_eq!(v.iter().map(|s| s["id"].as_u64().unwrap()).collect::<Vec<_>>(), [7, 21]);
        assert!(jsonl.ends_with('\n') && !jsonl.contains('\r'));

        let s = &v[1];
        assert_eq!(s["name"], "Akaviri Dragonguard");
        assert_eq!(s["const_name"], "AKAVIRI_DRAGONGUARD");
        let rows = s["bonuses"].as_array().unwrap();
        assert_eq!(rows.len(), 5);
        assert_eq!(rows[0].as_array().unwrap().len(), 0);
        assert_eq!(rows[1][0]["channel"], serde_json::json!({"Recovery": ["Magicka", "Additive"]}));
        assert_eq!(rows[1][0]["value"], 129.0);
        assert_eq!(rows[2][0]["channel"], "HealingTaken");
        assert_eq!(rows[2][0]["value"], 4.0);
        assert_eq!(rows[2][1]["channel"], "Power");
        assert_eq!(rows[2][1]["value"], 258.0);
        assert_eq!(rows[3].as_array().unwrap().len(), 0);
        // an unmapped stat is kept, with a null channel and its source numbers
        assert!(rows[4][0]["channel"].is_null());
        assert_eq!(rows[4][0]["stat"], 13);
        assert_eq!(rows[4][0]["value"], 2974.0);
        assert_eq!(report.unmapped, vec![(21, 5, 13, 2974)]);
        assert_eq!(report.bonuses, 3);
        assert_eq!(s["keystone"], 19000);
        assert_eq!(s["abilities"][0]["ability_id"], 19000);
        assert!(v[0].get("abilities").is_none());
    }

    #[test]
    fn the_engine_loads_exactly_what_the_compiler_writes() {
        use esosim::data::tables::sets::SetTable;
        let d = container(&[
            (20, vec![entry(2, 18, 0, 1.5), entry(4, 7, 0, 14.025), entry(4, 13, 0, 2.0)], vec![]),
            (21, vec![entry(1, 28, 0, 7.64)], vec![]),
        ]);
        let recs = parse(&d).unwrap();
        let perfected = BTreeMap::from([(21, 20)]);
        let (jsonl, _, _) = generate(&recs, &lang(&[(20, "Regular"), (21, "Perfected Regular")]), &perfected);
        let table = SetTable::from_jsonl(&jsonl).expect("engine parses the compiler output");
        assert_eq!(table.len(), 2);
        let s = table.get(20).unwrap();
        assert_eq!(s.name, "Regular");
        assert_eq!(s.bonuses[1][0].channel, Some(Channel::Recovery(Res::Stamina, Agg::Additive)));
        assert_eq!(s.bonuses[1][0].value, 129.0);
        assert_eq!(s.bonuses[3][0].channel, Some(Channel::Resource(Res::Health, Agg::Additive)));
        assert_eq!(s.bonuses[3][0].value, 1206.0);
        assert_eq!(s.bonuses[3][1].channel, None);
        assert_eq!(table.unperfected_of(21), Some(20));
        assert_eq!(table.unperfected_of(20), None);
    }

    #[test]
    fn set_ids_file_is_constants_only() {
        let d = container(&[(2, vec![], vec![]), (1, vec![], vec![]), (3, vec![], vec![])]);
        let recs = parse(&d).unwrap();
        let (_, ids, _) = generate(&recs, &lang(&[(1, "Zed"), (2, "Alpha")]), &BTreeMap::new());
        let consts: Vec<&str> = ids.lines().filter(|l| !l.starts_with("//") && !l.is_empty()).collect();
        assert_eq!(
            consts,
            [
                "pub const ZED_SET_ID: u16 = 1;",
                "pub const ALPHA_SET_ID: u16 = 2;",
                "pub const UNKNOWN_SET_3_SET_ID: u16 = 3;",
            ]
        );
        assert!(!ids.contains("Channel") && !ids.contains("use "));
    }

    #[test]
    fn unnamed_and_duplicate_names_are_handled() {
        let d = container(&[(847, vec![], vec![]), (848, vec![entry(1, 28, 0, 7.64)], vec![]), (849, vec![], vec![])]);
        let recs = parse(&d).unwrap();
        let (jsonl, ids, report) = generate(&recs, &lang(&[(848, "Same"), (849, "Same")]), &BTreeMap::new());
        assert_eq!(report.unnamed, vec![847]);
        assert!(ids.contains("pub const UNKNOWN_SET_847_SET_ID: u16 = 847;"));
        assert!(ids.contains("pub const SAME_SET_ID: u16 = 848;") && ids.contains("pub const SAME_849_SET_ID: u16 = 849;"));
        assert_eq!(lines(&jsonl)[0]["const_name"], "UNKNOWN_SET_847");
        assert!(lines(&jsonl)[0].get("name").is_none());
    }

    #[test]
    fn perfected_pairs_parse_and_dangling_ones_are_reported() {
        let m = parse_perfected("# comment\n528 313 # Titanic Cleave\n\n357 363\n").unwrap();
        assert_eq!(m, BTreeMap::from([(528, 313), (357, 363)]));
        assert!(parse_perfected("1 2\n1 3\n").unwrap_err().contains("duplicate"));
        assert!(parse_perfected("1\n").unwrap_err().contains("line 1"));
        // a pair in both directions is rejected (roadmap 2.1d)
        assert!(parse_perfected("528 313\n313 528\n").unwrap_err().contains("both a regular set"));

        let d = container(&[(10, vec![], vec![])]);
        let recs = parse(&d).unwrap();
        let (_, _, report) = generate(&recs, &lang(&[]), &BTreeMap::from([(10, 99)]));
        assert_eq!(report.dangling_perfected, vec![(10, 99)]);
    }

    #[test]
    fn the_shipped_perfected_pairs_file_parses() {
        let m = parse_perfected(PERFECTED_PAIRS).unwrap();
        assert!(m.len() > 30);
        assert!(m.iter().all(|(p, r)| p != r));
    }

    #[test]
    fn aliases_parse_and_reject_bad_input() {
        let m = parse_aliases("# c\n518 ARKASIS_GENIUS  # old spelling\n\n7 FOO\n").unwrap();
        assert_eq!(m, BTreeMap::from([(518, "ARKASIS_GENIUS".to_string()), (7, "FOO".to_string())]));
        assert!(parse_aliases("1 A\n1 B\n").unwrap_err().contains("duplicate set id"));
        assert!(parse_aliases("1 A\n2 A\n").unwrap_err().contains("duplicate name"));
        assert!(parse_aliases("1 lower\n").unwrap_err().contains("UPPER_SNAKE"));
        assert!(parse_aliases("1\n").unwrap_err().contains("line 1"));
    }

    #[test]
    fn an_alias_beats_the_derived_name_and_reserves_it() {
        let d = container(&[(1, vec![], vec![]), (2, vec![], vec![]), (3, vec![], vec![])]);
        let recs = parse(&d).unwrap();
        // the lang file now spells set 1 "Arkasiss Genius"; hand code still says ARKASIS_GENIUS.
        // set 2 is named "Arkasis Genius" in the lang file: it must not steal the alias name.
        let lang = lang(&[(1, "Arkasiss Genius"), (2, "Arkasis Genius")]);
        let aliases = BTreeMap::from([(1, "ARKASIS_GENIUS".to_string()), (9, "GONE".to_string())]);
        let (jsonl, ids, report) = generate_with(&recs, &lang, &BTreeMap::new(), &aliases);
        assert!(ids.contains("pub const ARKASIS_GENIUS_SET_ID: u16 = 1;"));
        assert!(ids.contains("pub const ARKASIS_GENIUS_2_SET_ID: u16 = 2;"));
        assert!(!ids.contains("ARKASISS_GENIUS_SET_ID"));
        assert_eq!(lines(&jsonl)[0]["const_name"], "ARKASIS_GENIUS");
        assert_eq!(report.dangling_aliases, vec![(9, "GONE".to_string())]);
        assert_eq!(report.alias_overrides, vec![(1, "ARKASIS_GENIUS".to_string(), "ARKASISS_GENIUS".to_string())]);
    }

    #[test]
    fn build_returns_files_and_roots_from_one_parse() {
        let d = container(&[(20, vec![entry(2, 2, 0, 1.5)], vec![block(1, 0, 19000, 0)])]);
        let out = build(&d, &lang(&[(20, "Regular")])).unwrap();
        assert_eq!(lines(&out.jsonl)[0]["name"], "Regular");
        assert!(out.ids_rs.contains("pub const REGULAR_SET_ID: u16 = 20;"));
        assert_eq!(out.ability_roots, BTreeSet::from([19000]));
        assert!(build(&d[1..], &lang(&[])).is_err());
    }

    #[test]
    fn the_shipped_alias_file_parses() {
        let m = parse_aliases(SET_ALIASES).unwrap();
        assert!(m.len() > 40);
    }

    #[test]
    fn channel_codes_cover_what_the_old_generator_knew() {
        assert_eq!(channel_of(2), Some(Channel::Power));
        assert_eq!(channel_of(29), Some(Channel::Resource(Res::Stamina, Agg::Additive)));
        assert_eq!(channel_of(13), None);
    }
}