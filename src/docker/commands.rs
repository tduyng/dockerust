use anyhow::{Context, Result};
use std::io::{self, Write};
use std::process::{exit, Command};

use crate::{download_image, isolate_fs, isolate_process};

pub fn run(args: &[String]) -> Result<()> {
    let image = &args[1];
    let command = &args[2];
    let command_args = &args[3..];
    let tempdir = tempfile::tempdir()?;
    let root = tempdir.path();

    // Download and extract docker image
    download_image(image, root)?;

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
