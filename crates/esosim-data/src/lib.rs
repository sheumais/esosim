//! Generated game data, written by `cargo xtask sync-data` (roadmap 1.2 / 1.3).
//!
//! Nothing in this crate is edited by hand:
//!
//! * `data/sets.jsonl`      one set per line (embedded below)
//! * `data/bundle.jsonl`    one resolved ability per line (not compiled in; no engine code reads it yet)
//! * `data/bundle.meta.json` statistics from an older compiler run, no longer written by the current compiler
//! * `src/set_ids.rs`       one `pub const <NAME>_SET_ID: u16` per set

pub mod set_ids;

/// `data/sets.jsonl`: one set per line, as written by the compiler.
pub const SETS_JSONL: &str = include_str!("../data/sets.jsonl");

#[cfg(test)]
mod tests {
    use super::*;

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
