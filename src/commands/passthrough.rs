use anyhow::Result;
use std::process::Command;

pub fn run(args: &[String]) -> Result<()> {
    let status = Command::new("git")
        .args(args)
        .status()?;

    if !status.success() {
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}
