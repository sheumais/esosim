//! Reference graph between ability records.
//!
//! The expensive part of the old Yew tool was chasing ids lazily
//! (`resolve_id`, morph lookups, `causes_ids` traversal). Here every reference is
//! resolved exactly once, validated against the set of ids that actually exist,
//! and indexed in both directions.

use crate::corpus::Corpus;
use crate::resolved::{BackRef, Edge, EdgeKind};
use eso_skill_data::enums::tooltip_type::TooltipType;
use eso_skill_data::{SkillData34, TypeLink};
use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// PLAN.md §10 `is_ability` heuristic: inside tooltip id lists, values below this
/// are small constants (e.g. a `MajorMinorBuff` code), not ability ids. Applied
/// ONLY to tooltip ids; every other field is validated purely by id membership.
pub const MIN_TOOLTIP_ABILITY_ID: u32 = 255;

/// One `<<n>>` placeholder of a parent's tooltip, attributed to the ability that carries
/// its value.
///
/// The tooltip format is: each `tooltip_data` entry holds parallel `tooltip_types` and
/// `tooltip_ids`; every (type, id) *pair* is one placeholder, numbered in order across all
/// entries (so `<<n>>` is pair `n - 1` of the flattened list). The id in the pair is often
/// not the ability holding the value: the id dictionary's `resolve_id` follows
/// `causes_ids` / the first tooltip entry / `u34c_ability_id` (up to 3 hops) until a record
/// that really has a value of that tooltip type is found. That record is the target here.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TooltipRef {
    pub parent: u32,
    /// 0-based index of the pair in the flattened list (`<<flat + 1>>`).
    pub flat: usize,
    /// Index of the `tooltip_data` entry the pair came from.
    pub entry: usize,
    pub ty: Option<TooltipType>,
    /// Hops `resolve_target` had to follow. 0 = the pair's own id carries the value;
    /// anything else is a guess made by the id dictionary's display heuristic and is only
    /// used when no direct reference exists (see `classify_one_with`).
    pub hops: u8,
}

/// Port of the id dictionary's `resolve_id`.
///
/// Returns the target and how many hops it took. The heuristic is built for *display* and can
/// walk off to an unrelated record (e.g. "Immovable" -> its snare -> its helper -> the block
/// passive "Burnished Scales", whose 4% then shows up as the movement speed penalty), so the
/// hop count is kept to rank the evidence.
pub fn resolve_target(corpus: &Corpus, initial: u32, ty: TooltipType) -> (u32, u8) {
    let mut current = initial;
    let mut hops = 0u8;
    for _ in 0..=3 {
        let Some(s) = corpus.skills.get(&current) else { break };
        if s.has_tooltip_value(ty) {
            break;
        }
        if s.causes_ids.len() == 1 {
            current = s.causes_ids[0];
            hops += 1;
        } else if let Some(t) = s.tooltip_data.first() {
            if t.tooltip_ids.len() == 1 {
                current = t.tooltip_ids.first().copied().unwrap_or(initial);
                hops += 1;
            } else if s.u15.u34c_ability_id > 0 {
                current = s.u15.u34c_ability_id;
                hops += 1;
            } else {
                break;
            }
        } else {
            break;
        }
    }
    (current, hops)
}

pub struct Graph {
    /// Placeholders per *value-carrying* ability (after `resolve_target`), self references
    /// included. This, not `incoming`, is what classification reads.
    pub tooltip_refs: BTreeMap<u32, Vec<TooltipRef>>,
    pub out: BTreeMap<u32, Vec<Edge>>,
    pub incoming: BTreeMap<u32, Vec<BackRef>>,
    /// References that pointed at an id not present in the corpus (per kind).
    pub dangling: BTreeMap<EdgeKind, usize>,
    /// Tooltip ids dropped by the `MIN_TOOLTIP_ABILITY_ID` heuristic.
    pub small_tooltip_ids: usize,
}

