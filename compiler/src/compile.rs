//! SkillData34 -> ResolvedAbility, root selection, and bundle writing.

use crate::classify;
use crate::corpus::Corpus;
use crate::graph::Graph;
use crate::resolved::*;
use eso_skill_data::SkillData34;
use eso_skill_data::enums::{
    ability_tag::AbilityTag, ability_type::AbilityType, damage_type::DamageType,
    major_minor::MajorMinorBuff, mechanic::Mechanic, skill_line::SkillLine,
};
use serde::Serialize;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::{self, File};
use std::io::{self, BufWriter, Write};
use std::path::Path;

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

/// Coverage of the regex classifier over the emitted abilities (`classify.rs`).
#[derive(Debug, Default, Serialize)]
pub struct ClassifyStats {
    /// Emitted abilities that carry a value or coefficient equation.
    pub with_value: usize,
    pub classified: usize,
    pub conflicts: usize,
    /// Why the rest were not classified (`no_references`, `no_rule_matched`, ...).
    pub unclassified: BTreeMap<String, usize>,
    /// Classified abilities per channel set, e.g. `Channel::Power`.
    pub by_channel: BTreeMap<String, usize>,
}

impl ClassifyStats {
    pub fn record(&mut self, c: &ResolvedClassification) {
        self.with_value += 1;
        match c.status {
            ClassStatus::Classified => {
                self.classified += 1;
                *self.by_channel.entry(crate::classify::label(&c.channels)).or_insert(0) += 1;
            }
            ClassStatus::Conflict => self.conflicts += 1,
            ClassStatus::Unclassified => {
                let why = c.reason.clone().unwrap_or_else(|| "unknown".to_string());
                *self.unclassified.entry(why).or_insert(0) += 1;
            }
        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct CompileStats {
    pub source_file: String,
    pub total_records: usize,
    pub roots: usize,
    pub emitted: usize,
    pub all_records_mode: bool,
    pub duplicate_records: usize,
    pub dangling_refs: BTreeMap<EdgeKind, usize>,
    pub small_tooltip_ids_dropped: usize,
    /// Things the enums don't know yet — each is a to-do for eso_skill_data.
    pub unknown_ability_types: BTreeMap<u8, usize>,
    pub unknown_mechanics: BTreeMap<u8, usize>,
    pub unknown_tags: BTreeMap<u16, usize>,
    pub unknown_skill_lines: BTreeMap<u32, usize>,
    pub unknown_major_minor_codes: BTreeMap<u8, usize>,
    pub classification: ClassifyStats,
}

pub fn compile(corpus: &Corpus, graph: &Graph, all: bool) -> (Vec<ResolvedAbility>, CompileStats) {
    let roots: Vec<u32> = corpus
        .skills
        .iter()
        .filter(|(_, s)| is_root(s))
        .map(|(id, _)| *id)
        .collect();

    let wanted: BTreeSet<u32> = if all {
        corpus.skills.keys().copied().collect()
    } else {
        graph.reachable(roots.iter().copied(), |_| true)
    };

    let mut stats = CompileStats {
        source_file: corpus.source_file.clone(),
        total_records: corpus.skills.len(),
        roots: roots.len(),
        emitted: wanted.len(),
        all_records_mode: all,
        duplicate_records: corpus.duplicate_records,
        dangling_refs: graph.dangling.clone(),
        small_tooltip_ids_dropped: graph.small_tooltip_ids,
        ..Default::default()
    };

    let mut out = Vec::with_capacity(wanted.len());
    for id in &wanted {
        let Some(s) = corpus.skills.get(id) else { continue };
        let r = resolve(s, corpus, graph);

        for t in &r.tags {
            if AbilityTag::from_id(t).is_none() {
                *stats.unknown_tags.entry(*t).or_insert(0) += 1;
            }
        }
        if let Some(c) = &r.classification {
            stats.classification.record(c);
        }
        out.push(r);
    }
    (out, stats)
}

/// `bundle.jsonl`: one ability per line, sorted by id, LF endings, no absolute
/// paths or timestamps — so `git diff` between patches shows only real changes.
/// `bundle.meta.json`: the stats above.
pub fn write_bundle(dir: &Path, abilities: &[ResolvedAbility], stats: &CompileStats) -> io::Result<()> {
    fs::create_dir_all(dir)?;

    let mut w = BufWriter::new(File::create(dir.join("bundle.jsonl"))?);
    for a in abilities {
        serde_json::to_writer(&mut w, a)?;
        w.write_all(b"\n")?;
    }
    w.flush()?;

    let mut m = BufWriter::new(File::create(dir.join("bundle.meta.json"))?);
    serde_json::to_writer_pretty(&mut m, stats)?;
    m.write_all(b"\n")?;
    m.flush()?;
    Ok(())
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
    fn unknown_ids_are_kept_and_counted_not_dropped() {
        let mut s = skill(1);
        s.ability_tags = vec![60000]; // surely not a real tag
        s.major_minor_id = 250; // not a real code
        let c = Corpus::from_skills(vec![s]);
        let g = Graph::build(&c);
        let (out, stats) = compile(&c, &g, true);
        assert_eq!(out[0].tags, vec![60000]);
        assert_eq!(out[0].major_minor_id, 250);
        assert_eq!(stats.unknown_tags.get(&60000), Some(&1));
        assert_eq!(stats.unknown_major_minor_codes.get(&250), Some(&1));
    }

    #[test]
    fn root_closure_pulls_in_referenced_records_only() {
        let mut root = skill(1);
        root.base_data.player_skill_index = 5;
        root.causes_ids = vec![2];
        root.link_types = vec![1];
        let c = Corpus::from_skills(vec![root, skill(2), skill(3)]);
        let g = Graph::build(&c);

        let (out, stats) = compile(&c, &g, false);
        assert_eq!(out.iter().map(|a| a.id).collect::<Vec<_>>(), vec![1, 2]);
        assert_eq!(stats.roots, 1);

        let (all, _) = compile(&c, &g, true);
        assert_eq!(all.len(), 3);

        // denormalised back-reference
        assert_eq!(out[1].referenced_by.len(), 1);
        assert_eq!(out[1].referenced_by[0].from, 1);
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
