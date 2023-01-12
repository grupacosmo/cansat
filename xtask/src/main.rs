//! Custom commands for cargo.
//!
//! See [cargo-xtask](https://github.com/matklad/cargo-xtask) for details.

use clap::{Parser, Subcommand};
use colored::*;
use eyre::{bail, eyre};
use std::{
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
    /// Run `cargo embed` on a specified package
    Embed {
        /// Package name
        pkg_name: String,
        /// Arguments for `cargo embed`
        args: Vec<String>,
    },
    /// `cd` and build each package
    Build {
        /// Arguments for `cargo build`
        args: Vec<String>,
    },
    /// `cd` and test each package
    ///
    /// Packages can be excluded with XTASK_TEST_EXCLUDE environment variable.
    Test {
        /// Arguments for `cargo test`
        args: Vec<String>,
    },
}

fn run() -> eyre::Result<()> {
    let cli = Cli::parse();
    match cli.cmd {
        Cmd::Embed { pkg_name, args } => {
            Command::new("cargo")
                .arg("embed")
                .args(&args)
                .current_dir(&pkg_name)
                .status()?;
        }
        Cmd::Build { args } => {
            let members = workspace_members()?;
            for member in members.iter().filter(|m| m != &"xtask") {
                let msg = format!("Running `cargo run` for `{member}`").blue().bold();
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
            let excluded: Vec<_> = env!("XTASK_TEST_EXCLUDE").split(',').collect();
            let members = workspace_members()?;
            let members = members.iter().filter(|m| !excluded.contains(&m.as_str()));
            for member in members {
                let msg = format!("Running `cargo test` for `{member}`").blue().bold();
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
    let dir = Path::new(env!("CARGO_WORKSPACE_DIR"));
    let path = dir.join("Cargo.toml");
    let manifest = cargo_toml::Manifest::from_path(path)?;
    let members = manifest
        .workspace
        .ok_or_else(|| eyre!("No `workspace` field in Cargo.toml"))?
        .members;
    Ok(members)
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("{}{} {e}", "error".red().bold(), ":".bold());
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
