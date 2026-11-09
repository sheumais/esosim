//! The champion point table (roadmap 2.2a): `ChampionPointData` records to `cp.jsonl`.
//!
//! Every star's *effect* is an ability like any other (`ability_id`), so the abilities of all
//! stars are roots of the `cp` category and arrive in `bundle.jsonl` with their tooltips. This
//! file only carries what the CP table adds: stages, cost per stage and value per stage.

use eso_skill_data::champion_points::ChampionPointData;
use esosim_types::{AggKind, Channel, ChannelValue, Condition, CpPerk, CpStar, EffectDef, EffectSource, Stacking};
use serde::Deserialize;
use std::collections::{BTreeMap, BTreeSet};

/// Whether a record is a star the game shows in the CP menu. The table also holds leftover
/// records (72 of 190 in patch data seen so far); every one of them has no position on the
/// constellation map, so it cannot be displayed and is not a star. A record without an ability
/// is not a star either. The record's own `name` field is always empty: a star is named after
/// its ability.
fn is_real(r: &ChampionPointData) -> bool {
    r.ability_id != 0 && !(r.pos_x == 0.0 && r.pos_y == 0.0)
}

/// One star per real record (see [`is_real`]), sorted by star id, each named by the lang name of
/// its ability (`ability_name`; empty if the lang has none). An unknown `perk_type` is an error:
/// the table has changed shape and guessing would hide it.
pub fn stars(
    records: &[ChampionPointData],
    ability_name: impl Fn(u32) -> Option<String>,
) -> Result<Vec<CpStar>, String> {
    let mut out = Vec::with_capacity(records.len());
    for r in records.iter().filter(|r| is_real(r)) {
        let name = ability_name(r.ability_id).unwrap_or_default();
        let perk = CpPerk::from_game(r.perk_type)
            .ok_or_else(|| format!("champion point {} (`{name}`): unknown perk_type {}", r.id(), r.perk_type))?;
        out.push(CpStar {
            id: r.id(),
            name,
            ability_id: r.ability_id,
            perk,
            max_stages: r.max_stages,
            cost_per_stage: r.cost_per_stage,
            value_per_stage: r.value_per_stage,
            links: r.all_link_ids().collect(),
            // worked out by `assign_trees` from `cp_trees.toml`
            tree: String::new(),
            entry: false,
            pos_x: r.pos_x,
            pos_y: r.pos_y,
        });
    }
    out.sort_by_key(|s| s.id);
    for pair in out.windows(2) {
        if pair[0].id == pair[1].id {
            return Err(format!("champion point id {} appears twice in the table", pair[0].id));
        }
    }
    Ok(out)
}

/// `cp.jsonl`: one star per line, sorted by id, LF endings.
pub fn to_jsonl(stars: &[CpStar]) -> Result<String, String> {
    let mut out = String::new();
    for s in stars {
        out.push_str(&serde_json::to_string(s).map_err(|e| format!("serialising champion point {}: {e}", s.id))?);
        out.push('\n');
    }
    Ok(out)
}

/// Every non-zero `ability_id` of the stars: the roots of the `cp` category.
pub fn ability_roots(stars: &[CpStar]) -> BTreeSet<u32> {
    stars.iter().map(|s| s.ability_id).filter(|id| *id != 0).collect()
}

/// The stars of a `cp.jsonl` text (used by `verify` and `suggest-effects`, which work on files).
pub fn stars_of_jsonl(cp_jsonl: &str) -> Result<Vec<CpStar>, String> {
    let mut stars = Vec::new();
    for (i, line) in cp_jsonl.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        stars.push(serde_json::from_str::<CpStar>(line).map_err(|e| format!("cp.jsonl line {}: {e}", i + 1))?);
    }
    Ok(stars)
}

/// [`ability_roots`] from a `cp.jsonl` text.
pub fn ability_ids_of_jsonl(cp_jsonl: &str) -> Result<BTreeSet<u32>, String> {
    Ok(ability_roots(&stars_of_jsonl(cp_jsonl)?))
}

