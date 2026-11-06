//! `esosim-data-compiler <path to eso.mnf>`: writes `bundle.jsonl`, `sets.jsonl` and
//! `set_ids.rs` into `./bundle`.

use std::path::PathBuf;
use std::process::ExitCode;

const OUT_DIR: &str = "bundle";

fn main() -> ExitCode {
    let mut args = std::env::args_os().skip(1);
    let (Some(mnf), None) = (args.next().map(PathBuf::from), args.next()) else {
        eprintln!("usage: esosim-data-compiler <path to eso.mnf>");
        return ExitCode::from(2);
    };
    match esosim_data_compiler::compile(&mnf).and_then(|b| b.write_to(OUT_DIR.as_ref())) {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}
