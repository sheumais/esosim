//! SkillData34 -> ResolvedAbility, root selection, and bundle writing.

use crate::classify;
use crate::corpus::Corpus;
use crate::graph::Graph;
use crate::resolved::*;
use crate::roots::Roots;
use eso_skill_data::SkillData34;
use std::collections::{BTreeMap, BTreeSet};

/// `skill_lines.csv`: `skill line id,ability id,ability id,...` one line per skill line, lines and
/// ids ascending (roadmap 1.4). Abilities that are in no skill line are left out. `scribed` is the scribing
/// grimoires' abilities by line (`scribing::lines`): the game gives them no line, so they are added here.
pub fn skill_lines_csv(abilities: &[ResolvedAbility], scribed: &BTreeMap<u32, Vec<u32>>) -> String {
    let mut by_line: BTreeMap<u32, BTreeSet<u32>> = BTreeMap::new();
    // an ability can be in one line only: a grimoire's own line wins over one taken from an ability it replaces
    let scribed_ids: BTreeSet<u32> = scribed.values().flatten().copied().collect();
    for a in abilities.iter().filter(|a| a.skill_line != 0 && !scribed_ids.contains(&a.id)) {
        by_line.entry(a.skill_line).or_default().insert(a.id);
    }
    for (line, ids) in scribed {
        by_line.entry(*line).or_default().extend(ids);
    }
    let mut out = String::new();
    for (line, ids) in by_line {
        out.push_str(&line.to_string());
        for id in ids {
            out.push(',');
            out.push_str(&id.to_string());
        }
        out.push('\n');
    }
    out
}

/// The ability `s` replaces (`u8c`), if any: the game's `replaces_id`. An empty field or the ability's own id means
/// it replaces nothing.
pub fn replaces_id(s: &SkillData34) -> Option<u32> {
    (s.u8c != 0 && s.u8c != s.ability_id1).then_some(s.u8c)
}

/// The base data an ability takes from the abilities it replaces: where it lives (`skill_line`) and what it costs and
/// reaches (`target_type`, `cast_time`, `cost`, `range`, `radius`, `duration`, `tick`). A replacement record (a newer id
/// the game casts in place of an older one) often leaves these empty, so it would otherwise be in no skill line and
/// have no range.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct BaseData {
    pub skill_line: u32,
    pub target_type: u16,
    pub duration_ms: u32,
    pub tick_ms: u32,
    pub cast_time_ms: u32,
    pub cost: u16,
    pub range: u32,
    pub radius: u32,
}

impl BaseData {
    fn of(s: &SkillData34) -> BaseData {
        let b = &s.base_data;
        BaseData {
            skill_line: b.skill_line_id,
            target_type: b.target_type,
            duration_ms: b.duration,
            tick_ms: b.tick,
            cast_time_ms: b.cast_time,
            cost: b.cost,
            range: b.range,
            radius: b.radius,
        }
    }

    /// Fill every empty field from `older`.
    fn fill_from(&mut self, older: &BaseData) {
        if self.skill_line == 0 {
            self.skill_line = older.skill_line;
        }
        if self.target_type == 0 {
            self.target_type = older.target_type;
        }
        if self.duration_ms == 0 {
            self.duration_ms = older.duration_ms;
        }
        if self.tick_ms == 0 {
            self.tick_ms = older.tick_ms;
        }
        if self.cast_time_ms == 0 {
            self.cast_time_ms = older.cast_time_ms;
        }
        if self.cost == 0 {
            self.cost = older.cost;
        }
        if self.range == 0 {
            self.range = older.range;
        }
        if self.radius == 0 {
            self.radius = older.radius;
        }
    }
}

/// `s`'s own base data, with each field it leaves empty taken from the ability it replaces, and failing that from the
/// one *that* replaces, and so on. A value the replacement record has itself always wins: this is a default, not an
/// override. Not taken over: `player_skill_index` and `scribing_index` (they say what a record *is*, and `is_root`
/// would see the replacement as a second player skill), `ability_type`, `mechanic`, the `value`s, the coefficients
/// and the tooltips.
pub fn base_data_with_replaced(s: &SkillData34, corpus: &Corpus) -> BaseData {
    let mut base = BaseData::of(s);
    let mut seen = BTreeSet::from([s.ability_id1]);
    let mut next = replaces_id(s);
    while let Some(id) = next {
        // a cycle, or a replaced ability the game data does not have: stop with what we have
        if !seen.insert(id) {
            break;
        }
        let Some(older) = corpus.skills.get(&id) else { break };
        base.fill_from(&BaseData::of(older));
        next = replaces_id(older);
    }
    base
}

pub fn is_root(s: &SkillData34) -> bool {
    s.base_data.player_skill_index != 0
}

