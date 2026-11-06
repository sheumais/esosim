//! `cargo xtask check` — roadmap 0.6: one command, one exit code.

use crate::{Opts, cargo, run_step};

/// The crates whose tests `check` runs, in dependency order.
const PACKAGES: &[&str] = &["esosim-types", "esosim-data", "esosim", "esosim-data-compiler"];

pub fn run(_opts: &Opts) -> Result<bool, String> {
    let mut results: Vec<(&str, Option<bool>)> = Vec::new();

    for &pkg in PACKAGES {
        let ok = run_step(&format!("{pkg} tests"), cargo().args(["test", "--package", pkg]));
        results.push((pkg, Some(ok)));
    }

    // Roadmap 0.6 also asks for `compiler verify`. The compiler has no such command at the
    // moment (verify.rs is not wired into lib.rs), so it is reported as skipped, not passed.
    results.push(("compiler verify (not available)", None));

    eprintln!("\n==> summary");
    for (name, r) in &results {
        let tag = match r {
            Some(true) => "ok     ",
            Some(false) => "FAILED ",
            None => "skipped",
        };
        eprintln!("    {tag} {name}");
    }
    Ok(results.iter().all(|(_, r)| *r != Some(false)))
}
