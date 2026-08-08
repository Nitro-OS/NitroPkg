use anyhow::{bail, Result};
use std::process::Command;

pub fn execute() -> Result<()> {
    let status = Command::new("pacman")
        .arg("-Qe")
        .status()?;

    if status.success() {
        Ok(())
    } else {
        bail!("pacman exited with status: {}", status)
    }
}
