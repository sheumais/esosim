use esosim_data_compiler::Options;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

const DEFAULT_OUT_DIR: &str = "bundle";

const USAGE: &str = "\
usage:
  esosim-data-compiler [compile] [--out <dir>] [--mnf] <eso.mnf> [inputs]
      read the game, write the generated files into <dir> (default `bundle`), then run `verify`
      on them; exits 1 when verify fails (the files are written either way, for inspection)
  esosim-data-compiler verify [--dir <dir>] [inputs]
      hold overrides.toml and roots.toml against <dir>/bundle.jsonl and <dir>/sets.jsonl;
      needs no game install
  esosim-data-compiler suggest-effects <category> [--dir <dir>] [inputs]
      print overrides.toml skeletons for the unreviewed roots of a category
      (skills, sets, cp, mundus, consumables, racial, buffs)

inputs (all commands):
  --roots <file>          roots.toml        (default: the one built into the compiler)
  --overrides <file>      overrides.toml    (default: the one built into the compiler)
  --hand-written <file>   an effects .jsonl written by hand elsewhere (the engine's core.jsonl,
                          legacy.jsonl); repeatable. Its ids are never generated, and an
                          override may not reuse them
";

#[derive(Default)]
struct Cli {
    command: String,
    category: Option<String>,
    mnf: Option<PathBuf>,
    out: Option<PathBuf>,
    dir: Option<PathBuf>,
    roots: Option<PathBuf>,
    overrides: Option<PathBuf>,
    hand_written: Vec<PathBuf>,
}

fn parse(args: impl Iterator<Item = String>) -> Result<Cli, String> {
    let mut cli = Cli::default();
    let mut args = args;
    let mut positional: Vec<String> = Vec::new();
    while let Some(a) = args.next() {
        let mut value = |flag: &str| args.next().ok_or_else(|| format!("{flag} needs a value"));
        match a.as_str() {
            "--out" => cli.out = Some(value("--out")?.into()),
            "--mnf" => cli.mnf = Some(value("--mnf")?.into()),
            "--dir" => cli.dir = Some(value("--dir")?.into()),
            "--roots" => cli.roots = Some(value("--roots")?.into()),
            "--overrides" => cli.overrides = Some(value("--overrides")?.into()),
            "--hand-written" => cli.hand_written.push(value("--hand-written")?.into()),
            "-h" | "--help" => return Err(String::new()),
            s if s.starts_with("--") => return Err(format!("unknown option {s}")),
            _ => positional.push(a.clone()),
        }
    }

    let mut positional = positional.into_iter();
    // `compile` is the default command, so a bare manifest path keeps working
    cli.command = match positional.next().as_deref() {
        Some(c @ ("compile" | "verify" | "suggest-effects")) => c.to_string(),
        Some(other) => {
            cli.mnf.get_or_insert_with(|| other.into());
            "compile".to_string()
        }
        None => "compile".to_string(),
    };
    match cli.command.as_str() {
        "suggest-effects" => cli.category = Some(positional.next().ok_or("suggest-effects needs a category")?),
        "compile" => {
            if let Some(p) = positional.next() {
                if cli.mnf.is_some() {
                    return Err(format!("unexpected argument {p}"));
                }
                cli.mnf = Some(p.into());
            }
        }
        _ => {}
    }
    if let Some(extra) = positional.next() {
        return Err(format!("unexpected argument {extra}"));
    }
    Ok(cli)
}

fn read(path: &Path) -> Result<String, String> {
    fs::read_to_string(path).map_err(|e| format!("reading {}: {e}", path.display()))
}

/// The hand-written inputs named on the command line, on top of the built-in defaults.
fn options(cli: &Cli) -> Result<Options, String> {
    let mut o = Options::default();
    if let Some(p) = &cli.roots {
        o.roots_toml = read(p)?;
    }
    if let Some(p) = &cli.overrides {
        o.overrides_toml = read(p)?;
    }
    for p in &cli.hand_written {
        let label = p.file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_else(|| p.display().to_string());
        o.hand_written.push((label, read(p)?));
    }
    Ok(o)
}

