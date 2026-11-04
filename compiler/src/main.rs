mod classify;
mod compile;
mod corpus;
mod graph;
mod probe;
mod resolved;
mod setdata;
mod verify;

use corpus::Corpus;
use graph::Graph;
use std::path::PathBuf;
use std::process::ExitCode;

const DEFAULT_SKILL_DIR: &str = r#"C:\Users\H\Documents\EsoExtractData v0.53\export\U52_pts\000\"#;
const DEFAULT_LANG_FILE: &str = r#"C:\Users\H\Documents\EsoExtractData v0.53\export\U52_pts\gamedata\lang\en.lang.csv"#;
const DEFAULT_SETS_FILE: &str = r#"C:\Users\H\Documents\EsoExtractData v0.53\export\U52_pts\000\1302240_Uncompressed.EsoFileData"#;

const USAGE: &str = "\
usage: esosim-data-compiler <command> [options]

commands:
  compile            resolve abilities and write bundle.jsonl + bundle.meta.json
  verify             check hand-typed ids/constants against the extracted data (non-zero exit on FAIL)
  major-minor        list every major_minor_id code with the names that use it (finds enum gaps)
  probe-edges        measure which u32 fields behave like ability ids (finds undiscovered edges)
  who-causes <id>    list every record that references <id>, and how
  show <id>          print one resolved ability as JSON (spot checks)
  classify           regex-classify every ability with a value/coef into an engine Channel from the
                     tooltips that reference it; print coverage, conflicts and the unmatched clauses
  classify-mapping   compare every <<n>> -> tooltip_data mapping hypothesis on the real data and dump
                     parents where placeholder count and entry count differ
  sets               read the set table; write <out>/sets.jsonl (one set per line, bonuses typed by
                     the engine's Channel, names from --lang) and <out>/set_ids.rs (id constants)

options:
  --skill-dir <dir>  directory holding *_Uncompressed.EsoFileData   [env ESO000_PATH]
  --lang <file>      en.lang.csv                                    [env ESO_LANG_PATH]
  --out <dir>        output directory for `compile` and `sets` (default: bundle)
  --sets-file <file> set table for `sets`                           [env ESO_SETS_PATH]
  --all              compile: emit every record instead of the root closure
  --limit <n>        classify: rows per section of the report (default 25)
";

struct Cli {
    command: String,
    args: Vec<String>,
    skill_dir: String,
    lang_file: String,
    sets_file: String,
    out: PathBuf,
    all: bool,
    limit: usize,
}

fn parse_args() -> Result<Cli, String> {
    let mut it = std::env::args().skip(1);
    let mut cli = Cli {
        command: String::new(),
        args: Vec::new(),
        skill_dir: std::env::var("ESO000_PATH").unwrap_or_else(|_| DEFAULT_SKILL_DIR.to_string()),
        lang_file: std::env::var("ESO_LANG_PATH").unwrap_or_else(|_| DEFAULT_LANG_FILE.to_string()),
        sets_file: std::env::var("ESO_SETS_PATH").unwrap_or_else(|_| DEFAULT_SETS_FILE.to_string()),
        out: PathBuf::from("bundle"),
        all: false,
        limit: 25,
    };
    while let Some(a) = it.next() {
        match a.as_str() {
            "--skill-dir" => cli.skill_dir = it.next().ok_or("--skill-dir needs a value")?,
            "--lang" => cli.lang_file = it.next().ok_or("--lang needs a value")?,
            "--sets-file" => cli.sets_file = it.next().ok_or("--sets-file needs a value")?,
            "--out" => cli.out = PathBuf::from(it.next().ok_or("--out needs a value")?),
            "--all" => cli.all = true,
            "--limit" => {
                cli.limit = it
                    .next()
                    .ok_or("--limit needs a value")?
                    .parse()
                    .map_err(|e| format!("bad --limit: {e}"))?
            }
            "-h" | "--help" => return Err(String::new()),
            s if s.starts_with("--") => return Err(format!("unknown option {s}")),
            _ => {
                if cli.command.is_empty() {
                    cli.command = a.clone();
                } else {
                    cli.args.push(a.clone());
                }
            }
        }
    }
    if cli.command.is_empty() {
        return Err(String::new());
    }
    Ok(cli)
}

fn id_arg(cli: &Cli) -> Result<u32, String> {
    cli.args
        .first()
        .ok_or_else(|| format!("`{}` needs an ability id", cli.command))?
        .parse::<u32>()
        .map_err(|e| format!("bad ability id: {e}"))
}

fn run() -> Result<ExitCode, String> {
    let cli = parse_args()?;
    // `sets` only needs the set table and the set names, so don't load the skill corpus.
    if cli.command == "sets" {
        let mut lang = corpus::load_lang(&cli.lang_file, &[corpus::LANG_SET_NAMES]);
        let names = lang.remove(&corpus::LANG_SET_NAMES).unwrap_or_default();
        setdata::run(&cli.sets_file, &names, &cli.out)?;
        return Ok(ExitCode::SUCCESS);
    }
    let corpus = Corpus::load(&cli.skill_dir, &cli.lang_file)?;
    let graph = Graph::build(&corpus);

    match cli.command.as_str() {
        "compile" => {
            let (abilities, stats) = compile::compile(&corpus, &graph, cli.all);
            compile::write_bundle(&cli.out, &abilities, &stats)
                .map_err(|e| format!("writing bundle to {:?}: {e}", cli.out))?;
            eprintln!(
                "Wrote {} abilities ({} roots of {} records{}) to {:?}",
                stats.emitted,
                stats.roots,
                stats.total_records,
                if cli.all { ", --all" } else { "" },
                cli.out
            );
            report_unknowns(&stats);
        }
        "verify" => {
            let report = verify::verify(&corpus, &graph);
            if report.fails > 0 {
                return Ok(ExitCode::FAILURE);
            }
        }
        "classify" => classify::report(&corpus, &graph, cli.limit),
        "classify-mapping" => classify::mapping_report(&corpus, &graph, cli.limit),
        "major-minor" => probe::major_minor_report(&corpus),
        "probe-edges" => probe::probe_edges(&corpus),
        "who-causes" => probe::who_causes(&corpus, &graph, id_arg(&cli)?),
        "show" => {
            let id = id_arg(&cli)?;
            let skill = corpus.skills.get(&id).ok_or_else(|| format!("no record with id {id}"))?;
            let resolved = compile::resolve(skill, &corpus, &graph);
            let json = serde_json::to_string_pretty(&resolved).map_err(|e| e.to_string())?;
            println!("{json}");
        }
        other => return Err(format!("unknown command '{other}'")),
    }
    Ok(ExitCode::SUCCESS)
}

fn report_unknowns(stats: &compile::CompileStats) {
    let mut any = false;
    let mut line = |label: &str, n: usize| {
        if n > 0 {
            any = true;
            eprintln!("  unrecognised {label}: {n} distinct value(s) — see bundle.meta.json");
        }
    };
    if !any {
        eprintln!("  every enum value was recognised");
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(msg) => {
            if msg.is_empty() {
                eprintln!("{USAGE}");
                return ExitCode::from(2);
            }
            eprintln!("error: {msg}\n\n{USAGE}");
            ExitCode::from(2)
        }
    }
}