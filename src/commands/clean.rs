use anyhow::{Ok, Result, bail};
use std::process::Command;

pub fn execute() -> Result<()> {
    println!("Cleaning package cache...");

    let status = Command::new("sudo").arg("paccache").arg("-r").status()?;

    if status.success() {
        println!("Package cache cleaned.");
        Ok(())
    } else {
        bail!("paccache exited with status: {}", status);
    }
}
