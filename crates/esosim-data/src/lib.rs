//! Generated game data, written by `cargo xtask sync-data` (roadmap 1.2 / 1.3).
//!
//! Nothing in this crate is edited by hand:
//!
//! * `data/sets.jsonl`      one set per line (embedded below)
//! * `data/set_behaviour.jsonl` one line per set: its behaviour class and classified ability blocks (roadmap 2.1b)
//! * `data/set_items.csv`  `set id,item id,...` per set: which items belong to which set (roadmap 2.1e)
//! * `data/skill_lines.csv` `skill line id,ability id,...` per skill line: which abilities are in which line (roadmap 1.4)
//! * `data/effects.jsonl` overrides + high-confidence auto-classified effects, one `EffectDef` per line plus a `provenance` (roadmap 2.0)
//! * `data/passive_ranks.csv` one passive per line: the ability ids of its ranks, lowest rank first (roadmap 2.4)
//! * `data/cp.jsonl`       one champion point star per line: stages, cost and value per stage, its tree (roadmap 2.2a, 2.2b)
//! * `data/cp_effects.jsonl` one `EffectDef` per modelled champion point star, from `cp.jsonl` and the compiler's `cp_effects.toml` (roadmap 2.2c)
//! * `data/bundle.jsonl`    one resolved ability per line (not compiled in; no engine code reads it yet)
//! * `data/bundle.meta.json` statistics from an older compiler run, no longer written by the current compiler
//! * `src/set_ids.rs`       one `pub const <NAME>_SET_ID: u16` per set

pub mod set_ids;

/// `data/sets.jsonl`: one set per line, as written by the compiler.
pub const SETS_JSONL: &str = include_str!("../data/sets.jsonl");

/// `data/set_behaviour.jsonl`: one `esosim_types::SetBehaviour` per set, sorted by set id.
pub const SET_BEHAVIOUR_JSONL: &str = include_str!("../data/set_behaviour.jsonl");

/// `data/set_items.csv`: `set id,item id,item id,...` one line per set (roadmap 2.1e).
pub const SET_ITEMS_CSV: &str = include_str!("../data/set_items.csv");

/// `data/passive_ranks.csv`: one passive line per row, `ability id,ability id,...` lowest rank first. Each id is the
/// `ability_id` of the effects written for that rank in `effects.jsonl`; a build lists the rank its character holds, and
/// when it lists several of one row only the highest counts (`Bundle::highest_ranks`).
pub const PASSIVE_RANKS_CSV: &str = include_str!("../data/passive_ranks.csv");

/// `data/skill_lines.csv`: `skill line id,ability id,ability id,...` one line per skill line (roadmap 1.4).
pub const SKILL_LINES_CSV: &str = include_str!("../data/skill_lines.csv");

/// `data/effects.jsonl`: one `esosim_types::EffectDef` per line (plus a `provenance` of `override`
/// or `auto`), sorted by id. Empty until `overrides.toml` has entries or the classifier is sure of
/// a listed buff. Not read by the engine yet: its rows are not unit-checked (see `effects.rs`
/// in the compiler).
pub const EFFECTS_JSONL: &str = include_str!("../data/effects.jsonl");

/// `data/cp.jsonl`: one `esosim_types::CpStar` per line, sorted by star id, each with its tree (`cp_trees.toml`). The abilities the stars
/// point at (with their tooltips) are in `bundle.jsonl`.
pub const CP_JSONL: &str = include_str!("../data/cp.jsonl");

/// `data/cp_effects.jsonl`: one `esosim_types::EffectDef` per modelled champion point star, sorted by
/// ability id. `PerStack` with `value_per_stack` = the star's value per stage and `max` = its stages, so
/// `ActiveEffects::set(ability_id, stages)` is a star with that many stages bought. The engine loads it.
pub const CP_EFFECTS_JSONL: &str = include_str!("../data/cp_effects.jsonl");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_effect_row_says_where_it_came_from_and_ids_ascend() {
        let mut last = 0u64;
        for l in EFFECTS_JSONL.lines().filter(|l| !l.trim().is_empty()) {
            assert!(l.ends_with("\"provenance\":\"override\"}") || l.ends_with("\"provenance\":\"auto\"}"), "{l}");
            let id: u64 = l["{\"id\":".len()..].split(|c: char| !c.is_ascii_digit()).next().unwrap().parse().unwrap();
            assert!(id > last, "ids must ascend and not repeat: {id} after {last}");
            last = id;
        }
    }

    #[test]
    fn there_is_one_behaviour_line_per_set_in_the_same_order() {
        let ids = |text: &str| -> Vec<String> {
            text.lines()
                .map(|l| l["{\"id\":".len()..].split(|c: char| !c.is_ascii_digit()).next().unwrap().to_string())
                .collect()
        };
        let mut sets = ids(SETS_JSONL);
        sets.sort_by_key(|s| s.parse::<u32>().unwrap());
        assert_eq!(ids(SET_BEHAVIOUR_JSONL), sets);
    }

    #[test]
    fn a_passive_rank_row_lists_two_or_more_ascending_ids_and_no_id_twice() {
        let mut seen = std::collections::HashSet::new();
        for row in PASSIVE_RANKS_CSV.lines().filter(|l| !l.trim().is_empty()) {
            let ids: Vec<u32> = row.split(',').map(|c| c.trim().parse().unwrap()).collect();
            assert!(ids.len() >= 2 && ids.windows(2).all(|w| w[0] < w[1]), "{row}");
            for id in ids {
                assert!(seen.insert(id), "{id} is in two rows");
            }
        }
    }

    #[test]
    fn every_set_line_starts_with_its_id() {
        let lines: Vec<&str> = SETS_JSONL.lines().filter(|l| !l.trim().is_empty()).collect();
        assert!(lines.len() > 600, "sets.jsonl looks truncated: {} lines", lines.len());
        for l in lines {
            assert!(l.starts_with("{\"id\":"), "unexpected line: {}", &l[..l.len().min(60)]);
        }
    }

    #[test]
    fn set_ids_are_constants_only() {
        let text = include_str!("set_ids.rs");
        for l in text.lines().filter(|l| !l.starts_with("//") && !l.trim().is_empty()) {
            assert!(l.starts_with("pub const ") && l.ends_with(';'), "not a constant: {l}");
        }
    }
}
