//! Where generated files live (roadmap 1.2): everything the compiler writes goes into
//! `crates/esosim-data`, and nowhere else.

use std::path::{Path, PathBuf};

/// Folder (relative to the workspace root) that receives `sets.jsonl` and `bundle.jsonl`.
/// The engine reads `sets.jsonl` through `esosim_data::SETS_JSONL`.
pub const DATA_DIR: &str = "crates/esosim-data/data";
/// The generated set-id constants file (`esosim_data::set_ids`).
pub const IDS_FILE: &str = "crates/esosim-data/src/set_ids.rs";

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
    ]
}
