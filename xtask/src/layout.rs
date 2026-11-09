//! Where generated files live (roadmap 1.2): everything the compiler writes goes into
//! `crates/esosim-data`, and nowhere else.

use std::path::{Path, PathBuf};

/// Folder (relative to the workspace root) that receives `sets.jsonl`, `bundle.jsonl`,
/// `set_behaviour.jsonl`, `set_items.csv`, `effects.jsonl`, `cp.jsonl` and `cp_effects.jsonl`.
/// The engine reads `sets.jsonl` through `esosim_data::SETS_JSONL`.
pub const DATA_DIR: &str = "crates/esosim-data/data";
/// The generated set-id constants file (`esosim_data::set_ids`).
pub const IDS_FILE: &str = "crates/esosim-data/src/set_ids.rs";

/// Effect files written by hand (relative to the workspace root). The compiler never generates an
/// effect for their ids, and `verify` rejects an override that reuses one.
pub const HAND_WRITTEN_EFFECTS: &[&str] =
    &["crates/esosim/src/data/effects/core.jsonl", "crates/esosim/src/data/effects/legacy.jsonl"];

/// `--hand-written <file>` for each of [`HAND_WRITTEN_EFFECTS`].
pub fn hand_written_args(root: &Path) -> Vec<String> {
    HAND_WRITTEN_EFFECTS
        .iter()
        .flat_map(|f| ["--hand-written".to_string(), root.join(f).to_string_lossy().into_owned()])
        .collect()
}

/// What the compiler produces, and where each file goes.
pub struct Output {
    pub name: &'static str,
    pub dest: PathBuf,
}

pub fn outputs(root: &Path) -> Vec<Output> {
    let data = root.join(DATA_DIR);
    vec![
        Output { name: "sets.jsonl", dest: data.join("sets.jsonl") },
        Output { name: "set_ids.rs", dest: root.join(IDS_FILE) },
        Output { name: "bundle.jsonl", dest: data.join("bundle.jsonl") },
        Output { name: "set_behaviour.jsonl", dest: data.join("set_behaviour.jsonl") },
        Output { name: "set_items.csv", dest: data.join("set_items.csv") },
        Output { name: "effects.jsonl", dest: data.join("effects.jsonl") },
        Output { name: "cp.jsonl", dest: data.join("cp.jsonl") },
        Output { name: "cp_effects.jsonl", dest: data.join("cp_effects.jsonl") },
    ]
}
