use anyhow::{Context, Result};
use std::io::{self, Write};
use std::process::{exit, Command};

use crate::{download_image, isolate_fs, isolate_process};

pub fn run(args: &[String]) -> Result<()> {
    // Extract CLI arguments
    let image_and_version = &args[2];
    let cmd_bin = &args[3];
    let cmd_args = &args[4..];

    // Create a temporary directory to extract the image
    let tempdir = tempfile::tempdir()?;
    let root = tempdir.path();

    // Download and extract docker image
    download_image(image_and_version, root)?;

    isolate_fs(root, cmd_bin)?;
    isolate_process()?;

    let output = Command::new(cmd_bin)
        .args(cmd_args)
        .output()
        .with_context(|| format!("Tried to run '{}' with arguments {:?}", cmd_bin, cmd_args))?;

    io::stdout().write_all(&output.stdout)?;
    io::stderr().write_all(&output.stderr)?;

    if let Some(code) = output.status.code() {
        exit(code);
    }

    Ok(())
}
