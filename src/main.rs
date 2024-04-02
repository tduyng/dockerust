use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::io::{self, Write};
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
