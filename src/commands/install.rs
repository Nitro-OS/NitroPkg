use anyhow::{Ok, Result, bail};
use std::process::Command;

pub fn execute(packages: Vec<String>) -> Result<()> {
    if packages.is_empty() {
        bail!("No package specified.")
    }

    println!("Installing: {}", packages.join(", "));

    let status = Command::new("sudo")
        .arg("pacman")
        .arg("-S")
        .args(&packages)
        .status()?;

    if status.success() {
        println!("Installation completed.");
        Ok(())
    } else {
        bail!("pacman exited with status: {}", status);
    }
}
