use anyhow::{bail, Result};
use std::process::Command;

pub fn execute(packages: Vec<String>) -> Result<()> {
    let mut command = Command::new("sudo");
    command.arg("pacman");

    if packages.is_empty() {
        println!("Upgrading entire system...");
        command.arg("-Syu");
    } else {
        println!("Upgrading: {}", packages.join(", "));
        command.arg("-S");
        command.args(&packages);
    }

    let status = command.status()?;

    if status.success() {
        println!("Upgrade completed.");
        Ok(())
    } else {
        bail!("pacman exited with status: {}", status);
    }
}