/// `<dir>/bundle.jsonl` and `<dir>/sets.jsonl`.
fn generated(cli: &Cli) -> Result<(String, String), String> {
    let dir = cli.dir.clone().unwrap_or_else(|| PathBuf::from(DEFAULT_OUT_DIR));
    Ok((read(&dir.join("bundle.jsonl"))?, read(&dir.join("sets.jsonl"))?))
}

/// Run a command; `Ok(false)` is "ran, and the verdict was no".
fn run(cli: &Cli) -> Result<bool, String> {
    let opts = options(cli)?;
    match cli.command.as_str() {
        "verify" => {
            let (bundle, sets) = generated(cli)?;
            let report = esosim_data_compiler::verify_files(&bundle, &sets, &opts);
            eprintln!("{}", report.summary());
            Ok(report.ok())
        }
        "suggest-effects" => {
            let (bundle, sets) = generated(cli)?;
            let category = cli.category.as_deref().unwrap_or_default();
            let (text, count) = esosim_data_compiler::suggest_effects(category, &bundle, &sets, &opts)?;
            print!("{text}");
            eprintln!("suggest-effects: {count} skeleton(s)");
            Ok(true)
        }
        _ => {
            let mnf = cli.mnf.as_deref().ok_or("no game manifest given (pass the path to eso.mnf)")?;
            let out_dir = cli.out.clone().unwrap_or_else(|| PathBuf::from(DEFAULT_OUT_DIR));
            let bundle = esosim_data_compiler::compile_with(mnf, &opts)?;
            eprintln!("{}\n{}", bundle.behaviour_report, bundle.roots_report);
            bundle.write_to(&out_dir)?;
            let report = bundle.verify(&opts);
            eprintln!("{}", report.summary());
            Ok(report.ok())
        }
    }
}

fn main() -> ExitCode {
    let cli = match parse(std::env::args().skip(1)) {
        Ok(c) => c,
        Err(msg) => {
            if msg.is_empty() {
                eprint!("{USAGE}");
            } else {
                eprintln!("error: {msg}\n\n{USAGE}");
            }
            return ExitCode::from(2);
        }
    };
    match run(&cli) {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(e) => {
            eprintln!("error: {e}");
            ExitCode::FAILURE
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cli(args: &[&str]) -> Result<Cli, String> {
        parse(args.iter().map(|s| s.to_string()))
    }

    #[test]
    fn a_bare_manifest_path_still_compiles() {
        let c = cli(&["eso.mnf"]).unwrap();
        assert_eq!((c.command.as_str(), c.mnf.as_deref()), ("compile", Some(Path::new("eso.mnf"))));
        let c = cli(&["--out", "o", "--mnf", "eso.mnf"]).unwrap();
        assert_eq!((c.command.as_str(), c.out.as_deref()), ("compile", Some(Path::new("o"))));
        assert_eq!(cli(&["compile", "eso.mnf"]).unwrap().mnf.as_deref(), Some(Path::new("eso.mnf")));
    }

    #[test]
    fn verify_and_suggest_take_their_arguments() {
        let c = cli(&["verify", "--dir", "d", "--hand-written", "a.jsonl", "--hand-written", "b.jsonl"]).unwrap();
        assert_eq!(c.command, "verify");
        assert_eq!(c.hand_written.len(), 2);
        let c = cli(&["suggest-effects", "sets", "--roots", "r.toml"]).unwrap();
        assert_eq!((c.category.as_deref(), c.roots.as_deref()), (Some("sets"), Some(Path::new("r.toml"))));
        assert!(cli(&["suggest-effects"]).is_err(), "a category is required");
        assert!(cli(&["verify", "extra"]).is_err());
        assert!(cli(&["--nope"]).is_err());
        assert!(cli(&["--dir"]).is_err(), "a flag needs a value");
    }
}
