use anyhow::Result;
use dockerust::run;

fn main() -> Result<()> {
    let args = std::env::args().collect::<Vec<String>>();
    let command = &args[1];
    match command.as_ref() {
        "run" => Ok(run(&args)?),
        _ => Ok(()),
    }
}