/// All candidate outgoing references of one record, *unvalidated* against the corpus.
pub fn raw_edges(s: &SkillData34) -> (Vec<Edge>, usize) {
    let me = s.ability_id1;
    let mut v: Vec<Edge> = Vec::new();
    let mut small = 0usize;

    {
        let mut push = |kind: EdgeKind, link_type: Option<u8>, to: u32| {
            if to != 0 && to != me {
                v.push(Edge { kind, link_type, to });
            }
        };

        for (i, id) in s.causes_ids.iter().enumerate() {
            push(EdgeKind::Causes, s.link_types.get(i).copied(), *id);
        }
        push(EdgeKind::CausedBy, None, s.base_data.caused_by);
        push(EdgeKind::Related, None, s.base_data.related_ability_id);
        push(EdgeKind::Replaces, None, s.u8c);
        if let Some(e) = s.list25.first() {
            // `base_ability_id()` also excludes scribed abilities; mirror it.
            if s.base_data.scribing_index == 0 {
                push(EdgeKind::BaseAbility, None, e.base_ability_id);
            }
        }
        match s.type_link() {
            Some(TypeLink::ActionList(id)) => push(EdgeKind::ActionList, None, id),
            Some(TypeLink::GrantsSynergy(id)) => push(EdgeKind::GrantsSynergy, None, id),
            Some(TypeLink::SetsCooldown(id)) => push(EdgeKind::SetsCooldown, None, id),
            Some(TypeLink::RemovesBuff(id)) => push(EdgeKind::RemovesBuff, None, id),
            // These two carry an item id / zone id, not an ability id.
            Some(TypeLink::CreatesItem(_)) | Some(TypeLink::AreaTeleport(_)) | None => {}
        }
        push(EdgeKind::FieldU33c, None, s.u13.u33c_ability_id);
        push(EdgeKind::FieldU34c, None, s.u15.u34c_ability_id);
        push(EdgeKind::FieldU374, None, s.u18.u374_ability_id);
        push(EdgeKind::FieldU39c, None, s.u22.u39c_ability_id);

        for td in &s.tooltip_data {
            for id in &td.tooltip_ids {
                if *id != 0 && *id < MIN_TOOLTIP_ABILITY_ID {
                    small += 1;
                } else {
                    push(EdgeKind::Tooltip, None, *id);
                }
            }
        }
    }

    v.sort();
    v.dedup();
    (v, small)
}

impl Graph {
    pub fn build(corpus: &Corpus) -> Graph {
        let mut g = Graph {
            tooltip_refs: BTreeMap::new(),
            out: BTreeMap::new(),
            incoming: BTreeMap::new(),
            dangling: BTreeMap::new(),
            small_tooltip_ids: 0,
        };

        for (&id, skill) in &corpus.skills {
            let (edges, small) = raw_edges(skill);
            g.small_tooltip_ids += small;
            let mut kept = Vec::with_capacity(edges.len());
            for e in edges {
                if corpus.skills.contains_key(&e.to) {
                    g.incoming.entry(e.to).or_default().push(BackRef {
                        from: id,
                        kind: e.kind,
                        link_type: e.link_type,
                    });
                    kept.push(e);
                } else {
                    *g.dangling.entry(e.kind).or_insert(0) += 1;
                }
            }
            if !kept.is_empty() {
                g.out.insert(id, kept);
            }
        }
        for refs in g.incoming.values_mut() {
            refs.sort();
        }

        for (&parent, skill) in &corpus.skills {
            let mut flat = 0usize;
            for (entry, td) in skill.tooltip_data.iter().enumerate() {
                for (j, &tid) in td.tooltip_ids.iter().enumerate() {
                    let idx = flat;
                    flat += 1;
                    if tid < MIN_TOOLTIP_ABILITY_ID {
                        continue; // a constant / buff code, not an ability
                    }
                    let ty = td.tooltip_types.get(j).and_then(TooltipType::from_id);
                    let (target, hops) = match ty {
                        Some(t) if SkillData34::is_value_tooltip(t) => resolve_target(corpus, tid, t),
                        _ => (tid, 0),
                    };
                    if corpus.skills.contains_key(&target) {
                        g.tooltip_refs.entry(target).or_default().push(TooltipRef { parent, flat: idx, entry, ty, hops });
                    }
                }
            }
        }
        g
    }

