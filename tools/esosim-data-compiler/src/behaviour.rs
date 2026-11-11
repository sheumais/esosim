//! Sort every set into a behaviour class (roadmap 2.1b) -> `set_behaviour.jsonl`.
//!
//! A set's stat rows are already data. What is left is its *ability blocks* (the proc / buff /
//! rule on top). For each block the ability graph answers which kind it is:
//!
//! | the block's ability ...                                        | class               |
//! |----------------------------------------------------------------|---------------------|
//! | is not in the bundle                                           | `Unclassified`      |
//! | has a combat trigger (a `list26` entry whose `u2[0]` is not 0) | `CombatTriggered`   |
//! | is a Major/Minor buff, or points at one, and has no trigger    | `GrantsBuff`        |
//! | anything else                                                  | `StaticConditional` |
//!
//! `u2[0] == 0` is the always-on passive marker (Velothi Ur-Mage's Amulet, Barkskin, ...), not a
//! trigger, so those blocks fall through to the last two rows.
//!
//! A set's class is the largest class among its blocks (`StatOnly` with no blocks), so one set can
//! be `CombatTriggered` and still carry an automatic `GrantsBuff` block: `Automated Defense` gives
//! Minor Aegis at 3 pieces *and* has a proc. The per-block list is what generated grant rules
//! (2.1c) read. The functions work on plain inputs ([`SetInput`], `&[ResolvedAbility]`) so the
//! same code runs on the live game tables and on the shipped `sets.jsonl` + `bundle.jsonl`
//! (see the golden test at the bottom).

use crate::resolved::{EdgeKind, ResolvedAbility};
use crate::setdata::SetRecord;
use esosim_types::{BehaviourClass, BlockBehaviour, SetBehaviour, SetKind};
use std::collections::{BTreeMap, BTreeSet, HashMap};

/// What the classifier needs to know about one set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SetInput {
    pub id: u32,
    pub name: String,
    /// Slots (1..=5) that have at least one stat row.
    pub filled_slots: BTreeSet<u8>,
    /// Ability id of each ability block, in table order.
    pub blocks: Vec<u32>,
    /// For a perfected set: the regular set it is the perfected version of (`perfected_of` in
    /// `sets.jsonl`; `sets_perfected.txt` lists each pair once, perfected first). `None` on a
    /// regular set, whether or not a perfected set points at it.
    pub partner: Option<u32>,
    /// From `set_kinds.txt`: a kind the shape of the table row cannot tell. Beats [`kind_of`].
    pub kind_override: Option<SetKind>,
    /// The set table marks it craftable (`craftable` in `sets.jsonl`): [`SetKind::Crafted`].
    pub craftable: bool,
    /// The class a class set belongs to (`class_id` in `sets.jsonl`): [`SetKind::Class`].
    pub class_id: Option<u8>,
}

/// `set_kinds.txt`: the sets whose [`SetKind`] is listed by hand rather than inferred.
pub const KIND_OVERRIDES: &str = include_str!("set_kinds.txt");

/// `set id -> kind` from `set_kinds.txt` (`#` starts a comment, blank lines are skipped).
pub fn parse_kind_overrides(text: &str) -> Result<BTreeMap<u32, SetKind>, String> {
    let mut out = BTreeMap::new();
    for (i, line) in text.lines().enumerate() {
        let line = line.split('#').next().unwrap_or("").trim();
        if line.is_empty() {
            continue;
        }
        let at = |msg: String| format!("set_kinds.txt line {}: {msg}", i + 1);
        let mut parts = line.split_whitespace();
        let (Some(id), Some(kind), None) = (parts.next(), parts.next(), parts.next()) else {
            return Err(at("expected `<set id> <kind>`".into()));
        };
        let id: u32 = id.parse().map_err(|_| at(format!("expected `<set id> <kind>`, `{id}` is not a set id")))?;
        let kind: SetKind =
            serde_json::from_str(&format!("\"{kind}\"")).map_err(|_| at(format!("unknown kind `{kind}`")))?;
        if out.insert(id, kind).is_some() {
            return Err(at(format!("set {id} is listed twice")));
        }
    }
    Ok(out)
}

/// Build the inputs from the parsed set table. `names` are the lang names keyed by set id (the
/// game appends grammar markers after `^`; they are cut off, as in `sets.jsonl`).
pub fn inputs_from_records(
    records: &[SetRecord],
    names: &BTreeMap<u32, String>,
    perfected: &BTreeMap<u32, u32>,
    kind_overrides: &BTreeMap<u32, SetKind>,
) -> Vec<SetInput> {
    records
        .iter()
        .map(|r| SetInput {
            id: r.id,
            name: names
                .get(&r.id)
                .map(|n| n.split('^').next().unwrap_or("").trim().to_string())
                .unwrap_or_default(),
            filled_slots: r.bonuses.iter().map(|b| b.slot).collect(),
            blocks: r.abilities.iter().map(|a| a.ability_id).collect(),
            partner: perfected.get(&r.id).copied(),
            kind_override: kind_overrides.get(&r.id).copied(),
            craftable: r.craftable(),
            class_id: r.class_id,
        })
        .collect()
}

