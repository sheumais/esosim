//! What a set's ability blocks *do*, sorted into behaviour classes (roadmap 2.1b).
//!
//! The set table gives every set 0-2 "ability blocks" (the proc / buff / special rule that comes
//! on top of the per-piece stat rows). The compiler looks each block's ability up in the ability
//! graph and writes one [`SetBehaviour`] per set to `set_behaviour.jsonl`. Nothing here is
//! engine logic: it is plain data, so the compiler can write it and the engine can read it.

use serde::{Deserialize, Serialize};

/// How hard a set (or one of its blocks) is to model. Ordered: a set's class is the *largest*
/// class among its blocks, so `class >= CombatTriggered` means "needs the simulator".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BehaviourClass {
    /// Nothing beyond the per-piece stat rows (no ability blocks at all). Fully automatic.
    StatOnly,
    /// "Gain Minor Slayer / Major Courage ... at all times": a block with no trigger that points at
    /// a Major/Minor buff. Fully automatic: becomes a grant rule with [`BlockBehaviour::pieces`].
    GrantsBuff,
    /// No trigger and no buff: a fixed rule ("reduces the cost of Sprint", "while sneaking"). The
    /// stat part is not derivable from the text, so these are hand-written overrides (2.0).
    StaticConditional,
    /// Procs on a cast / crit / damage taken / ... (a `list26` entry with a nonzero kind; kind 0
    /// is the always-on passive marker). Needs the simulator
    /// (Phase 6); the calculator shows it as "conditional/proc".
    CombatTriggered,
    /// A block whose ability is not in the bundle. A report entry, never a crash.
    Unclassified,
}

/// What *kind* of set this is, for the rules that depend on it (roadmap 2.1d). Perfected sets
/// are not a kind of their own: they are linked to their regular set (`perfected_of` in
/// `sets.jsonl`) and share its kind, so a perfected arena weapon is an [`ArenaWeapon`].
///
/// **Inferred from the shape of the set table, not read from it** (the table has no such field),
/// the same way [`BlockBehaviour::pieces`] is. The shipped counts are pinned by tests, so a change
/// in the data that moves a set from one kind to another fails loudly.
///
/// [`ArenaWeapon`]: SetKind::ArenaWeapon
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SetKind {
    /// Everything else: dungeon, trial, overland, PvP ... Crafted and class sets are kinds of their own
    /// (below); until `sets.jsonl` carries the table's flags for them they read as `Regular`.
    #[default]
    Regular,
    /// Head and shoulders only, so at most 2 pieces: a stat row at 1 piece (or 1 and 2) and the
    /// ability at 2. Shape: stat rows only in slots 1-2, slot 1 filled, one ability block at 2.
    Monster,
    /// One item, one worn at a time. Shape: a single ability block at 1 piece and at most one stat
    /// row (slot 1), and not an arena weapon.
    Mythic,
    /// Dragonstar / Maelstrom / Blackrose / Vateshran weapons and shields: no stat rows, a single
    /// ability at 1 piece, and a perfected version whose only bonus is at 2 pieces. A two-handed
    /// weapon counts as two pieces, so one is enough for the perfected 2-piece row.
    ArenaWeapon,
    /// Made at a crafting station: the table flags it (`craftable` in `sets.jsonl`, the second byte of the
    /// item-list header, on exactly the 83 craftable sets). Not inferred from the shape. A full set needs
    /// its trait count (2.1e), which this kind lets the engine ask for.
    Crafted,
    /// One class's set (`class_id` in `sets.jsonl`: the class sets are the only ones with the longer
    /// tail in the table). Worth wearing only as that class.
    Class,
}

impl SetKind {
    pub fn is_regular(&self) -> bool {
        *self == SetKind::Regular
    }
}

/// One ability block of a set, classified.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockBehaviour {
    pub ability_id: u32,
    pub class: BehaviourClass,
    /// Item count at which the block applies, **inferred** from the gaps in the set's stat rows
    /// (the stat rows fill every slot except those that hold an ability; grants take the earliest
    /// gap, anything left over is the 5-piece). `None` when the set has no stat rows at all
    /// (mythics, arena weapons: roadmap 2.1d). Cross-checked against the hand-written
    /// `SLAYER_SETS` / `AEGIS_SETS` lists (since deleted), which all used 3.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pieces: Option<u8>,
    /// `GrantsBuff`: the buff abilities the block gives (the ids in `major_minor.rs`, e.g. 76617
    /// for Minor Slayer). `CombatTriggered`: the buffs it can apply when it fires.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub buffs: Vec<u32>,
    /// `CombatTriggered`: the trigger kind (`list26[0].u2[0]` of the block's ability).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trigger: Option<u32>,
}

