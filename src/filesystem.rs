use anyhow::{Ok, Result};
use std::fs::{copy, File};
use std::os::unix::fs::chroot;
use std::path::Path;

use crate::utils::mkdir;

pub fn isolate_fs(root: &Path, command: &str) -> Result<()> {
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

pub fn isolate_process() -> Result<()> {
    unsafe { libc::unshare(libc::CLONE_NEWPID) };
    Ok(())
}