/// What the run found, for the compiler's summary line and for review.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// Sets per class.
    pub by_class: BTreeMap<BehaviourClass, usize>,
    /// `(set id, ability id)` of blocks whose ability is not in the bundle.
    pub unclassified: Vec<(u32, u32)>,
    /// Sets whose piece counts could not be inferred for a `GrantsBuff` block (no stat rows).
    pub grants_without_pieces: Vec<u32>,
    /// Sets with at least one `GrantsBuff` block of their own, whatever their overall class.
    pub sets_with_grant_blocks: usize,
    /// Sets per [`SetKind`] (a perfected set counts under its regular set's kind).
    pub by_kind: BTreeMap<SetKind, usize>,
    /// Ids listed in `set_kinds.txt` that are not in the set table (a patch removed them): a
    /// report line, not an error.
    pub dangling_kind_overrides: Vec<u32>,
}

impl Report {
    pub fn summary(&self) -> String {
        let n = |c| self.by_class.get(&c).copied().unwrap_or(0);
        let mut s = format!(
            "set behaviour: {} stat_only, {} grants_buff, {} static_conditional, {} combat_triggered, {} unclassified",
            n(BehaviourClass::StatOnly),
            n(BehaviourClass::GrantsBuff),
            n(BehaviourClass::StaticConditional),
            n(BehaviourClass::CombatTriggered),
            n(BehaviourClass::Unclassified),
        );
        s.push_str(&format!(" ({} sets have an automatic buff-grant block)", self.sets_with_grant_blocks));
        let k = |k| self.by_kind.get(&k).copied().unwrap_or(0);
        s.push_str(&format!(
            "\n  set kinds: {} monster, {} mythic, {} arena weapon (perfected included), {} crafted, {} class, {} other",
            k(SetKind::Monster),
            k(SetKind::Mythic),
            k(SetKind::ArenaWeapon),
            k(SetKind::Crafted),
            k(SetKind::Class),
            k(SetKind::Regular),
        ));
        for (set, ability) in &self.unclassified {
            s.push_str(&format!("\n  unclassified: set {set} block ability {ability} is not in the bundle"));
        }
        if !self.dangling_kind_overrides.is_empty() {
            s.push_str(&format!(
                "\n  set_kinds.txt lists {} set(s) that are not in the set table: {:?}",
                self.dangling_kind_overrides.len(),
                self.dangling_kind_overrides
            ));
        }
        if !self.grants_without_pieces.is_empty() {
            s.push_str(&format!(
                "\n  {} set(s) grant a buff but have no stat rows to infer the piece count from: {:?}",
                self.grants_without_pieces.len(),
                self.grants_without_pieces
            ));
        }
        s
    }
}

/// Edges that mean "this ability puts that one on you" (not "removes it", "is caused by it",
/// "starts its cooldown", "is a morph of it").
fn is_grant_edge(kind: EdgeKind) -> bool {
    !matches!(
        kind,
        EdgeKind::RemovesBuff | EdgeKind::CausedBy | EdgeKind::SetsCooldown | EdgeKind::BaseAbility | EdgeKind::Replaces
    )
}

/// The Major/Minor buff abilities `ability` gives: if it is itself a buff, the target of its edges
/// that carries the same buff code (Minor Slayer's grant `147226` -> the buff `76617`), else
/// itself; if it is not, every buff its edges point at. Sorted, de-duplicated.
fn buffs_of(ability: &ResolvedAbility, index: &HashMap<u32, &ResolvedAbility>) -> Vec<u32> {
    let code = ability.major_minor_id;
    let mut buffs: Vec<u32> = ability
        .edges
        .iter()
        .filter(|e| is_grant_edge(e.kind))
        .filter(|e| {
            index
                .get(&e.to)
                .is_some_and(|t| t.major_minor_id != 0 && (code == 0 || t.major_minor_id == code))
        })
        .map(|e| e.to)
        .collect();
    buffs.sort_unstable();
    buffs.dedup();
    if buffs.is_empty() && code != 0 {
        buffs.push(ability.id);
    }
    // The same buff often exists twice (the generic record and a set-specific copy, e.g. Minor
    // Force 61746 and 193448): keep the lowest id per buff code so it is granted once.
    let mut seen = BTreeSet::new();
    buffs.retain(|id| seen.insert(index.get(id).map_or(*id, |b| u32::from(b.major_minor_id))));
    buffs
}

