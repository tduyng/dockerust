use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::fs::{copy, DirBuilder, File};
use std::io::{self, Write};
use std::os::unix::fs::chroot;
use std::path::Path;
use std::process::{exit, Command, ExitStatus};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run docker command
    Run {
        /// The arguments of executed commands
        args: Vec<String>,
    },
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Run { args } => {
            let status = run(&args)?;

            if let Some(code) = status.code() {
                exit(code);
            }
        }
    }

    Ok(())
}

fn run(args: &[String]) -> Result<ExitStatus> {
    let command = &args[1];
    let command_args = &args[2..];
    let tempdir = tempfile::tempdir()?;
    let root = tempdir.path();

    isolate_fs(root, command)?;
    unsafe { libc::unshare(libc::CLONE_NEWPID) };

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
    Ok(output.status)
}

fn isolate_fs(root: &Path, command: &str) -> Result<()> {
    let cmd_path = Path::new(command);
    let parent_path = cmd_path.parent().unwrap().to_str().unwrap();
    let parent_dir = &parent_path[1..];
    let cmd_name = cmd_path.file_name().unwrap();

    let abs_dir = root.join(parent_dir);
    let abs_path = abs_dir.join(cmd_name);
    let dev_dir = root.join("dev");
    let dev_null = dev_dir.join("null");

    mkdir(&dev_dir)?;
    mkdir(&abs_dir)?;
    File::create(dev_null)?;
    copy(cmd_path, abs_path)?;

    chroot(root)?;
    std::env::set_current_dir("/")?;

    Ok(())
}

fn mkdir(path: &Path) -> Result<()> {
    DirBuilder::new().recursive(true).create(path)?;
    Ok(())
}
