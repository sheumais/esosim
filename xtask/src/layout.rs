//! Where generated files live (roadmap 1.2): everything the compiler writes goes into
//! `crates/esosim-data`, and nowhere else.

use std::path::{Path, PathBuf};

/// Folder (relative to the workspace root) that receives `sets.jsonl`, `bundle.jsonl`,
/// `set_behaviour.jsonl`, `set_items.csv`, `skill_lines.csv`, `effects.jsonl`, `cp.jsonl` and `cp_effects.jsonl`.
/// The engine reads `sets.jsonl` through `esosim_data::SETS_JSONL`.
pub const DATA_DIR: &str = "crates/esosim-data/data";
/// The generated set-id constants file (`esosim_data::set_ids`).
pub const IDS_FILE: &str = "crates/esosim-data/src/set_ids.rs";

/// The generated item id -> weapon type / armour weight / jewellery file (roadmap 2.8). It is the one
/// generated file that lives in the engine crate: `data/tables/item_type.rs` includes it.
pub const ITEM_TYPES_FILE: &str = "crates/esosim/src/data/item_types.csv";

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
        Output { name: "skill_lines.csv", dest: data.join("skill_lines.csv") },
        Output { name: "effects.jsonl", dest: data.join("effects.jsonl") },
        Output { name: "cp.jsonl", dest: data.join("cp.jsonl") },
        Output { name: "cp_effects.jsonl", dest: data.join("cp_effects.jsonl") },
        Output { name: "item_types.csv", dest: root.join(ITEM_TYPES_FILE) },
    ]
}