/// A classified block before piece counts are known.
struct Classified {
    ability_id: u32,
    class: BehaviourClass,
    buffs: Vec<u32>,
    trigger: Option<u32>,
    /// The ability has no `list26` entries at all (not even the always-on marker).
    plain: bool,
}

fn classify_block(ability_id: u32, index: &HashMap<u32, &ResolvedAbility>) -> Classified {
    let Some(ability) = index.get(&ability_id) else {
        return Classified { ability_id, class: BehaviourClass::Unclassified, buffs: Vec::new(), trigger: None, plain: false };
    };
    let buffs = buffs_of(ability, index);
    let plain = ability.triggers.is_empty();
    let trigger = ability.triggers.iter().filter_map(|t| t.u2.first().copied()).find(|&k| k != 0);
    if trigger.is_some() {
        return Classified { ability_id, class: BehaviourClass::CombatTriggered, buffs, trigger, plain };
    }
    if !buffs.is_empty() {
        return Classified { ability_id, class: BehaviourClass::GrantsBuff, buffs, trigger: None, plain };
    }
    Classified { ability_id, class: BehaviourClass::StaticConditional, buffs: Vec::new(), trigger: None, plain }
}

/// Item count of each block, inferred from the stat rows (see [`BlockBehaviour::pieces`]).
///
/// The stat rows fill slots 1..=5 except where an ability sits, so the empty slots in 2..=5 are the
/// ability slots. Grants take the earliest ones, plain grants (no `list26` entry) before always-on
/// passives (all 43 hand-listed Slayer/Aegis sets come out at 3); any block beyond the gaps is the
/// 5-piece (a set whose 5th slot has stats *and* a proc). Special shapes first:
///
/// * Stat rows in slot 1 only: a monster set (ability at 2) or a mythic (ability at 1): a
///   combat-triggered block is 2, anything else 1.
/// * Stat rows in slots 1 and 2 only (Mighty Chudan, Slimecraw, Immolator Charr): a monster set.
///   Head + shoulders is the most pieces it can have, so the ability is at 2, sharing the slot
///   with the second stat row, not in the first "gap" (3).
/// * No stat rows and exactly one block (every mythic that has no stat, every arena weapon): a
///   1-item set, the block is at 1. Two or more blocks with no stat rows (Blessing of the
///   Potentates) cannot be told apart and get `None`.
///
/// Unclassified blocks get `None` and do not use up a gap.
fn infer_pieces(filled: &BTreeSet<u8>, blocks: &[Classified]) -> Vec<Option<u8>> {
    let mut out = vec![None; blocks.len()];
    let known = |i: &usize| blocks[*i].class != BehaviourClass::Unclassified;
    if filled.is_empty() {
        if blocks.len() == 1 && known(&0) {
            out[0] = Some(1);
        }
        return out;
    }
    if *filled == BTreeSet::from([1]) {
        for i in (0..blocks.len()).filter(known) {
            out[i] = Some(if blocks[i].class == BehaviourClass::CombatTriggered { 2 } else { 1 });
        }
        return out;
    }
    if *filled == BTreeSet::from([1, 2]) {
        for i in (0..blocks.len()).filter(known) {
            out[i] = Some(2);
        }
        return out;
    }
    let gaps: Vec<u8> = (2..=5).filter(|s| !filled.contains(s)).collect();
    let mut order: Vec<usize> = (0..blocks.len()).filter(known).collect();
    order.sort_by_key(|&i| (!(blocks[i].class == BehaviourClass::GrantsBuff && blocks[i].plain), i));
    for (rank, &i) in order.iter().enumerate() {
        out[i] = Some(gaps.get(rank).copied().unwrap_or(5));
    }
    out
}

/// The kind of a set: the listed one if `set_kinds.txt` has it, else from the shape of its own table
/// row (see [`SetKind`]). `has_perfected`:
/// some perfected set points at this one. A perfected set takes its regular set's kind in
/// [`classify`], so this is only called with sets that own their blocks.
///
/// The rules read the *shape* (stat slots, block count, inferred pieces), not the classes, so a
/// block whose ability is missing from the bundle (Prototype Mythic Necklace) still counts.
fn kind_of(set: &SetInput, blocks: &[BlockBehaviour], has_perfected: bool) -> SetKind {
    if let Some(kind) = set.kind_override {
        return kind;
    }
    // the table says so: a class set (the longer tail), a craftable one (the item-list flag)
    if set.class_id.is_some() {
        return SetKind::Class;
    }
    if set.craftable {
        return SetKind::Crafted;
    }
    let [block] = blocks else { return SetKind::Regular };
    let filled = &set.filled_slots;
    if filled.is_empty() {
        return if has_perfected { SetKind::ArenaWeapon } else { SetKind::Mythic };
    }
    if filled.contains(&1) && filled.iter().all(|&s| s <= 2) {
        return match block.pieces {
            Some(2) => SetKind::Monster,
            Some(1) if filled.len() == 1 => SetKind::Mythic,
            _ => SetKind::Regular,
        };
    }
    SetKind::Regular
}

