use anyhow::Result;
use std::fs::DirBuilder;
use std::path::Path;

pub fn mkdir(path: &Path) -> Result<()> {
    DirBuilder::new().recursive(true).create(path)?;
    Ok(())
}
