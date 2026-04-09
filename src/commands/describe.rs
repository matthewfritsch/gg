use anyhow::{Context, Result};

pub fn run(message: Option<&str>) -> Result<()> {
    let repo = crate::repo::open()?;

    let head = repo.head().context("no commits to describe")?;
    let head_commit = head.peel_to_commit()?;
    let short = &head_commit.id().to_string()[..8];

    let message = match message {
        Some(m) => m.to_string(),
        None => {
            let existing = head_commit.message().unwrap_or("");
            crate::editor::edit_message(existing)?
        }
    };

    head_commit.amend(Some("HEAD"), None, None, None, Some(&message), None)?;

    println!("Updated commit {short}: {message}");

    Ok(())
}