fn blocks_of(set: &SetInput, index: &HashMap<u32, &ResolvedAbility>) -> Vec<BlockBehaviour> {
    let classified: Vec<Classified> = set.blocks.iter().map(|&id| classify_block(id, index)).collect();
    let pieces = if set.kind_override == Some(SetKind::Mythic) {
        // one item: whatever the stat rows look like, nothing can be at 2 or more pieces
        classified.iter().map(|c| (c.class != BehaviourClass::Unclassified).then_some(1)).collect()
    } else {
        infer_pieces(&set.filled_slots, &classified)
    };
    classified
        .into_iter()
        .zip(pieces)
        .map(|(c, pieces)| BlockBehaviour {
            ability_id: c.ability_id,
            class: c.class,
            pieces,
            buffs: c.buffs,
            trigger: c.trigger,
        })
        .collect()
}

fn class_of(blocks: &[BlockBehaviour]) -> BehaviourClass {
    blocks.iter().map(|b| b.class).max().unwrap_or(BehaviourClass::StatOnly)
}

/// One behaviour per set, sorted by set id.
///
/// A set without blocks of its own whose partner has some (a perfected set) takes the partner's
/// blocks and records it in `inherits`. `abilities` is the compiled bundle.
pub fn classify(sets: &[SetInput], abilities: &[ResolvedAbility]) -> (Vec<SetBehaviour>, Report) {
    let index: HashMap<u32, &ResolvedAbility> = abilities.iter().map(|a| (a.id, a)).collect();
    let own: BTreeMap<u32, Vec<BlockBehaviour>> = sets.iter().map(|s| (s.id, blocks_of(s, &index))).collect();
    let has_perfected: BTreeSet<u32> = sets.iter().filter_map(|s| s.partner).collect();
    let own_kind: BTreeMap<u32, SetKind> =
        sets.iter().map(|s| (s.id, kind_of(s, &own[&s.id], has_perfected.contains(&s.id)))).collect();

    let mut sorted: Vec<&SetInput> = sets.iter().collect();
    sorted.sort_by_key(|s| s.id);

    let mut report = Report::default();
    let mut out = Vec::with_capacity(sorted.len());
    for set in sorted {
        let mut blocks = own[&set.id].clone();
        let mut inherits = None;
        let mut kind = own_kind[&set.id];
        if blocks.is_empty() {
            if let Some(partner) = set.partner.filter(|p| own.get(p).is_some_and(|b| !b.is_empty())) {
                blocks = own[&partner].clone();
                inherits = Some(partner);
                kind = own_kind[&partner];
            }
        }
        let class = class_of(&blocks);
        *report.by_class.entry(class).or_insert(0) += 1;
        *report.by_kind.entry(kind).or_insert(0) += 1;
        if inherits.is_none() {
            if blocks.iter().any(|b| b.class == BehaviourClass::GrantsBuff) {
                report.sets_with_grant_blocks += 1;
            }
            for b in blocks.iter().filter(|b| b.class == BehaviourClass::Unclassified) {
                report.unclassified.push((set.id, b.ability_id));
            }
            if blocks.iter().any(|b| b.class == BehaviourClass::GrantsBuff && b.pieces.is_none()) {
                report.grants_without_pieces.push(set.id);
            }
        }
        out.push(SetBehaviour { id: set.id, name: set.name.clone(), class, kind, inherits, blocks });
    }
    (out, report)
}