    /// Everything reachable by following outgoing edges (for which `follow`
    /// returns true) from `roots`. Roots themselves are included.
    pub fn reachable<I, F>(&self, roots: I, follow: F) -> BTreeSet<u32>
    where
        I: IntoIterator<Item = u32>,
        F: Fn(EdgeKind) -> bool,
    {
        let mut seen: BTreeSet<u32> = BTreeSet::new();
        let mut queue: VecDeque<u32> = VecDeque::new();
        for r in roots {
            if seen.insert(r) {
                queue.push_back(r);
            }
        }
        while let Some(id) = queue.pop_front() {
            let Some(edges) = self.out.get(&id) else { continue };
            for e in edges {
                if follow(e.kind) && seen.insert(e.to) {
                    queue.push_back(e.to);
                }
            }
        }
        seen
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use eso_skill_data::TooltipData;

    fn skill(id: u32) -> SkillData34 {
        let mut s = SkillData34::default();
        s.ability_id1 = id;
        s
    }

    #[test]
    fn causes_edges_keep_link_type_and_build_back_refs() {
        let mut a = skill(1000);
        a.causes_ids = vec![2000, 3000];
        a.link_types = vec![22, 4];
        let c = Corpus::from_skills(vec![a, skill(2000), skill(3000)]);
        let g = Graph::build(&c);

        let out = &g.out[&1000];
        assert_eq!(out.len(), 2);
        assert!(out.contains(&Edge { kind: EdgeKind::Causes, link_type: Some(22), to: 2000 }));
        assert!(out.contains(&Edge { kind: EdgeKind::Causes, link_type: Some(4), to: 3000 }));

        assert_eq!(
            g.incoming[&2000],
            vec![BackRef { from: 1000, kind: EdgeKind::Causes, link_type: Some(22) }]
        );
    }

    #[test]
    fn dangling_and_self_references_are_not_edges() {
        let mut a = skill(1000);
        a.causes_ids = vec![1000, 9999]; // self + missing
        let c = Corpus::from_skills(vec![a]);
        let g = Graph::build(&c);
        assert!(g.out.get(&1000).is_none());
        assert_eq!(g.dangling.get(&EdgeKind::Causes), Some(&1));
    }

    #[test]
    fn small_tooltip_ids_are_constants_not_edges() {
        let mut a = skill(1000);
        a.tooltip_data = vec![TooltipData { tooltip_ids: vec![7, 5000], ..Default::default() }];
        let c = Corpus::from_skills(vec![a, skill(5000)]);
        let g = Graph::build(&c);
        assert_eq!(g.small_tooltip_ids, 1);
        assert_eq!(g.out[&1000], vec![Edge { kind: EdgeKind::Tooltip, link_type: None, to: 5000 }]);
    }

    #[test]
    fn reachability_follows_only_requested_kinds() {
        let mut a = skill(1);
        a.causes_ids = vec![2];
        a.link_types = vec![1];
        let mut b = skill(2);
        b.base_data.related_ability_id = 3;
        let c = Corpus::from_skills(vec![a, b, skill(3)]);
        let g = Graph::build(&c);

        let all = g.reachable([1], |_| true);
        assert_eq!(all.into_iter().collect::<Vec<_>>(), vec![1, 2, 3]);

        let causes_only = g.reachable([1], |k| k == EdgeKind::Causes);
        assert_eq!(causes_only.into_iter().collect::<Vec<_>>(), vec![1, 2]);
    }
}