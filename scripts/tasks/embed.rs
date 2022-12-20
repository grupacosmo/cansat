#!/usr/bin/env rust-script
//! ```cargo
//! [dependencies]
//! anyhow = "*"
//! ```

use anyhow::Context;
use std::env;
use std::process::{Command, ExitCode, ExitStatus};

fn run() -> anyhow::Result<ExitStatus> {
    let mut args = env::args().skip(1);
    let dir = args.next().context("Missing binary crate path")?;
    env::set_current_dir(&dir).context("Binary crate path is invalid")?;
    Command::new("cargo")
        .arg("embed")
        .args(args)
        .status()
        .context("Failed to execute cargo embed")
}

fn main() -> ExitCode {
    if let Err(e) = run() {
        eprintln!("Error: {e:?}");
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