/// `set_behaviour.jsonl`: one set per line, sorted by id, LF endings.
pub fn to_jsonl(behaviours: &[SetBehaviour]) -> Result<String, String> {
    let mut out = String::new();
    for b in behaviours {
        out.push_str(&serde_json::to_string(b).map_err(|e| format!("serialising set {}: {e}", b.id))?);
        out.push('\n');
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use esosim_types::BehaviourClass as C;
    use esosim_types::SetKind;

    /// A bundle entry from the fields the classifier reads: `edges` is `(kind, to)` pairs.
    fn ability(id: u32, major_minor_id: u8, trigger: Option<u32>, edges: &[(&str, u32)]) -> ResolvedAbility {
        let edges: Vec<String> =
            edges.iter().map(|(k, to)| format!(r#"{{"kind":"{k}","to":{to}}}"#)).collect();
        let triggers = trigger.map(|t| format!(r#"[{{"u2":[{t},0,0,0]}}]"#)).unwrap_or_else(|| "[]".into());
        serde_json::from_str(&format!(
            r#"{{"id":{id},"name":"a{id}","major_minor_id":{major_minor_id},"triggers":{triggers},"edges":[{}]}}"#,
            edges.join(",")
        ))
        .unwrap()
    }

    fn set(id: u32, filled: &[u8], blocks: &[u32], partner: Option<u32>) -> SetInput {
        SetInput {
            id,
            name: format!("Set {id}"),
            filled_slots: filled.iter().copied().collect(),
            blocks: blocks.to_vec(),
            partner,
            kind_override: None,
            craftable: false,
            class_id: None,
        }
    }

    /// 10 = grant ability pointing at buff 11 (both code 46); 12 = proc that applies buff 11;
    /// 13 = plain passive; 14 = buff with a trigger-less edge to nothing known.
    fn bundle() -> Vec<ResolvedAbility> {
        vec![
            ability(10, 46, None, &[("tooltip", 11), ("field_u34c", 11)]),
            ability(11, 46, None, &[]),
            ability(12, 0, Some(78), &[("causes", 11), ("sets_cooldown", 11)]),
            ability(13, 0, None, &[]),
            ability(14, 51, None, &[]),
            // removes the buff instead of granting it
            ability(15, 0, None, &[("removes_buff", 11)]),
        ]
    }

    fn run(sets: &[SetInput]) -> (Vec<SetBehaviour>, Report) {
        classify(sets, &bundle())
    }

    #[test]
    fn a_buff_grant_names_the_buff_not_the_grant_ability() {
        let (b, _) = run(&[set(1, &[2, 4], &[10], None)]);
        assert_eq!(b[0].class, C::GrantsBuff);
        assert_eq!(b[0].blocks[0].buffs, [11]);
        assert_eq!(b[0].blocks[0].pieces, Some(3));
        assert_eq!(b[0].grants().collect::<Vec<_>>(), [(11, 3)]);
    }

    #[test]
    fn a_buff_that_is_its_own_record_is_its_own_grant() {
        let (b, _) = run(&[set(1, &[2, 4], &[14], None)]);
        assert_eq!(b[0].blocks[0].buffs, [14]);
    }

    #[test]
    fn a_proc_is_combat_triggered_and_keeps_trigger_and_applied_buffs() {
        let (b, _) = run(&[set(1, &[2, 3, 4], &[12], None)]);
        let blk = &b[0].blocks[0];
        assert_eq!((b[0].class, blk.class), (C::CombatTriggered, C::CombatTriggered));
        assert_eq!((blk.trigger, blk.buffs.as_slice(), blk.pieces), (Some(78), &[11][..], Some(5)));
    }

    #[test]
    fn a_plain_passive_is_static_and_removing_a_buff_is_not_granting_it() {
        let (b, _) = run(&[set(1, &[2, 3, 4], &[13], None), set(2, &[2, 3, 4], &[15], None)]);
        assert_eq!(b[0].class, C::StaticConditional);
        assert_eq!(b[1].class, C::StaticConditional);
        assert!(b[1].blocks[0].buffs.is_empty());
    }

    #[test]
    fn the_always_on_marker_is_not_a_trigger_but_a_real_one_beside_it_is() {
        // 16 = grant ability with only the passive marker (kind 0); 17 = marker plus a real trigger
        let mut abilities = bundle();
        abilities.push(ability(16, 0, Some(0), &[("tooltip", 11)]));
        let mut with_both = ability(17, 0, Some(0), &[]);
        with_both.triggers.push(serde_json::from_str(r#"{"u2":[2,0,0,0]}"#).unwrap());
        abilities.push(with_both);
        let (b, _) = classify(&[set(1, &[2, 3, 4], &[16], None), set(2, &[2, 3, 4], &[17], None)], &abilities);
        assert_eq!((b[0].class, b[0].blocks[0].trigger, b[0].blocks[0].buffs.as_slice()), (C::GrantsBuff, None, &[11][..]));
        assert_eq!((b[1].class, b[1].blocks[0].trigger), (C::CombatTriggered, Some(2)));
        // a marker-only passive without a buff is a static rule
        let (b, _) = classify(&[set(1, &[2, 3, 4], &[18], None)], &[ability(18, 0, Some(0), &[])]);
        assert_eq!(b[0].class, C::StaticConditional);
    }

    #[test]
    fn a_buff_that_exists_twice_is_granted_once_under_the_lowest_id() {
        // Minor Force 61746 and its set-specific copy 193448 share a buff code
        let abilities = vec![
            ability(10, 0, Some(0), &[("tooltip", 61746), ("causes", 193448)]),
            ability(61746, 44, None, &[]),
            ability(193448, 44, None, &[]),
        ];
        let (b, _) = classify(&[set(1, &[1], &[10], None)], &abilities);
        assert_eq!(b[0].blocks[0].buffs, [61746]);
    }

    #[test]
    fn a_set_whose_only_stat_row_is_slot_1_is_a_monster_or_a_mythic() {
        // monster set: a proc at 2; mythic (Velothi Ur-Mage's Amulet): an always-on grant at 1
        let (b, _) = run(&[set(1, &[1], &[12], None), set(2, &[1], &[10], None)]);
        assert_eq!(b[0].blocks[0].pieces, Some(2));
        assert_eq!(b[1].blocks[0].pieces, Some(1));
    }

    #[test]
    fn a_plain_grant_takes_the_earlier_gap_than_an_always_on_one() {
        // Infallible Mage: the buff-granting passive (kind 0) is first in the table, but the plain
        // Minor Slayer grant is the 3-piece.
        let mut abilities = bundle();
        abilities.push(ability(16, 0, Some(0), &[("tooltip", 11)]));
        let (b, _) = classify(&[set(1, &[2, 4, 5], &[16, 10], None)], &abilities);
        let pieces: Vec<_> = b[0].blocks.iter().map(|x| x.pieces).collect();
        assert_eq!(pieces, [Some(5), Some(3)]);
    }

    #[test]
    fn no_blocks_is_stat_only() {
        let (b, r) = run(&[set(1, &[1, 2, 3, 4, 5], &[], None)]);
        assert_eq!((b[0].class, b[0].blocks.is_empty()), (C::StatOnly, true));
        assert_eq!(r.by_class[&C::StatOnly], 1);
        assert_eq!(r.sets_with_grant_blocks, 0);
    }

    #[test]
    fn the_set_class_is_the_hardest_block_but_the_grant_stays_listed() {
        // Automated Defense: Minor Aegis at 3 and a proc at 5, in either table order.
        for blocks in [[10, 12], [12, 10]] {
            let (b, r) = run(&[set(1, &[2, 4], &blocks, None)]);
            assert_eq!(r.sets_with_grant_blocks, 1);
            assert_eq!(b[0].class, C::CombatTriggered);
            assert_eq!(b[0].grants().collect::<Vec<_>>(), [(11, 3)], "{blocks:?}");
            let proc_block = b[0].blocks.iter().find(|x| x.class == C::CombatTriggered).unwrap();
            assert_eq!(proc_block.pieces, Some(5));
        }
    }

    #[test]
    fn a_proc_sharing_slot_5_with_a_stat_row_is_the_five_piece() {
        // Eternal Warrior: stats at 2, 4, 5; the only gap (3) goes to the grant.
        let (b, _) = run(&[set(1, &[2, 4, 5], &[12, 10], None)]);
        let pieces: Vec<_> = b[0].blocks.iter().map(|x| (x.class, x.pieces)).collect();
        assert_eq!(pieces, [(C::CombatTriggered, Some(5)), (C::GrantsBuff, Some(3))]);
    }

    #[test]
    fn a_monster_set_with_a_stat_row_at_two_has_its_ability_at_two_not_three() {
        // Mighty Chudan / Slimecraw: stat rows at 1 and 2 and Major Resolve at 2. Head + shoulders
        // is as many pieces as a monster set can have, so the "first gap" (3) is impossible.
        let (b, _) = run(&[set(1, &[1, 2], &[10], None), set(2, &[1, 2], &[12], None)]);
        assert_eq!(b[0].grants().collect::<Vec<_>>(), [(11, 2)]);
        assert_eq!(b[1].blocks[0].pieces, Some(2));
        assert_eq!((b[0].kind, b[1].kind), (SetKind::Monster, SetKind::Monster));
    }

    #[test]
    fn a_set_with_no_stat_rows_and_one_block_has_it_at_one_piece() {
        // Sea-Serpent's Coil, Oakensoul Ring, ... (mythics) and the arena weapons are 1-item sets
        let (b, r) = run(&[set(1, &[], &[10], None)]);
        assert_eq!(b[0].blocks[0].pieces, Some(1));
        assert_eq!(b[0].grants().collect::<Vec<_>>(), [(11, 1)]);
        assert!(r.grants_without_pieces.is_empty());
    }

    #[test]
    fn a_set_with_no_stat_rows_and_two_blocks_has_no_inferred_pieces_and_is_reported() {
        // Blessing of the Potentates: two blocks and nothing to tell them apart
        let (b, r) = run(&[set(1, &[], &[10, 13], None)]);
        assert_eq!(b[0].blocks.iter().map(|x| x.pieces).collect::<Vec<_>>(), [None, None]);
        assert_eq!(b[0].grants().count(), 0);
        assert_eq!(r.grants_without_pieces, [1]);
        assert_eq!(b[0].kind, SetKind::Regular);
    }

    #[test]
    fn kinds_follow_the_shape_of_the_set() {
        use SetKind::*;
        let (b, r) = run(&[
            set(1, &[1], &[12], None),        // monster: stat at 1, proc at 2
            set(2, &[1, 2], &[10], None),     // monster: Major Resolve at 2
            set(3, &[1], &[10], None),        // mythic with a stat row (Velothi Ur-Mage's Amulet)
            set(4, &[], &[10], None),         // mythic with none (Oakensoul Ring)
            set(5, &[], &[12], None),         // arena weapon: has a perfected version ...
            set(6, &[2], &[], Some(5)),       // ... which is one too, and carries 5's blocks
            set(7, &[2, 3, 4], &[10], None),  // regular
            set(8, &[1, 2, 3, 4, 5], &[], None),
            set(9, &[1, 2, 3, 4, 5], &[12], None), // a proc beside five stat rows
            set(10, &[3], &[13], None),       // not slot 1: not a monster
        ]);
        let kinds: Vec<_> = b.iter().map(|x| x.kind).collect();
        assert_eq!(kinds, [Monster, Monster, Mythic, Mythic, ArenaWeapon, ArenaWeapon, Regular, Regular, Regular, Regular]);
        assert_eq!(b[5].inherits, Some(5));
        assert_eq!((r.by_kind[&Monster], r.by_kind[&Mythic], r.by_kind[&ArenaWeapon]), (2, 2, 2));
        assert_eq!(r.by_kind[&Regular], 4);
    }

    /// The table's own signals decide crafted and class sets, ahead of the shape; a perfected set takes
    /// its partner's kind as always, and a listed kind still beats both.
    #[test]
    fn the_tables_craftable_flag_and_class_id_make_crafted_and_class_sets() {
        use SetKind::*;
        let crafted = SetInput { craftable: true, ..set(1, &[2, 3, 4], &[10], None) };
        let class = SetInput { class_id: Some(117), ..set(2, &[2, 3, 4], &[13], None) };
        // a one-block set with a stat row at 1 and a proc has the monster shape, but the flag wins
        let flagged = SetInput { craftable: true, ..set(3, &[1], &[12], None) };
        let listed = SetInput { craftable: true, kind_override: Some(Regular), ..set(4, &[1, 2, 3, 4, 5], &[13], None) };
        let (b, r) = run(&[crafted, class, flagged, listed]);
        assert_eq!(b.iter().map(|x| x.kind).collect::<Vec<_>>(), [Crafted, Class, Crafted, Regular]);
        assert_eq!((r.by_kind[&Crafted], r.by_kind[&Class], r.by_kind[&Regular]), (2, 1, 1));
        assert!(r.summary().contains("2 crafted, 1 class"));
    }

    #[test]
    fn a_listed_kind_beats_the_shape_and_a_listed_mythic_is_one_piece() {
        // Stormweaver's Cavort looks like a monster set (a stat row at 1, a proc), but is a mythic
        let plain = set(1, &[1], &[12], None);
        let listed = SetInput { id: 2, kind_override: Some(SetKind::Mythic), ..plain.clone() };
        let (b, _) = run(&[plain, listed]);
        assert_eq!((b[0].kind, b[0].blocks[0].pieces), (SetKind::Monster, Some(2)));
        assert_eq!(b[1].kind, SetKind::Mythic);
        assert_eq!(b[1].blocks[0].pieces, Some(1));
    }

    #[test]
    fn the_kind_list_parses_and_rejects_what_it_cannot_read() {
        let m = parse_kind_overrides("# comment\n\n675 mythic  # Stormweaver's Cavort\n12 arena_weapon\n").unwrap();
        assert_eq!(m, BTreeMap::from([(675, SetKind::Mythic), (12, SetKind::ArenaWeapon)]));
        for (bad, why) in [
            ("675 mythical\n", "unknown kind"),
            ("675\n", "expected"),
            ("x mythic\n", "expected"),
            ("675 mythic\n675 monster\n", "twice"),
        ] {
            let e = parse_kind_overrides(bad).unwrap_err();
            assert!(e.contains(why), "{bad:?}: {e}");
        }
    }

    #[test]
    fn an_unclassified_block_does_not_stop_a_no_stat_set_being_a_mythic() {
        // Prototype Mythic Necklace: its ability is not in the bundle, but the shape is a mythic's
        let (b, _) = run(&[set(1, &[], &[999], None)]);
        assert_eq!((b[0].blocks[0].pieces, b[0].kind), (None, SetKind::Mythic));
        // while the same block beside stat rows in 1..=2 is not a monster: its piece count is unknown
        let (b, _) = run(&[set(1, &[1], &[999], None)]);
        assert_eq!(b[0].kind, SetKind::Regular);
    }

    #[test]
    fn a_missing_ability_is_a_report_entry_not_a_crash() {
        let (b, r) = run(&[set(1, &[2, 4], &[999, 10], None)]);
        assert_eq!(b[0].class, C::Unclassified);
        assert_eq!(b[0].blocks[0].pieces, None);
        assert_eq!(b[0].blocks[1].pieces, Some(3)); // the unclassified block used no gap
        assert_eq!(r.unclassified, [(1, 999)]);
        assert!(r.summary().contains("set 1 block ability 999"));
    }

    #[test]
    fn a_perfected_set_without_blocks_takes_its_partners() {
        let (b, r) = run(&[set(1, &[2, 4], &[12], Some(2)), set(2, &[5], &[], Some(1))]);
        assert_eq!(b[1].inherits, Some(1));
        assert_eq!(b[1].class, C::CombatTriggered);
        assert_eq!(b[1].blocks, b[0].blocks);
        assert_eq!(b[0].inherits, None); // the regular set has its own
        assert_eq!(r.by_class[&C::CombatTriggered], 2);
    }

    #[test]
    fn output_is_sorted_by_set_id_one_line_each() {
        let (b, _) = run(&[set(5, &[], &[], None), set(2, &[], &[], None)]);
        let text = to_jsonl(&b).unwrap();
        let ids: Vec<u32> = text.lines().map(|l| serde_json::from_str::<SetBehaviour>(l).unwrap().id).collect();
        assert_eq!(ids, [2, 5]);
        assert!(text.ends_with('\n'));
    }

    // ---- the shipped data -------------------------------------------------------------------

    const SETS: &str = include_str!("../../../crates/esosim-data/data/sets.jsonl");
    const BUNDLE: &str = include_str!("../../../crates/esosim-data/data/bundle.jsonl");
    const BEHAVIOUR: &str = include_str!("../../../crates/esosim-data/data/set_behaviour.jsonl");

    fn shipped() -> (Vec<SetInput>, Vec<ResolvedAbility>) {
        let overrides = parse_kind_overrides(KIND_OVERRIDES).unwrap();
        let sets = SETS
            .lines()
            .map(|l| {
                let v: serde_json::Value = serde_json::from_str(l).unwrap();
                SetInput {
                    id: v["id"].as_u64().unwrap() as u32,
                    name: v["name"].as_str().unwrap_or("").to_string(),
                    filled_slots: v["bonuses"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .enumerate()
                        .filter(|(_, row)| !row.as_array().unwrap().is_empty())
                        .map(|(i, _)| i as u8 + 1)
                        .collect(),
                    blocks: v["abilities"]
                        .as_array()
                        .map(|a| a.iter().map(|b| b["ability_id"].as_u64().unwrap() as u32).collect())
                        .unwrap_or_default(),
                    partner: v["perfected_of"].as_u64().map(|p| p as u32),
                    kind_override: overrides.get(&(v["id"].as_u64().unwrap() as u32)).copied(),
                    craftable: v["craftable"].as_bool().unwrap_or(false),
                    class_id: v["class_id"].as_u64().map(|c| c as u8),
                }
            })
            .collect();
        let abilities = BUNDLE.lines().map(|l| serde_json::from_str(l).unwrap()).collect();
        (sets, abilities)
    }

    /// The data file is exactly what this code makes of the shipped tables. If this fails after
    /// a classifier change, regenerate it with
    /// `UPDATE_GOLDEN=1 cargo test -p esosim-data-compiler the_shipped_behaviour_file`
    /// (or `cargo xtask sync-data` against the game files), then **diff the result** before
    /// committing: the test passing afterwards only says the file and the code agree.
    #[test]
    fn the_shipped_behaviour_file_matches_the_classifier() {
        let (sets, abilities) = shipped();
        let (b, _) = classify(&sets, &abilities);
        let got = to_jsonl(&b).unwrap();
        if std::env::var_os("UPDATE_GOLDEN").is_some() {
            let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../../crates/esosim-data/data/set_behaviour.jsonl");
            std::fs::write(path, &got).unwrap();
            return;
        }
        // not assert_eq!: on a mismatch that would print both 250 KB files
        assert!(got == BEHAVIOUR, "set_behaviour.jsonl is stale: re-run with UPDATE_GOLDEN=1 and review the diff");
    }

    #[test]
    fn every_shipped_set_has_a_class_and_only_one_block_is_unclassified() {
        let (sets, abilities) = shipped();
        let (b, r) = classify(&sets, &abilities);
        assert_eq!(b.len(), sets.len());
        // Prototype Mythic Necklace - u49 Mythic Proto 1: its ability is not in the bundle.
        assert_eq!(r.unclassified.len(), 1);
    }
}