pub fn resolve(s: &SkillData34, corpus: &Corpus, graph: &Graph) -> ResolvedAbility {
    let id = s.ability_id1;
    let b = &s.base_data;

    let damage_type = s.u4[3];
    let base = base_data_with_replaced(s, corpus);
    ResolvedAbility {
        id,
        name: corpus.name_of(id),
        ability_type: b.ability_type,
        mechanic: s.mechanic,
        skill_line: base.skill_line,
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
        target_type: base.target_type,
        duration_ms: base.duration_ms,
        tick_ms: base.tick_ms,
        cast_time_ms: base.cast_time_ms,
        cost: base.cost,
        range: base.range,
        radius: base.radius,
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
            .map(|l| ResolvedTrigger {
                u2: l.u2.clone(),
                list3: l.list3.clone(),
                event: l.u2.first().and_then(|k| esosim_types::TriggerEvent::from_list26_kind(*k)),
            })
            .collect(),
        tooltips: s
            .tooltip_data
            .iter()
            .map(|t| ResolvedTooltip { types: t.tooltip_types.clone(), ids: t.tooltip_ids.clone() })
            .collect(),
        tooltip_text: corpus.tooltips.get(&id).cloned(),
        classification: classify::classify_one(s, corpus, graph),
        categories: Vec::new(),
        morph_of: s.base_ability_id(),
        rank_index: s.list25.first().map(|e| e.rank_index).filter(|&r| r != 0),
        edges: graph.out.get(&id).cloned().unwrap_or_default(),
        referenced_by: graph.incoming.get(&id).cloned().unwrap_or_default(),
    }
}

