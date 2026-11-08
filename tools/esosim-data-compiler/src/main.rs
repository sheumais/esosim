use std::path::PathBuf;
use std::process::ExitCode;

const DEFAULT_OUT_DIR: &str = "bundle";
const USAGE: &str = "usage: esosim-data-compiler [--out <dir>] [--mnf] <path to eso.mnf>";

fn parse() -> Option<(PathBuf, PathBuf)> {
    let mut mnf: Option<PathBuf> = None;
    let mut out = PathBuf::from(DEFAULT_OUT_DIR);
    let mut args = std::env::args_os().skip(1);
    while let Some(a) = args.next() {
        match a.to_str() {
            Some("--out") => out = args.next()?.into(),
            Some("--mnf") => mnf = Some(args.next()?.into()),
            _ if mnf.is_none() => mnf = Some(a.into()),
            _ => return None,
        }
    }
    Some((mnf?, out))
}

fn main() -> ExitCode {
    let Some((mnf, out_dir)) = parse() else {
        eprintln!("{USAGE}");
        return ExitCode::from(2);
    };
    let compiled = esosim_data_compiler::compile(&mnf).and_then(|b| {
        eprintln!("{}", b.behaviour_report);
        b.write_to(&out_dir)
    });
    match compiled {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}