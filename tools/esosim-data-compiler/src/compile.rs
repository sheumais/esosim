//! SkillData34 -> ResolvedAbility, root selection, and bundle writing.

use crate::classify;
use crate::corpus::Corpus;
use crate::graph::Graph;
use crate::resolved::*;
use eso_skill_data::SkillData34;
use std::collections::BTreeSet;

pub fn is_root(s: &SkillData34) -> bool {
    s.base_data.player_skill_index != 0
}

pub fn resolve(s: &SkillData34, corpus: &Corpus, graph: &Graph) -> ResolvedAbility {
    let id = s.ability_id1;
    let b = &s.base_data;

    let damage_type = s.u4[3];
    ResolvedAbility {
        id,
        name: corpus.name_of(id),
        ability_type: b.ability_type,
        mechanic: s.mechanic,
        skill_line: b.skill_line_id,
        player_skill_index: b.player_skill_index,
        scribing_index: b.scribing_index,
        major_minor_id: s.major_minor_id,
        damage_type,
        tags: s.ability_tags.clone(),
        flags: s
            .flags
            .iter()
            .enumerate()
            .filter(|(_, v)| **v != 0)
            .map(|(i, v)| (i as u8, *v))
            .collect(),
        target_type: b.target_type,
        duration_ms: b.duration,
        tick_ms: b.tick,
        cast_time_ms: b.cast_time,
        cost: b.cost,
        range: b.range,
        radius: b.radius,
        value0: b.value0,
        value1: b.value1,
        value2: b.value2,
        target_cap: s.target_cap(),
        coef: s
            .coef
            .iter()
            .map(|c| ResolvedCoef {
                types: [c.type1, c.type2, c.type3, c.type4],
                coefs: [c.coef1, c.coef2, c.coef3, c.coef4],
                u20: c.u20,
            })
            .collect(),
        execute: s.execute().map(|e| ResolvedExecute {
            threshold_below_health_pct: e.threshold_below_health_pct,
            bonus_up_to_pct: e.bonus_up_to_pct,
        }),
        triggers: s
            .list26
            .iter()
            .map(|l| ResolvedTrigger { u2: l.u2.clone(), list3: l.list3.clone() })
            .collect(),
        tooltips: s
            .tooltip_data
            .iter()
            .map(|t| ResolvedTooltip { types: t.tooltip_types.clone(), ids: t.tooltip_ids.clone() })
            .collect(),
        tooltip_text: corpus.tooltips.get(&id).cloned(),
        classification: classify::classify_one(s, corpus, graph),
        morph_of: s.base_ability_id(),
        edges: graph.out.get(&id).cloned().unwrap_or_default(),
        referenced_by: graph.incoming.get(&id).cloned().unwrap_or_default(),
    }
}

/// Every ability reachable from a root, resolved and sorted by id.
///
/// Roots are the player skills plus `extra_roots` (the set abilities). Extra ids that have
/// no record in the skill data are skipped.
pub fn compile(corpus: &Corpus, graph: &Graph, extra_roots: &BTreeSet<u32>) -> Vec<ResolvedAbility> {
    let roots = corpus
        .skills
        .iter()
        .filter(|(_, s)| is_root(s))
        .map(|(id, _)| *id)
        .chain(extra_roots.iter().copied().filter(|id| corpus.skills.contains_key(id)));
    let wanted = graph.reachable(roots, |_| true);

    wanted
        .iter()
        .filter_map(|id| corpus.skills.get(id))
        .map(|s| resolve(s, corpus, graph))
        .collect()
}

/// `bundle.jsonl`: one ability per line, sorted by id, LF endings, no absolute paths or
/// timestamps, so `git diff` between patches shows only real changes.
pub fn to_jsonl(abilities: &[ResolvedAbility]) -> Result<String, String> {
    let mut out = String::new();
    for a in abilities {
        out.push_str(&serde_json::to_string(a).map_err(|e| format!("serialising ability {}: {e}", a.id))?);
        out.push('\n');
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn skill(id: u32) -> SkillData34 {
        let mut s = SkillData34::default();
        s.ability_id1 = id;
        s
    }

    #[test]
    fn major_minor_code_resolves_to_name() {
        let mut s = skill(61665);
        s.major_minor_id = 2;
        let c = Corpus::from_skills(vec![s.clone()]);
        let g = Graph::build(&c);
        let r = resolve(&s, &c, &g);
        assert_eq!(r.major_minor_id, 2);
    }

    #[test]
    fn unknown_ids_are_kept_not_dropped() {
        let mut s = skill(1);
        s.ability_tags = vec![60000];
        s.major_minor_id = 250;
        let c = Corpus::from_skills(vec![s]);
        let g = Graph::build(&c);
        let out = compile(&c, &g, &BTreeSet::from([1]));
        assert_eq!(out[0].tags, vec![60000]);
        assert_eq!(out[0].major_minor_id, 250);
    }

    #[test]
    fn root_closure_pulls_in_referenced_records_only() {
        let mut root = skill(1);
        root.base_data.player_skill_index = 5;
        root.causes_ids = vec![2];
        root.link_types = vec![1];
        let c = Corpus::from_skills(vec![root, skill(2), skill(3)]);
        let g = Graph::build(&c);

        let out = compile(&c, &g, &BTreeSet::new());
        assert_eq!(out.iter().map(|a| a.id).collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(out[1].referenced_by.len(), 1);
        assert_eq!(out[1].referenced_by[0].from, 1);
    }

    #[test]
    fn extra_roots_are_emitted_with_their_closure_and_missing_ids_are_skipped() {
        let mut set_ability = skill(57162);
        set_ability.causes_ids = vec![57163];
        set_ability.link_types = vec![1];
        let c = Corpus::from_skills(vec![set_ability, skill(57163), skill(99999)]);
        let g = Graph::build(&c);

        // without the extra root nothing is emitted: no player skill reaches these records
        assert!(compile(&c, &g, &BTreeSet::new()).is_empty());

        let out = compile(&c, &g, &BTreeSet::from([57162, 1234]));
        assert_eq!(out.iter().map(|a| a.id).collect::<Vec<_>>(), vec![57162, 57163]);
    }

    #[test]
    fn schema_round_trips_through_json() {
        let mut s = skill(10);
        s.base_data.duration = 1500;
        s.flags[79] = 1;
        let c = Corpus::from_skills(vec![s.clone()]);
        let g = Graph::build(&c);
        let r = resolve(&s, &c, &g);
        let json = serde_json::to_string(&r).unwrap();
        let back: ResolvedAbility = serde_json::from_str(&json).unwrap();
        assert_eq!(r, back);
        assert_eq!(back.flags, vec![(79, 1)]);
    }
}
