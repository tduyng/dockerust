use anyhow::Result;
use clap::Parser;
use dockerust::{run, Args, Commands};

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::Run { args } => {
            run(&args)?;
        }
    }

    Ok(())
}
