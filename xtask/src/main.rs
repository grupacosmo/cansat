//! Custom commands for cargo.
//!
//! See [cargo-xtask](https://github.com/matklad/cargo-xtask) for details.

use clap::{Parser, Subcommand};
use colored::*;
use eyre::{bail, eyre, Context, ContextCompat};
use std::{
    env,
    ffi::OsString,
    path::{Path, PathBuf},
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
    /// `cd` into specified or default package and run `cargo embed`
    #[clap(visible_alias = "e")]
    Embed {
        /// Name of the package
        ///
        /// If not provided a default specified in XTASK_EMBED_DEFAULT env variable will be used.
        #[arg(short, long)]
        package: Option<OsString>,
        /// Arguments for `cargo embed`
        args: Vec<String>,
    },
    /// `cd` into each package and run `cargo build`
    #[clap(visible_alias = "b")]
    Build {
        /// Name of the package
        #[arg(short, long)]
        package: Option<OsString>,
        /// Arguments for `cargo build`
        args: Vec<String>,
    },
    /// `cd` into each package and run `cargo test`
    ///
    /// Packages can be excluded with XTASK_TEST_EXCLUDE environment variable.
    #[clap(visible_alias = "t")]
    Test {
        /// Name of the package
        #[arg(short, long)]
        package: Option<OsString>,
        /// Arguments for `cargo test`
        args: Vec<String>,
    },
}

fn run() -> eyre::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Embed { package, args } => {
            let members = workspace_members()?;

            let default = match env::var("XTASK_EMBED_DEFAULT") {
                Ok(v) => Some(v.into()),
                Err(env::VarError::NotPresent) => None,
                Err(e) => bail!(e),
            };

            let package = package.or(default).ok_or(eyre!(
                "No package to run.\
                    Either pass the name to the crate with `-p <package>` oprion, \
                    or define XTASK_EMBED_DEFAULT=<package> env variable."
            ))?;

            let path = members
                .iter()
                .find(|path| path.file_name().unwrap() == &package)
                .wrap_err("Thre is no such package")?;

            Command::new("cargo")
                .arg("embed")
                .args(&args)
                .current_dir(path)
                .status()?;
        }
        Cmd::Build { package, args } => {
            let members = workspace_members()?;

            let build = |member: &PathBuf| {
                let msg = format!(
                    "   xtask: Running `cargo build{}` in `{}`",
                    format_cmd_args(&args)
                        .map(|s| " ".to_owned() + &s)
                        .unwrap_or_else(|| "".to_owned()),
                    member.display()
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
                    bail!("`cargo build` failed for {}", member.display());
                }
                Ok(())
            };

            if let Some(package) = package {
                let member = members
                    .iter()
                    .find(|path| path.file_name().unwrap() == &package)
                    .wrap_err("Thre is no such package")?;
                build(member)?;
            } else {
                for member in members.iter().filter(|m| m != &Path::new("xtask")) {
                    build(member)?;
                }
            }
        }
        Cmd::Test { package, args } => {
            let members = workspace_members()?;

            let test = |member: &PathBuf| {
                let msg = format!(
                    "   xtask: Running `cargo test{}` in `{}`",
                    format_cmd_args(&args)
                        .map(|s| " ".to_owned() + &s)
                        .unwrap_or_else(|| "".to_owned()),
                    member.display()
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
                    bail!("`cargo test` failed for {}", member.display());
                }
                Ok(())
            };

            if let Some(package) = package {
                let member = members
                    .iter()
                    .find(|path| path.file_name().unwrap() == &package)
                    .wrap_err("Thre is no such package")?;
                test(member)?;
            } else {
                let excluded = match env::var("XTASK_TEST_EXCLUDE") {
                    Ok(v) => Some(v),
                    Err(env::VarError::NotPresent) => None,
                    Err(e) => bail!(e),
                };
                let excluded: Vec<OsString> = excluded
                    .as_ref()
                    .map(|s| s.split(',').map(|s| s.into()).collect())
                    .unwrap_or_default();
                let members = members
                    .iter()
                    .filter(|m| !excluded.contains(&m.file_name().unwrap().to_os_string()));

                for member in members {
                    test(member)?;
                }
            }
        }
    };
    Ok(())
}

fn workspace_members() -> eyre::Result<Vec<PathBuf>> {
    let dir = env::var("CARGO_WORKSPACE_DIR")
        .wrap_err("`CARGO_WORKSPACE_DIR` env variable is missing")?;
    let dir = Path::new(&dir);
    let path = dir.join("Cargo.toml");
    let manifest = cargo_toml::Manifest::from_path(&path)
        .wrap_err_with(|| format!("Failed to read top-level Cargo.toml: {path:?}"))?;
    let members = manifest
        .workspace
        .ok_or_else(|| eyre!("No `workspace` field in Cargo.toml"))?
        .members
        .iter()
        .map(|s| s.into())
        .collect();
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