/// Every ability reachable from a root, resolved and sorted by id, each labelled with the root
/// categories whose closure contains it.
///
/// `roots` holds the player skills, the set abilities and what `roots.toml` lists; ids with no
/// record in the skill data are skipped (`Roots::build` already reports them).
pub fn compile(corpus: &Corpus, graph: &Graph, roots: &Roots) -> Vec<ResolvedAbility> {
    let wanted = graph.reachable(roots.all(), |_| true);

    let mut labels: BTreeMap<u32, Vec<RootCategory>> = BTreeMap::new();
    for (category, ids) in &roots.by_category {
        for id in graph.reachable(ids.iter().copied(), |_| true) {
            labels.entry(id).or_default().push(*category);
        }
    }

    wanted.iter().filter_map(|id| corpus.skills.get(id)).map(|s| {
        let mut a = resolve(s, corpus, graph);
        a.categories = labels.remove(&s.ability_id1).unwrap_or_default();
        a
    }).collect()
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
    use crate::roots::RootConfig;
    use std::collections::BTreeSet;

    /// The roots of `c`: its player skills plus `sets` as set roots, nothing else.
    fn roots(c: &Corpus, sets: &[u32]) -> Roots {
        Roots::build(c, &sets.iter().copied().collect::<BTreeSet<u32>>(), &BTreeSet::new(), &RootConfig::default()).0
    }

    fn skill(id: u32) -> SkillData34 {
        let mut s = SkillData34::default();
        s.ability_id1 = id;
        s
    }

    #[test]
    fn scribed_abilities_join_the_lines_of_the_game_data_in_id_order() {
        let ability = |id: u32, line: u32| {
            let mut s = skill(id);
            s.base_data.skill_line_id = line;
            let c = Corpus::from_skills(vec![s.clone()]);
            resolve(&s, &c, &Graph::build(&c))
        };
        let abilities = [ability(500, 44), ability(100, 44), ability(7, 0), ability(9, 22)];
        let none = BTreeMap::new();
        assert_eq!(skill_lines_csv(&abilities, &none), "22,9\n44,100,500\n");
        let scribed = BTreeMap::from([(44, vec![240150, 217699]), (72, vec![216803])]);
        assert_eq!(skill_lines_csv(&abilities, &scribed), "22,9\n44,100,500,217699,240150\n72,216803\n");
        // an id both sources list is written once
        assert_eq!(skill_lines_csv(&abilities, &BTreeMap::from([(44, vec![100])])), "22,9\n44,100,500\n");
    }

    fn replacing(id: u32, replaces: u32) -> SkillData34 {
        let mut s = skill(id);
        s.u8c = replaces;
        s
    }

    fn resolved(list: Vec<SkillData34>, id: u32) -> ResolvedAbility {
        let c = Corpus::from_skills(list);
        resolve(&c.skills[&id], &c, &Graph::build(&c))
    }

    #[test]
    fn a_replacement_takes_the_base_data_it_leaves_empty_from_the_ability_it_replaces() {
        // 36508 is the old Incapacitating Strike (line 38, range 700, cost 70); 113105 replaces it and has its own cost
        let mut old = skill(36508);
        old.base_data.skill_line_id = 38;
        old.base_data.player_skill_index = 95;
        old.base_data.range = 700;
        old.base_data.radius = 600;
        old.base_data.cost = 70;
        old.base_data.target_type = 3;
        let mut new = replacing(113105, 36508);
        new.base_data.cost = 120;
        let r = resolved(vec![old, new], 113105);
        assert_eq!((r.skill_line, r.range, r.radius, r.target_type), (38, 700, 600, 3), "taken from the replaced ability");
        assert_eq!(r.cost, 120, "its own value is not overridden");
        assert_eq!(r.player_skill_index, 0, "what a record is stays its own: the replacement is not a second player skill");
    }

    #[test]
    fn replacement_base_data_follows_a_chain_and_survives_a_cycle_or_a_missing_record() {
        let mut oldest = skill(1);
        oldest.base_data.skill_line_id = 40;
        oldest.base_data.range = 500;
        let mut middle = replacing(2, 1);
        middle.base_data.range = 900;
        let newest = replacing(3, 2);
        let r = resolved(vec![oldest, middle, newest], 3);
        assert_eq!((r.skill_line, r.range), (40, 900), "line from the oldest, range from the nearest that has one");

        // 5 and 6 replace each other: no line anywhere, and no endless loop
        let r = resolved(vec![replacing(5, 6), replacing(6, 5)], 5);
        assert_eq!((r.skill_line, r.range), (0, 0));
        // replaces a record the game data does not have
        assert_eq!(resolved(vec![replacing(7, 99999)], 7).skill_line, 0);
        // `u8c` equal to its own id, or empty, replaces nothing
        assert_eq!(replaces_id(&replacing(8, 8)), None);
        assert_eq!(replaces_id(&replacing(8, 0)), None);
        assert_eq!(replaces_id(&replacing(8, 9)), Some(9));
    }

    #[test]
    fn a_replacement_is_in_the_skill_line_of_the_ability_it_replaces() {
        let mut old = skill(36508);
        old.base_data.skill_line_id = 38;
        let list = vec![old, replacing(113105, 36508)];
        let abilities = [resolved(list.clone(), 36508), resolved(list, 113105)];
        assert_eq!(skill_lines_csv(&abilities, &BTreeMap::new()), "38,36508,113105\n");
        // a grimoire's own line wins over an inherited one: an id is in one line only
        assert_eq!(skill_lines_csv(&abilities, &BTreeMap::from([(72, vec![113105])])), "38,36508\n72,113105\n");
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
        let out = compile(&c, &g, &roots(&c, &[1]));
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

        let out = compile(&c, &g, &roots(&c, &[]));
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
        assert!(compile(&c, &g, &roots(&c, &[])).is_empty());

        let out = compile(&c, &g, &roots(&c, &[57162, 1234]));
        assert_eq!(out.iter().map(|a| a.id).collect::<Vec<_>>(), vec![57162, 57163]);
    }

    #[test]
    fn every_ability_is_labelled_with_the_categories_whose_closure_holds_it() {
        // 1 is a player skill and causes 2; 5 is a set ability and also causes 2; 3 is in no closure
        let mut skill1 = skill(1);
        skill1.base_data.player_skill_index = 5;
        skill1.causes_ids = vec![2];
        skill1.link_types = vec![1];
        let mut set5 = skill(5);
        set5.causes_ids = vec![2];
        set5.link_types = vec![1];
        let c = Corpus::from_skills(vec![skill1, skill(2), skill(3), set5]);
        let g = Graph::build(&c);

        let out = compile(&c, &g, &roots(&c, &[5]));
        let labels: Vec<(u32, Vec<RootCategory>)> = out.iter().map(|a| (a.id, a.categories.clone())).collect();
        assert_eq!(
            labels,
            vec![
                (1, vec![RootCategory::Skills]),
                (2, vec![RootCategory::Skills, RootCategory::Sets]),
                (5, vec![RootCategory::Sets]),
            ]
        );
    }

    #[test]
    fn listed_roots_from_roots_toml_are_emitted_and_labelled() {
        let mut c = Corpus::from_skills(vec![skill(61747), skill(7)]);
        c.names.insert(61747, "Major Force".into());
        let g = Graph::build(&c);
        let cfg = crate::roots::parse_config("[buffs]\nids = [61747]\n").unwrap();
        let (r, _) = Roots::build(&c, &BTreeSet::new(), &BTreeSet::new(), &cfg);

        let out = compile(&c, &g, &r);
        assert_eq!(out.iter().map(|a| a.id).collect::<Vec<_>>(), vec![61747]);
        assert_eq!(out[0].categories, vec![RootCategory::Buffs]);
        assert!(to_jsonl(&out).unwrap().contains("\"categories\":[\"buffs\"]"));
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