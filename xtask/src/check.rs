//! `cargo xtask check` — roadmap 0.6: one command, one exit code.

use crate::{Opts, cargo, compiler, run_step};

pub fn run(opts: &Opts) -> Result<bool, String> {
    let mut results: Vec<(&str, Option<bool>)> = Vec::new();

    let ok = run_step("engine tests", cargo().args(["test", "--package", "esosim"]));
    results.push(("engine tests (esosim)", Some(ok)));

    let ok = run_step("compiler tests", cargo().args(["test", "--package", "esosim-data-compiler"]));
    results.push(("compiler tests (esosim-data-compiler)", Some(ok)));

    if opts.skip_verify {
        results.push(("compiler verify", None));
    } else {
        let ok = run_step("compiler verify (against the extracted game data)", &mut compiler(&["verify"], &opts.data));
        results.push(("compiler verify", Some(ok)));
    }

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
