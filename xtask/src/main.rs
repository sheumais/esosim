//! Repo automation (roadmap 0.6 and 1.3). Run with `cargo xtask <command>`.
//!
//!   cargo xtask check      tests of every crate (types, data, engine, compiler)   (roadmap 0.6)
//!   cargo xtask sync-data  run the compiler from eso.mnf -> crates/esosim-data, diff   (roadmap 1.3)
//!
//! Where generated files land is described by [`layout`] (`crates/esosim-data`, roadmap 1.2).

mod check;
mod diff;
mod layout;
mod sync;

use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

const USAGE: &str = "\
usage: cargo xtask <command> [options]

commands:
  check       run the tests of every crate; one exit code
  sync-data   run the compiler on the game manifest, write the generated files into
              crates/esosim-data, and print which ids were added / removed / changed

sync-data options:
  --mnf <file>        the game manifest (eso.mnf)                    [env ESO_MNF_PATH]
  --dry-run           stage and diff, but do not write into the repo
";

/// Compiler input flags (`--mnf <file>`), forwarded verbatim.
#[derive(Default, Clone)]
pub struct DataArgs(Vec<String>);

impl DataArgs {
    pub fn as_slice(&self) -> &[String] {
        &self.0
    }
}

#[derive(Default)]
pub struct Opts {
    pub data: DataArgs,
    pub dry_run: bool,
}

fn parse(args: impl Iterator<Item = String>) -> Result<(String, Opts), String> {
    let mut it = args;
    let mut cmd = String::new();
    let mut o = Opts::default();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--mnf" => {
                let v = it.next().ok_or_else(|| format!("{a} needs a value"))?;
                o.data.0.push(a);
                o.data.0.push(v);
            }
            "--dry-run" => o.dry_run = true,
            "-h" | "--help" => return Err(String::new()),
            s if s.starts_with('-') => return Err(format!("unknown option {s}")),
            _ if cmd.is_empty() => cmd = a,
            _ => return Err(format!("unexpected argument {a}")),
        }
    }
    if cmd.is_empty() {
        return Err(String::new());
    }
    Ok((cmd, o))
}

/// The workspace root: the directory that holds the root `Cargo.toml` (parent of `xtask/`).
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).parent().expect("xtask lives in the workspace root").to_path_buf()
}

pub fn cargo() -> Command {
    let mut c = Command::new(std::env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
    c.current_dir(root());
    c
}

/// `cargo run -p esosim-data-compiler -- <args> [--mnf <file>]` (release is not needed; data size is modest).
pub fn compiler(args: &[&str], data: &DataArgs) -> Command {
    let mut c = cargo();
    c.args(["run", "--quiet", "--package", "esosim-data-compiler", "--"]).args(args).args(data.as_slice());
    c
}

/// Run a command with inherited output, print it first, return whether it succeeded.
pub fn run_step(label: &str, cmd: &mut Command) -> bool {
    eprintln!("\n==> {label}");
    match cmd.status() {
        Ok(s) => s.success(),
        Err(e) => {
            eprintln!("    could not start: {e}");
            false
        }
    }
}

fn main() -> ExitCode {
    let (cmd, opts) = match parse(std::env::args().skip(1)) {
        Ok(v) => v,
        Err(msg) => {
            if msg.is_empty() {
                eprint!("{USAGE}");
            } else {
                eprintln!("error: {msg}\n\n{USAGE}");
            }
            return ExitCode::from(2);
        }
    };
    let result = match cmd.as_str() {
        "check" => check::run(&opts),
        "sync-data" => sync::run(&opts),
        other => Err(format!("unknown command '{other}'")),
    };
    match result {
        Ok(true) => ExitCode::SUCCESS,
        Ok(false) => ExitCode::FAILURE,
        Err(msg) => {
            eprintln!("error: {msg}");
            ExitCode::from(2)
        }
    }
}
