use anyhow::{Context, Result};
use git2::Repository;

pub fn run(revision: Option<&str>, message: Option<&str>) -> Result<()> {
    let repo = crate::repo::open()?;

    let parent_commit = if let Some(rev) = revision {
        resolve_revision(&repo, rev)?
    } else {
        repo.head()
            .context("no commits — run `gg commit` first")?
            .peel_to_commit()?
    };

    let tree = parent_commit.tree()?;
    let sig = repo.signature()?;
    let message = message.unwrap_or("(no description)");

    let oid = repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        message,
        &tree,
        &[&parent_commit],
    )?;

    // Reset the working directory to match the new commit
    let commit = repo.find_commit(oid)?;
    let obj = commit.as_object();
    repo.reset(obj, git2::ResetType::Mixed, None)?;

    let short = &oid.to_string()[..8];
    println!("Created new change {short}");

    Ok(())
}

fn resolve_revision<'a>(repo: &'a Repository, rev: &str) -> Result<git2::Commit<'a>> {
    // Try as a branch name first
    if let Ok(reference) = repo.find_branch(rev, git2::BranchType::Local)
        && let Some(target) = reference.get().target()
    {
        return Ok(repo.find_commit(target)?);
    }

    // Try as a commit hash prefix
    let obj = repo
        .revparse_single(rev)
        .with_context(|| format!("cannot resolve revision '{rev}'"))?;
    obj.peel_to_commit()
        .map_err(|e| anyhow::anyhow!("'{rev}' does not point to a commit: {e}"))
}
