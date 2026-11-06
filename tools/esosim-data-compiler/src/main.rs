//! `esosim-data-compiler <path to eso.mnf>`: writes `bundle.jsonl`, `sets.jsonl`, `set_ids.rs` and
//! `set_behaviour.jsonl` into `./bundle`, and prints the set behaviour summary.

use std::path::PathBuf;
use std::process::ExitCode;

const OUT_DIR: &str = "bundle";

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    // `--set-table <raw table file> <out csv>`: only `set_items.csv`, from an extracted set table.
    if args.len() == 3 && args.next().is_some_and(|a| a == "--set-table") {
        let (Some(src), Some(dst)) = (args.next(), args.next()) else { return ExitCode::from(2) };
        let result = std::fs::read(&src)
            .map_err(|e| format!("reading {src:?}: {e}"))
            .and_then(|t| esosim_data_compiler::set_items_from_table(&t))
            .and_then(|csv| std::fs::write(&dst, csv).map_err(|e| format!("writing {dst:?}: {e}")));
        return match result {
            Ok(()) => ExitCode::SUCCESS,
            Err(e) => {
                eprintln!("error: {e}");
                ExitCode::FAILURE
            }
        };
    }
    let (Some(mnf), None) = (args.next().map(PathBuf::from), args.next()) else {
        eprintln!("usage: esosim-data-compiler <path to eso.mnf>");
        return ExitCode::from(2);
    };
    let compiled = esosim_data_compiler::compile(&mnf).and_then(|b| {
        eprintln!("{}", b.behaviour_report);
        b.write_to(OUT_DIR.as_ref())
    });
    match compiled {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
