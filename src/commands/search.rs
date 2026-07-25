use anyhow::{Ok, Result, bail};
use std::process::Command;

pub fn execute(query: String) -> Result<()> {
    if query.is_empty() {
        bail!("No package specified.")
    }

    println!("Searching for: {}", query);

    let status = Command::new("pacman").arg("-Ss").arg(&query).status()?;

    if status.success() {
        println!("Installation completed.");
        Ok(())
    } else {
        bail!("pacman exited with status: {}", status);
    }
}