// ---------------------------------------------------------------------------------------------
// Roadmap 2.2c: stars -> effects
// ---------------------------------------------------------------------------------------------

/// The embedded default, `data/cp_effects.toml`.
pub const CP_EFFECTS_TOML: &str = include_str!("../data/cp_effects.toml");

fn one() -> f64 {
    1.0
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawChannel {
    channel: Channel,
    #[serde(default = "one")]
    scale: f64,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawStar {
    id: u32,
    name: String,
    channels: Vec<RawChannel>,
    #[serde(default)]
    condition: Condition,
    #[serde(default)]
    note: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawFile {
    #[serde(default)]
    star: Vec<RawStar>,
}

/// What a star feeds: written by hand in `cp_effects.toml`. The number is not here, it comes from the
/// star's `value_per_stage`.
#[derive(Debug, Clone, PartialEq)]
pub struct StarEffect {
    /// Star id (`CpStar::id`).
    pub id: u32,
    pub name: String,
    /// Channel and the factor applied to `value_per_stage`.
    pub channels: Vec<(Channel, f64)>,
    pub condition: Condition,
    pub note: Option<String>,
}

/// Parse `cp_effects.toml`. Rejects unknown fields, a repeated star and a star with no channels.
pub fn parse_effects(text: &str) -> Result<Vec<StarEffect>, String> {
    let raw: RawFile = toml::from_str(text).map_err(|e| format!("cp_effects.toml: {e}"))?;
    let mut seen = BTreeSet::new();
    let mut out = Vec::with_capacity(raw.star.len());
    for r in raw.star {
        if !seen.insert(r.id) {
            return Err(format!("cp_effects.toml: star {} (`{}`) is listed twice", r.id, r.name));
        }
        if r.channels.is_empty() {
            return Err(format!("cp_effects.toml: star {} (`{}`) has no channels", r.id, r.name));
        }
        out.push(StarEffect {
            id: r.id,
            name: r.name,
            channels: r.channels.into_iter().map(|c| (c.channel, c.scale)).collect(),
            condition: r.condition,
            note: r.note,
        });
    }
    Ok(out)
}

/// The table stores `value_per_stage` as an `f32`; going through its shortest decimal keeps `34.61`
/// as `34.61` instead of `34.61000061035156`.
fn f32_as_f64(v: f32) -> f64 {
    v.to_string().parse().unwrap_or(v as f64)
}

/// `value_per_stage * scale` without the binary noise of the product (`3 * 0.01` must be `0.03`, not
/// `0.030000000000000002`): rounded to nine decimals.
fn tidy(v: f64) -> f64 {
    (v * 1e9).round() / 1e9
}

/// One `EffectDef` per mapped star, sorted by ability id: `id` is the star's ability (that is what
/// `ActiveEffects` is keyed by), the source remembers the star, and each stage adds
/// `value_per_stage * scale` to each channel (`PerStack` capped at the star's `max_stages`).
/// A star the table does not have, one whose name differs, or one worth nothing is an error: the
/// mapping was written against another table.
///
/// Units: the game stores a percentage as `3` for 3 %. A channel that aggregates multiplicatively
/// ([`Channel::agg_kind`]) holds a *fraction* (`0.03`), so its entry needs `scale = 0.01`; the
/// compiler refuses a multiplicative channel that would reach more than 100 % at the star's last
/// stage, which is what forgetting the scale looks like.
pub fn effects(stars: &[CpStar], map: &[StarEffect]) -> Result<Vec<EffectDef>, String> {
    let mut out = Vec::with_capacity(map.len());
    for m in map {
        let at = |msg: String| format!("cp_effects.toml: star {} (`{}`): {msg}", m.id, m.name);
        let star = stars
            .iter()
            .find(|s| s.id == m.id)
            .ok_or_else(|| at("is not a star of the CP table".to_string()))?;
        if star.name != m.name {
            return Err(at(format!("the table calls it `{}`", star.name)));
        }
        if star.value_per_stage == 0.0 {
            return Err(at("the table gives it no value per stage".to_string()));
        }
        let per_stage = f32_as_f64(star.value_per_stage);
        let mut channels = Vec::with_capacity(m.channels.len());
        for (channel, scale) in &m.channels {
            let value_per_stack = tidy(per_stage * scale);
            if channel.agg_kind() == AggKind::Multiplicative && (value_per_stack * star.max_stages as f64).abs() > 1.0 {
                return Err(at(format!(
                    "{channel:?} is multiplicative, so its value is a fraction, but {} stage(s) of {value_per_stack} reach {}: \
                     a percentage needs `scale = 0.01`",
                    star.max_stages,
                    value_per_stack * star.max_stages as f64
                )));
            }
            channels.push(ChannelValue { channel: *channel, value: 0.0, value_per_stack });
        }
        out.push(EffectDef {
            id: star.ability_id,
            name: star.name.clone(),
            source: EffectSource::ChampionPoint { cp_id: star.id },
            channels,
            condition: m.condition.clone(),
            stacking: Stacking::PerStack { max: star.max_stages },
            duration_ms: None,
            cooldown_ms: None,
            note: m.note.clone(),
        });
    }
    out.sort_by_key(|d| d.id);
    for pair in out.windows(2) {
        if pair[0].id == pair[1].id {
            return Err(format!("cp_effects.toml: ability {} is the effect of two stars", pair[0].id));
        }
    }
    Ok(out)
}

/// `cp_effects.jsonl`: one `EffectDef` per line, sorted by id, LF endings.
pub fn effects_to_jsonl(defs: &[EffectDef]) -> Result<String, String> {
    let mut out = String::new();
    for d in defs {
        out.push_str(&serde_json::to_string(d).map_err(|e| format!("serialising star effect {}: {e}", d.id))?);
        out.push('\n');
    }
    Ok(out)
}

// ---------------------------------------------------------------------------------------------
// Roadmap 2.2b: which tree a star is in
// ---------------------------------------------------------------------------------------------

/// The embedded default, `data/cp_trees.toml`.
pub const CP_TREES_TOML: &str = include_str!("../data/cp_trees.toml");

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTree {
    name: String,
    blob: u32,
    #[serde(default)]
    floating: Vec<u32>,
    #[serde(default)]
    entry: Vec<u32>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct RawTrees {
    #[serde(default)]
    tree: Vec<RawTree>,
}

/// One tree as written in `cp_trees.toml`. All ids are *ability* ids, as that is what the
/// game's other files and the tooltips talk about.
#[derive(Debug, Clone, PartialEq)]
pub struct TreeSpec {
    pub name: String,
    /// Any star of the tree's large connected group of stars.
    pub blob: u32,
    /// The bonus stars that are not connected to anything.
    pub floating: Vec<u32>,
    /// Stars that can be bought with no connected star (ways into the tree). May be empty: the
    /// tree's prerequisites are then not checked.
    pub entry: Vec<u32>,
}

/// Parse `cp_trees.toml`. An empty file is fine: no trees, so no tree rules.
pub fn parse_trees(text: &str) -> Result<Vec<TreeSpec>, String> {
    let raw: RawTrees = toml::from_str(text).map_err(|e| format!("cp_trees.toml: {e}"))?;
    let mut names = BTreeSet::new();
    let mut out = Vec::with_capacity(raw.tree.len());
    for t in raw.tree {
        if t.name.trim().is_empty() {
            return Err("cp_trees.toml: a tree has no name".to_string());
        }
        if !names.insert(t.name.clone()) {
            return Err(format!("cp_trees.toml: tree `{}` is listed twice", t.name));
        }
        out.push(TreeSpec { name: t.name, blob: t.blob, floating: t.floating, entry: t.entry });
    }
    Ok(out)
}

/// Stars whose link to another star is not repeated by that star (`a` lists `b` but `b` does not
/// list `a`), as `(a, b)` pairs. The table has two, both from Piercing; prerequisites follow each
/// star's own list, so this is only reported.
pub fn one_way_links(stars: &[CpStar]) -> Vec<(u32, u32)> {
    let by_id: BTreeMap<u32, &CpStar> = stars.iter().map(|s| (s.id, s)).collect();
    let mut out = Vec::new();
    for s in stars {
        for l in &s.links {
            if let Some(other) = by_id.get(l) {
                if !other.links.contains(&s.id) && !out.contains(&(s.id, *l)) {
                    out.push((s.id, *l));
                }
            }
        }
    }
    out.sort_unstable();
    out
}

/// Give every star its tree and mark the entry stars. A tree is its blob (the stars connected to
/// the one `blob` names, links read in both directions) plus its floating stars. With no trees
/// configured nothing happens. With any, every star must end up in exactly one tree: a star in
/// none, in two, a blob that two trees share, or a "floating" star that is in fact linked to
/// something are all errors, because they mean the table and the file disagree.
///
/// Returns one line for the report.
pub fn assign_trees(stars: &mut [CpStar], trees: &[TreeSpec]) -> Result<String, String> {
    if trees.is_empty() {
        return Ok("no trees configured (cp_trees.toml): slot and prerequisite rules cannot be checked".to_string());
    }
    let by_ability: BTreeMap<u32, usize> = stars.iter().enumerate().map(|(i, s)| (s.ability_id, i)).collect();
    let find = |tree: &str, what: &str, ability: u32| -> Result<usize, String> {
        by_ability
            .get(&ability)
            .copied()
            .ok_or_else(|| format!("cp_trees.toml: tree `{tree}`: {what} {ability} is not the ability of a star of the CP table"))
    };

    // connected groups, links read both ways
    let index_of: BTreeMap<u32, usize> = stars.iter().enumerate().map(|(i, s)| (s.id, i)).collect();
    let mut adjacent: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); stars.len()];
    for (i, s) in stars.iter().enumerate() {
        for l in &s.links {
            if let Some(&j) = index_of.get(l) {
                adjacent[i].insert(j);
                adjacent[j].insert(i);
            }
        }
    }
    let group_of = |start: usize| -> BTreeSet<usize> {
        let mut seen = BTreeSet::new();
        let mut todo = vec![start];
        while let Some(i) = todo.pop() {
            if seen.insert(i) {
                todo.extend(adjacent[i].iter().copied());
            }
        }
        seen
    };

    let mut owner: Vec<Option<usize>> = vec![None; stars.len()];
    let claim = |owner: &mut Vec<Option<usize>>, stars: &[CpStar], i: usize, t: usize| -> Result<(), String> {
        match owner[i] {
            None => {
                owner[i] = Some(t);
                Ok(())
            }
            Some(prev) if prev == t => Ok(()),
            Some(prev) => Err(format!(
                "cp_trees.toml: star `{}` is in both `{}` and `{}`",
                stars[i].name, trees[prev].name, trees[t].name
            )),
        }
    };

    for (t, tree) in trees.iter().enumerate() {
        let blob = find(&tree.name, "blob star", tree.blob)?;
        let group = group_of(blob);
        if group.len() == 1 {
            return Err(format!(
                "cp_trees.toml: tree `{}`: `{}` is linked to nothing, so it is not in a blob; list it as floating",
                tree.name, stars[blob].name
            ));
        }
        for &i in &group {
            claim(&mut owner, stars, i, t)?;
        }
        for &ability in &tree.floating {
            let i = find(&tree.name, "floating star", ability)?;
            if group_of(i).len() != 1 {
                return Err(format!(
                    "cp_trees.toml: tree `{}`: `{}` is listed as floating but is linked to other stars",
                    tree.name, stars[i].name
                ));
            }
            claim(&mut owner, stars, i, t)?;
        }
    }

    if let Some(i) = owner.iter().position(Option::is_none) {
        let loose: Vec<&str> = owner.iter().enumerate().filter(|(_, o)| o.is_none()).map(|(i, _)| stars[i].name.as_str()).collect();
        return Err(format!(
            "cp_trees.toml: {} star(s) are in no tree, e.g. `{}` (ability {}). A new floating star? All of them: {}",
            loose.len(),
            stars[i].name,
            stars[i].ability_id,
            loose.join(", ")
        ));
    }

    for (i, s) in stars.iter_mut().enumerate() {
        s.tree = trees[owner[i].expect("every star was claimed")].name.clone();
        s.entry = false;
    }
    for (t, tree) in trees.iter().enumerate() {
        for &ability in &tree.entry {
            let i = find(&tree.name, "entry star", ability)?;
            if owner[i] != Some(t) {
                return Err(format!(
                    "cp_trees.toml: tree `{}`: entry star `{}` is in tree `{}`",
                    tree.name,
                    stars[i].name,
                    stars[i].tree
                ));
            }
            stars[i].entry = true;
        }
    }

    let parts: Vec<String> = trees
        .iter()
        .map(|t| {
            let n = stars.iter().filter(|s| s.tree == t.name).count();
            if t.entry.is_empty() {
                format!("{} {n} (no entry star: prerequisites not checked)", t.name)
            } else {
                format!("{} {n} ({} entry)", t.name, t.entry.len())
            }
        })
        .collect();
    Ok(format!("trees: {}", parts.join(", ")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn star(id: u32, links: &[u32]) -> CpStar {
        CpStar {
            id,
            name: format!("S{id}"),
            ability_id: 1000 + id,
            perk: CpPerk::Passive,
            max_stages: 2,
            cost_per_stage: 10,
            value_per_stage: 3.0,
            links: links.to_vec(),
            tree: String::new(),
            entry: false,
            pos_x: 1.0,
            pos_y: 1.0,
        }
    }

    fn spec(name: &str, blob: u32, floating: &[u32], entry: &[u32]) -> TreeSpec {
        TreeSpec { name: name.into(), blob: 1000 + blob, floating: floating.iter().map(|i| 1000 + i).collect(), entry: entry.iter().map(|i| 1000 + i).collect() }
    }

    /// Blob 1-2-3 (links written one way only for 3), blob 4-5, floating 6 and 7.
    fn stars() -> Vec<CpStar> {
        vec![star(1, &[2]), star(2, &[1, 3]), star(3, &[]), star(4, &[5]), star(5, &[4]), star(6, &[]), star(7, &[])]
    }

    fn named(stars: &[CpStar]) -> Vec<(u32, &str, bool)> {
        stars.iter().map(|s| (s.id, s.tree.as_str(), s.entry)).collect()
    }

    #[test]
    fn a_tree_is_its_blob_read_both_ways_plus_its_floating_stars() {
        let mut s = stars();
        let report = assign_trees(&mut s, &[spec("A", 1, &[6], &[2]), spec("B", 5, &[7], &[])]).unwrap();
        assert_eq!(
            named(&s),
            [(1, "A", false), (2, "A", true), (3, "A", false), (4, "B", false), (5, "B", false), (6, "A", false), (7, "B", false)]
        );
        assert!(report.contains("A 4 (1 entry)") && report.contains("B 3 (no entry star"), "{report}");
    }

    #[test]
    fn no_trees_means_no_tree_rules() {
        let mut s = stars();
        assert!(assign_trees(&mut s, &[]).unwrap().contains("no trees configured"));
        assert!(s.iter().all(|s| s.tree.is_empty() && !s.entry));
    }

    #[test]
    fn a_star_in_no_tree_is_an_error_that_names_it() {
        let e = assign_trees(&mut stars(), &[spec("A", 1, &[6], &[]), spec("B", 5, &[], &[])]).unwrap_err();
        assert!(e.contains("1 star(s) are in no tree") && e.contains("S7"), "{e}");
    }

    #[test]
    fn two_trees_may_not_share_a_star() {
        let e = assign_trees(&mut stars(), &[spec("A", 1, &[6], &[]), spec("B", 3, &[7], &[])]).unwrap_err();
        assert!(e.contains("is in both"), "{e}");
        let e = assign_trees(&mut stars(), &[spec("A", 1, &[6, 7], &[]), spec("B", 5, &[7], &[])]).unwrap_err();
        assert!(e.contains("`S7` is in both `A` and `B`"), "{e}");
    }

    #[test]
    fn floating_and_blob_are_checked_against_the_links() {
        let e = assign_trees(&mut stars(), &[spec("A", 1, &[4], &[]), spec("B", 5, &[], &[])]).unwrap_err();
        assert!(e.contains("`S4` is listed as floating but is linked"), "{e}");
        let e = assign_trees(&mut stars(), &[spec("A", 6, &[], &[])]).unwrap_err();
        assert!(e.contains("`S6` is linked to nothing"), "{e}");
        let e = assign_trees(&mut stars(), &[spec("A", 99, &[], &[])]).unwrap_err();
        assert!(e.contains("blob star 1099 is not the ability of a star"), "{e}");
    }

    #[test]
    fn an_entry_star_must_be_in_its_own_tree() {
        let e = assign_trees(&mut stars(), &[spec("A", 1, &[6], &[4]), spec("B", 5, &[7], &[])]).unwrap_err();
        assert!(e.contains("entry star `S4` is in tree `B`"), "{e}");
    }

    #[test]
    fn trees_toml_rejects_duplicates_and_unknown_fields() {
        let ok = "[[tree]]\nname = \"A\"\nblob = 1\nfloating = [2]\n";
        assert_eq!(parse_trees(ok).unwrap(), vec![TreeSpec { name: "A".into(), blob: 1, floating: vec![2], entry: vec![] }]);
        assert!(parse_trees("").unwrap().is_empty());
        assert!(parse_trees(&format!("{ok}{ok}")).unwrap_err().contains("listed twice"));
        assert!(parse_trees("[[tree]]\nname = \"A\"\nblob = 1\nbolb = 2\n").is_err());
    }

    #[test]
    fn one_way_links_are_found() {
        assert_eq!(one_way_links(&stars()), vec![(2, 3)]);
    }

    fn map(text: &str) -> Vec<StarEffect> {
        parse_effects(text).unwrap()
    }

    fn one_star(perk: CpPerk, stages: u32, per_stage: f32) -> CpStar {
        CpStar { perk, max_stages: stages, value_per_stage: per_stage, ..star(1, &[]) }
    }

    #[test]
    fn a_percentage_in_a_multiplicative_channel_needs_the_scale() {
        let stars = [one_star(CpPerk::Passive, 2, 3.0)];
        let channel = r#"{ DamageDone = ["Direct", "Multiplicative"] }"#;
        let bad = map(&format!("[[star]]\nid = 1\nname = \"S1\"\nchannels = [{{ channel = {channel} }}]\n"));
        assert!(effects(&stars, &bad).unwrap_err().contains("`scale = 0.01`"));

        let good = map(&format!("[[star]]\nid = 1\nname = \"S1\"\nchannels = [{{ channel = {channel}, scale = 0.01 }}]\n"));
        let d = &effects(&stars, &good).unwrap()[0];
        assert_eq!(d.channels[0].value_per_stack, 0.03, "no binary noise");
        assert_eq!(d.stacking, Stacking::PerStack { max: 2 });
        assert_eq!(d.source, EffectSource::ChampionPoint { cp_id: 1 });
    }

    #[test]
    fn an_additive_channel_may_hold_a_big_number_and_a_reduction_is_negative() {
        let stars = [one_star(CpPerk::Passive, 50, 34.61)];
        let m = map("[[star]]\nid = 1\nname = \"S1\"\nchannels = [{ channel = { Armour = \"All\" } }]\n");
        assert_eq!(effects(&stars, &m).unwrap()[0].channels[0].value_per_stack, 34.61);

        let stars = [one_star(CpPerk::Passive, 2, 1.0)];
        let m = map("[[star]]\nid = 1\nname = \"S1\"\nchannels = [{ channel = { DamageTaken = [{ Type = \"Poison\" }, \"Multiplicative\"] }, scale = -0.01 }]\n");
        assert_eq!(effects(&stars, &m).unwrap()[0].channels[0].value_per_stack, -0.01);
    }

    #[test]
    fn a_mapping_written_against_another_table_is_an_error() {
        let stars = [one_star(CpPerk::Passive, 2, 3.0)];
        let wrong_name = map("[[star]]\nid = 1\nname = \"Other\"\nchannels = [{ channel = \"CriticalChance\" }]\n");
        assert!(effects(&stars, &wrong_name).unwrap_err().contains("the table calls it `S1`"));
        let missing = map("[[star]]\nid = 9\nname = \"S9\"\nchannels = [{ channel = \"CriticalChance\" }]\n");
        assert!(effects(&stars, &missing).unwrap_err().contains("not a star of the CP table"));
        let worthless = [one_star(CpPerk::Passive, 2, 0.0)];
        let ok = map("[[star]]\nid = 1\nname = \"S1\"\nchannels = [{ channel = \"CriticalChance\" }]\n");
        assert!(effects(&worthless, &ok).unwrap_err().contains("no value per stage"));
    }

    // -- the shipped files hold together ---------------------------------------------------------

    const CP_JSONL: &str = include_str!("../../../crates/esosim-data/data/cp.jsonl");
    const CP_EFFECTS_JSONL: &str = include_str!("../../../crates/esosim-data/data/cp_effects.jsonl");

    /// `cp.jsonl` as the compiler would write it from the shipped `cp_trees.toml`.
    #[test]
    fn the_shipped_cp_jsonl_has_the_trees_the_shipped_file_gives() {
        let mut stars = stars_of_jsonl(CP_JSONL).unwrap();
        let before: Vec<(String, bool)> = stars.iter().map(|s| (s.tree.clone(), s.entry)).collect();
        assert!(before.iter().all(|(t, _)| !t.is_empty()), "cp.jsonl has no trees: run sync-data");
        for s in &mut stars {
            (s.tree, s.entry) = (String::new(), false);
        }
        assign_trees(&mut stars, &parse_trees(CP_TREES_TOML).unwrap()).unwrap();
        let after: Vec<(String, bool)> = stars.iter().map(|s| (s.tree.clone(), s.entry)).collect();
        assert_eq!(before, after);
        assert_eq!(to_jsonl(&stars).unwrap(), CP_JSONL);
    }

    /// `cp_effects.jsonl` as the compiler would write it from the shipped mapping.
    #[test]
    fn the_shipped_cp_effects_jsonl_is_what_the_shipped_mapping_gives() {
        let stars = stars_of_jsonl(CP_JSONL).unwrap();
        let defs = effects(&stars, &parse_effects(CP_EFFECTS_TOML).unwrap()).unwrap();
        assert_eq!(effects_to_jsonl(&defs).unwrap(), CP_EFFECTS_JSONL);
    }

    #[test]
    fn every_star_of_the_shipped_table_is_in_exactly_one_tree() {
        let stars = stars_of_jsonl(CP_JSONL).unwrap();
        assert_eq!(stars.len(), 118);
        let mut count: BTreeMap<&str, usize> = BTreeMap::new();
        for s in &stars {
            *count.entry(s.tree.as_str()).or_default() += 1;
        }
        assert_eq!(count.values().sum::<usize>(), 118);
        assert_eq!(count.len(), 3, "{count:?}");
    }
}
