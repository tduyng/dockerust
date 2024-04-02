use anyhow::Result;
use docker_starter_rust::run;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    run(&args)?;

    Ok(())
}
