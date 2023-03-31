//! Custom commands for cargo.
//!
//! See [cargo-xtask](https://github.com/matklad/cargo-xtask) for details.

use clap::{Parser, Subcommand};
use colored::*;
use eyre::{bail, eyre, Context};
use std::{
    env,
    path::Path,
    process::{Command, ExitCode},
};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Run `cargo embed` on a default or specified package
    #[clap(visible_alias = "e")]
    Embed {
        /// Path to the package
        ///
        /// If not provided a default specified in XTASK_EMBED_DEFAULT env variable will be used.
        #[arg(short, long)]
        pkg_name: Option<String>,
        /// Arguments for `cargo embed`
        args: Vec<String>,
    },
    /// `cd` and build each package
    #[clap(visible_alias = "b")]
    Build {
        /// Arguments for `cargo build`
        args: Vec<String>,
    },
    /// `cd` and test each package
    ///
    /// Packages can be excluded with XTASK_TEST_EXCLUDE environment variable.
    #[clap(visible_alias = "t")]
    Test {
        /// Arguments for `cargo test`
        args: Vec<String>,
    },
}

fn run() -> eyre::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Embed {
            pkg_name: pkg_path,
            args,
        } => {
            let default = match env::var("XTASK_EMBED_DEFAULT") {
                Ok(v) => Some(v),
                Err(env::VarError::NotPresent) => None,
                Err(e) => bail!(e),
            };
            let dir = pkg_path.or(default).ok_or(eyre!(
                "No pkg to run.\
                 Either pass the path to the crate with `-p <path>` option, \
                 or define XTASK_EMBED_DEFAULT env variable."
            ))?;
            Command::new("cargo")
                .arg("embed")
                .args(&args)
                .current_dir(&dir)
                .status()?;
        }
        Cmd::Build { args } => {
            let members = workspace_members()?;
            for member in members.iter().filter(|m| m != &"xtask") {
                let msg = format!(
                    "xtask: Running `cargo build{}` in `{member}` package",
                    format_cmd_args(&args)
                        .map(|s| " ".to_owned() + &s)
                        .unwrap_or_else(|| "".to_owned())
                )
                .blue()
                .bold();
                println!("{msg}");
                let status = Command::new("cargo")
                    .arg("build")
                    .args(&args)
                    .current_dir(member)
                    .status()?;
                if !status.success() {
                    bail!("`cargo build` failed for {member}");
                }
            }
        }
        Cmd::Test { args } => {
            let excluded = match env::var("XTASK_TEST_EXCLUDE") {
                Ok(v) => Some(v),
                Err(env::VarError::NotPresent) => None,
                Err(e) => bail!(e),
            };
            let excluded: Vec<_> = excluded
                .as_ref()
                .map(|s| s.split(',').collect())
                .unwrap_or_default();
            let members = workspace_members()?;
            let members = members.iter().filter(|m| !excluded.contains(&m.as_str()));
            for member in members {
                let msg = format!(
                    "xtask: Running `cargo test{}` in `{member}` package",
                    format_cmd_args(&args)
                        .map(|s| " ".to_owned() + &s)
                        .unwrap_or_else(|| "".to_owned())
                )
                .blue()
                .bold();
                println!("{msg}");
                let status = Command::new("cargo")
                    .arg("test")
                    .args(&args)
                    .current_dir(member)
                    .status()?;
                if !status.success() {
                    bail!("`cargo test` failed for {member}");
                }
            }
        }
    };
    Ok(())
}

fn workspace_members() -> eyre::Result<Vec<String>> {
    let dir = env::var("CARGO_WORKSPACE_DIR")
        .wrap_err("`CARGO_WORKSPACE_DIR` env variable is missing")?;
    let dir = Path::new(&dir);
    let path = dir.join("Cargo.toml");
    let manifest = cargo_toml::Manifest::from_path(&path)
        .wrap_err_with(|| format!("Failed to read top-level Cargo.toml: {path:?}"))?;
    let members = manifest
        .workspace
        .ok_or_else(|| eyre!("No `workspace` field in Cargo.toml"))?
        .members;
    Ok(members)
}

fn format_cmd_args(args: &[String]) -> Option<String> {
    if args.is_empty() {
        None
    } else {
        Some(args.join(" "))
    }
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{}{} {e}", "error".red().bold(), ":".bold());
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
