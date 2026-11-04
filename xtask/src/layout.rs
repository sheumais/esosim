//! Where generated files live. When roadmap 1.2 moves the engine data into
//! `crates/esosim-data`, change only the two paths below.

use std::path::{Path, PathBuf};

/// Folder (relative to the workspace root) that receives `sets.jsonl`, `bundle.jsonl`
/// and `bundle.meta.json`. Today the engine `include_str!`s `sets.jsonl` from here.
pub const DATA_DIR: &str = "esosim/src/data/bundle";
/// The generated set-id constants file.
pub const IDS_FILE: &str = "esosim/src/data/tables/set_ids.rs";

/// What `compile` and `sets` produce, and where each file goes.
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
        Output { name: "bundle.meta.json", dest: data.join("bundle.meta.json") },
    ]
}
