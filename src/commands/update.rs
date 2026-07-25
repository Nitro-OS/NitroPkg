use anyhow::{bail, Result};
use std::process::Command;

pub fn execute() -> Result<()> {
    println!("Updating package database...");

    let status = Command::new("sudo")
        .arg("pacman")
        .arg("-Sy")
        .status()?;

    if status.success() {
        println!("Package database updated.");
        Ok(())
    } else {
        bail!("pacman exited with status: {}", status);
    }
}