/// One line of `set_behaviour.jsonl`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SetBehaviour {
    pub id: u32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    /// The largest class among `blocks` (`StatOnly` when there are none).
    pub class: BehaviourClass,
    /// See [`SetKind`]; left out of the file when `Regular`. A perfected set has its regular
    /// partner's kind.
    #[serde(default, skip_serializing_if = "SetKind::is_regular")]
    pub kind: SetKind,
    /// A perfected set carries no ability blocks of its own in the table; this is the set it is
    /// paired with, whose blocks are listed below.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub inherits: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<BlockBehaviour>,
}

impl SetBehaviour {
    /// The `(buff id, pieces)` pairs of every `GrantsBuff` block: what a generated `GrantRule`
    /// is made of. Blocks whose piece count could not be inferred are left out.
    pub fn grants(&self) -> impl Iterator<Item = (u32, u8)> + '_ {
        self.blocks
            .iter()
            .filter(|b| b.class == BehaviourClass::GrantsBuff)
            .filter_map(|b| b.pieces.map(|p| (b, p)))
            .flat_map(|(b, p)| b.buffs.iter().map(move |buff| (*buff, p)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classes_are_ordered_by_difficulty() {
        use BehaviourClass::*;
        assert!(StatOnly < GrantsBuff && GrantsBuff < StaticConditional);
        assert!(StaticConditional < CombatTriggered && CombatTriggered < Unclassified);
    }

    #[test]
    fn a_line_round_trips_and_omits_empty_fields() {
        let b = SetBehaviour {
            id: 702,
            name: "Ansuul's Torment".into(),
            class: BehaviourClass::CombatTriggered,
            kind: SetKind::Regular,
            inherits: None,
            blocks: vec![
                BlockBehaviour {
                    ability_id: 147226,
                    class: BehaviourClass::GrantsBuff,
                    pieces: Some(3),
                    buffs: vec![76617],
                    trigger: None,
                },
                BlockBehaviour {
                    ability_id: 194100,
                    class: BehaviourClass::CombatTriggered,
                    pieces: Some(5),
                    buffs: vec![],
                    trigger: Some(78),
                },
            ],
        };
        let json = serde_json::to_string(&b).unwrap();
        assert_eq!(
            json,
            r#"{"id":702,"name":"Ansuul's Torment","class":"combat_triggered","blocks":[{"ability_id":147226,"class":"grants_buff","pieces":3,"buffs":[76617]},{"ability_id":194100,"class":"combat_triggered","pieces":5,"trigger":78}]}"#
        );
        assert_eq!(serde_json::from_str::<SetBehaviour>(&json).unwrap(), b);
        assert_eq!(b.grants().collect::<Vec<_>>(), [(76617, 3)]);
    }

    #[test]
    fn crafted_and_class_are_written_as_snake_case_names() {
        for (kind, name) in [(SetKind::Crafted, "crafted"), (SetKind::Class, "class")] {
            let line = format!(r#"{{"id":1,"class":"stat_only","kind":"{name}"}}"#);
            assert_eq!(serde_json::from_str::<SetBehaviour>(&line).unwrap().kind, kind);
            assert!(!kind.is_regular());
        }
    }

    #[test]
    fn a_non_regular_kind_is_written_and_read_back_and_regular_is_the_default() {
        let line = r#"{"id":256,"name":"Mighty Chudan","class":"grants_buff","kind":"monster"}"#;
        let b: SetBehaviour = serde_json::from_str(line).unwrap();
        assert_eq!(b.kind, SetKind::Monster);
        assert_eq!(serde_json::to_string(&b).unwrap(), line);
        // files written before 2.1d have no `kind`
        let old: SetBehaviour = serde_json::from_str(r#"{"id":1,"class":"stat_only"}"#).unwrap();
        assert_eq!(old.kind, SetKind::Regular);
        assert!(!serde_json::to_string(&old).unwrap().contains("kind"));
        for kind in [SetKind::Mythic, SetKind::ArenaWeapon, SetKind::Crafted, SetKind::Class] {
            assert_eq!(serde_json::from_str::<SetKind>(&serde_json::to_string(&kind).unwrap()).unwrap(), kind);
        }
    }
}
