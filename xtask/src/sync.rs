//! `cargo xtask sync-data` — roadmap 1.3.
//!
//! compile + sets are written to a staging folder first (so a failed run never leaves the
//! tree half-updated), then each file is copied to its destination from [`layout`], `verify`
//! runs, and a per-file id diff is printed. `git status` afterwards shows only real changes
//! because byte-identical files are not rewritten.
//!
//! Roadmap A.7 also lists `cp` and `effects` in this chain; add them to `STEPS` when those
//! compiler commands exist.

use crate::{Opts, compiler, diff, layout, root, run_step};
use std::fs;

/// Compiler commands run into the staging folder, in order.
const STEPS: &[&str] = &["compile", "sets"];

pub fn run(opts: &Opts) -> Result<bool, String> {
    let root = root();
    let stage = root.join("target").join("xtask").join("sync-data");
    if stage.exists() {
        fs::remove_dir_all(&stage).map_err(|e| format!("clearing {}: {e}", stage.display()))?;
    }
    fs::create_dir_all(&stage).map_err(|e| format!("creating {}: {e}", stage.display()))?;
    let stage_arg = stage.to_str().ok_or("staging path is not valid UTF-8")?;

    for step in STEPS {
        if !run_step(&format!("compiler {step}"), &mut compiler(&[step, "--out", stage_arg], &opts.data)) {
            return Err(format!("`{step}` failed; nothing was written to the repo"));
        }
    }

    eprintln!("\n==> {}", if opts.dry_run { "diff (dry run, nothing written)" } else { "writing + diff" });
    for out in layout::outputs(&root) {
        let src = stage.join(out.name);
        let new = fs::read_to_string(&src).map_err(|e| format!("{} was not produced: {e}", out.name))?;
        let old = fs::read_to_string(&out.dest).ok();
        let shown = out.dest.strip_prefix(&root).unwrap_or(&out.dest).display().to_string();

        if old.as_deref() == Some(new.as_str()) {
            eprintln!("    unchanged  {shown}");
            continue;
        }
        match out.name {
            n if n.ends_with(".jsonl") => {
                let d = diff::jsonl(old.as_deref().unwrap_or(""), &new);
                eprintln!(
                    "    updated    {shown}  (+{} added, -{} removed, ~{} changed)",
                    d.added.len(),
                    d.removed.len(),
                    d.changed.len()
                );
                list("added", &d.added);
                list("removed", &d.removed);
            }
            "set_ids.rs" => {
                eprintln!("    updated    {shown}");
                for (id, a, b) in diff::renamed(old.as_deref().unwrap_or(""), &new) {
                    eprintln!("      renamed {id}: {a} -> {b}   (hand code using the old name will break; see roadmap 1.4)");
                }
            }
            _ => eprintln!("    updated    {shown}"),
        }
        if !opts.dry_run {
            if let Some(dir) = out.dest.parent() {
                fs::create_dir_all(dir).map_err(|e| format!("creating {}: {e}", dir.display()))?;
            }
            fs::write(&out.dest, &new).map_err(|e| format!("writing {shown}: {e}"))?;
        }
    }

    if opts.skip_verify {
        return Ok(true);
    }
    Ok(run_step("compiler verify", &mut compiler(&["verify"], &opts.data)))
}

/// Print up to 20 ids per line group, then a count of the rest.
fn list(label: &str, ids: &[u32]) {
    if ids.is_empty() {
        return;
    }
    let shown: Vec<String> = ids.iter().take(20).map(u32::to_string).collect();
    let more = ids.len().saturating_sub(20);
    eprintln!("      {label}: {}{}", shown.join(", "), if more > 0 { format!(" … (+{more} more)") } else { String::new() });
}
