use anyhow::{Context, Result};
use colored::Colorize;
use crate::cli::BranchAction;

pub fn run(action: BranchAction, simple: bool) -> Result<()> {
    match action {
        BranchAction::List => list(simple),
        BranchAction::Create { name } => create(&name),
        BranchAction::Delete { name } => delete(&name),
        BranchAction::Set { name } => set(&name),
    }
}

fn list(simple: bool) -> Result<()> {
    let repo = crate::repo::open()?;

    let branches = repo.branches(Some(git2::BranchType::Local))?;
    let head = repo.head().ok();
    let head_name = head
        .as_ref()
        .and_then(|h| h.shorthand().map(String::from));

    for branch in branches {
        let (branch, _) = branch?;
        let name = branch.name()?.unwrap_or("???");
        let is_current = head_name.as_deref() == Some(name);

        if simple {
            let marker = if is_current { "* " } else { "  " };
            println!("{marker}{name}");
        } else if is_current {
            println!("{} {}", "●".green(), name.green().bold());
        } else {
            println!("  {name}");
        }
    }

    Ok(())
}

fn create(name: &str) -> Result<()> {
    let repo = crate::repo::open()?;
    let head = repo.head().context("no commits yet")?;
    let commit = head.peel_to_commit()?;
    repo.branch(name, &commit, false)?;
    let short = &commit.id().to_string()[..8];
    println!("Created branch {name} at {short}");
    Ok(())
}

fn delete(name: &str) -> Result<()> {
    let repo = crate::repo::open()?;
    let mut branch = repo
        .find_branch(name, git2::BranchType::Local)
        .with_context(|| format!("branch '{name}' not found"))?;
    branch.delete()?;
    println!("Deleted branch {name}");
    Ok(())
}

fn set(name: &str) -> Result<()> {
    let repo = crate::repo::open()?;
    let head = repo.head().context("no commits yet")?;
    let commit = head.peel_to_commit()?;
    repo.branch(name, &commit, true)?;
    let short = &commit.id().to_string()[..8];
    println!("Set branch {name} to {short}");
    Ok(())
}
