use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::io::{self, Write};
use std::process::{Command, Stdio};

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

// Usage: your_docker.sh run <image> <command> <arg1> <arg2> ...
fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Run { args } => {
            let command = &args[1];
            let command_args = &args[2..];
            let output = Command::new(command)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .args(command_args)
                .output()
                .with_context(|| {
                    format!(
                        "Tried to run '{}' with arguments {:?}",
                        command, command_args
                    )
                })?;
            if !output.status.success() {
                std::process::exit(1);
            }
            io::stdout().write_all(&output.stdout)?;
            io::stderr().write_all(&output.stderr)?;
        }
    }

    Ok(())
}
