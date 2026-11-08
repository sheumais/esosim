//! `cargo xtask check` — roadmap 0.6: one command, one exit code.

use crate::{Opts, compiler, layout, root, run_step};

/// The crates whose tests `check` runs, in dependency order.
const PACKAGES: &[&str] = &["esosim-types", "esosim-data", "esosim", "esosim-data-compiler"];

pub fn run(_opts: &Opts) -> Result<bool, String> {
    let mut results: Vec<(&str, Option<bool>)> = Vec::new();

    for &pkg in PACKAGES {
        let ok = run_step(&format!("{pkg} tests"), crate::cargo().args(["test", "--package", pkg]));
        results.push((pkg, Some(ok)));
    }

    // Roadmap 0.6: `verify` on the data that is checked in. It works on files, so it needs no game.
    // A bundle written before root categories existed only produces warnings about roots.toml;
    // run `cargo xtask sync-data` to get the categories.
    let data = root().join(layout::DATA_DIR);
    let dir = data.to_str().ok_or("data path is not valid UTF-8")?;
    let ok = run_step("compiler verify", &mut compiler(&["verify", "--dir", dir], &Default::default()));
    results.push(("compiler verify", Some(ok)));

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
