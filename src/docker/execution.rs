use anyhow::{Context, Result};
use std::io::{self, Write};
use std::process::{exit, Command};

use crate::{isolate_fs, isolate_process};


pub fn run(args: &[String]) -> Result<()> {
    let command = &args[1];
    let command_args = &args[2..];
    let tempdir = tempfile::tempdir()?;
    let root = tempdir.path();

    isolate_fs(root, command)?;
    isolate_process()?;

    let output = Command::new(command)
        .args(command_args)
        .output()
        .with_context(|| {
            format!(
                "Tried to run '{}' with arguments {:?}",
                command, command_args
            )
        })?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    if let Some(code) = output.status.code() {
        exit(code);
    }

    Ok(())
}
